// Copyright 2024. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::path::PathBuf;

use anyhow::Error;
use async_trait::async_trait;
use log::{error, info};
use regex::Regex;
use tari_common::configuration::Network;

use crate::{
    download_utils::download_file_with_retries, github, progress_tracker_old::ProgressTracker,
    APPLICATION_FOLDER_ID,
};

use super::binaries_resolver::{LatestVersionApiAdapter, VersionAsset, VersionDownloadInfo};

pub const LOG_TARGET: &str = "tari::universe::adapter_github";

pub struct GithubReleasesAdapter {
    pub repo: String,
    pub owner: String,
    pub specific_name: Option<Regex>,
}

#[async_trait]
impl LatestVersionApiAdapter for GithubReleasesAdapter {
    async fn fetch_releases_list(&self) -> Result<Vec<VersionDownloadInfo>, Error> {
        let releases = github::list_releases(&self.owner, &self.repo).await?;
        Ok(releases.clone())
    }

    async fn download_and_get_checksum_path(
        &self,
        directory: PathBuf,
        download_info: VersionDownloadInfo,
        progress_tracker: ProgressTracker,
    ) -> Result<PathBuf, Error> {
        let asset = self.find_version_for_platform(&download_info)?;
        let checksum_path = directory
            .join("in_progress")
            .join(format!("{}.sha256", asset.name));
        let checksum_url = format!("{}.sha256", asset.url);

        match download_file_with_retries(&checksum_url, &checksum_path, progress_tracker).await {
            Ok(_) => Ok(checksum_path),
            Err(e) => {
                error!(target: LOG_TARGET, "Failed to download checksum file: {}", e);
                Err(e)
            }
        }
    }

    fn get_binary_folder(&self) -> Result<PathBuf, Error> {
        let cache_path =
            dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Failed to get cache directory"))?;

        let binary_folder_path = cache_path
            .join(APPLICATION_FOLDER_ID)
            .join("binaries")
            .join(&self.repo)
            .join(
                Network::get_current_or_user_setting_or_default()
                    .to_string()
                    .to_lowercase(),
            );

        if !binary_folder_path.exists() {
            std::fs::create_dir_all(&binary_folder_path).unwrap_or_else(|e| {
                error!(target: LOG_TARGET, "Failed to create directory: {}", e);
            });
        };

        Ok(binary_folder_path)
    }

    fn find_version_for_platform(
        &self,
        version: &VersionDownloadInfo,
    ) -> Result<VersionAsset, Error> {
        let mut name_suffix = "";
        // TODO: add platform specific logic
        if cfg!(target_os = "windows") {
            name_suffix = r"windows-x64.*\.zip";
        }

        if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
            name_suffix = r"macos-x86_64.*\.zip";
        }

        if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
            name_suffix = r"macos-arm64.*\.zip";
        }
        if cfg!(target_os = "linux") {
            name_suffix = r"linux-x86_64.*\.zip";
        }
        if name_suffix.is_empty() {
            panic!("Unsupported OS");
        }

        info!(target: LOG_TARGET, "Looking for platform with suffix: {}", name_suffix);

        let name_sufix_regex = Regex::new(name_suffix)
            .map_err(|error| anyhow::anyhow!("Failed to create regex: {}", error))?;

        let platform = version
            .assets
            .iter()
            .find(|a| {
                if let Some(ref specific) = self.specific_name {
                    specific.is_match(&a.name) && name_sufix_regex.is_match(&a.name)
                } else {
                    name_sufix_regex.is_match(&a.name)
                }
            })
            .ok_or(anyhow::anyhow!("Failed to get platform asset"))?;
        info!(target: LOG_TARGET, "Found platform: {:?}", platform);
        Ok(platform.clone())
    }
}

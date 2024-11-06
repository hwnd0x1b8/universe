use std::{path::PathBuf, time::SystemTime};
use sys_locale::get_locale;

use anyhow::anyhow;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use tari_common::configuration::Network;
use tokio::fs;

use crate::{consts::DEFAULT_MONERO_ADDRESS, internal_wallet::generate_password};

const LOG_TARGET: &str = "tari::universe::app_config";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct AppConfigFromFile {
    #[serde(default = "default_version")]
    version: u32,
    #[serde(default = "default_mode")]
    mode: String,
    #[serde(default = "default_display_mode")]
    display_mode: String,
    #[serde(default = "default_true")]
    auto_mining: bool,
    #[serde(default = "default_true")]
    mine_on_app_start: bool,
    #[serde(default = "default_true")]
    p2pool_enabled: bool,
    #[serde(default = "default_system_time")]
    last_binaries_update_timestamp: SystemTime,
    #[serde(default = "default_false")]
    allow_telemetry: bool,
    #[serde(default = "default_anon_id")]
    anon_id: String,
    #[serde(default = "default_monero_address")]
    monero_address: String,
    #[serde(default = "default_true")]
    gpu_mining_enabled: bool,
    #[serde(default = "default_true")]
    cpu_mining_enabled: bool,
    #[serde(default = "default_false")]
    has_system_language_been_proposed: bool,
    #[serde(default = "default_false")]
    should_always_use_system_language: bool,
    #[serde(default = "default_false")]
    should_auto_launch: bool,
    #[serde(default = "default_application_language")]
    application_language: String,
    #[serde(default = "default_true")]
    airdrop_ui_enabled: bool,
    #[serde(default = "default_true")]
    use_tor: bool,
    #[serde(default = "default_false")]
    paper_wallet_enabled: bool,
    #[serde(default = "default_false")]
    reset_earnings: bool,
    eco_mode_cpu_threads: Option<isize>,
    ludicrous_mode_cpu_threads: Option<isize>,
    eco_mode_cpu_options: Vec<String>,
    ludicrous_mode_cpu_options: Vec<String>,
    custom_mode_cpu_options: Vec<String>,
    #[serde(default = "default_false")]
    mmproxy_use_monero_fail: bool,
    #[serde(default = "default_monero_nodes")]
    mmproxy_monero_nodes: Vec<String>,
    #[serde(default = "default_custom_max_cpu_usage")]
    custom_max_cpu_usage: Option<isize>,
    #[serde(default = "default_custom_max_gpu_usage")]
    custom_max_gpu_usage: Option<isize>,
    #[serde(default = "default_true")]
    auto_update: bool,
    #[serde(default = "default_false")]
    custom_power_levels_enabled: bool,
    #[serde(default = "default_false")]
    sharing_enabled: bool,
}

impl Default for AppConfigFromFile {
    fn default() -> Self {
        Self {
            version: default_version(),
            mode: default_mode(),
            display_mode: default_display_mode(),
            auto_mining: true,
            mine_on_app_start: true,
            p2pool_enabled: true,
            last_binaries_update_timestamp: default_system_time(),
            allow_telemetry: false,
            anon_id: default_anon_id(),
            monero_address: default_monero_address(),
            gpu_mining_enabled: true,
            cpu_mining_enabled: true,
            has_system_language_been_proposed: false,
            should_always_use_system_language: false,
            should_auto_launch: false,
            application_language: default_application_language(),
            custom_max_cpu_usage: None,
            custom_max_gpu_usage: None,
            airdrop_ui_enabled: true,
            paper_wallet_enabled: false,
            use_tor: true,
            eco_mode_cpu_options: Vec::new(),
            ludicrous_mode_cpu_options: Vec::new(),
            custom_mode_cpu_options: Vec::new(),
            eco_mode_cpu_threads: None,
            ludicrous_mode_cpu_threads: None,
            mmproxy_monero_nodes: vec!["https://xmr-01.tari.com".to_string()],
            mmproxy_use_monero_fail: false,
            auto_update: true,
            reset_earnings: false,
            custom_power_levels_enabled: true,
            sharing_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DisplayMode {
    System,
    Dark,
    Light,
}

impl DisplayMode {
    pub fn from_str(s: &str) -> Option<DisplayMode> {
        match s {
            "system" => Some(DisplayMode::System),
            "dark" => Some(DisplayMode::Dark),
            "light" => Some(DisplayMode::Light),
            _ => None,
        }
    }

    pub fn to_str(t: DisplayMode) -> String {
        match t {
            DisplayMode::System => String::from("system"),
            DisplayMode::Dark => String::from("dark"),
            DisplayMode::Light => String::from("light"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MiningMode {
    Eco,
    Ludicrous,
    Custom,
}

impl MiningMode {
    pub fn from_str(s: &str) -> Option<MiningMode> {
        match s {
            "Eco" => Some(MiningMode::Eco),
            "Ludicrous" => Some(MiningMode::Ludicrous),
            "Custom" => Some(MiningMode::Custom),
            _ => None,
        }
    }

    pub fn to_str(m: MiningMode) -> String {
        match m {
            MiningMode::Eco => String::from("Eco"),
            MiningMode::Ludicrous => String::from("Ludicrous"),
            MiningMode::Custom => String::from("Custom"),
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AppConfig {
    config_version: u32,
    config_file: Option<PathBuf>,
    mode: MiningMode,
    display_mode: DisplayMode,
    auto_mining: bool,
    mine_on_app_start: bool,
    p2pool_enabled: bool,
    last_binaries_update_timestamp: SystemTime,
    allow_telemetry: bool,
    anon_id: String,
    monero_address: String,
    gpu_mining_enabled: bool,
    cpu_mining_enabled: bool,
    has_system_language_been_proposed: bool,
    should_always_use_system_language: bool,
    should_auto_launch: bool,
    application_language: String,
    airdrop_ui_enabled: bool,
    paper_wallet_enabled: bool,
    use_tor: bool,
    reset_earnings: bool,
    eco_mode_cpu_threads: Option<isize>,
    ludicrous_mode_cpu_threads: Option<isize>,
    eco_mode_cpu_options: Vec<String>,
    ludicrous_mode_cpu_options: Vec<String>,
    custom_mode_cpu_options: Vec<String>,
    mmproxy_use_monero_fail: bool,
    mmproxy_monero_nodes: Vec<String>,
    custom_max_cpu_usage: Option<isize>,
    custom_max_gpu_usage: Option<isize>,
    auto_update: bool,
    custom_power_levels_enabled: bool,
    sharing_enabled: bool,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            config_version: default_version(),
            config_file: None,
            mode: MiningMode::Eco,
            display_mode: DisplayMode::Light,
            auto_mining: true,
            mine_on_app_start: true,
            p2pool_enabled: true,
            last_binaries_update_timestamp: default_system_time(),
            allow_telemetry: true,
            anon_id: generate_password(20),
            monero_address: DEFAULT_MONERO_ADDRESS.to_string(),
            gpu_mining_enabled: true,
            cpu_mining_enabled: true,
            has_system_language_been_proposed: false,
            should_always_use_system_language: false,
            should_auto_launch: false,
            application_language: default_application_language(),
            airdrop_ui_enabled: true,
            use_tor: true,
            custom_max_cpu_usage: None,
            custom_max_gpu_usage: None,
            paper_wallet_enabled: false,
            reset_earnings: false,
            eco_mode_cpu_options: Vec::new(),
            ludicrous_mode_cpu_options: Vec::new(),
            custom_mode_cpu_options: Vec::new(),
            eco_mode_cpu_threads: None,
            ludicrous_mode_cpu_threads: None,
            mmproxy_use_monero_fail: false,
            mmproxy_monero_nodes: vec!["https://xmr-01.tari.com".to_string()],
            custom_power_levels_enabled: true,
            auto_update: true,
            sharing_enabled: true,
        }
    }

    pub async fn load_or_create(&mut self, config_path: PathBuf) -> Result<(), anyhow::Error> {
        let file: PathBuf = config_path.join("app_config.json");
        self.config_file = Some(file.clone());

        if file.exists() {
            debug!(target: LOG_TARGET, "Loading app config from file: {:?}", file);
            let config = fs::read_to_string(&file).await?;
            self.apply_loaded_config(config);
        } else {
            info!(target: LOG_TARGET, "App config does not exist or is corrupt. Creating new one");
        }
        self.update_config_file().await?;
        Ok(())
    }

    pub fn apply_loaded_config(&mut self, config: String) {
        match serde_json::from_str::<AppConfigFromFile>(&config) {
            Ok(config) => {
                debug!("Loaded config from file {:?}", config);
                self.config_version = config.version;
                self.mode = MiningMode::from_str(&config.mode).unwrap_or(MiningMode::Eco);
                if Network::get_current_or_user_setting_or_default() == Network::Esmeralda {
                    self.display_mode =
                        DisplayMode::from_str(&config.display_mode).unwrap_or(DisplayMode::Light);
                } else {
                    self.display_mode = DisplayMode::Light;
                }
                self.auto_mining = config.auto_mining;
                self.mine_on_app_start = config.mine_on_app_start;
                self.p2pool_enabled = config.p2pool_enabled;
                self.last_binaries_update_timestamp = config.last_binaries_update_timestamp;
                self.allow_telemetry = config.allow_telemetry;
                self.anon_id = config.anon_id;
                self.monero_address = config.monero_address;
                self.gpu_mining_enabled = config.gpu_mining_enabled;
                self.cpu_mining_enabled = config.cpu_mining_enabled;
                self.has_system_language_been_proposed = config.has_system_language_been_proposed;
                self.should_always_use_system_language = config.should_always_use_system_language;
                self.should_auto_launch = config.should_auto_launch;
                self.application_language = config.application_language;
                self.airdrop_ui_enabled = config.airdrop_ui_enabled;
                self.use_tor = config.use_tor;
                self.paper_wallet_enabled = config.paper_wallet_enabled;
                self.eco_mode_cpu_options = config.eco_mode_cpu_options;
                self.eco_mode_cpu_threads = config.eco_mode_cpu_threads;
                self.ludicrous_mode_cpu_options = config.ludicrous_mode_cpu_options;
                self.ludicrous_mode_cpu_threads = config.ludicrous_mode_cpu_threads;
                self.custom_mode_cpu_options = config.custom_mode_cpu_options;
                self.mmproxy_monero_nodes = config.mmproxy_monero_nodes;
                self.mmproxy_use_monero_fail = config.mmproxy_use_monero_fail;
                self.custom_max_cpu_usage = config.custom_max_cpu_usage;
                self.custom_max_gpu_usage = config.custom_max_gpu_usage;
                self.auto_update = config.auto_update;
                self.reset_earnings = config.reset_earnings;
                self.custom_power_levels_enabled = config.custom_power_levels_enabled;
                if Network::get_current_or_user_setting_or_default() == Network::Esmeralda {
                    self.reset_earnings = config.reset_earnings;
                } else {
                    self.reset_earnings = false;
                }
                self.sharing_enabled = config.sharing_enabled;
            }
            Err(e) => {
                warn!(target: LOG_TARGET, "Failed to parse app config: {}", e.to_string());
            }
        }

        // Migrate
        if self.config_version <= 6 {
            // Change the default value of p2pool_enabled to false in version 7
            self.config_version = 7;
            self.p2pool_enabled = true;
        }
        if self.config_version <= 7 {
            self.config_version = 8;
            self.airdrop_ui_enabled = true;
        }
        if self.config_version <= 8 {
            self.config_version = 9;
            self.mine_on_app_start = true;
        }

        if self.config_version <= 9 {
            self.auto_update = true;
            self.config_version = 10;
        }
    }

    pub fn mmproxy_monero_nodes(&self) -> &Vec<String> {
        &self.mmproxy_monero_nodes
    }

    pub fn mmproxy_use_monero_fail(&self) -> bool {
        self.mmproxy_use_monero_fail
    }

    pub fn eco_mode_cpu_options(&self) -> &Vec<String> {
        &self.eco_mode_cpu_options
    }

    pub fn ludicrous_mode_cpu_options(&self) -> &Vec<String> {
        &self.ludicrous_mode_cpu_options
    }

    pub fn custom_mode_cpu_options(&self) -> &Vec<String> {
        &self.custom_mode_cpu_options
    }

    pub fn eco_mode_cpu_threads(&self) -> Option<isize> {
        self.eco_mode_cpu_threads
    }

    pub fn ludicrous_mode_cpu_threads(&self) -> Option<isize> {
        self.ludicrous_mode_cpu_threads
    }

    pub fn anon_id(&self) -> &str {
        &self.anon_id
    }

    pub async fn set_mode(
        &mut self,
        mode: String,
        custom_max_cpu_usage: Option<isize>,
        custom_max_gpu_usage: Option<isize>,
    ) -> Result<(), anyhow::Error> {
        let new_mode = match mode.as_str() {
            "Eco" => MiningMode::Eco,
            "Ludicrous" => MiningMode::Ludicrous,
            "Custom" => MiningMode::Custom,
            _ => return Err(anyhow!("Invalid mode")),
        };
        self.mode = new_mode;
        self.update_config_file().await?;
        if let Some(custom_max_cpu_usage) = custom_max_cpu_usage {
            self.set_max_cpu_usage(custom_max_cpu_usage).await?;
        }
        if let Some(custom_max_gpu_usage) = custom_max_gpu_usage {
            self.set_max_gpu_usage(custom_max_gpu_usage).await?;
        }
        Ok(())
    }
    pub async fn set_display_mode(&mut self, display_mode: String) -> Result<(), anyhow::Error> {
        let new_display_mode = match display_mode.as_str() {
            "system" => DisplayMode::System,
            "dark" => DisplayMode::Dark,
            "light" => DisplayMode::Light,
            _ => return Err(anyhow!("Invalid display_mode")),
        };
        self.display_mode = new_display_mode;
        self.update_config_file().await?;
        Ok(())
    }

    pub fn mode(&self) -> MiningMode {
        self.mode
    }

    pub fn custom_gpu_usage(&self) -> Option<isize> {
        self.custom_max_cpu_usage
    }

    pub async fn set_max_gpu_usage(
        &mut self,
        custom_max_gpu_usage: isize,
    ) -> Result<(), anyhow::Error> {
        self.custom_max_gpu_usage = Some(custom_max_gpu_usage);
        self.update_config_file().await?;
        Ok(())
    }

    pub fn custom_cpu_usage(&self) -> Option<isize> {
        self.custom_max_cpu_usage
    }

    pub async fn set_max_cpu_usage(
        &mut self,
        custom_max_cpu_usage: isize,
    ) -> Result<(), anyhow::Error> {
        self.custom_max_cpu_usage = Some(custom_max_cpu_usage);
        self.update_config_file().await?;
        Ok(())
    }

    pub async fn set_cpu_mining_enabled(&mut self, enabled: bool) -> Result<bool, anyhow::Error> {
        self.cpu_mining_enabled = enabled;
        self.update_config_file().await?;
        Ok(self.cpu_mining_enabled)
    }

    pub async fn set_gpu_mining_enabled(&mut self, enabled: bool) -> Result<bool, anyhow::Error> {
        self.gpu_mining_enabled = enabled;
        self.update_config_file().await?;
        Ok(self.gpu_mining_enabled)
    }

    pub fn cpu_mining_enabled(&self) -> bool {
        self.cpu_mining_enabled
    }

    pub fn gpu_mining_enabled(&self) -> bool {
        self.gpu_mining_enabled
    }

    pub fn p2pool_enabled(&self) -> bool {
        self.p2pool_enabled
    }

    pub async fn set_p2pool_enabled(&mut self, p2pool_enabled: bool) -> Result<(), anyhow::Error> {
        self.p2pool_enabled = p2pool_enabled;
        self.update_config_file().await?;
        Ok(())
    }

    pub fn auto_mining(&self) -> bool {
        self.auto_mining
    }

    // pub async fn set_airdrop_ui_enabled(&mut self, airdrop_ui_enabled: bool) -> Result<(), anyhow::Error> {
    //     self.airdrop_ui_enabled = airdrop_ui_enabled;
    //     self.update_config_file().await?;
    //     Ok(())
    // }

    pub fn should_auto_launch(&self) -> bool {
        self.should_auto_launch
    }

    pub async fn set_should_auto_launch(
        &mut self,
        should_auto_launch: bool,
    ) -> Result<(), anyhow::Error> {
        self.should_auto_launch = should_auto_launch;
        self.update_config_file().await?;
        Ok(())
    }

    pub async fn set_mine_on_app_start(
        &mut self,
        mine_on_app_start: bool,
    ) -> Result<(), anyhow::Error> {
        self.mine_on_app_start = mine_on_app_start;
        self.update_config_file().await?;
        Ok(())
    }

    pub async fn set_allow_telemetry(
        &mut self,
        allow_telemetry: bool,
    ) -> Result<(), anyhow::Error> {
        self.allow_telemetry = allow_telemetry;
        self.update_config_file().await?;
        Ok(())
    }

    pub fn allow_telemetry(&self) -> bool {
        self.allow_telemetry
    }
    pub fn monero_address(&self) -> &str {
        &self.monero_address
    }

    pub async fn set_monero_address(&mut self, address: String) -> Result<(), anyhow::Error> {
        self.monero_address = address;
        self.update_config_file().await?;
        Ok(())
    }

    pub fn last_binaries_update_timestamp(&self) -> SystemTime {
        self.last_binaries_update_timestamp
    }

    pub async fn set_last_binaries_update_timestamp(
        &mut self,
        timestamp: SystemTime,
    ) -> Result<(), anyhow::Error> {
        self.last_binaries_update_timestamp = timestamp;
        self.update_config_file().await?;
        Ok(())
    }

    pub fn application_language(&self) -> &str {
        &self.application_language
    }

    pub async fn set_application_language(
        &mut self,
        language: String,
    ) -> Result<(), anyhow::Error> {
        self.application_language = language;
        self.update_config_file().await?;
        Ok(())
    }

    pub async fn set_should_always_use_system_language(
        &mut self,
        should_always_use_system_language: bool,
    ) -> Result<(), anyhow::Error> {
        self.should_always_use_system_language = should_always_use_system_language;
        self.update_config_file().await?;
        Ok(())
    }

    pub async fn propose_system_language(&mut self) -> Result<(), anyhow::Error> {
        if self.has_system_language_been_proposed | !self.should_always_use_system_language {
            Ok(())
        } else {
            let system_language = get_locale().unwrap_or_else(|| String::from("en-US"));
            info!(target: LOG_TARGET, "Proposing system language: {}", system_language);
            self.application_language = system_language;
            self.has_system_language_been_proposed = true;
            self.update_config_file().await?;
            Ok(())
        }
    }

    pub fn use_tor(&self) -> bool {
        self.use_tor
    }

    pub async fn set_use_tor(&mut self, use_tor: bool) -> Result<(), anyhow::Error> {
        self.use_tor = use_tor;
        self.update_config_file().await?;
        Ok(())
    }

    pub async fn set_auto_update(&mut self, auto_update: bool) -> Result<(), anyhow::Error> {
        self.auto_update = auto_update;
        self.update_config_file().await?;
        Ok(())
    }

    pub async fn set_monerod_config(
        &mut self,
        use_monero_fail: bool,
        monero_nodes: Vec<String>,
    ) -> Result<(), anyhow::Error> {
        self.mmproxy_use_monero_fail = use_monero_fail;
        self.mmproxy_monero_nodes = monero_nodes;
        self.update_config_file().await?;
        Ok(())
    }

    // Allow needless update because in future there may be fields that are
    // missing
    #[allow(clippy::needless_update)]
    pub async fn update_config_file(&mut self) -> Result<(), anyhow::Error> {
        let file = self
            .config_file
            .clone()
            .ok_or_else(|| anyhow!("Config file not set"))?;

        let config = &AppConfigFromFile {
            version: self.config_version,
            mode: MiningMode::to_str(self.mode),
            display_mode: DisplayMode::to_str(self.display_mode),
            auto_mining: self.auto_mining,
            mine_on_app_start: self.mine_on_app_start,
            p2pool_enabled: self.p2pool_enabled,
            last_binaries_update_timestamp: self.last_binaries_update_timestamp,
            allow_telemetry: self.allow_telemetry,
            anon_id: self.anon_id.clone(),
            monero_address: self.monero_address.clone(),
            gpu_mining_enabled: self.gpu_mining_enabled,
            cpu_mining_enabled: self.cpu_mining_enabled,
            has_system_language_been_proposed: self.has_system_language_been_proposed,
            should_always_use_system_language: self.should_always_use_system_language,
            should_auto_launch: self.should_auto_launch,
            application_language: self.application_language.clone(),
            airdrop_ui_enabled: self.airdrop_ui_enabled,
            paper_wallet_enabled: self.paper_wallet_enabled,
            custom_max_cpu_usage: self.custom_max_cpu_usage,
            custom_max_gpu_usage: self.custom_max_gpu_usage,
            use_tor: self.use_tor,
            reset_earnings: self.reset_earnings,
            eco_mode_cpu_options: self.eco_mode_cpu_options.clone(),
            ludicrous_mode_cpu_options: self.ludicrous_mode_cpu_options.clone(),
            custom_mode_cpu_options: self.custom_mode_cpu_options.clone(),
            eco_mode_cpu_threads: self.eco_mode_cpu_threads,
            ludicrous_mode_cpu_threads: self.ludicrous_mode_cpu_threads,
            mmproxy_monero_nodes: self.mmproxy_monero_nodes.clone(),
            mmproxy_use_monero_fail: self.mmproxy_use_monero_fail,
            auto_update: self.auto_update,
            custom_power_levels_enabled: self.custom_power_levels_enabled,
            sharing_enabled: self.sharing_enabled,
        };
        let config = serde_json::to_string(config)?;
        debug!(target: LOG_TARGET, "Updating config file: {:?} {:?}", file, self.clone());
        fs::write(file, config).await?;

        Ok(())
    }
}

fn default_version() -> u32 {
    10
}

fn default_custom_max_cpu_usage() -> Option<isize> {
    None
}

fn default_custom_max_gpu_usage() -> Option<isize> {
    None
}

fn default_mode() -> String {
    "Eco".to_string()
}

fn default_display_mode() -> String {
    "light".to_string()
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_anon_id() -> String {
    generate_password(20)
}

fn default_system_time() -> SystemTime {
    SystemTime::UNIX_EPOCH
}

fn default_monero_address() -> String {
    DEFAULT_MONERO_ADDRESS.to_string()
}

fn default_application_language() -> String {
    "en".to_string()
}

fn default_monero_nodes() -> Vec<String> {
    vec!["https://xmr-01.tari.com".to_string()]
}

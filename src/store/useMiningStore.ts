import { MinerMetrics } from '@app/types/app-status';
import { create } from './create';

import { invoke } from '@tauri-apps/api';
import { useAppStateStore } from './appStateStore';
import { useAppConfigStore } from './useAppConfigStore';
import { modeType } from './types';

import { useBlockchainVisualisationStore } from './useBlockchainVisualisationStore';
import * as Sentry from '@sentry/react';

interface State extends MinerMetrics {
    hashrateReady?: boolean;
    miningInitiated: boolean;
    miningControlsEnabled: boolean;
    isChangingMode: boolean;
    excludedGpuDevices: number[];
    counter: number;
}

interface Actions {
    setMiningMetrics: (metrics: MinerMetrics, isNewBlock?: boolean) => void;
    startMining: () => Promise<void>;
    stopMining: () => Promise<void>;
    pauseMining: () => Promise<void>;
    changeMiningMode: (mode: modeType) => Promise<void>;
    setMiningControlsEnabled: (miningControlsEnabled: boolean) => void;
    setIsChangingMode: (isChangingMode: boolean) => void;
    setExcludedGpuDevice: (excludeGpuDevice: number[]) => Promise<void>;
}
type MiningStoreState = State & Actions;

const initialState: State = {
    sha_network_hash_rate: 0,
    randomx_network_hash_rate: 0,
    counter: 0,
    hashrateReady: false,
    miningInitiated: false,
    isChangingMode: false,
    miningControlsEnabled: true,
    excludedGpuDevices: [],
    cpu: {
        hardware: undefined,
        mining: {
            is_mining: false,
            hash_rate: 0,
            estimated_earnings: 0,
            connection: { is_connected: false },
        },
    },
    gpu: {
        hardware: [],
        mining: {
            is_mining: false,
            hash_rate: 0,
            estimated_earnings: 0,
            is_available: true,
        },
    },
    base_node: {
        block_height: 0,
        block_time: 0,
        is_synced: false,
        is_connected: false,
        connected_peers: [],
    },
};

export const useMiningStore = create<MiningStoreState>()((set, getState) => ({
    ...initialState,
    setMiningMetrics: (metrics) => set({ ...metrics }),
    startMining: async () => {
        console.info('Mining starting....');
        set({ miningInitiated: true });
        useBlockchainVisualisationStore
            .getState()
            .setDisplayBlockTime({ daysString: '', hoursString: '', minutes: '00', seconds: '00' });
        try {
            await invoke('start_mining', {});
            console.info('Mining started.');
        } catch (e) {
            Sentry.captureException(e);
            const appStateStore = useAppStateStore.getState();
            console.error('Failed to start mining: ', e);
            appStateStore.setError(e as string);
            set({ miningInitiated: false });
        }
    },
    stopMining: async () => {
        console.info('Mining stopping...');
        set({ miningInitiated: false });
        try {
            await invoke('stop_mining', {});
            console.info('Mining stopped.');
        } catch (e) {
            Sentry.captureException(e);
            const appStateStore = useAppStateStore.getState();
            console.error('Failed to stop mining: ', e);
            appStateStore.setError(e as string);
            set({ miningInitiated: true });
        }
    },
    pauseMining: async () => {
        console.info('Mining pausing...');
        try {
            await invoke('stop_mining', {});
            console.info('Mining paused.');
        } catch (e) {
            Sentry.captureException(e);
            const appStateStore = useAppStateStore.getState();
            console.error('Failed to pause (stop) mining: ', e);
            appStateStore.setError(e as string);
            set({ miningInitiated: true });
        }
    },
    changeMiningMode: async (mode: modeType) => {
        console.info('Changing mode...');
        const state = getState();

        set({ isChangingMode: true });
        if (state.cpu.mining.is_mining || state.gpu.mining.is_mining) {
            await state.pauseMining();
        }
        try {
            const appConfigState = useAppConfigStore.getState();
            await appConfigState.setMode(mode as modeType);
            if (state.miningInitiated) {
                await state.startMining();
            }

            console.info(`Mode changed to ${mode}`);
            set({ isChangingMode: false });
        } catch (e) {
            Sentry.captureException(e);
            console.error('Failed to change mode: ', e);
            set({ isChangingMode: false });
        }
    },
    setMiningControlsEnabled: (miningControlsEnabled) => set({ miningControlsEnabled }),
    setIsChangingMode: (isChangingMode) => set({ isChangingMode }),
    setExcludedGpuDevice: async (excludedGpuDevices) => {
        set({ excludedGpuDevices });
        try {
            await invoke('set_excluded_gpu_devices', { excludedGpuDevices });
        } catch (e) {
            Sentry.captureException(e);
            const appStateStore = useAppStateStore.getState();
            console.error('Could not set excluded gpu device: ', e);
            appStateStore.setError(e as string);
            set({ excludedGpuDevices: undefined });
        }
    },
}));

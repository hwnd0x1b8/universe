import { useState } from 'react';
import { useConfigCoreStore, useConfigMiningStore } from '@app/store/useAppConfigStore.ts';
import P2pMarkup from './P2pMarkup.tsx';
import P2PoolStats from './P2PoolStats.tsx';
import { PoolMiningStats } from './PoolMiningStats.tsx';

export const PoolMiningSettings = () => {
    const [disabledStats, setDisabledStats] = useState(false);
    const isP2poolEnabled = useConfigCoreStore((s) => s.is_p2pool_enabled);
    const cpuMiningEnabled = useConfigMiningStore((s) => s.cpu_mining_enabled);
    return (
        <>
            {cpuMiningEnabled && <PoolMiningStats />}
            <P2pMarkup setDisabledStats={setDisabledStats} />
            {isP2poolEnabled && !disabledStats && <P2PoolStats />}
        </>
    );
};

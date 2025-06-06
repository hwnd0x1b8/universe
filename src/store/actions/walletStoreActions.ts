import { invoke } from '@tauri-apps/api/core';
import { ALREADY_FETCHING } from '@app/App/sentryIgnore.ts';
import { WalletAddress, WalletBalance } from '@app/types/app-status.ts';
import { useWalletStore } from '../useWalletStore';
import { restartMining } from './miningStoreActions';
import { setError } from './appStateStoreActions';
import { setExchangeContent } from '@app/store/useExchangeStore.ts';

interface TxArgs {
    continuation: boolean;
    limit?: number;
}

export const fetchTransactionsHistory = async ({ continuation, limit }: TxArgs) => {
    if (useWalletStore.getState().is_transactions_history_loading) {
        return [];
    }

    try {
        useWalletStore.setState({ is_transactions_history_loading: true });
        const currentTxs = useWalletStore.getState().transactions;
        const fetchedTxs = await invoke('get_transactions_history', { continuation, limit });

        const transactions = continuation ? [...currentTxs, ...fetchedTxs] : fetchedTxs;
        const has_more_transactions = fetchedTxs.length > 0 && (!limit || fetchedTxs.length === limit);
        useWalletStore.setState({
            has_more_transactions,
            transactions,
        });
        return transactions;
    } catch (error) {
        if (error !== ALREADY_FETCHING.HISTORY) {
            console.error('Could not get transaction history: ', error);
        }
        return [];
    } finally {
        useWalletStore.setState({ is_transactions_history_loading: false });
    }
};
export const importSeedWords = async (seedWords: string[]) => {
    try {
        useWalletStore.setState({ is_wallet_importing: true });
        await invoke('import_seed_words', { seedWords });
    } catch (error) {
        setError(`Could not import seed words: ${error}`, true);
        useWalletStore.setState({ is_wallet_importing: false });
    }
};
export const initialFetchTxs = () =>
    fetchTransactionsHistory({ continuation: false, limit: 20 }).then((tx) => {
        if (tx?.length) {
            useWalletStore.setState({ newestTxIdOnInitialFetch: tx[0]?.tx_id });
        }
    });

export const refreshTransactions = async () => {
    const limit = useWalletStore.getState().transactions.length;
    return fetchTransactionsHistory({ continuation: false, limit: Math.max(limit, 20) });
};

export const setGeneratedTariAddress = async (newAddress: string) => {
    await invoke('set_tari_address', { address: newAddress })
        .then(() => {
            setExchangeContent(null);
            restartMining();
            console.info('New Tari address set successfully to:', newAddress);
        })
        .catch((e) => {
            console.error('Could not set Monero address', e);
            setError('Could not change Monero address');
        });
};

export const setWalletAddress = (addresses: Partial<WalletAddress>) => {
    useWalletStore.setState({
        tari_address_base58: addresses.tari_address_base58,
        tari_address_emoji: addresses.tari_address_emoji,
        is_tari_address_generated: addresses.is_tari_address_generated,
    });
};
export const setWalletBalance = (balance: WalletBalance) => {
    const calculated_balance =
        balance.available_balance + balance.timelocked_balance + balance.pending_incoming_balance;

    const pendingSendAmount = useWalletStore
        .getState()
        .pending_transactions.reduce((total, tx) => total + tx.amount, 0);

    useWalletStore.setState({
        balance: {
            ...balance,
            available_balance: balance.available_balance - pendingSendAmount,
            pending_outgoing_balance: balance.pending_outgoing_balance + pendingSendAmount,
        },
        calculated_balance,
    });
};

export const setIsSwapping = (isSwapping: boolean) => {
    useWalletStore.setState({ is_swapping: isSwapping });
};

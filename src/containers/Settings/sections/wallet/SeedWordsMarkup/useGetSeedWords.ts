import { useCallback, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import * as Sentry from '@sentry/react';

export function useGetSeedWords() {
    const [seedWords, setSeedWords] = useState<string[]>([]);
    const [seedWordsFetching, setSeedWordsFetching] = useState(false);

    const getSeedWords = useCallback(async () => {
        setSeedWordsFetching(true);
        try {
            const seedWords = await invoke('get_seed_words');
            setSeedWords(seedWords);
        } catch (e) {
            Sentry.captureException(e);
            console.error('Could not get seed words', e);
        } finally {
            setSeedWordsFetching(false);
        }
    }, []);

    return {
        seedWords,
        getSeedWords,
        seedWordsFetched: seedWords.length > 0,
        seedWordsFetching,
    };
}

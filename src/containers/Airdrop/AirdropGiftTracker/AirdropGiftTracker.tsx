import { useAirdropStore } from '@app/store/useAirdropStore';
import { Title, TitleWrapper, Wrapper } from './styles';
import LoggedOut from './sections/LoggedOut/LoggedOut';
import LoggedIn from './sections/LoggedIn/LoggedIn';
import { useAirdropSyncState } from '@app/hooks/airdrop/useAirdropSyncState';
import { useAppConfigStore } from '@app/store/useAppConfigStore';
import { useTranslation } from 'react-i18next';
import InfoTooltip from './components/InfoTooltip/InfoTooltip';
import { useWebsocket } from '@app/hooks/airdrop/useWebsocket.ts';

export default function AirdropGiftTracker() {
    useAirdropSyncState();
    useWebsocket();

    const { t } = useTranslation(['airdrop'], { useSuspense: false });
    const airdrop_ui_enabled = useAppConfigStore((s) => s.airdrop_ui_enabled);

    const { airdropTokens } = useAirdropStore();

    if (!airdrop_ui_enabled) return null;
    const isLoggedIn = !!airdropTokens;

    return (
        <Wrapper layout>
            <TitleWrapper>
                <Title>{t('airdropGame')}</Title>
                <InfoTooltip title={t('topTooltipTitle')} text={t('topTooltipText')} />
            </TitleWrapper>

            {isLoggedIn ? <LoggedIn /> : <LoggedOut />}
        </Wrapper>
    );
}

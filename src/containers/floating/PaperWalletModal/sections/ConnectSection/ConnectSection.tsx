import { ContentWrapper, HeroImage, StoreWrapper, Text, Title, Wrapper } from './styles';
import paperWalletImage from '../../images/paper-wallet-phone.png';
import AppleStoreIcon from '../../icons/AppleStoreIcon';
import GoogleStoreIcon from '../../icons/GoogleStoreIcon';
import { BlackButton } from '../../styles';
import { PaperWalletModalSectionType } from '../../PaperWalletModal';
import QRTooltip from './QRTooltip/QRTooltip';
import qrTooltipImage from '../../images/qr-tooltip.png';
import { useTranslation } from 'react-i18next';
import { useCallback, useState } from 'react';
import { usePaperWalletStore } from '@app/store/usePaperWalletStore';
import { invoke } from '@tauri-apps/api/core';
import LoadingSvg from '@app/components/svgs/LoadingSvg';
import { useAirdropSetTokenToUuid } from '@app/hooks/airdrop/stateHelpers/useAirdropSetTokens';

interface Props {
    setSection: (section: PaperWalletModalSectionType) => void;
}

export default function ConnectSection({ setSection }: Props) {
    const { t } = useTranslation(['paper-wallet'], { useSuspense: false });
    const [isLoading, setIsLoading] = useState(false);
    const { setQrCodeValue, setIdentificationCode } = usePaperWalletStore();
    const setTokenToUuid = useAirdropSetTokenToUuid();

    const handleBlackButtonClick = () => {
        loadPaperWalletData();
    };

    const loadPaperWalletData = useCallback(async () => {
        setIsLoading(true);

        try {
            const authUuid = await setTokenToUuid();
            const r = await invoke('get_paper_wallet_details', { authUuid });

            if (r) {
                const url = r.qr_link;
                const password = r.password;

                setQrCodeValue(url);
                setIdentificationCode(password);
                setSection('QRCode');
            }
        } catch (e) {
            console.error('Failed to get paper wallet details', e);
        }

        setIsLoading(false);
    }, [setIdentificationCode, setIsLoading, setQrCodeValue, setSection, setTokenToUuid]);

    return (
        <Wrapper>
            <HeroImage src={paperWalletImage} alt="Wallet image" />

            <ContentWrapper>
                <Title>{t('connect.title')}</Title>

                <Text>{t('connect.text')}</Text>

                <StoreWrapper>
                    <QRTooltip trigger={<AppleStoreIcon />} text={t('connect.scan')} codeImage={qrTooltipImage} />

                    <QRTooltip trigger={<GoogleStoreIcon />} text={t('connect.scan')} codeImage={qrTooltipImage} />
                </StoreWrapper>

                <BlackButton onClick={handleBlackButtonClick} disabled={isLoading}>
                    {isLoading ? <LoadingSvg /> : <span>{t('connect.blackButton')}</span>}
                </BlackButton>

                {/* <TextButton onClick={handleTextButtonClick}>{t('connect.textButton')}</TextButton> */}
            </ContentWrapper>
        </Wrapper>
    );
}

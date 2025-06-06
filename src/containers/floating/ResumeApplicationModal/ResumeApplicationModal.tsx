import { memo, useEffect, useMemo, useState } from 'react';
import { CircularProgress } from '@app/components/elements/CircularProgress';
import { Text, Title, Wrapper, ProgressWrapper, TextWrapper } from './styles';
import { useTranslation } from 'react-i18next';
import { useUIStore } from '@app/store';
import { useSetupStore } from '@app/store/useSetupStore';
import { setShowResumeAppModal } from '@app/store/actions/uiStoreActions';
import { FloatingNode, FloatingPortal, useFloating, useFloatingNodeId } from '@floating-ui/react';

const ResumeApplicationModal = memo(function ResumeApplicationModal() {
    const [open, setOpen] = useState(false);
    const nodeId = useFloatingNodeId();
    const { t } = useTranslation('setup-progresses');
    const { refs } = useFloating({
        nodeId,
        open,
        onOpenChange: setOpen,
    });

    const status = useUIStore((s) => s.connectionStatus);
    const corePhaseInfoPayload = useSetupStore((state) => state.core_phase_setup_payload);
    const hardwarePhaseInfoPayload = useSetupStore((state) => state.hardware_phase_setup_payload);
    const nodePhaseInfoPayload = useSetupStore((state) => state.node_phase_setup_payload);
    const unknownPhaseInfoPayload = useSetupStore((state) => state.unknown_phase_setup_payload);
    const walletPhaseInfoPayload = useSetupStore((state) => state.wallet_phase_setup_payload);

    const isAppUnlocked = useSetupStore((state) => state.appUnlocked);
    const isSetupFinished = useSetupStore((state) => state.isInitialSetupFinished);
    const shouldShowModal = useUIStore((state) => state.showResumeAppModal);

    const showModal = useMemo(() => {
        const shouldShowModalForInitialSetup = isAppUnlocked && !isSetupFinished;
        const shouldShowModalForResume = shouldShowModal && status === 'connected';
        return shouldShowModalForResume || shouldShowModalForInitialSetup;
    }, [isAppUnlocked, isSetupFinished, shouldShowModal, status]);

    const currentPhaseToShow = useMemo(() => {
        if (walletPhaseInfoPayload?.is_complete && Boolean(unknownPhaseInfoPayload)) {
            return unknownPhaseInfoPayload;
        }

        if (hardwarePhaseInfoPayload?.is_complete && Boolean(walletPhaseInfoPayload)) {
            return walletPhaseInfoPayload;
        }

        if (nodePhaseInfoPayload?.is_complete && Boolean(hardwarePhaseInfoPayload)) {
            return hardwarePhaseInfoPayload;
        }

        if (corePhaseInfoPayload?.is_complete && Boolean(nodePhaseInfoPayload)) {
            return nodePhaseInfoPayload;
        }

        return corePhaseInfoPayload;
    }, [
        corePhaseInfoPayload,
        hardwarePhaseInfoPayload,
        nodePhaseInfoPayload,
        unknownPhaseInfoPayload,
        walletPhaseInfoPayload,
    ]);

    const [stageProgress, stageTotal] = useMemo(() => {
        if (unknownPhaseInfoPayload?.is_complete && walletPhaseInfoPayload?.is_complete) {
            setShowResumeAppModal(false);
            return [5, 5];
        }

        if (walletPhaseInfoPayload?.is_complete) {
            return [4, 5];
        }

        if (hardwarePhaseInfoPayload?.is_complete) {
            return [3, 5];
        }

        if (nodePhaseInfoPayload?.is_complete) {
            return [2, 5];
        }

        if (corePhaseInfoPayload?.is_complete) {
            return [1, 5];
        }

        return [0, 5];
    }, [
        corePhaseInfoPayload?.is_complete,
        hardwarePhaseInfoPayload?.is_complete,
        nodePhaseInfoPayload?.is_complete,
        unknownPhaseInfoPayload?.is_complete,
        walletPhaseInfoPayload?.is_complete,
    ]);

    const setupPhaseTitle = currentPhaseToShow?.phase_title;
    const setupTitle = currentPhaseToShow?.title;
    const setupParams = currentPhaseToShow?.title_params ? { ...currentPhaseToShow.title_params } : {};

    useEffect(() => {
        setOpen(showModal && Boolean(currentPhaseToShow));
    }, [currentPhaseToShow, showModal]);

    return (
        <FloatingNode id={nodeId}>
            <FloatingPortal>
                {open && (
                    <Wrapper ref={refs.setFloating}>
                        <TextWrapper>
                            <Title>{t(`phase-title.${setupPhaseTitle}`)}</Title>
                            <Text>{t(`title.${setupTitle}`, { ...setupParams })}</Text>
                        </TextWrapper>
                        <ProgressWrapper>
                            <Title>
                                {stageProgress} / {stageTotal}
                            </Title>
                            <CircularProgress />
                        </ProgressWrapper>
                    </Wrapper>
                )}
            </FloatingPortal>
        </FloatingNode>
    );
});

export default ResumeApplicationModal;

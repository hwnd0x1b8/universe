import BonusGems from './BonusGems/BonusGems';
import FriendAccepted from './FriendAccepted/FriendAccepted';
import GoalComplete from './GoalComplete/GoalComplete';
import { Wrapper } from './styles';
import { useEffect } from 'react';
import { setFlareAnimationType } from '@app/store';

interface Props {
    gems: number;
    animationType: 'GoalComplete' | 'FriendAccepted' | 'BonusGems';
}

const GOAL_COMPLETE_DURATION = 1000 * 11.5;
const REFERRAL_DURATION = 1000 * 10;
const BONUS_GEMS_DURATION = 3500;

const durations = {
    GoalComplete: GOAL_COMPLETE_DURATION,
    FriendAccepted: REFERRAL_DURATION,
    BonusGems: BONUS_GEMS_DURATION,
};

export default function Flare({ gems, animationType }: Props) {
    useEffect(() => {
        if (!animationType) return;
        const duration = durations[animationType] || 0;
        const animationTimeout = setTimeout(setFlareAnimationType, duration);
        return () => {
            clearTimeout(animationTimeout);
        };
    }, [animationType]);

    return (
        <Wrapper
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={() => setFlareAnimationType()}
        >
            {animationType === 'GoalComplete' && gems > 0 && <GoalComplete gems={gems} />}
            {animationType === 'FriendAccepted' && <FriendAccepted gems={gems} />}
            {animationType === 'BonusGems' && <BonusGems gems={gems} />}
        </Wrapper>
    );
}

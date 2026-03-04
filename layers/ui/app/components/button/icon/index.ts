import type { ButtonOrAnchorProps } from '../or-anchor';

export type ButtonIconSize = 'default' | 'small' | 'tiny';

export interface ButtonIconProps extends ButtonOrAnchorProps {
  size?: ButtonIconSize;
}

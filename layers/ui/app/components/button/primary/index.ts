import type { ButtonOrAnchorProps } from '../or-anchor';

export type ButtonPrimarySize = 'default' | 'large';
export type ButtonPrimaryWidth = 'default' | 'full';
export type ButtonPrimaryColor = 'default' | 'gray';

export interface ButtonPrimaryProps extends ButtonOrAnchorProps {
  size?: ButtonPrimarySize;
  width?: ButtonPrimaryWidth;
  color?: ButtonPrimaryColor;
}

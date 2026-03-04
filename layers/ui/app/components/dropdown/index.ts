export type DropdownPositionX = 'left' | 'right' | 'center';
export type DropdownPositionY = 'bottom' | 'top';
export type DropdownWidth = 'default' | 'auto';

export interface DropdownProps {
  contentPositionX?: DropdownPositionX;
  contentPositionY?: DropdownPositionY;
  openOnHover?: boolean;
  width?: DropdownWidth;
}

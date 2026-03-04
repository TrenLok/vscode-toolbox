export interface ButtonOrAnchorProps {
  href?: string;
  isDisabled?: boolean;
}

export interface ButtonOrAnchorEmits {
  click: [event: MouseEvent];
  pointerenter: [event: PointerEvent];
}

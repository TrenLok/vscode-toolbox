export interface ButtonOrAnchorProps {
  href?: string | null;
  isDisabled?: boolean;
}

export interface ButtonOrAnchorEmits {
  click: [event: MouseEvent];
  pointerenter: [event: PointerEvent];
}

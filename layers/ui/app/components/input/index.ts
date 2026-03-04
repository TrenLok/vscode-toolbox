export interface InputProps {
  name?: string;
  placeholder?: string;
  isFocused?: boolean;
  isRequired?: boolean;
  type?: string;
  isClearable?: boolean;
}

export interface InputEmits {
  click: [event: MouseEvent];
}

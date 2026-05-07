export type NotificationVariant = 'default' | 'success' | 'error';

export interface NotificationProps {
  variant: NotificationVariant;
}

export interface NotificationEmits {
  close: [void];
}

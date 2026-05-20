import { toValue } from 'vue';
import type { MaybeRefOrGetter } from 'vue';
import type UiInput from '../../layers/ui/app/components/input/input.vue';

type RevealShortcutInput = InstanceType<typeof UiInput>;

const modifierKeys = new Set(['Alt', 'Control', 'Meta', 'Shift']);

function formatShortcut(value: string | null) {
  if (!value) return '';

  return value
    .replaceAll('+', ' + ')
    .replaceAll('Control', 'Ctrl')
    .replaceAll('Command', 'Cmd')
    .replaceAll('Option', 'Alt')
    .replaceAll('Key', '');
}

function keyboardEventToShortcut(event: KeyboardEvent) {
  if (event.repeat || modifierKeys.has(event.key)) return;

  const parts: string[] = [];
  if (event.metaKey) parts.push('Cmd');
  if (event.ctrlKey) parts.push('Ctrl');
  if (event.altKey) parts.push('Alt');
  if (event.shiftKey) parts.push('Shift');

  let key = event.code;
  if (key.startsWith('Key')) {
    key = key.slice(3);
  } else if (key.startsWith('Digit')) {
    key = key.slice(5);
  }

  parts.push(key);

  return parts.join('+');
}

export function useRevealShortcut(input: MaybeRefOrGetter<RevealShortcutInput | null>) {
  const appSettings = useAppSettings();
  const { notify } = useNotification();
  const blurBus = useEventBus('blur-window');

  const formattedShortcut = computed(() => formatShortcut(appSettings.revealShortcut.value));

  function blurInput() {
    toValue(input)?.blur();
  }

  async function setShortcut(shortcut: string | null, errorMessage: string, logMessage: string) {
    try {
      await appSettings.setRevealShortcut(shortcut);
      blurInput();
    } catch (error_) {
      notify({
        text: errorMessage,
        type: 'error',
      });
      useTauriLogError(`${logMessage}: ${error_}`);
    }
  }

  async function onKeydown(event: KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation();

    const shortcut = keyboardEventToShortcut(event);
    if (!shortcut) return;

    await setShortcut(shortcut, 'Failed to set shortcut', 'Couldn\'t set reveal shortcut');
  }

  async function reset() {
    await setShortcut(null, 'Failed to reset shortcut', 'Couldn\'t reset reveal shortcut');
  }

  const stopBlur = blurBus.on(blurInput);

  onBeforeUnmount(() => {
    stopBlur();
  });

  return {
    formattedShortcut,
    onKeydown,
    reset,
  };
}

export function useSearchInputFocusGuard() {
  function shouldFocusSearchInput(event: KeyboardEvent): boolean {
    const isHandledOrShortcut = [
      event.defaultPrevented,
      event.isComposing,
      event.ctrlKey,
      event.altKey,
      event.metaKey,
      event.key.length !== 1,
    ].some(Boolean);

    if (isHandledOrShortcut) {
      return false;
    }

    const target = event.target;

    if (!(target instanceof HTMLElement)) {
      return true;
    }

    const isIgnoredTarget = [
      target.isContentEditable,
      Boolean(target.closest('.modal')),
      ['INPUT', 'SELECT', 'TEXTAREA'].includes(target.tagName),
    ].some(Boolean);

    return !isIgnoredTarget;
  }

  return {
    shouldFocusSearchInput,
  };
}

export function useProjectKeyboardNavigation() {
  function focusProjectByArrowKey(event: KeyboardEvent): void {
    if (!isProjectNavigationKey(event)) {
      return;
    }

    const projectButtons = getProjectButtons();

    if (!projectButtons.length) {
      return;
    }

    const currentIndex = projectButtons.indexOf(document.activeElement as HTMLButtonElement);
    const nextIndex = getNextProjectIndex(event.key, currentIndex, projectButtons.length);

    event.preventDefault();
    projectButtons[nextIndex]?.focus();
  }

  function isProjectNavigationKey(event: KeyboardEvent): boolean {
    return (
      (event.key === 'ArrowDown' || event.key === 'ArrowUp')
      && !event.ctrlKey
      && !event.altKey
      && !event.metaKey
    );
  }

  function getProjectButtons(): HTMLButtonElement[] {
    return [
      ...document.querySelectorAll<HTMLButtonElement>('.main-page__projects .project__button'),
    ];
  }

  function getNextProjectIndex(key: string, currentIndex: number, length: number): number {
    if (key === 'ArrowDown') {
      return currentIndex < 0 ? 0 : Math.min(currentIndex + 1, length - 1);
    }

    return currentIndex < 0 ? length - 1 : Math.max(currentIndex - 1, 0);
  }

  return {
    focusProjectByArrowKey,
    isProjectNavigationKey,
  };
}

export function useProjectKeyboardNavigation() {
  const closeProjectDropdownBus = useEventBus('close-project-dropdown');
  let shouldStartFromFirstProject = false;
  let lastFocusedProject: Element | null = null;
  let resetProject: Element | null = null;

  function focusProjectByArrowKey(event: KeyboardEvent): void {
    if (!isProjectNavigationKey(event)) {
      return;
    }

    const projectButtons = getProjectButtons();

    if (!projectButtons.length) {
      return;
    }

    const currentIndex = shouldStartFromFirstProject && shouldIgnoreCurrentProjectFocus()
      ? -1
      : getCurrentProjectIndex(projectButtons);
    const nextIndex = getNextProjectIndex(event.key, currentIndex, projectButtons.length);

    event.preventDefault();
    closeProjectDropdownBus.emit();
    projectButtons[nextIndex]?.focus();
    lastFocusedProject = projectButtons[nextIndex]?.closest('.project') ?? null;
    shouldStartFromFirstProject = false;
    resetProject = null;
  }

  function isProjectNavigationKey(event: KeyboardEvent): boolean {
    return (
      (event.key === 'ArrowDown' || event.key === 'ArrowUp')
      && !event.ctrlKey
      && !event.altKey
      && !event.metaKey
    );
  }

  function resetProjectNavigationFocus(): void {
    shouldStartFromFirstProject = true;
    resetProject = getActiveProject() ?? lastFocusedProject;

    const activeElement = document.activeElement;

    if (!(activeElement instanceof HTMLElement)) {
      return;
    }

    const currentProject = activeElement.closest('.project');

    if (!currentProject) {
      return;
    }

    activeElement.blur();
  }

  function shouldIgnoreCurrentProjectFocus(): boolean {
    const currentProject = getActiveProject();

    if (!currentProject) {
      return true;
    }

    return currentProject === resetProject;
  }

  function getActiveProject(): Element | null {
    if (!(document.activeElement instanceof HTMLElement)) {
      return null;
    }

    return document.activeElement.closest('.project');
  }

  function getProjectButtons(): HTMLButtonElement[] {
    return [
      ...document.querySelectorAll<HTMLButtonElement>('.main-page__projects .project__button'),
    ];
  }

  function getCurrentProjectIndex(projectButtons: HTMLButtonElement[]): number {
    if (!(document.activeElement instanceof HTMLElement)) {
      return -1;
    }

    const currentProject = getActiveProject();

    if (!currentProject) {
      return -1;
    }

    lastFocusedProject = currentProject;
    return projectButtons.findIndex((projectButton) => projectButton.closest('.project') === currentProject);
  }

  function getNextProjectIndex(key: string, currentIndex: number, length: number): number {
    if (currentIndex < 0) {
      return 0;
    }

    if (key === 'ArrowDown') {
      return Math.min(currentIndex + 1, length - 1);
    }

    return Math.max(currentIndex - 1, 0);
  }

  return {
    focusProjectByArrowKey,
    isProjectNavigationKey,
    resetProjectNavigationFocus,
  };
}

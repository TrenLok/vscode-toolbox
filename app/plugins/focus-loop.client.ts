const FOCUSABLE_SELECTOR = [
  'a[href]',
  'button:not([disabled])',
  'input:not([disabled])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[tabindex]:not([tabindex="-1"])',
  '[contenteditable="true"]',
].join(',');

export default defineNuxtPlugin(() => {
  document.addEventListener('keydown', onKeyDown, { capture: true });
});

function onKeyDown(event: KeyboardEvent): void {
  if (event.key !== 'Tab' || event.altKey || event.ctrlKey || event.metaKey) return;

  const focusableElements = getFocusableElements();

  if (!focusableElements.length) return;

  const firstElement = focusableElements.at(0);
  const lastElement = focusableElements.at(-1);

  if (!firstElement || !lastElement) return;

  const activeElement = document.activeElement;
  const activeIndex = activeElement instanceof HTMLElement
    ? focusableElements.indexOf(activeElement)
    : -1;

  if (event.shiftKey) {
    if (activeIndex > 0) return;

    event.preventDefault();
    event.stopPropagation();
    lastElement.focus({ preventScroll: true });
    return;
  }

  if (activeIndex >= 0 && activeElement !== lastElement) return;

  event.preventDefault();
  event.stopPropagation();
  firstElement.focus({ preventScroll: true });
}

function getFocusableElements(): HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)]
    .filter((element) => isFocusable(element));
}

function isFocusable(element: HTMLElement): boolean {
  if (element.tabIndex < 0) return false;
  if (element.hasAttribute('disabled')) return false;
  if (element.getAttribute('aria-hidden') === 'true') return false;

  const style = globalThis.getComputedStyle(element);

  if (style.display === 'none' || style.visibility === 'hidden') return false;

  return element.offsetParent !== null || style.position === 'fixed';
}

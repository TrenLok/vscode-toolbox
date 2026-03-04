export function calculatePercentage(current: number, total: number): number {
  if (total === 0) {
    return 0;
  }
  const percentage = (current / total) * 100;
  return Math.trunc(percentage);
}

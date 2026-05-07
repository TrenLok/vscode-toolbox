export function delay(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

export async function withMinDuration<T>(
  callback: () => Promise<T>,
  minDuration = 500,
): Promise<T> {
  const startedAt = Date.now();

  try {
    return await callback();
  } finally {
    const elapsed = Date.now() - startedAt;
    const remaining = minDuration - elapsed;

    if (remaining > 0) {
      await delay(remaining);
    }
  }
}

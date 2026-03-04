export async function normalizeFolderPath(folder: string) {
  const normalizedPathePath: string = await useTauriPathNormalize(folder);
  const absolutePathePath: string = await useTauriPathResolve(normalizedPathePath);
  return absolutePathePath;
}

export async function getNormalizedAndResolvedFolderPath(folder: string): Promise<string> {
  const normalizedPath = await normalizeFolderPath(folder);
  const normalizedWindowsPath = normalizeWindowsPathDrive(normalizedPath);
  return normalizedWindowsPath;
}

type DriveCase = 'lower' | 'upper';

export function normalizeWindowsPathDrive(
  path: string,
  driveCase: DriveCase = 'lower',
): string {
  const driveRegex = /^([A-Z]):/i;
  const match = driveRegex.exec(path);
  if (!match?.[1]) return path;

  const driveLetter = driveCase === 'upper' ? match[1].toUpperCase() : match[1].toLowerCase();

  return `${driveLetter}:${path.slice(match[0].length)}`;
}

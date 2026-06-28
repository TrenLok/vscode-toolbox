import { safeDecodeURIComponent } from '~/utils/uri';

export async function normalizeFolderPath(folder: string) {
  const normalizedPathePath: string = await useTauriPathNormalize(folder);
  const absolutePathePath: string = await useTauriPathResolve(normalizedPathePath);
  return absolutePathePath;
}

export async function getNormalizedAndResolvedFolderPath(folder: string): Promise<string> {
  if (isVSCodeRemoteUri(folder)) return folder;

  const normalizedPath = await normalizeFolderPath(folder);
  const normalizedWindowsPath = normalizeWindowsPathDrive(normalizedPath);
  return normalizedWindowsPath;
}

export function isVSCodeRemoteUri(path: string): boolean {
  return /^vscode-remote:\/\//i.test(path);
}

export function getProjectFolderName(folder: string): string {
  if (!isVSCodeRemoteUri(folder)) return '';

  try {
    const url = new URL(folder);
    const segments = url.pathname.split('/').filter(Boolean);
    const name = segments.at(-1);

    return name ? safeDecodeURIComponent(name) : url.host;
  } catch {
    return '';
  }
}

export function getVSCodeRemoteDisplay(label: string): null | { folder: string; name: string } {
  const normalizedLabel = label.trim();
  if (!normalizedLabel) return null;

  const bracketMatch = /\s*(\[[^\]]+\])\s*$/.exec(normalizedLabel);
  const bracket = bracketMatch?.[1] ?? '';
  const subtitle = bracketMatch
    ? normalizedLabel.slice(0, bracketMatch.index).trim()
    : normalizedLabel;
  const titleBase = getLastPathSegment(subtitle) || subtitle;

  return {
    name: [titleBase, bracket].filter(Boolean).join(' '),
    folder: subtitle,
  };
}

export function getVSCodeRemoteDisplayFromUri(uri: string): null | { folder: string; name: string } {
  if (!isVSCodeRemoteUri(uri)) return null;

  try {
    const url = new URL(uri);
    const folder = getVSCodeRemoteFolderDisplay(url.pathname);
    const bracket = getVSCodeRemoteAuthorityBracket(url.host);
    const name = [getLastPathSegment(folder), bracket].filter(Boolean).join(' ');

    return name
      ? { folder, name }
      : null;
  } catch {
    return null;
  }
}

export function getVSCodeRemoteCoderBracket(remoteAuthority?: string): string {
  if (!remoteAuthority) return '';

  return getVSCodeRemoteAuthorityBracket(remoteAuthority);
}

function getVSCodeRemoteFolderDisplay(pathname: string): string {
  const decodedPath = safeDecodeURIComponent(pathname);

  return decodedPath.replace(/^\/home\/[^/]+(?=\/|$)/, '~');
}

function getVSCodeRemoteAuthorityBracket(host: string): string {
  const authority = safeDecodeURIComponent(host).replace(/^ssh-remote\+/i, '');
  const coderWorkspace = getCoderWorkspaceDisplay(authority);

  return coderWorkspace ? `[Coder: ${coderWorkspace}]` : '';
}

function getCoderWorkspaceDisplay(authority: string): string {
  const parts = authority.split('--');
  if (parts.length < 2) return '';

  return parts
    .slice(1)
    .flatMap((part) => part.split('.'))
    .filter(Boolean)
    .join('\u2215');
}

function getLastPathSegment(path: string): string {
  const segments = path.split(/[\\/∕]/).filter(Boolean);
  return segments.at(-1) ?? '';
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

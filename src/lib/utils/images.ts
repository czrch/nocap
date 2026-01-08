import type { ImageFile } from '../types';

export const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'] as const;

export function imageFileFromPath(path: string): ImageFile {
  const pathParts = path.split(/[/\\]/);
  const filename = pathParts[pathParts.length - 1] ?? path;
  const extensionMatch = filename.match(/\.([^.]+)$/);
  const extension = extensionMatch ? extensionMatch[1].toLowerCase() : '';

  return {
    path,
    filename,
    extension,
  };
}

export function isSupportedImage(path: string): boolean {
  const extensionMatch = path.match(/\.([^.]+)$/);
  const extension = extensionMatch ? extensionMatch[1].toLowerCase() : '';
  return IMAGE_EXTENSIONS.includes(extension as (typeof IMAGE_EXTENSIONS)[number]);
}

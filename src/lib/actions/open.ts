import { invoke } from '@tauri-apps/api/core';
import { message, open } from '@tauri-apps/plugin-dialog';
import type { ImageFile } from '../types';

const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'] as const;

function imageFileFromPath(path: string): ImageFile {
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

export async function pickImageFile(): Promise<ImageFile | null> {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'Images',
        extensions: [...IMAGE_EXTENSIONS],
      },
    ],
  });

  if (!selected || typeof selected !== 'string') return null;
  return imageFileFromPath(selected);
}

export async function pickFolderPath(): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
  });

  if (!selected || typeof selected !== 'string') return null;
  return selected;
}

export async function scanFolderForImages(folderPath: string): Promise<ImageFile[]> {
  return invoke<ImageFile[]>('scan_folder_for_images', {
    folderPath,
  });
}

export async function pickImageFolder(): Promise<ImageFile[] | null> {
  const selected = await pickFolderPath();
  if (!selected) return null;

  const images = await scanFolderForImages(selected);

  if (!images || images.length === 0) {
    await message('No images found in the selected folder.', {
      title: 'nocap',
      kind: 'info',
    });
    return null;
  }

  return images;
}

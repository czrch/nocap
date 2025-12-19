import { invoke } from '@tauri-apps/api/core';
import type {
  MediaFile,
  MediaFileWithDetails,
  Tag,
  Annotation,
  FilterOptions,
  SortOptions,
  EmbeddedMetadata,
} from '../types';

class MediaApiError extends Error {
  constructor(
    message: string,
    public command: string,
  ) {
    super(message);
    this.name = 'MediaApiError';
  }
}

async function invokeMedia<T>(command: string, args: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    const msg = error instanceof Error ? error.message : String(error);
    throw new MediaApiError(`Media API failed for ${command}: ${msg}`, command);
  }
}

export async function scanDirectory(path: string): Promise<MediaFile[]> {
  return invokeMedia('scan_directory', { path });
}

export async function getMediaFiles(
  filters: FilterOptions = {
    mediaTypes: [],
    minRating: 0,
    maxRating: 5,
    tags: [],
    favorite: null,
    searchQuery: ''
  },
  sort: SortOptions = { field: 'createdAt', direction: 'desc' },
): Promise<MediaFile[]> {
  return invokeMedia('get_media_files', { filters, sort });
}

export async function getMediaDetails(id: number): Promise<MediaFileWithDetails> {
  return invokeMedia('get_media_details', { id });
}

export async function extractMetadata(id: number): Promise<EmbeddedMetadata> {
  return invokeMedia('extract_metadata', { id });
}

export async function createTag(name: string, color?: string): Promise<Tag> {
  return invokeMedia('create_tag', { name, color });
}

export async function updateTag(id: number, name: string, color?: string): Promise<Tag> {
  return invokeMedia('update_tag', { id, name, color });
}

export async function deleteTag(id: number): Promise<void> {
  return invokeMedia('delete_tag', { id });
}

export async function listTags(): Promise<Tag[]> {
  return invokeMedia('list_tags', {});
}

export async function addTagToMedia(mediaId: number, tagId: number): Promise<void> {
  return invokeMedia('add_tag_to_media', { mediaId, tagId });
}

export async function removeTagFromMedia(mediaId: number, tagId: number): Promise<void> {
  return invokeMedia('remove_tag_from_media', { mediaId, tagId });
}

export async function updateAnnotation(
  mediaId: number,
  annotation: Partial<Annotation>,
): Promise<Annotation> {
  return invokeMedia('update_annotation', { mediaId, annotation });
}

export async function generateThumbnail(id: number, size: number): Promise<string> {
  return invokeMedia('generate_thumbnail', { id, size });
}

export async function generateDescription(id: number, provider: string): Promise<string> {
  return invokeMedia('generate_description', { id, provider });
}

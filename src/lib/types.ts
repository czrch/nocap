// ============================================================================
// Legacy Viewer Types (kept for backward compatibility)
// ============================================================================

export interface ImageFile {
  path: string;
  filename: string;
  extension: string;
}

export interface ViewerState {
  currentImage: ImageFile | null;
  imageList: ImageFile[];
  currentIndex: number;
  zoomLevel: number;
  fitToWindow: boolean;
}

// ============================================================================
// Media Manager Core Types
// ============================================================================

/**
 * Supported media types for the media manager
 */
export type MediaType = 'image' | 'video' | 'audio';

/**
 * Represents a media file in the database.
 * Corresponds to the media_files table.
 */
export interface MediaFile {
  id: number;
  path: string;
  filename: string;
  extension: string;
  mediaType: MediaType;
  fileSize: number | null;
  createdAt: string | null;
  modifiedAt: string | null;
  thumbnailPath: string | null;
}

/**
 * Embedded metadata extracted from media files.
 * Includes EXIF data for images, ID3 tags for audio, and codec info for video.
 * Corresponds to the embedded_metadata table.
 */
export interface EmbeddedMetadata {
  // Dimensions (images/video)
  width: number | null;
  height: number | null;
  durationSeconds: number | null;
  // Image EXIF data
  cameraMake: string | null;
  cameraModel: string | null;
  focalLength: string | null;
  aperture: string | null;
  shutterSpeed: string | null;
  iso: number | null;
  takenAt: string | null;
  gpsLatitude: number | null;
  gpsLongitude: number | null;
  // Audio ID3 tags
  title: string | null;
  artist: string | null;
  album: string | null;
  year: number | null;
  genre: string | null;
}

/**
 * User-defined tag for organizing media.
 * Corresponds to the tags table.
 */
export interface Tag {
  id: number;
  name: string;
  color: string | null;
}

/**
 * User annotations and AI-generated metadata for a media file.
 * Corresponds to the annotations table.
 */
export interface Annotation {
  rating: number; // 0-5 stars
  comment: string | null;
  notes: string | null;
  favorite: boolean;
  aiDescription: string | null;
  aiTags: string[] | null;
}

/**
 * Complete media file information including all related metadata,
 * annotations, and tags. Used for detail view and editing.
 */
export interface MediaFileWithDetails extends MediaFile {
  metadata: EmbeddedMetadata | null;
  annotation: Annotation | null;
  tags: Tag[];
}

/**
 * Represents a media project (a folder being managed).
 * Projects are identified by their root path.
 */
export interface Project {
  path: string;
  name: string;
  mediaCount: number;
  lastOpened: string;
}

/**
 * Filtering options for the media gallery.
 * All filters are ANDed together.
 */
export interface FilterOptions {
  mediaTypes: MediaType[];
  minRating: number;
  maxRating: number;
  tags: number[]; // Tag IDs to filter by
  favorite: boolean | null; // null = no filter, true/false = filter by favorite status
  searchQuery: string;
}

/**
 * Sorting options for the media gallery
 */
export interface SortOptions {
  field: 'filename' | 'createdAt' | 'modifiedAt' | 'rating' | 'fileSize';
  direction: 'asc' | 'desc';
}

/**
 * AI provider configuration for metadata generation.
 * Corresponds to the ai_settings table.
 */
export interface AIProvider {
  provider: 'openai' | 'openrouter' | 'ollama';
  apiKey: string | null;
  baseUrl: string | null;
  model: string;
  isActive: boolean;
}

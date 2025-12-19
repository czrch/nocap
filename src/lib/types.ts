export interface ImageFile {
  path: string;
  filename: string;
  extension: string;
}

export interface ImageMetadata {
  path: string;
  width: number;
  height: number;
  size: number;
  format: string;
}

export interface ExifSummary {
  date_taken: string | null;
  camera_make: string | null;
  camera_model: string | null;
  lens_model: string | null;
  author: string | null;
  description: string | null;
  copyright: string | null;
  software: string | null;
  orientation: string | null;
}

export interface UserMetadata {
  description: string;
  tags: string[];
  rating: number | null;
  author: string;
  date: string | null; // YYYY-MM-DD
}

export interface ViewerState {
  currentImage: ImageFile | null;
  imageList: ImageFile[];
  currentIndex: number;
  zoomLevel: number;
  fitToWindow: boolean;
}

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

export interface ViewerState {
  currentImage: ImageFile | null;
  imageList: ImageFile[];
  currentIndex: number;
  zoomLevel: number;
  fitToWindow: boolean;
}

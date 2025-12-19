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

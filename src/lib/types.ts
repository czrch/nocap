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

export type FsEntryKind = 'file' | 'directory';

export interface FsEntry {
  path: string;
  name: string;
  kind: FsEntryKind;
  children: FsEntry[];
}

export interface FsChangeEvent {
  paths: string[];
}

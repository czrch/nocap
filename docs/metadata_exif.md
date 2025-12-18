# EXIF Inspector (Read-Only)

nocap currently supports **viewing** a small set of common EXIF fields in the right-side Inspector panel.

## Supported files

EXIF viewing is attempted for:
- `jpg` / `jpeg`
- `tif` / `tiff`

Other formats (PNG/WebP/etc.) will typically show **No EXIF data found**.

## Displayed fields (best-effort)

When present in EXIF data, nocap shows:
- Date taken (`DateTimeOriginal`, falling back to `DateTimeDigitized` / `DateTime`)
- Camera make / model
- Lens model
- Author (`Artist`)
- Description (`ImageDescription`)
- Software
- Copyright
- Orientation

If EXIF is missing or malformed, nocap treats it as "no EXIF" (no error UI).


# phone-backup-image 🖼️

Specialist crate for image EXIF metadata extraction, camera settings discovery, perceptual hashing, and photo timeline generation.

## 🏗 Architecture & Modules

- **`domain/`**: Image models (`ImageItem`, `ExifMetadata`, camera model, exposure settings, GPS coordinates, dimensions).
- **`parsers/`**: EXIF metadata parser, JFIF/PNG/HEIC header analyzer, and date-taken timestamp extractors.
- **`timeline/`**: Chronological photo timeline builder clustering images by capture date and location.
- **`exporters/`**: Photo gallery HTML generator, geo-tagged KML map exporter, and JSON catalogs.

## 🚀 Key Features

- **EXIF Metadata Extraction**: Extracts capture timestamp, camera model, ISO, shutter speed, focal length, and GPS geo-tags.
- **HEIC & Next-Gen Format Support**: Seamlessly processes standard JPEG/PNG as well as modern Apple/Samsung HEIC and WebP photo formats.
- **Timeline Organization**: Groups media chronologically for intuitive photo gallery restoration.

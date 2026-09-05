# phone-backup-video 🎥

Specialist crate for video container analysis, MP4/MOV/MKV header inspection, resolution tiering, and video library cataloging.

## 🏗 Architecture & Modules

- **`domain/`**: Video entities (`VideoItem`, `VideoContainer`, `ResolutionTier`, duration, bitrate, frame dimensions).
- **`parsers/`**: Lightweight MP4 `moov`/`mvhd`/`tkhd` box parser and Matroska header analyzer.
- **`tiering/`**: Resolution classifier categorizing video assets into SD, 720p HD, 1080p Full HD, 4K UHD, and 8K.
- **`exporters/`**: Video catalog builder generating HTML galleries, CSV sheets, and JSON manifests.

## 🚀 Key Features

- **Fast Header Inspection**: Extracts resolution, duration, and container format directly from file headers in milliseconds without full video decoding.
- **Automated Quality Tiering**: Groups video files into quality buckets for selective backup policies (e.g. prioritize 4K/UHD camera footage).
- **Format Support**: MP4, QuickTime (MOV), MKV, WebM, 3GP, and AVI.

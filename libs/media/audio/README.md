# phone-backup-audio 🎵

Specialist crate for audio format detection, ID3 tag parsing, voice recording classification, and waveform audio cataloging.

## 🏗 Architecture & Modules

- **`domain/`**: Audio models (`AudioItem`, `AudioCategory`, ID3 tags, artist, album, track, duration, bitrate, codec).
- **`parsers/`**: ID3v1 / ID3v2 tag parser, AAC/M4A metadata extractor, and voice note filename classifier.
- **`classifier/`**: Smart classifier differentiating music tracks, call recordings (`Call_Recording_...`), WhatsApp voice notes (`PTT-...opus`), and system ringtones.
- **`exporters/`**: Audio playlist generator (.m3u8), HTML player catalog, and JSON metadata trees.

## 🚀 Key Features

- **Smart Audio Classification**: Automatically identifies and categorizes music files, voice memos, call recordings, and chat voice notes based on audio headers and naming heuristics.
- **ID3 Tag & Cover Art Metadata**: Extracts song title, artist, album, genre, release year, and duration.
- **Codec Support**: MP3, AAC, M4A, FLAC, OGG, OPUS, WAV, and AMR.

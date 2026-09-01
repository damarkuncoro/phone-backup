-- Identitas Konten (Logical Chunk)
CREATE TABLE IF NOT EXISTS chunks (
    id TEXT PRIMARY KEY,
    content_hash TEXT NOT NULL UNIQUE,
    plaintext_size INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);

-- Representasi Penyimpanan (Physical Object)
CREATE TABLE IF NOT EXISTS chunk_objects (
    id TEXT PRIMARY KEY,
    chunk_id TEXT NOT NULL,
    object_hash TEXT NOT NULL UNIQUE,
    storage_key TEXT NOT NULL,
    stored_size INTEGER NOT NULL,
    compression_alg TEXT NOT NULL,
    encryption_version INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY(chunk_id) REFERENCES chunks(id) ON DELETE CASCADE
);

-- Tabel Utama File
CREATE TABLE IF NOT EXISTS files (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    path TEXT NOT NULL,
    name TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    modified_at TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    permissions TEXT NOT NULL,
    hash_sha256 TEXT,
    thumbnail_hash TEXT,
    media_info TEXT,
    FOREIGN KEY(device_id) REFERENCES devices(id) ON DELETE CASCADE
);

-- Relasi Urutan Chunk dalam sebuah File
CREATE TABLE IF NOT EXISTS file_chunks (
    file_id TEXT NOT NULL,
    chunk_id TEXT NOT NULL,
    chunk_offset INTEGER NOT NULL,
    chunk_length INTEGER NOT NULL,
    sequence INTEGER NOT NULL,
    PRIMARY KEY(file_id, sequence),
    FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY(chunk_id) REFERENCES chunks(id)
);

CREATE INDEX IF NOT EXISTS idx_chunks_hash ON chunks(content_hash);
CREATE INDEX IF NOT EXISTS idx_objects_hash ON chunk_objects(object_hash);
CREATE INDEX IF NOT EXISTS idx_files_device_id ON files(device_id);

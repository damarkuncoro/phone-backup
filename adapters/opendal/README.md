# phone-backup-adapter-opendal

A `StoragePort` implementation using **OpenDAL** for multi-cloud and object storage support.

## 🌩 Multi-Cloud Capabilities

By leveraging OpenDAL, this adapter allows the phone-backup system to store data blobs in various remote backends. Currently configured for:

- **AWS S3**
- **Cloudflare R2**
- **MinIO**
- Any S3-compatible object storage.

## 🛠 Features

- **Blocking Operator**: Optimized for the synchronous backup engine.
- **Environment Support**: Keys and endpoints can be passed via CLI flags or environment variables.
- **Uniform API**: Transparently handles object existence checks (`stat`), writes, and reads across different cloud providers.

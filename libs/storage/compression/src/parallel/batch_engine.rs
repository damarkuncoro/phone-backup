use crate::config::FileMetadataContext;
use crate::engine::smart_engine::SmartCompressionEngine;
use crate::stats::CompressionStats;
use anyhow::Result;
use std::sync::Arc;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Single unit of work in a batch compression request.
#[derive(Debug, Clone)]
pub struct BatchChunkItem<'a> {
    pub id: String,
    pub data: &'a [u8],
    pub context: &'a FileMetadataContext,
}

impl<'a> BatchChunkItem<'a> {
    pub fn new(id: impl Into<String>, data: &'a [u8], context: &'a FileMetadataContext) -> Self {
        Self {
            id: id.into(),
            data,
            context,
        }
    }
}

/// Output result of a parallel batch chunk compression.
#[derive(Debug)]
pub struct BatchCompressedChunk {
    pub id: String,
    pub compressed: Vec<u8>,
    pub stats: CompressionStats,
}

/// Parallel batch compression executor using thread pool throttling.
pub struct ParallelBatchCompressor {
    engine: Arc<SmartCompressionEngine>,
    threads: usize,
}

impl ParallelBatchCompressor {
    pub fn new(engine: Arc<SmartCompressionEngine>, threads: usize) -> Self {
        Self {
            engine,
            threads: threads.max(1),
        }
    }

    /// Automatically sizes the thread pool based on available CPU cores to prevent system locking.
    pub fn with_balanced_cpu(engine: Arc<SmartCompressionEngine>) -> Self {
        #[cfg(feature = "parallel")]
        let cores = rayon::current_num_threads();
        #[cfg(not(feature = "parallel"))]
        let cores = 2;

        let threads = (cores / 2).max(1);
        Self::new(engine, threads)
    }

    /// Compresses a slice of chunk items concurrently.
    pub fn compress_batch<'a>(
        &self,
        items: &[BatchChunkItem<'a>],
    ) -> Vec<Result<BatchCompressedChunk>> {
        #[cfg(feature = "parallel")]
        {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(self.threads)
                .build();

            if let Ok(p) = pool {
                return p.install(|| {
                    items
                        .par_iter()
                        .map(|item| {
                            let (compressed, stats) =
                                self.engine.compress(item.data, item.context)?;
                            Ok(BatchCompressedChunk {
                                id: item.id.clone(),
                                compressed,
                                stats,
                            })
                        })
                        .collect()
                });
            }
        }

        // Fallback or sequential execution
        items
            .iter()
            .map(|item| {
                let (compressed, stats) = self.engine.compress(item.data, item.context)?;
                Ok(BatchCompressedChunk {
                    id: item.id.clone(),
                    compressed,
                    stats,
                })
            })
            .collect()
    }
}

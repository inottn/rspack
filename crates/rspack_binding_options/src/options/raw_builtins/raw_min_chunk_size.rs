use napi_derive::napi;
use rspack_plugin_min_chunk_size::MinChunkSizePluginOptions;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct RawMinChunkSizePluginOptions {
  // Constant overhead for a chunk.
  pub chunk_overhead: Option<f64>,
  // Multiplicator for initial chunks.
  pub entry_chunk_multiplicator: Option<f64>,
  // Minimum number of characters.
  pub min_chunk_size: f64,
}

impl From<RawMinChunkSizePluginOptions> for MinChunkSizePluginOptions {
  fn from(value: RawMinChunkSizePluginOptions) -> Self {
    Self {
      chunk_overhead: value.chunk_overhead,
      entry_chunk_multiplicator: value.entry_chunk_multiplicator,
      min_chunk_size: value.min_chunk_size,
    }
  }
}

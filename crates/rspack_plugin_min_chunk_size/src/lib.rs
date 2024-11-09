use std::cmp::Ordering;

use rspack_collections::UkeyMap;
use rspack_core::{
  ApplyContext, ChunkSizeOptions, ChunkUkey, Compilation, CompilationOptimizeChunks,
  CompilerOptions, Plugin, PluginContext,
};
use rspack_error::Result;
use rspack_hook::{plugin, plugin_hook};

#[plugin]
#[derive(Debug)]
pub struct MinChunkSizePlugin {
  chunk_overhead: Option<f64>,
  entry_chunk_multiplicator: Option<f64>,
  min_chunk_size: f64,
}

impl MinChunkSizePlugin {
  pub fn new(options: MinChunkSizePluginOptions) -> Self {
    let MinChunkSizePluginOptions {
      chunk_overhead,
      entry_chunk_multiplicator,
      min_chunk_size,
    } = options;
    Self::new_inner(chunk_overhead, entry_chunk_multiplicator, min_chunk_size)
  }
}

#[derive(Debug)]
pub struct MinChunkSizePluginOptions {
  pub chunk_overhead: Option<f64>,
  pub entry_chunk_multiplicator: Option<f64>,
  pub min_chunk_size: f64,
}

#[plugin_hook(CompilationOptimizeChunks for MinChunkSizePlugin, stage = Compilation::OPTIMIZE_CHUNKS_STAGE_ADVANCED)]
fn optimize_chunks(&self, compilation: &mut Compilation) -> Result<Option<bool>> {
  let min_chunk_size = self.min_chunk_size;
  let chunks_ukeys = compilation
    .chunk_by_ukey
    .keys()
    .copied()
    .collect::<Vec<_>>();

  let chunk_by_ukey = &compilation.chunk_by_ukey.clone();
  let chunk_group_by_ukey = &compilation.chunk_group_by_ukey.clone();
  let chunk_graph = &compilation.chunk_graph.clone();
  let mut new_chunk_by_ukey = std::mem::take(&mut compilation.chunk_by_ukey);
  let mut new_chunk_group_by_ukey = std::mem::take(&mut compilation.chunk_group_by_ukey);
  let mut new_chunk_graph = std::mem::take(&mut compilation.chunk_graph);

  let chunk_size_options = ChunkSizeOptions {
    chunk_overhead: self.chunk_overhead,
    entry_chunk_multiplicator: self.entry_chunk_multiplicator,
  };
  let module_graph = compilation.get_module_graph();
  let mut combinations = vec![];
  let mut small_chunks = vec![];
  let mut visited_chunks: Vec<&ChunkUkey> = vec![];
  let mut chunk_sizes_map = UkeyMap::default();

  for a in &chunks_ukeys {
    let chunk_size = chunk_graph.get_chunk_size(
      a,
      &ChunkSizeOptions {
        chunk_overhead: Some(1.0),
        entry_chunk_multiplicator: Some(1.0),
      },
      chunk_by_ukey,
      chunk_group_by_ukey,
      &module_graph,
      compilation,
    );

    if chunk_size < min_chunk_size {
      small_chunks.push(a);
      for b in &visited_chunks {
        if chunk_graph.can_chunks_be_integrated(b, a, chunk_by_ukey, chunk_group_by_ukey) {
          combinations.push([b, a]);
        }
      }
    } else {
      for b in &small_chunks {
        if chunk_graph.can_chunks_be_integrated(b, a, chunk_by_ukey, chunk_group_by_ukey) {
          combinations.push([b, a]);
        }
      }
    }

    chunk_sizes_map.insert(
      *a,
      chunk_graph.get_chunk_size(
        a,
        &chunk_size_options,
        chunk_by_ukey,
        chunk_group_by_ukey,
        &module_graph,
        compilation,
      ),
    );
    visited_chunks.push(a);
  }

  let mut sorted_size_filtered_extended_pair_combinations = combinations
    .into_iter()
    .map(|[a, b]| {
      let a_size = *chunk_sizes_map.get(a).expect("should have size");
      let b_size = *chunk_sizes_map.get(b).expect("should have size");
      let integrated_size = chunk_graph.get_integrated_chunks_size(
        a,
        b,
        &chunk_size_options,
        chunk_by_ukey,
        chunk_group_by_ukey,
        &module_graph,
        compilation,
      );

      (a_size + b_size - integrated_size, integrated_size, a, b)
    })
    .collect::<Vec<(f64, f64, &ChunkUkey, &ChunkUkey)>>();

  sorted_size_filtered_extended_pair_combinations.sort_by(|a, b| {
    b.0
      .partial_cmp(&a.0)
      .unwrap_or(Ordering::Equal)
      .then_with(|| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
  });

  if sorted_size_filtered_extended_pair_combinations.is_empty() {
    compilation.chunk_by_ukey = new_chunk_by_ukey;
    compilation.chunk_group_by_ukey = new_chunk_group_by_ukey;
    compilation.chunk_graph = new_chunk_graph;
    return Ok(None);
  }

  if let Some(first) = sorted_size_filtered_extended_pair_combinations.first() {
    new_chunk_graph.integrate_chunks(
      first.2,
      first.3,
      &mut new_chunk_by_ukey,
      &mut new_chunk_group_by_ukey,
      &module_graph,
    );
    new_chunk_by_ukey.remove(first.3);
    compilation.chunk_by_ukey = new_chunk_by_ukey;
    compilation.chunk_group_by_ukey = new_chunk_group_by_ukey;
    compilation.chunk_graph = new_chunk_graph;
  }

  Ok(Some(true))
}

impl Plugin for MinChunkSizePlugin {
  fn name(&self) -> &'static str {
    "rspack.MinChunkSizePlugin"
  }

  fn apply(&self, ctx: PluginContext<&mut ApplyContext>, _options: &CompilerOptions) -> Result<()> {
    ctx
      .context
      .compilation_hooks
      .optimize_chunks
      .tap(optimize_chunks::new(self));
    Ok(())
  }
}

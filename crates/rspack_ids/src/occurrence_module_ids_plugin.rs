use std::collections::HashMap;

use rspack_core::{
  compare_modules_by_pre_order_index_or_identifier, ApplyContext, ChunkGraph, CompilationModuleIds,
  CompilerOptions, ModuleGraph, ModuleIdentifier, Plugin, PluginContext,
};
use rspack_error::Result;
use rspack_hook::{plugin, plugin_hook};

use crate::id_helpers::{assign_ascending_module_ids, get_used_module_ids_and_modules};

pub struct OccurrenceModuleIdsPluginOptions {
  pub prioritise_initial: bool,
}

#[plugin]
#[derive(Debug, Default)]
pub struct OccurrenceModuleIdsPlugin {
  prioritise_initial: bool,
}

impl OccurrenceModuleIdsPlugin {
  pub fn new(option: OccurrenceModuleIdsPluginOptions) -> Self {
    Self::new_inner(option.prioritise_initial)
  }
}

#[plugin_hook(CompilationModuleIds for OccurrenceModuleIdsPlugin)]
fn module_ids(&self, compilation: &mut rspack_core::Compilation) -> Result<()> {
  let (used_ids, mut modules_in_occurrence_order) =
    get_used_module_ids_and_modules(compilation, None);

  let mut chunk_graph = std::mem::take(&mut compilation.chunk_graph);
  let module_graph = compilation.get_module_graph();
  let chunk_group_by_ukey = &compilation.chunk_group_by_ukey;
  let prioritise_initial = self.prioritise_initial;

  let mut occurs_in_initial_chunks_map = HashMap::new();
  let mut occurs_in_all_chunks_map = HashMap::new();

  let mut initial_chunk_chunk_map = HashMap::new();
  let mut entry_count_map = HashMap::new();

  for module_id in &modules_in_occurrence_order {
    let mut initial = 0;
    let mut entry = 0;
    for chunk_ukey in chunk_graph.get_module_chunks(*module_id) {
      let chunk = compilation.chunk_by_ukey.expect_get(chunk_ukey);
      if chunk.can_be_initial(chunk_group_by_ukey) {
        initial += 1;
      }
      if chunk_graph.is_entry_module_in_chunk(module_id, *chunk_ukey) {
        entry += 1;
      }
    }
    initial_chunk_chunk_map.insert(*module_id, initial);
    entry_count_map.insert(*module_id, entry);
  }

  fn count_occurs_in_entry(
    module_graph: &ModuleGraph,
    module_id: &ModuleIdentifier,
    initial_chunk_chunk_map: &HashMap<ModuleIdentifier, usize>,
  ) -> usize {
    let mut sum = 0;

    for (referencing_module, connections) in
      module_graph.get_incoming_connections_by_origin_module(module_id)
    {
      if referencing_module.is_none() {
        continue;
      }

      if !connections
        .iter()
        .any(|connection| connection.is_target_active(module_graph, None))
      {
        continue;
      }

      sum += initial_chunk_chunk_map.get(module_id).unwrap_or(&0);
    }

    sum
  }

  fn count_occurs(
    chunk_graph: &ChunkGraph,
    module_graph: &ModuleGraph,
    module_id: &ModuleIdentifier,
  ) -> usize {
    let mut sum = 0;
    for (referencing_module, connections) in
      module_graph.get_incoming_connections_by_origin_module(module_id)
    {
      let Some(referencing_module) = referencing_module else {
        continue;
      };
      let chunk_modules = chunk_graph.get_number_of_module_chunks(referencing_module);
      for c in connections {
        if !c.is_target_active(module_graph, None) {
          continue;
        }
        let Some(dependency) = module_graph.dependency_by_id(&c.dependency_id) else {
          continue;
        };
        let factor = dependency.get_number_of_id_occurrences();
        if factor == 0 {
          continue;
        }
        sum += factor * chunk_modules;
      }
    }
    sum
  }

  if prioritise_initial {
    for module_id in &modules_in_occurrence_order {
      let result = count_occurs_in_entry(&module_graph, module_id, &initial_chunk_chunk_map)
        + initial_chunk_chunk_map.get(module_id).unwrap_or(&0)
        + entry_count_map.get(module_id).unwrap_or(&0);
      occurs_in_initial_chunks_map.insert(*module_id, result);
    }
  }

  for module_id in &modules_in_occurrence_order {
    let result = count_occurs(&chunk_graph, &module_graph, module_id)
      + chunk_graph.get_number_of_module_chunks(*module_id)
      + entry_count_map.get(module_id).unwrap_or(&0);
    occurs_in_all_chunks_map.insert(*module_id, result);
  }

  modules_in_occurrence_order.sort_unstable_by(|a, b| {
    if prioritise_initial {
      let a_entry_occurs = occurs_in_initial_chunks_map.get(a).unwrap_or(&0);
      let b_entry_occurs = occurs_in_initial_chunks_map.get(b).unwrap_or(&0);
      if a_entry_occurs != b_entry_occurs {
        return b_entry_occurs.cmp(a_entry_occurs);
      }
    }

    let a_occurs = occurs_in_all_chunks_map.get(a).unwrap_or(&0);
    let b_occurs = occurs_in_all_chunks_map.get(b).unwrap_or(&0);
    if a_occurs != b_occurs {
      return b_occurs.cmp(a_occurs);
    }

    compare_modules_by_pre_order_index_or_identifier(&module_graph, a, b)
  });

  let modules_in_occurrence_order = modules_in_occurrence_order
    .into_iter()
    .filter_map(|i| module_graph.module_by_identifier(&i))
    .collect::<Vec<_>>();

  assign_ascending_module_ids(&used_ids, modules_in_occurrence_order, &mut chunk_graph);

  compilation.chunk_graph = chunk_graph;

  Ok(())
}

impl Plugin for OccurrenceModuleIdsPlugin {
  fn name(&self) -> &'static str {
    "OccurrenceModuleIdsPlugin"
  }

  fn apply(&self, ctx: PluginContext<&mut ApplyContext>, _options: &CompilerOptions) -> Result<()> {
    ctx
      .context
      .compilation_hooks
      .module_ids
      .tap(module_ids::new(self));
    Ok(())
  }
}

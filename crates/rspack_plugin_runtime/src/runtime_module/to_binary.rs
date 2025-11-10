use rspack_collections::Identifier;
use rspack_core::{Compilation, RuntimeModule, impl_runtime_module};

#[impl_runtime_module]
#[derive(Debug)]
pub struct ToBinaryRuntimeModule {
  id: Identifier,
}

impl Default for ToBinaryRuntimeModule {
  fn default() -> Self {
    Self::with_default(Identifier::from("webpack/runtime/to_binary"))
  }
}

#[async_trait::async_trait]
impl RuntimeModule for ToBinaryRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn template(&self) -> Vec<(String, String)> {
    vec![(
      self.id.to_string(),
      include_str!("runtime/to_binary.ejs").to_string(),
    )]
  }

  async fn generate(&self, compilation: &Compilation) -> rspack_error::Result<String> {
    let condition_names = &compilation.options.resolve.condition_names.as_ref();
    let is_web_platform =
      condition_names.is_some_and(|names| names.contains(&"browser".to_string()));
    let is_node_platform = condition_names.is_some_and(|names| names.contains(&"node".to_string()));

    let source = compilation.runtime_template.render(
      &self.id,
      Some(serde_json::json!({
        "IS_WEB_PLATFORM": is_web_platform,
        "IS_NODE_PLATFORM": is_node_platform,
        "IS_NEUTRAL_PLATFORM": !is_web_platform && !is_node_platform,
      })),
    )?;

    Ok(source)
  }
}

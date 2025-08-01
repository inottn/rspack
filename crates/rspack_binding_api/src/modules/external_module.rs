use crate::{impl_module_methods, Module, MODULE_PROPERTIES_BUFFER};

#[napi]
#[repr(C)]
pub struct ExternalModule {
  pub(crate) module: Module,
}

impl ExternalModule {
  pub(crate) fn custom_into_instance(
    mut self,
    env: &napi::Env,
  ) -> napi::Result<napi::bindgen_prelude::ClassInstance<Self>> {
    let (_, module) = self.as_ref()?;
    let user_request = env.create_string(module.user_request())?;

    MODULE_PROPERTIES_BUFFER.with(|ref_cell| {
      let mut properties = ref_cell.borrow_mut();
      properties.clear();

      properties.push(
        napi::Property::new()
          .with_utf8_name("userRequest")?
          .with_value(&user_request),
      );
      Self::new_inherited(self, env, &mut properties)
    })
  }

  fn as_ref(&mut self) -> napi::Result<(&rspack_core::Compilation, &rspack_core::ExternalModule)> {
    let (compilation, module) = self.module.as_ref()?;
    match module.as_external_module() {
      Some(external_module) => Ok((compilation, external_module)),
      None => Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Module is not a ExternalModule",
      )),
    }
  }
}

impl_module_methods!(ExternalModule);

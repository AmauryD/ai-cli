use core::gpt_call::GptCallImpl;
use crate::{config::load_from_file::LoadConfig, context::get_context::GetContext};

pub fn resolve_gpt<T: LoadConfig, C: GetContext>(
    config_loader: T,
    context_loader: C
) -> Result<GptCallImpl, String> {
   Ok(GptCallImpl::new(
        config_loader.load_config().map_err(|e| e.to_string())?, 
        context_loader.get_context().map_err(|e| e.to_string())?
    ))
}
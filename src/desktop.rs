use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<NoAccessoryView<R>> {
  Ok(NoAccessoryView(app.clone()))
}

/// Access to the no-accessory-view APIs.
pub struct NoAccessoryView<R: Runtime>(AppHandle<R>);

impl<R: Runtime> NoAccessoryView<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}

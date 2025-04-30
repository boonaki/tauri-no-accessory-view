use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::NoAccessoryView;
#[cfg(mobile)]
use mobile::NoAccessoryView;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the no-accessory-view APIs.
pub trait NoAccessoryViewExt<R: Runtime> {
  fn no_accessory_view(&self) -> &NoAccessoryView<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NoAccessoryViewExt<R> for T {
  fn no_accessory_view(&self) -> &NoAccessoryView<R> {
    self.state::<NoAccessoryView<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("no-accessory-view")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(target_os = "ios")]
      let no_accessory_view = mobile::init(app, api)?;
      #[cfg(desktop)]
      let no_accessory_view = desktop::init(app, api)?;
      app.manage(no_accessory_view);
      Ok(())
    })
    .build()
}

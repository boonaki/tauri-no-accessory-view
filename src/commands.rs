use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::NoAccessoryViewExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.no_accessory_view().ping(payload)
}

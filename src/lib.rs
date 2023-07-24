use serde::{ser::Serializer, Deserialize, Serialize};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State, Window,
};

use everything_rs::{Everything, EverythingError, EverythingRequestFlags, EverythingSort};
use std::sync::RwLock;
use ts_rs::TS;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Everything(#[from] EverythingError),

    #[error("Lock poisoned")]
    Poisoned,

    #[error("Invalid Request Flags")]
    InvalidRequestFlags,
}

impl From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, Everything>>> for Error {
    fn from(_: std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, Everything>>) -> Self {
        Self::Poisoned
    }
}

impl From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, Everything>>> for Error {
    fn from(_: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, Everything>>) -> Self {
        Self::Poisoned
    }
}

type Result<T> = std::result::Result<T, Error>;

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

struct EverythingState(RwLock<Everything>);

impl Default for EverythingState {
    fn default() -> Self {
        Self(RwLock::new(Everything::new()))
    }
}

#[command]
async fn set_search<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, EverythingState>,
    query: String,
) -> Result<()> {
    let ev = state.0.write()?;
    ev.set_search(&query);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub enum RequestFlags {
    FileName,
    Path,
    FullPathAndFileName,
    Extension,
    Size,
    DateCreated,
    DateModified,
    DateAccessed,
    Attributes,
    FileListFileName,
    RunCount,
    DateRun,
    DateRecentlyChanged,
    HighlightedFileName,
    HighlightedPath,
    HighlightedFullPathAndFileName,
}

impl From<RequestFlags> for EverythingRequestFlags {
    fn from(value: RequestFlags) -> Self {
        match value {
            RequestFlags::FileName => EverythingRequestFlags::FileName,
            RequestFlags::Path => EverythingRequestFlags::Path,
            RequestFlags::FullPathAndFileName => EverythingRequestFlags::FullPathAndFileName,
            RequestFlags::Extension => EverythingRequestFlags::Extension,
            RequestFlags::Size => EverythingRequestFlags::Size,
            RequestFlags::DateCreated => EverythingRequestFlags::DateCreated,
            RequestFlags::DateModified => EverythingRequestFlags::DateModified,
            RequestFlags::DateAccessed => EverythingRequestFlags::DateAccessed,
            RequestFlags::Attributes => EverythingRequestFlags::Attributes,
            RequestFlags::FileListFileName => EverythingRequestFlags::FileListFileName,
            RequestFlags::RunCount => EverythingRequestFlags::RunCount,
            RequestFlags::DateRun => EverythingRequestFlags::DateRun,
            RequestFlags::DateRecentlyChanged => EverythingRequestFlags::DateRecentlyChanged,
            RequestFlags::HighlightedFileName => EverythingRequestFlags::HighlightedFileName,
            RequestFlags::HighlightedPath => EverythingRequestFlags::HighlightedPath,
            RequestFlags::HighlightedFullPathAndFileName => {
                EverythingRequestFlags::HighlightedFullPathAndFileName
            }
        }
    }
}

#[command]
async fn set_request_flags(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
    flags: Vec<RequestFlags>,
) -> Result<()> {
    let ev = state.0.write()?;
    let mut flags = flags;
    if let Some(flag) = flags.pop() {
        let mut flags = EverythingRequestFlags::from(flag);
        for flag in flags {
            flags |= EverythingRequestFlags::from(flag);
        }
        ev.set_request_flags(flags);
        Ok(())
    } else {
        Err(Error::InvalidRequestFlags)
    }
}

#[command]
async fn set_sort(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
    sort: EverythingSort,
) -> Result<()> {
    let ev = state.0.write()?;
    ev.set_sort(sort);
    Ok(())
}

#[command]
async fn set_result_offset(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
    offset: u32,
) -> Result<()> {
    let ev = state.0.write()?;
    ev.set_result_offset(offset);
    Ok(())
}

#[command]
async fn set_max_results(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
    max_results: u32,
) -> Result<()> {
    let ev = state.0.write()?;
    ev.set_max_results(max_results);
    Ok(())
}

#[command]
async fn query(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
) -> Result<()> {
    let ev = state.0.write()?;
    ev.query()?;
    Ok(())
}

#[command]
async fn get_num_results(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
) -> Result<u32> {
    let ev = state.0.read()?;
    Ok(ev.get_total_results())
}

#[command]
async fn get_full_path_results(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
) -> Result<Vec<String>> {
    let ev = state.0.read()?;
    let mut file_res = Vec::with_capacity(ev.get_max_results() as usize);
    for file_path in ev.full_path_iter() {
        if let Ok(file_path) = file_path {
            file_res.push(file_path);
        }
    }
    Ok(file_res)
}

#[command]
async fn get_result_full_path(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
    index: u32,
) -> Result<String> {
    let ev = state.0.read()?;
    Ok(ev.get_result_full_path(index)?)
}

#[command]
async fn get_file_name_results(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
) -> Result<Vec<String>> {
    let ev = state.0.read()?;
    let mut file_res = Vec::with_capacity(ev.get_max_results() as usize);
    for file_name in ev.name_iter() {
        if let Ok(file_name) = file_name {
            file_res.push(file_name);
        }
    }
    Ok(file_res)
}

#[command]
async fn get_result_file_name(
    _app: AppHandle<impl Runtime>,
    _window: Window<impl Runtime>,
    state: State<'_, EverythingState>,
    index: u32,
) -> Result<String> {
    let ev = state.0.read()?;
    Ok(ev.get_result_file_name(index)?)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("everything")
        .invoke_handler(tauri::generate_handler![
            set_search,
            set_request_flags,
            set_sort,
            set_result_offset,
            set_max_results,
            query,
            get_num_results,
            get_full_path_results,
            get_result_full_path,
            get_file_name_results,
            get_result_file_name
        ])
        .setup(|app| {
            app.manage(EverythingState::default());
            Ok(())
        })
        .build()
}

use app_core::domain::theme::ThemeMode;
use tauri::{AppHandle, Manager, State, WebviewWindow};
use tauri_plugin_store::StoreExt;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

/// FE-06: フロントの初期化が完了したらメイン画面から呼び出す。スプラッシュを閉じて
/// メインウィンドウを表示する。デスクトップ・モバイル双方の window 構成で動作する。
#[tauri::command]
#[specta::specta]
pub fn close_splashscreen(app: AppHandle) -> AppResult<()> {
    if let Some(splash) = app.get_webview_window("splashscreen") {
        splash.close().map_err(|e| AppError::Io(e.to_string()))?;
    }
    if let Some(main) = app.get_webview_window("main") {
        main.show().map_err(|e| AppError::Io(e.to_string()))?;
    }
    Ok(())
}

pub(crate) const THEME_STORE_FILE: &str = "settings.json";
pub(crate) const THEME_STORE_KEY: &str = "theme";

/// テーマ記事の検証用コマンド。`light`/`dark` は明示的な `Theme` を、`system` は `None` を
/// `set_theme` に渡す。`None` を渡した場合だけ `WindowEvent::ThemeChanged` が配信されるという
/// ドキュメント記載の非対称性を、この経路の実装で実機検証する。
#[tauri::command]
#[specta::specta]
pub fn set_theme_value(
    value: u8,
    window: WebviewWindow,
    app: AppHandle,
    state: State<AppState>,
) -> AppResult<u8> {
    let mode = ThemeMode::from_u8(value);
    let requested = match mode {
        ThemeMode::Light => Some(tauri::Theme::Light),
        ThemeMode::Dark => Some(tauri::Theme::Dark),
        ThemeMode::System => None,
    };

    window
        .set_theme(requested)
        .map_err(|e| AppError::Io(e.to_string()))?;
    let resolved = window.theme().map_err(|e| AppError::Io(e.to_string()))?;

    state.set_theme_value(mode.to_u8());
    persist_theme_value(&app, mode.to_u8())?;

    Ok(if resolved == tauri::Theme::Dark { 1 } else { 0 })
}

/// `AppState` が保持する現在のテーマ数値をそのまま返す。起動直後、まだ
/// `theme-changed` イベントが発生していない時点の初期表示に使う。
#[tauri::command]
#[specta::specta]
pub fn get_theme_value(state: State<AppState>) -> u8 {
    state.theme_value()
}

pub(crate) fn persist_theme_value(app: &AppHandle, value: u8) -> AppResult<()> {
    let store = app
        .store(THEME_STORE_FILE)
        .map_err(|e| AppError::Io(e.to_string()))?;
    store.set(THEME_STORE_KEY, value);
    store.save().map_err(|e| AppError::Io(e.to_string()))?;
    Ok(())
}

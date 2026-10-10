use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

/// アプリ全体で共有する状態の雛形（RS-06）。
///
/// - `tauri::State` 経由でコマンドに注入する
/// - ロックを保持したまま `await` しない（レビュー観点 §2）。値を取り出したら早めに drop する
#[derive(Default)]
pub struct AppState {
    inner: Arc<RwLock<AppStateInner>>,
    next_task_id: AtomicU64,
    tasks: Arc<RwLock<HashMap<String, CancellationToken>>>,
    /// `app_core::domain::theme::ThemeMode` の数値表現。ウィンドウ生成前後で読み書きするため
    /// 同期の `std::sync::Mutex` で持つ（`await` を挟まない短い操作のみ）。
    theme: std::sync::Mutex<u8>,
}

#[derive(Default)]
struct AppStateInner {
    /// サンプル値。実際の永続設定は tauri-plugin-store（RS-08）を使う。
    greet_count: u64,
}

impl AppState {
    pub async fn record_greeting(&self) -> u64 {
        let mut inner = self.inner.write().await;
        inner.greet_count += 1;
        inner.greet_count
    }

    pub fn theme_value(&self) -> u8 {
        *self
            .theme
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn set_theme_value(&self, value: u8) {
        *self
            .theme
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = value;
    }

    /// RS-10: 長時間処理の雛形。新しいタスク ID を発行し、キャンセル用トークンを登録する。
    pub async fn register_task(&self) -> (String, CancellationToken) {
        let id = self.next_task_id.fetch_add(1, Ordering::SeqCst);
        let task_id = format!("task-{id}");
        let token = CancellationToken::new();

        let mut tasks = self.tasks.write().await;
        tasks.insert(task_id.clone(), token.clone());
        drop(tasks);

        (task_id, token)
    }

    pub async fn cancel_task(&self, task_id: &str) {
        let tasks = self.tasks.read().await;
        if let Some(token) = tasks.get(task_id) {
            token.cancel();
        }
    }

    pub async fn finish_task(&self, task_id: &str) {
        let mut tasks = self.tasks.write().await;
        tasks.remove(task_id);
    }
}

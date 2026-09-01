/// UI に表示するテーマの3状態と、フロント・ストア間でやり取りする数値表現の相互変換。
/// `tauri` に依存しない純粋ロジックとして置き、ウィンドウ描画への反映は
/// `src-tauri/src/commands/window.rs` が担う。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl ThemeMode {
    /// 未知の数値は `System` として扱う。
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Light,
            1 => Self::Dark,
            _ => Self::System,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::Light => 0,
            Self::Dark => 1,
            Self::System => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_zero_to_light() {
        assert_eq!(ThemeMode::from_u8(0), ThemeMode::Light);
    }

    #[test]
    fn converts_one_to_dark() {
        assert_eq!(ThemeMode::from_u8(1), ThemeMode::Dark);
    }

    #[test]
    fn converts_two_to_system() {
        assert_eq!(ThemeMode::from_u8(2), ThemeMode::System);
    }

    #[test]
    fn falls_back_to_system_for_unknown_values() {
        assert_eq!(ThemeMode::from_u8(255), ThemeMode::System);
    }

    #[test]
    fn round_trips_through_u8() {
        for mode in [ThemeMode::Light, ThemeMode::Dark, ThemeMode::System] {
            assert_eq!(ThemeMode::from_u8(mode.to_u8()), mode);
        }
    }
}

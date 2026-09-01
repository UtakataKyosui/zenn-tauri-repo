import { getThemeValue, setThemeValue } from "@/lib/api/theme";
import { type Theme, useUiStore } from "@/stores/ui-store";
import { useEffect, useRef } from "react";

const THEME_VALUES: Record<Theme, number> = { light: 0, dark: 1, system: 2 };
const VALUE_TO_THEME: Record<number, Theme> = { 0: "light", 1: "dark", 2: "system" };

/**
 * FE-02: ライト／ダーク／OS 追従のテーマ切替。`.dark` クラスの付与だけを行い、
 * 実際の色はトークン（src/styles/globals.css）が持つ。
 * Rust 側へは `setThemeValue` でウィンドウのネイティブ描画切り替えと選択の永続化を依頼する。
 * ブラウザプレビュー等 Tauri IPC が使えない環境でも UI 表示に影響しないよう、失敗は無視する。
 *
 * マウント直後は Zustand の初期値（system）ではなく、`getThemeValue` で Rust 側が
 * 前回永続化した値を読み込んでから同期する。ここを飛ばすと、起動直後に system の
 * 初期値をそのまま push してしまい、Rust 側が復元した選択を上書きしてしまう。
 */
export function useTheme() {
  const theme = useUiStore((s) => s.theme);
  const setTheme = useUiStore((s) => s.setTheme);
  const hydrated = useRef(false);

  useEffect(() => {
    const root = document.documentElement;
    const media = window.matchMedia("(prefers-color-scheme: dark)");

    const apply = () => {
      const isDark = theme === "dark" || (theme === "system" && media.matches);
      root.classList.toggle("dark", isDark);
    };

    apply();

    if (theme === "system") {
      media.addEventListener("change", apply);
      return () => media.removeEventListener("change", apply);
    }
    return undefined;
  }, [theme]);

  // biome-ignore lint/correctness/useExhaustiveDependencies: 起動時に一度だけ実行する
  useEffect(() => {
    let cancelled = false;
    getThemeValue()
      .then((value) => {
        if (cancelled) return;
        hydrated.current = true;
        setTheme(VALUE_TO_THEME[value] ?? "system");
      })
      .catch(() => {
        hydrated.current = true;
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!hydrated.current) return;
    setThemeValue(THEME_VALUES[theme]).catch(() => undefined);
  }, [theme]);

  return { theme, setTheme };
}

import { getThemeValue } from "@/lib/api/theme";
import { useUiStore } from "@/stores/ui-store";
import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useTheme } from "./use-theme";

vi.mock("@/lib/api/theme", () => ({
  setThemeValue: vi.fn().mockResolvedValue(0),
  getThemeValue: vi.fn().mockResolvedValue(2),
}));

const THEME_TO_VALUE = { light: 0, dark: 1, system: 2 } as const;

function mockMatchMedia(matches: boolean) {
  const listeners = new Set<(e: MediaQueryListEvent) => void>();
  const mql = {
    matches,
    media: "(prefers-color-scheme: dark)",
    addEventListener: (_: string, cb: (e: MediaQueryListEvent) => void) => listeners.add(cb),
    removeEventListener: (_: string, cb: (e: MediaQueryListEvent) => void) => listeners.delete(cb),
  } as unknown as MediaQueryList;
  vi.stubGlobal("matchMedia", vi.fn().mockReturnValue(mql));
  return mql;
}

describe("useTheme", () => {
  beforeEach(() => {
    useUiStore.setState({ theme: "system" });
    document.documentElement.classList.remove("dark");
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("adds the dark class when theme is dark", () => {
    mockMatchMedia(false);
    useUiStore.setState({ theme: "dark" });
    vi.mocked(getThemeValue).mockResolvedValue(THEME_TO_VALUE.dark);

    renderHook(() => useTheme());

    expect(document.documentElement.classList.contains("dark")).toBe(true);
  });

  it("removes the dark class when theme is light", () => {
    mockMatchMedia(true);
    useUiStore.setState({ theme: "light" });
    vi.mocked(getThemeValue).mockResolvedValue(THEME_TO_VALUE.light);

    renderHook(() => useTheme());

    expect(document.documentElement.classList.contains("dark")).toBe(false);
  });

  it("follows the OS preference when theme is system", () => {
    mockMatchMedia(true);
    useUiStore.setState({ theme: "system" });
    vi.mocked(getThemeValue).mockResolvedValue(THEME_TO_VALUE.system);

    renderHook(() => useTheme());

    expect(document.documentElement.classList.contains("dark")).toBe(true);
  });

  it("exposes the current theme and a setter", async () => {
    useUiStore.setState({ theme: "light" });
    mockMatchMedia(false);
    vi.mocked(getThemeValue).mockResolvedValue(THEME_TO_VALUE.light);

    const { result } = renderHook(() => useTheme());
    await act(async () => {
      await Promise.resolve();
    });

    expect(result.current.theme).toBe("light");
    act(() => result.current.setTheme("dark"));
    expect(useUiStore.getState().theme).toBe("dark");
  });

  it("hydrates the store from the value Rust restored on startup", async () => {
    mockMatchMedia(false);
    useUiStore.setState({ theme: "system" });
    vi.mocked(getThemeValue).mockResolvedValue(THEME_TO_VALUE.dark);

    const { result } = renderHook(() => useTheme());
    await act(async () => {
      await Promise.resolve();
    });

    expect(result.current.theme).toBe("dark");
  });
});

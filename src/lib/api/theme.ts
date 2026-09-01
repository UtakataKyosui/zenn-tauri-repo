import { commands } from "@/lib/bindings";

/**
 * `bindings.ts` を直接呼ばず、ここで一段ラップする（docs/testing.md §3）。
 * `value` は 0=light, 1=dark, 2=system。戻り値は `set_theme` 後に実際に採用された
 * `Theme`（0=light, 1=dark）で、`system` を渡した場合も呼び出し側が判定不要になる。
 */
export async function setThemeValue(value: number): Promise<number> {
  const result = await commands.setThemeValue(value);
  if (result.status === "error") {
    throw new Error(describeError(result.error));
  }
  return result.data;
}

export async function getThemeValue(): Promise<number> {
  return commands.getThemeValue();
}

function describeError(error: unknown): string {
  if (
    typeof error === "object" &&
    error !== null &&
    "kind" in error &&
    (error as { kind: unknown }).kind === "Core"
  ) {
    const core = (error as unknown as { message: { kind: string; message: unknown } }).message;
    return `${core.kind}: ${core.message ?? ""}`;
  }
  return "unexpected error";
}

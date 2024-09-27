#!/usr/bin/env -S deno run --allow-read --allow-write
// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

const exports = {
  symbols: [
    "uv_default_loop",
    "uv_close",
    "uv_idle_init",
  ],
};

const symbolExportLists = {
  linux: `{ ${exports.symbols.map((s) => `"${s}"`).join("; ")}; };`,
  windows: `LIBRARY\nEXPORTS\n${
    exports.symbols
      .map((symbol) => "  " + symbol)
      .join("\n")
  }`,
  macos: exports.symbols.map((symbol) => "_" + symbol).join("\n"),
};

for await (const [os, def] of Object.entries(symbolExportLists)) {
  const defUrl = new URL(
    `./generated_symbol_exports_list_${os}.def`,
    import.meta.url,
  );
  await Deno.writeTextFile(defUrl.pathname, def, { create: true });
}

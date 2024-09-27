import { indented } from "./util.ts";
import $ from "jsr:@david/dax";

async function findRoot() {
  if (await $.path("libuv-1.48.0-include").exists()) {
    return;
  }
  Deno.chdir("../");
  if (!await $.path("libuv-1.48.0-include").exists()) {
    throw new Error("Couldn't find include path");
  }
}

async function generateFunctions() {
  const temp = await Deno.makeTempFile();
  await $`bindgen -o ${temp} --merge-extern-blocks --allowlist-function="uv_.+" --blocklist-function="uv_loop_configure" --no-recursive-allowlist  libuv-1.48.0-include/uv.h -- -Ilibuv-1.48.0-include/`;

  const contents = await Deno.readTextFile(temp);

  const indentedContents = contents.split("\n").map((s) =>
    " ".repeat(6) + s.replace("pub ", "")
  )
    .join("\n");

  const sliced = indentedContents.substring(indentedContents.indexOf("extern"));

  const generated = indented`
    use crate::generate;
    use crate::types::*;

    generate! {
      ${sliced}
    }
    extern "C" {
      pub fn uv_loop_configure(loop_: *mut uv_loop_t, option: uv_loop_option, ...);
    }
  `;

  await Deno.writeTextFile("./src/functions.rs", generated);

  await $`cargo fmt`;
}

await findRoot();
await generateFunctions();

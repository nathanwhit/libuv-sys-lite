import { join } from "@std/path";
const files: string[] = [];
const base = "../libuv-1.48.0/include/";
const bases = [base, join(base, "uv")];
for (const base of bases) {
  for await (const entry of Deno.readDir(base)) {
    if (entry.isFile) {
      files.push(join(base, entry.name));
    }
  }
}

let out = "";
for (const file of files) {
  out += "--allowlist-file=" + file.replace(/^\.\.\//, "") + " ";
}
console.log(out);

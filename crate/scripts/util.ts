export function trimIndent(str: string): string {
  // Split the string into an array of lines
  const lines = str.split("\n");

  // Find the minimum indentation across all lines
  const minIndent = lines.reduce((min, line) => {
    const trimmed = line.trimStart();
    return Math.min(min, line.length - trimmed.length);
  }, Infinity);

  // Trim the leading whitespace from each line based on the minimum indentation
  return lines.map((line) => line.slice(minIndent)).join("\n");
}

const identity = (strings: TemplateStringsArray, ...values: string[]) =>
  String.raw({ raw: strings }, ...values);

export function indented(strings: TemplateStringsArray, ...values: string[]) {
  const substituted = identity(strings, ...values);
  return trimIndent(substituted);
}

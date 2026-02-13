import { execSync } from "child_process";
import { readFileSync } from "fs";
import { join } from "path";

export interface Variables {
  TEMPLATE_VERSION: string;
  POLKADOT_SDK_VERSION: string;
  ZOMBIENET_VERSION: string;
  POLKADOT_OMNI_NODE_VERSION: string;
  CHAIN_SPEC_BUILDER_VERSION: string;
}

interface YamlNode {
  [key: string]: string | YamlNode;
}

function parseSimpleYaml(content: string): YamlNode {
  const root: YamlNode = {};
  const stack: { indent: number; obj: YamlNode }[] = [{ indent: -1, obj: root }];

  for (const rawLine of content.split("\n")) {
    // Skip comments and blank lines
    if (/^\s*(#|$)/.test(rawLine)) continue;

    const match = rawLine.match(/^(\s*)([\w]+):\s*(.*)/);
    if (!match) continue;

    const indent = match[1].length;
    const key = match[2];
    let value = match[3].trim();

    // Pop stack to find parent at lower indent
    while (stack.length > 1 && stack[stack.length - 1].indent >= indent) {
      stack.pop();
    }
    const parent = stack[stack.length - 1].obj;

    if (value === "" || value.startsWith("#")) {
      // This key is a parent node
      const child: YamlNode = {};
      parent[key] = child;
      stack.push({ indent, obj: child });
    } else {
      // Strip surrounding quotes
      value = value.replace(/^["']|["']$/g, "");
      // Strip inline comments
      value = value.replace(/\s+#.*$/, "");
      parent[key] = value;
    }
  }

  return root;
}

function getNestedValue(obj: YamlNode, path: string): string | undefined {
  const keys = path.split(".");
  let current: string | YamlNode = obj;
  for (const key of keys) {
    if (typeof current !== "object" || current === null) return undefined;
    current = current[key];
    if (current === undefined) return undefined;
  }
  return typeof current === "string" ? current : undefined;
}

export function loadVariables(): Variables {
  const repoRoot = execSync("git rev-parse --show-toplevel", {
    encoding: "utf-8",
  }).trim();

  const content = readFileSync(join(repoRoot, "versions.yml"), "utf-8");
  const yaml = parseSimpleYaml(content);

  // Resolve SDK version: template override takes precedence over root default
  const sdkVersion =
    getNestedValue(yaml, "parachain_template.polkadot_sdk.release_tag") ??
    getNestedValue(yaml, "polkadot_sdk.release_tag");

  if (!sdkVersion) throw new Error("Missing polkadot_sdk.release_tag in versions.yml");

  const templateVersion = getNestedValue(yaml, "parachain_template.version");
  if (!templateVersion) throw new Error("Missing parachain_template.version in versions.yml");

  const zombienetVersion = getNestedValue(yaml, "zombienet.version");
  if (!zombienetVersion) throw new Error("Missing zombienet.version in versions.yml");

  const omniNodeVersion = getNestedValue(yaml, "parachain_template.crates.polkadot_omni_node.version");
  if (!omniNodeVersion) throw new Error("Missing parachain_template.crates.polkadot_omni_node.version in versions.yml");

  const chainSpecVersion = getNestedValue(yaml, "parachain_template.crates.chain_spec_builder.version");
  if (!chainSpecVersion) throw new Error("Missing parachain_template.crates.chain_spec_builder.version in versions.yml");

  return {
    TEMPLATE_VERSION: templateVersion,
    POLKADOT_SDK_VERSION: sdkVersion,
    ZOMBIENET_VERSION: zombienetVersion,
    POLKADOT_OMNI_NODE_VERSION: omniNodeVersion,
    CHAIN_SPEC_BUILDER_VERSION: chainSpecVersion,
  };
}

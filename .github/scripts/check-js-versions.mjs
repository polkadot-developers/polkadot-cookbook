#!/usr/bin/env node
// Treats versions.yml javascript_packages entries as a minimum floor: reports
// when any package.json under polkadot-docs/, recipes/, migration/ pins a
// tracked dep to a version older than the floor. A harness ahead of the pin
// is allowed — bump versions.yml if the cookbook standard should catch up.
// Pure Node, no dependencies.
//
// Usage:
//   node .github/scripts/check-js-versions.mjs              # dev mode, exits 1 on drift
//   node .github/scripts/check-js-versions.mjs --fix        # rewrite behind specs to the floor
//   node .github/scripts/check-js-versions.mjs --markdown   # emit a markdown report, always exit 0 (used by the issue-manager workflow)

import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..");
const SCAN_ROOTS = ["polkadot-docs", "recipes", "migration"];
const SKIP_DIRS = new Set(["node_modules", ".test-workspace", ".papi"]);

function loadTrackedVersions() {
  const text = readFileSync(join(REPO_ROOT, "versions.yml"), "utf8");
  const map = new Map();
  let inBlock = false;
  let pendingName = null;
  for (const line of text.split("\n")) {
    if (/^[A-Za-z_]/.test(line)) {
      inBlock = line.startsWith("javascript_packages:");
      pendingName = null;
      continue;
    }
    if (!inBlock) continue;
    const name = line.match(/^ {4}name:\s*"?([^"\s]+)"?\s*$/);
    if (name) {
      pendingName = name[1];
      continue;
    }
    const version = line.match(/^ {4}version:\s*"?([^"\s]+)"?\s*$/);
    if (version && pendingName) {
      map.set(pendingName, version[1]);
      pendingName = null;
    }
  }
  if (map.size === 0) {
    throw new Error("Parsed 0 entries from versions.yml javascript_packages — parser broken or section missing");
  }
  return map;
}

function findPackageJsonFiles() {
  const results = [];
  for (const root of SCAN_ROOTS) {
    walk(join(REPO_ROOT, root), results);
  }
  return results.sort();
}

function walk(dir, out) {
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return;
  }
  for (const entry of entries) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) {
      if (SKIP_DIRS.has(entry.name)) continue;
      walk(p, out);
    } else if (entry.isFile() && entry.name === "package.json") {
      out.push(p);
    }
  }
}

// Splits "^1.2.3" into { prefix: "^", version: "1.2.3" }. Returns null for
// non-plain-semver specs (file:, workspace:, git+, url, tag, range, etc.) so
// the caller skips them. Pre-release suffixes intentionally unsupported —
// versions.yml entries are always plain "x.y.z".
function parseSemverSpec(spec) {
  const m = spec.match(/^([\^~]?)(\d+\.\d+\.\d+)$/);
  if (!m) return null;
  return { prefix: m[1], version: m[2] };
}

// Numeric 3-part semver compare. Returns negative if a<b, positive if a>b, 0
// if equal. Both arguments must match /^\d+\.\d+\.\d+$/.
function compareSemver(a, b) {
  const pa = a.split(".").map(Number);
  const pb = b.split(".").map(Number);
  for (let i = 0; i < 3; i++) {
    const d = (pa[i] ?? 0) - (pb[i] ?? 0);
    if (d !== 0) return d;
  }
  return 0;
}

function analyze(pkgPath, tracked) {
  const raw = readFileSync(pkgPath, "utf8");
  const pkg = JSON.parse(raw);
  const drifts = [];
  for (const section of ["dependencies", "devDependencies"]) {
    const deps = pkg[section];
    if (!deps) continue;
    for (const [name, spec] of Object.entries(deps)) {
      const floor = tracked.get(name);
      if (!floor) continue;
      const parsed = parseSemverSpec(spec);
      if (!parsed) continue;
      if (compareSemver(parsed.version, floor) < 0) {
        drifts.push({ section, name, spec, floor, prefix: parsed.prefix });
      }
    }
  }
  return { raw, pkg, drifts };
}

function applyFix(pkgPath, raw, drifts) {
  // Edit the raw text to preserve formatting. Each drifted entry has a unique
  // `"name": "spec"` occurrence within its section — we rewrite by exact match.
  let text = raw;
  for (const d of drifts) {
    const needle = `"${d.name}": "${d.spec}"`;
    const replacement = `"${d.name}": "${d.prefix}${d.floor}"`;
    if (!text.includes(needle)) {
      throw new Error(`Cannot locate \`${needle}\` in ${pkgPath} for --fix`);
    }
    text = text.replace(needle, replacement);
  }
  writeFileSync(pkgPath, text);
}

// Stable heading the issue-manager workflow greps for to decide whether to
// open/close the drift issue. Keep in sync with check-js-versions.yml.
const DRIFT_HEADING = "## JS Dependency Drift";
const CLEAN_HEADING = "## JS Dependencies: OK";

function emitMarkdown(driftedFiles, totalDrifts, tracked, pkgFilesCount) {
  if (driftedFiles.length === 0) {
    console.log(CLEAN_HEADING);
    console.log("");
    console.log(`All ${tracked.size} tracked JS packages are at or above the \`versions.yml\` floor across ${pkgFilesCount} \`package.json\` files.`);
    return;
  }
  console.log(DRIFT_HEADING);
  console.log("");
  console.log(`${totalDrifts} tracked dependency spec${totalDrifts === 1 ? "" : "s"} across ${driftedFiles.length} \`package.json\` file${driftedFiles.length === 1 ? "" : "s"} ${totalDrifts === 1 ? "is" : "are"} below the floor declared in [\`versions.yml\`](../blob/master/versions.yml) (\`javascript_packages\`).`);
  console.log("");
  console.log("| File | Dependency | Current | Floor |");
  console.log("|------|------------|---------|-------|");
  for (const { pkgPath, drifts } of driftedFiles) {
    const rel = relative(REPO_ROOT, pkgPath);
    for (const d of drifts) {
      console.log(`| \`${rel}\` | \`${d.name}\` | \`${d.spec}\` | \`${d.floor}\` |`);
    }
  }
  console.log("");
  console.log("### How to resolve");
  console.log("");
  console.log("Either raise the specs in `package.json` to the floor (recommended when the floor is correct):");
  console.log("");
  console.log("```bash");
  console.log("node .github/scripts/check-js-versions.mjs --fix");
  console.log("# then, in each affected harness:");
  console.log("npm install");
  console.log("```");
  console.log("");
  console.log("…or lower the tracked version in `versions.yml` if the cookbook should track an older pin.");
  console.log("");
  console.log("---");
  console.log("");
  console.log("*This issue is managed automatically by [`.github/workflows/check-js-versions.yml`](../blob/master/.github/workflows/check-js-versions.yml). It will close itself once every tracked dep is at or above the floor on `master`.*");
}

function main() {
  const fix = process.argv.includes("--fix");
  const markdown = process.argv.includes("--markdown");
  const tracked = loadTrackedVersions();
  const pkgFiles = findPackageJsonFiles();

  const driftedFiles = [];
  let totalDrifts = 0;

  for (const pkgPath of pkgFiles) {
    const { raw, drifts } = analyze(pkgPath, tracked);
    if (drifts.length === 0) continue;
    driftedFiles.push({ pkgPath, drifts });
    totalDrifts += drifts.length;
    if (fix) applyFix(pkgPath, raw, drifts);
  }

  if (markdown) {
    emitMarkdown(driftedFiles, totalDrifts, tracked, pkgFiles.length);
    process.exit(0);
  }

  if (driftedFiles.length === 0) {
    console.log(`OK — all tracked JS dependencies are at or above the versions.yml floor (${tracked.size} tracked packages, ${pkgFiles.length} package.json files scanned).`);
    process.exit(0);
  }

  if (fix) {
    console.log(`Fixed ${totalDrifts} behind-floor drift${totalDrifts === 1 ? "" : "s"} in ${driftedFiles.length} file${driftedFiles.length === 1 ? "" : "s"}:`);
    for (const { pkgPath, drifts } of driftedFiles) {
      console.log(`\n  ${relative(REPO_ROOT, pkgPath)}`);
      for (const d of drifts) {
        console.log(`    ${d.name}: ${d.spec} → ${d.prefix}${d.floor}`);
      }
    }
    process.exit(0);
  }

  console.error(`Found ${totalDrifts} behind-floor drift${totalDrifts === 1 ? "" : "s"} in ${driftedFiles.length} file${driftedFiles.length === 1 ? "" : "s"}:`);
  for (const { pkgPath, drifts } of driftedFiles) {
    console.error(`\n  ${relative(REPO_ROOT, pkgPath)}`);
    for (const d of drifts) {
      console.error(`    ${d.name}: ${d.spec} is below the ${d.floor} floor in versions.yml`);
    }
  }
  console.error(`\nRun \`node .github/scripts/check-js-versions.mjs --fix\` to raise specs to the floor, or lower the floor in versions.yml.`);
  process.exit(1);
}

main();

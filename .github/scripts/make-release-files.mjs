// Writes the two files a release publishes besides the installer itself:
//
//   latest.json  what the app's update check reads
//                (releases/latest/download/latest.json): the newest version,
//                its notes, and where to download the installer, with the
//                installer's SHA-256 so an in-app updater can verify what it
//                downloaded.
//   notes.md     the release description on GitHub.
//
// Notes come from the app's own changelog (gpui/js/dist/changelog.json,
// produced by gpui/build.rs from gpui/js/lib/changelog.ts), so the release
// page and the in-app "What's new" always say the same thing.
//
// Usage: node make-release-files.mjs <tag> <installer-path> <repo> <out-dir>

import { createHash } from "node:crypto";
import { readFileSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";

const [tag, installerPath, repo, outDir] = process.argv.slice(2);
if (!tag || !installerPath || !repo || !outDir) {
  console.error("usage: make-release-files.mjs <tag> <installer-path> <repo> <out-dir>");
  process.exit(2);
}

const version = tag.replace(/^v/, "");
const changelog = JSON.parse(readFileSync("gpui/js/dist/changelog.json", "utf8"));
const entry = changelog.find((item) => item.version === version);
if (!entry) {
  console.error(`gpui/js/lib/changelog.ts has no entry for ${version}; add one before releasing.`);
  process.exit(1);
}

const notes = entry.changes.map((change) => `- ${change}`).join("\n");
const installer = readFileSync(installerPath);
const asset = path.basename(installerPath);

const latest = {
  version,
  notes,
  pub_date: new Date().toISOString(),
  platforms: {
    "windows-x86_64": {
      url: `https://github.com/${repo}/releases/download/${tag}/${asset}`,
      sha256: createHash("sha256").update(installer).digest("hex"),
      size: statSync(installerPath).size,
    },
  },
};

writeFileSync(path.join(outDir, "latest.json"), `${JSON.stringify(latest, null, 2)}\n`);
writeFileSync(path.join(outDir, "notes.md"), `${notes}\n`);
console.log(`wrote latest.json and notes.md for ${version}`);

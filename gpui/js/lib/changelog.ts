// Hand-maintained changelog shown in Settings (components/ChangelogModal.tsx)
// — not generated from commit history, since that does not say what actually
// matters to someone deciding whether to install an update. Add a new entry at
// the top each time the version in gpui/Cargo.toml is bumped for a release.
//
// User-facing only — no CI/build-fix/internal-cleanup entries. A version
// bump needed purely to fix a broken previous release (a build that failed
// CI, a warning cleanup that broke Windows, ...) doesn't get its own line;
// fold whatever it was actually trying to ship into the entry for the
// version that ends up working, the way 0.0.13-0.0.15 collapsed into one
// entry here — the reader wants "what's new for me," not "how the last
// release attempt went."

export interface ChangelogEntry {
  version: string;
  /** ISO date (YYYY-MM-DD), not a Date — this is display-only, never compared. */
  date: string;
  changes: string[];
}

export const CHANGELOG: ChangelogEntry[] = [
  {
    version: "0.0.1",
    date: "2026-10-20",
    changes: [
      "First beta of the native OpenMouse Desktop app",
      "New: per-game mouse profiles, applied automatically when a game launches",
      "New: device controls for supported mice across many brands, including DPI, polling rate, lift-off, buttons and lighting",
    ],
  },
];

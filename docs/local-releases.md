# Local releases

The local release hub reads `local-release.json` from the selected source commit, runs its declared recipe, and publishes its declared assets with checksums and provenance. Packaging scripts do not publish directly. Recipes receive `LOCAL_RELEASE_VERSION`, `LOCAL_RELEASE_SOURCE_SHA`, and `LOCAL_RELEASE_REPOSITORY`, with the repository root as their working directory.

The replaced GitHub Actions release workflows are retired and disabled. Their source remains as a reference for the previous packaging and publishing behavior. Other workflows are unchanged.

`windows-x64` requires Node, npm, Rust, the MSVC x64 build tools, and Windows SDK. The requested version must equal both GPUI Cargo manifests and have an entry in `gpui/js/lib/changelog.ts`. The existing release metadata generator preserves the stable `OpenMouseDesktopSetup.exe` download name and the `latest.json` installer URL and SHA-256. Generated release notes are retained as `release/notes.md` and supplied to the hub as the release body. This recipe builds the application and installer without running the old workflow's tests.

The fork has no registered Actions workflows to disable through the API. The retired release job also has an explicit false condition, so enabling fork workflows cannot run its hosted release job.

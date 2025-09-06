# Release Workflow

This repository includes automated release workflows to publish new versions of RustyRoad to crates.io.

## Automatic Releases

The release workflow is triggered automatically when a new GitHub release is published. This workflow will:

1. Run the complete test suite
2. Build release binaries for Linux, Windows, and macOS
3. Publish the new version to crates.io
4. Upload platform-specific binaries to the GitHub release

## Manual Releases

You can also trigger a release manually using the workflow dispatch feature:

1. Go to the Actions tab in the GitHub repository
2. Select the "Release" workflow
3. Click "Run workflow"
4. Enter the version number you want to release

## Prerequisites

Before triggering a release, ensure:

1. The `CRATES_TOKEN` secret is configured in the repository settings
   - Go to Settings → Secrets and variables → Actions
   - Add a new repository secret named `CRATES_TOKEN`
   - Use a token from https://crates.io/me that has publish permissions

2. The version in `Cargo.toml` has been updated to the target version

3. All tests are passing on the main branch

## Creating a Release

To create a new release:

1. Update the version in `Cargo.toml`
2. Commit and push the version bump
3. Create a new release on GitHub:
   - Go to Releases → Create a new release
   - Create a new tag with the version (e.g., `v1.0.22`)
   - Fill in the release notes
   - Publish the release

The automated workflow will handle the rest!
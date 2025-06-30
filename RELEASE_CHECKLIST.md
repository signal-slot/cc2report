# Release Checklist for cc2report

Use this checklist before creating a new release.

## Pre-Release Checklist

### Code Quality
- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] Examples work correctly
- [ ] No security vulnerabilities: `cargo audit`

### Version Updates
- [ ] Version bumped in `Cargo.toml`
- [ ] Version follows semver guidelines
- [ ] CHANGELOG.md updated with:
  - [ ] Release version
  - [ ] Release date
  - [ ] All changes documented
  - [ ] Links updated at bottom

### Documentation
- [ ] README.md is up to date
- [ ] API.md reflects current API
- [ ] Examples are current
- [ ] Installation instructions accurate

### Dependencies
- [ ] No path dependencies in Cargo.toml
- [ ] No git dependencies in Cargo.toml
- [ ] Dependencies use stable versions
- [ ] Run `cargo update` if needed

### Final Checks
- [ ] `cargo publish --dry-run` succeeds
- [ ] No uncommitted changes: `git status`
- [ ] On main branch: `git branch`
- [ ] Pulled latest changes: `git pull`

## Release Process

### 1. Create Git Tag
```bash
# Format: v{MAJOR}.{MINOR}.{PATCH}
git tag -a v2.2.0 -m "Release version 2.2.0

- Add feature X
- Fix issue Y
- Improve performance of Z"

# Push tag
git push origin v2.2.0
```

### 2. Publish to crates.io
```bash
# Login if needed
cargo login

# Publish
cargo publish
```

### 3. Create GitHub Release
1. Go to: https://github.com/signal-slot/cc2report/releases/new
2. Select the tag you just created
3. Title: "v2.2.0"
4. Copy release notes from CHANGELOG.md
5. Attach pre-built binaries (if CI didn't)
6. Publish release

### 4. Post-Release
- [ ] Verify crate published: https://crates.io/crates/cc2report
- [ ] Test installation: `cargo install cc2report`
- [ ] Update README if needed
- [ ] Announce release (if major/minor)

## Version Guidelines

### Patch Version (2.2.X)
- Bug fixes
- Documentation updates
- Dependency updates (non-breaking)

### Minor Version (2.X.0)
- New features
- Deprecations
- Minor breaking changes with migration path

### Major Version (X.0.0)
- Breaking API changes
- Major architectural changes
- Removal of deprecated features

## Rollback Procedure

If issues are discovered post-release:

1. **Yank the release** (if critical):
   ```bash
   cargo yank --vers 2.2.0
   ```

2. **Fix the issue**
3. **Release patch version**
4. **Document in CHANGELOG**

## Communication Template

### Release Announcement

```markdown
# cc2report v2.2.0 Released! 🎉

We're excited to announce the release of cc2report v2.2.0!

## What's New
- Feature X for better Y
- Performance improvements in Z
- Bug fixes and stability improvements

## Installation
```bash
cargo install cc2report
```

## Full Changelog
See: https://github.com/signal-slot/cc2report/blob/main/CHANGELOG.md

## Feedback
Please report issues at: https://github.com/signal-slot/cc2report/issues
```

## Platform-Specific Notes

### macOS
- Ensure both x86_64 and aarch64 builds work
- Test on both Intel and Apple Silicon

### Windows
- Test on Windows 10/11
- Ensure paths work correctly
- Check CRLF handling

### Linux
- Test on major distributions
- Ensure no glibc version issues

## Metrics to Track

After release, monitor:
- Download count on crates.io
- GitHub stars/issues
- Community feedback
- Performance reports
- Bug reports

## Emergency Contacts

- Crates.io support: help@crates.io
- GitHub support: https://support.github.com
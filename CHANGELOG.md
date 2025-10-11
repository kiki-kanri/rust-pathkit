# Changelog

## v0.8.1

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.8.0...v0.8.1)

### 🚀 Enhancements

- Change `Path` struct to tuple, and add more derive and traits, add `to_path_buf` method ([f03a498](https://github.com/kiki-kanri/rust-pathkit/commit/f03a498))
- Add `with_extension` method and remove redundant methods identical to underlying implementation, use deref for direct calls ([4d1eff7](https://github.com/kiki-kanri/rust-pathkit/commit/4d1eff7))

### 🔥 Performance

- Minimize `tokio` features ([c90f8e4](https://github.com/kiki-kanri/rust-pathkit/commit/c90f8e4))

### 🩹 Fixes

- Resolve ownership consumption issue in div operation ([fccd844](https://github.com/kiki-kanri/rust-pathkit/commit/fccd844))

### 💅 Refactors

- Replace some `Path` impls with `derive` ([02ed791](https://github.com/kiki-kanri/rust-pathkit/commit/02ed791))

### 🏡 Chore

- Upgrade dependencies ([60a214d](https://github.com/kiki-kanri/rust-pathkit/commit/60a214d))
- Upgrade dependencies ([0876246](https://github.com/kiki-kanri/rust-pathkit/commit/0876246))
- Update `.gitignore` and `modify-files-permissions.sh` ([ad5c35f](https://github.com/kiki-kanri/rust-pathkit/commit/ad5c35f))
- Update scripts ([64d1874](https://github.com/kiki-kanri/rust-pathkit/commit/64d1874))
- Update vscode setting ([679cf39](https://github.com/kiki-kanri/rust-pathkit/commit/679cf39))
- Add cargo alias config ([23ef3d6](https://github.com/kiki-kanri/rust-pathkit/commit/23ef3d6))
- Upgrade dependencies ([36c663b](https://github.com/kiki-kanri/rust-pathkit/commit/36c663b))
- Remove makefile ([8412ad6](https://github.com/kiki-kanri/rust-pathkit/commit/8412ad6))
- Add `.editorconfig` ([75290f7](https://github.com/kiki-kanri/rust-pathkit/commit/75290f7))
- Update release script ([8c85f4f](https://github.com/kiki-kanri/rust-pathkit/commit/8c85f4f))
- Update scripts ([0982503](https://github.com/kiki-kanri/rust-pathkit/commit/0982503))
- Lint code ([1574193](https://github.com/kiki-kanri/rust-pathkit/commit/1574193))
- Replace exclude with include from Cargo.toml ([c7e536a](https://github.com/kiki-kanri/rust-pathkit/commit/c7e536a))
- Lint code ([8bffd72](https://github.com/kiki-kanri/rust-pathkit/commit/8bffd72))

### 🎨 Styles

- Add rustfmt config and format all code ([2c74d28](https://github.com/kiki-kanri/rust-pathkit/commit/2c74d28))

### ❤️ Contributors

- Kiki-kanri

## v0.8.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.7.1...v0.8.0)

### 🚀 Enhancements

- Extend div operator overloading to support more concatenation types ([7129469](https://github.com/kiki-kanri/rust-pathkit/commit/7129469))

### 🩹 Fixes

- **Cargo.toml:** Correctly configure exclude field for files not to be uploaded ([9eaad83](https://github.com/kiki-kanri/rust-pathkit/commit/9eaad83))

### ❤️ Contributors

- kiki-kanri

## v0.7.1

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.7.0...v0.7.1)

### 💅 Refactors

- Change `read_json` generic parameter T type to `for<'de> Deserialize<'de>` ([d6d0af7](https://github.com/kiki-kanri/rust-pathkit/commit/d6d0af7))

### 🏡 Chore

- Add ignore for cargo publish ([52e1e6f](https://github.com/kiki-kanri/rust-pathkit/commit/52e1e6f))

### ❤️ Contributors

- kiki-kanri

## v0.7.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.6.0...v0.7.0)

### 🚀 Enhancements

- Add `read_json` and `write_json` methods to `FsOps` ([85f7504](https://github.com/kiki-kanri/rust-pathkit/commit/85f7504))

### 🩹 Fixes

- Ensure methods end with `_sync` in `SyncFsOps` ([cd34b00](https://github.com/kiki-kanri/rust-pathkit/commit/cd34b00))

### ✅ Tests

- Add initial `chmod` unit tests and dependencies ([ad0a617](https://github.com/kiki-kanri/rust-pathkit/commit/ad0a617))

### ❤️ Contributors

- kiki-kanri

## v0.6.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.5.0...v0.6.0)

### 🚀 Enhancements

- Add `read_to_string` method to `FsOps` ([1fff3d4](https://github.com/kiki-kanri/rust-pathkit/commit/1fff3d4))
- Add `read` and `read_dir` methods to `FsOps` ([58988cf](https://github.com/kiki-kanri/rust-pathkit/commit/58988cf))
- Add `write` method to `FsOps` ([d1cdf8c](https://github.com/kiki-kanri/rust-pathkit/commit/d1cdf8c))

### 🏡 Chore

- Upgrade dependencies ([cbaa8eb](https://github.com/kiki-kanri/rust-pathkit/commit/cbaa8eb))

### ✅ Tests

- Add unit tests for `core.rs` ([f4fb9d8](https://github.com/kiki-kanri/rust-pathkit/commit/f4fb9d8))
- Add unit tests for `traits.rs` ([f060ede](https://github.com/kiki-kanri/rust-pathkit/commit/f060ede))
- Change test functions name in `core.rs` ([78efe5b](https://github.com/kiki-kanri/rust-pathkit/commit/78efe5b))
- Add unit tests for `div.rs` ([e85f96c](https://github.com/kiki-kanri/rust-pathkit/commit/e85f96c))
- Add `Makefile.toml` and test-related commands ([facdaf7](https://github.com/kiki-kanri/rust-pathkit/commit/facdaf7))

### ❤️ Contributors

- kiki-kanri

## v0.5.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.4.0...v0.5.0)

### 🚀 Enhancements

- Add `SyncFsOps` trait and impl ([84055c0](https://github.com/kiki-kanri/rust-pathkit/commit/84055c0))
- Add `metadata` method to `FsOps` ([d5ee9a7](https://github.com/kiki-kanri/rust-pathkit/commit/d5ee9a7))
- Add `empty_dir` and `get_file_size` methods to `FsOps` ([1a615d2](https://github.com/kiki-kanri/rust-pathkit/commit/1a615d2))
- Add `remove_dir_all` method to `FsOps` ([ba87e22](https://github.com/kiki-kanri/rust-pathkit/commit/ba87e22))

### 💅 Refactors

- ⚠️ Rename methods in `FsOps` ([60395aa](https://github.com/kiki-kanri/rust-pathkit/commit/60395aa))

#### ⚠️ Breaking Changes

- ⚠️ Rename methods in `FsOps` ([60395aa](https://github.com/kiki-kanri/rust-pathkit/commit/60395aa))

### ❤️ Contributors

- kiki-kanri

## v0.4.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.3.0...v0.4.0)

### 🚀 Enhancements

- Add `chmod` and `set_permissions` methods to `AsyncFsOps` ([4842f83](https://github.com/kiki-kanri/rust-pathkit/commit/4842f83))
- Add `chown` methods to `AsyncFsOps` ([e2b651f](https://github.com/kiki-kanri/rust-pathkit/commit/e2b651f))
- Add `rmdir` and `truncate` methods to `AsyncFsOps` ([9f279e7](https://github.com/kiki-kanri/rust-pathkit/commit/9f279e7))
- Add methods to `AsyncFsOps` implementation for checking path types (file, fifo, dir, etc.) ([2d2344e](https://github.com/kiki-kanri/rust-pathkit/commit/2d2344e))

### 🩹 Fixes

- Correct incorrect '.release.toml' file name ([e57cd7b](https://github.com/kiki-kanri/rust-pathkit/commit/e57cd7b))

### 💅 Refactors

- Modify `chmod` operation to exclude compilation on non-Unix systems ([bb34ad9](https://github.com/kiki-kanri/rust-pathkit/commit/bb34ad9))
- ⚠️ Remove `mkdirs` method from `AsyncFsOps` ([30f8a89](https://github.com/kiki-kanri/rust-pathkit/commit/30f8a89))

### 🏡 Chore

- Add nodejs config to `.gitignore` ([dc01dfb](https://github.com/kiki-kanri/rust-pathkit/commit/dc01dfb))
- Add tool to generate changelog and modify `.release.toml` ([66fe864](https://github.com/kiki-kanri/rust-pathkit/commit/66fe864))
- Add `CHANGELOG.md` ([c84ac9a](https://github.com/kiki-kanri/rust-pathkit/commit/c84ac9a))
- Modify `description` field in `Cargo.toml` ([f08a3f0](https://github.com/kiki-kanri/rust-pathkit/commit/f08a3f0))
- Add `modify-files-permissions.sh` script ([57a5a51](https://github.com/kiki-kanri/rust-pathkit/commit/57a5a51))
- Remove `release.toml` ([225d53d](https://github.com/kiki-kanri/rust-pathkit/commit/225d53d))
- Automate release process with script ([f2c4b19](https://github.com/kiki-kanri/rust-pathkit/commit/f2c4b19))
- Update release script to disallow any uncommitted files ([7da0de1](https://github.com/kiki-kanri/rust-pathkit/commit/7da0de1))

#### ⚠️ Breaking Changes

- ⚠️ Remove `mkdirs` method from `AsyncFsOps` ([30f8a89](https://github.com/kiki-kanri/rust-pathkit/commit/30f8a89))

### ❤️ Contributors

- kiki-kanri

## v0.3.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.2.0...v0.3.0)

### 🚀 Enhancements

- Add `canonicalize` method to path impl ([f002203](https://github.com/kiki-kanri/rust-pathkit/commit/f002203))
- Add absolutize-related methods to path impl ([ccf4428](https://github.com/kiki-kanri/rust-pathkit/commit/ccf4428))
- Add some methods to path impl ([fafe1e7](https://github.com/kiki-kanri/rust-pathkit/commit/fafe1e7))
- Add initial implementation of 'AsyncFsOps' structure for extending 'Path' functionality ([a91eb8b](https://github.com/kiki-kanri/rust-pathkit/commit/a91eb8b))

### 🩹 Fixes

- Correct incorrect import paths in `lib.rs` ([1d0edcd](https://github.com/kiki-kanri/rust-pathkit/commit/1d0edcd))
- Resolve issues in `async_fs_ops.rs` ([4915e70](https://github.com/kiki-kanri/rust-pathkit/commit/4915e70))

### 💅 Refactors

- Remove `return` statements ([c1a7035](https://github.com/kiki-kanri/rust-pathkit/commit/c1a7035))
- Simplify parts of the code ([c3a3cdb](https://github.com/kiki-kanri/rust-pathkit/commit/c3a3cdb))
- Remove 'path' folder and move files to the root directory ([f6c4d7d](https://github.com/kiki-kanri/rust-pathkit/commit/f6c4d7d))
- Explicitly use 'return' for function return values ([dfbf607](https://github.com/kiki-kanri/rust-pathkit/commit/dfbf607))

### 📖 Documentation

- Edit readme ([4e064df](https://github.com/kiki-kanri/rust-pathkit/commit/4e064df))

### 🏡 Chore

- Add required dependencies ([7c5a82d](https://github.com/kiki-kanri/rust-pathkit/commit/7c5a82d))

### ❤️ Contributors

- kiki-kanri

## v0.2.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/v0.1.0...v0.2.0)

### 🚀 Enhancements

- Implement `AsRef`, `Clone`, `Display`, `Eq`, and `PartialEq` traits for path struct ([7e7d7c6](https://github.com/kiki-kanri/rust-pathkit/commit/7e7d7c6))
- Add join method and implement div operator overloading for path ([2b45288](https://github.com/kiki-kanri/rust-pathkit/commit/2b45288))
- Add `parent` method to path impl ([1f19c95](https://github.com/kiki-kanri/rust-pathkit/commit/1f19c95))

### 💅 Refactors

- Split path struct and impl into separate files ([ba7ec6f](https://github.com/kiki-kanri/rust-pathkit/commit/ba7ec6f))
- Modify `lib.rs` to keep path file private and expose only Path struct ([d8bdf79](https://github.com/kiki-kanri/rust-pathkit/commit/d8bdf79))

### ❤️ Contributors

- kiki-kanri

## v0.1.0

[compare changes](https://github.com/kiki-kanri/rust-pathkit/compare/fcf92e4...v0.1.0)

### 🚀 Enhancements

- Add base `Path` struct ([22d3e3b](https://github.com/kiki-kanri/rust-pathkit/commit/22d3e3b))

### 🩹 Fixes

- Remove incorrect items from categories list in Cargo.toml ([150b72d](https://github.com/kiki-kanri/rust-pathkit/commit/150b72d))

### 🏡 Chore

- Add .gitignore ([1534395](https://github.com/kiki-kanri/rust-pathkit/commit/1534395))
- Add vscode setting ([63e1800](https://github.com/kiki-kanri/rust-pathkit/commit/63e1800))
- Add Cargo.toml and package info ([3b2fc17](https://github.com/kiki-kanri/rust-pathkit/commit/3b2fc17))
- Add empty README ([3629f48](https://github.com/kiki-kanri/rust-pathkit/commit/3629f48))
- Rename package to pathkit ([4d66bab](https://github.com/kiki-kanri/rust-pathkit/commit/4d66bab))
- Add `.release.toml` ([102da03](https://github.com/kiki-kanri/rust-pathkit/commit/102da03))

### ❤️ Contributors

- kiki-kanri

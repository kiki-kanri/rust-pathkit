# Changelog

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

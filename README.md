# README

[tauri](https://tauri.app/v1/guides) + [umijs](https://umijs.org/docs/max/introduce) Template

<details>
<summary>My env</summary>

umijs create config:

```bash
$ pnpm dlx create-umi@latest
Ant Design Pro
pnpm
taobao
```

```bash
$ pnpm exec max v
umi@4.0.69
```

```bash
$ pnpm run tauri info:

[✔] Environment
    - OS: Mac OS 12.6.0 X64
    ✔ Xcode Command Line Tools: installed
    ✔ rustc: 1.71.0-nightly (39c6804b9 2023-04-19)
    ✔ Cargo: 1.71.0-nightly (d0a4cbcee 2023-04-16)
    ✔ rustup: 1.25.2 (fae52a197 2023-02-01)
    ✔ Rust toolchain: nightly-x86_64-apple-darwin (default)
    - node: 18.15.0
    - pnpm: 8.5.1
    - npm: 9.5.0

[-] Packages
    - tauri [RUST]: 1.3.0
    - tauri-build [RUST]: 1.3.0
    - wry [RUST]: 0.24.3
    - tao [RUST]: 0.16.2
    - @tauri-apps/api [NPM]: 1.3.0
    - @tauri-apps/cli [NPM]: 1.3.0 (outdated, latest: 1.3.1)

[-] App
    - build-type: bundle
    - CSP: unset
    - distDir: ../dist
    - devPath: <http://localhost:8000/>
    - framework: React
```

</details>

## Requirements

- [rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [node](https://nodejs.org/en)
- [pnpm](https://github.com/pnpm/pnpm)

About the tauri requirements, see [here](https://tauri.app/v1/guides/getting-started/prerequisites).

## Usage

1. Click 'Use the template'
1. Clone your new repository
1. Run `pnpm i` in the repository.
1. Run `pnpm run start` to start the app to develop.

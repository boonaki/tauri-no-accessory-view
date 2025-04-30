# tauri-plugin-no-accessory-view
> This plugin only works with Tauri 2.x only.

Completely removes the accessory bar when using a tauri application on iOS.

## Install

```bash
git clone https://github.com/boonaki/tauri-no-accessory-view.git
```

## Usage

### rust
```rust

tauri::Builder::default()
    .plugin(tauri_plugin_no_accessory_view::init())
    ...
```

## How to run example

### build plugin first to gen webview-dist

```shell
npm i
npm build
```

### entry example and run

```shell
cd examples/vanilla
npm i
npm tauri dev
```

## Support

| MacOS | Linux | Windows | Android | iOS |
| ----- | ----- | ------- | ------- | ------- |
|     |     |       |      | ✅ (passes App Review) |

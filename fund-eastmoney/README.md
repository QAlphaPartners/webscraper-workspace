
# `fund-eastmoney web data scraper tauri-plugin`

A Tauri Plugin for fund-eastmoney web data scraper.

**Reference** <https://github.com/timfish/sentry-tauri>

## Installation

Add the required dependencies in `Cargo.toml`:

```toml
[dependencies]
fund-easymoney = "0.1"
```

`sentry` and `sentry-rust-minidump` are re-exported by `fund-easymoney` so you
don't need to add them as dependencies.

```rust
fn main() {
    let client = fund_easymoney::sentry::init((
        "__YOUR_DSN__",
        fund_easymoney::sentry::ClientOptions {
            release: fund_easymoney::sentry::release_name!(),
            ..Default::default()
        },
    ));

    // Everything before here runs in both app and crash reporter processes
    let _guard = fund_easymoney::minidump::init(&client);
    // Everything after here runs in only the app process

    tauri::Builder::default()
        .plugin(fund_easymoney::plugin())
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
```

## The Plugin

- Injects and initialises `@sentry/browser` in every web-view
- Includes `beforeSend` and `beforeBreadcrumb` hooks that intercept events and breadcrumbs and passes
  them to the Rust SDK via the Tauri `invoke` API
- Tauri + `serde` + existing Sentry Rust types = Deserialisation mostly Just Works™️

## Example App

1. Clone this repository, install dependencies:

```shell
> pnpm install
> pnpm build 
```

2. Build ts into js and build rust(see scripts in package.json)

```shell
> pnpm build 
```

3. Run in development mode:

```shell
> pnpm example
```

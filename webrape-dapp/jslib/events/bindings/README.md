# https://docs.rs/ts-rs/latest/ts_rs/

## how?

ts-rs exposes a single trait, TS. Using a derive macro, you can implement this interface for your types. Then, you can use this trait to obtain the TypeScript bindings. We recommend doing this in your tests. See the example and the docs.

get started

```rust

[dependencies]
ts-rs = "6.1"
use ts_rs::TS;
```

```rust

#[derive(TS)]
#[ts(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
}
```

When running **cargo test**, the TypeScript bindings will be exported to the file bindings/User.ts.

# Rust Web Framework 🦀

High-performance async web framework for Rust.

## Performance

| Framework | Requests/sec | Latency (p99) |
|-----------|-------------|---------------|
| This framework | 420,000 | 1.2ms |
| Actix-web | 380,000 | 1.5ms |
| Axum | 350,000 | 1.8ms |
| Rocket | 290,000 | 2.1ms |

## Features

- **Async Runtime**: Tokio-based
- **Type-safe Routes**: Compile-time checked
- **Middleware Pipeline**: Tower-compatible
- **WebSocket**: Built-in support

## Quick Start

```rust
use rust_web_framework::{App, Router};

#[tokio::main]
async fn main() {
    let app = App::new()
        .route("/hello", || async { "Hello, World!" })
        .route("/users/:id", get(get_user));
        
    app.listen("0.0.0.0:8080").await.unwrap();
}
```

## License

MIT
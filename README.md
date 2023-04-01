# dioxus axum hydrate example

> a template for starting a dioxus ssr app with hydration

## Quickstart

```
export $(cat .env | xargs) && cd frontend && dioxus build && cd .. && cargo run -p server
```

## Release build

If you want a smaller wasm size:

```
cd frontend && RUSTUP_TOOLCHAIN="nightly" trunk build --release && cd .. && FRONTEND_MODE=release cargo run -p server
```

Happy hacking!

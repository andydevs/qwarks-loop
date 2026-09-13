# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

This repo has two independent Rust crates (not a Cargo workspace — each has its own `Cargo.lock`/`target`):

- **`qwarks_loop/`** — the library. A safe, `wasm-bindgen`-based wrapper around the browser's `requestAnimationFrame`/`cancelAnimationFrame` APIs, compiled as an `rlib` and consumed by other Rust crates (not published to npm directly).
- **`demo/`** — a WebAssembly demo site. A `cdylib` crate that depends on `qwarks_loop` via a local path dependency, exposes a `#[wasm_bindgen]` type to JavaScript, and is built with `wasm-pack` + webpack into a static site (`demo/dist`).

## Commands

Run from the `qwarks_loop/` directory (the library):

```sh
wasm-pack --test firefox   # run the library's wasm-bindgen tests in headless Firefox
cargo build                # plain native build/typecheck of the library crate
```

Run from the `demo/` directory (the demo site) — requires `npm install` first:

```sh
npm start        # build the Rust side once, then run wasm-pack in watch mode + webpack-dev-server in parallel
npm run build    # one-shot production build (build:rust then build:web) into demo/dist
npm run build:rust  # wasm-pack build --target bundler --out-dir pkg  (rebuilds demo/pkg from Rust sources)
npm run build:web   # webpack --mode production (bundles demo/index.js + pkg into demo/dist)
```

Note per the README: run `npm start` in a separate terminal from whatever you use to rebuild the wasm library, since `start:rust` watches `Cargo.toml`/`src/**/*.rs` (in both `demo/` and the library) and rebuilds `pkg/` on change while `start:web` serves it.

## Architecture

### `qwarks_loop` library

The public API is intentionally small: `lib.rs` re-exports only `RAFLoop` and `FrameCtx`; everything else (`browser`, `context`, `telemetry`) is a private module.

- **`RAFLoop`** (`rafloop.rs`) is the main entry point. `RAFLoop::new(callback)` starts the loop immediately — the first frame is requested before the constructor returns — and the loop runs until the returned `RAFLoop` value is dropped, at which point `Drop` cancels the pending `requestAnimationFrame` request. There is no separate "stop" method; callers stop the loop by dropping the handle (see how `demo`'s `RAFDemoHandler::stop` does this via `self.rafloop.take()`).
- **`browser::Adapter`** is a trait abstraction over `window.requestAnimationFrame`/`cancelAnimationFrame`. `BrowserAdapter` is the real implementation; `RAFLoop::with_adapter` is the internal constructor that takes an `Rc<dyn Adapter>`, which exists specifically so tests can inject a mock adapter instead of a real browser `window`. `RAFLoop::new` is just `with_adapter` with `BrowserAdapter`.
- **`Telemetry`** (`telemetry.rs`) holds the mutable per-loop state (`frame_count`, `last_timestamp`, `pending_id`) shared between the outer `RAFLoop` handle and the recurring `Closure` via `Rc<RefCell<Telemetry>>`. Each invocation of the frame closure reads a `FrameCtx` from `Telemetry` *before* calling the user callback, then requests the next frame and updates `Telemetry` (incrementing `frame_count`, storing the new timestamp/request id) *after*. `FrameCtx::delta` is computed as the difference from the previous frame's timestamp, and is `0.0` on the first frame.
- The recurring `Closure<dyn FnMut(f64)>` is stored in `Rc<RefCell<Option<Closure<...>>>>` so the closure can reference itself (to re-request the next frame) despite Rust not natively supporting self-referential closures.

### `demo` crate

- `RAFDemoHandler` (`lib.rs`) is the single `#[wasm_bindgen]`-exported type. It holds an `Option<qwarks_loop::RAFLoop>` (present only while running) and an `Rc<RenderCtx>` shared into the frame callback.
- `RenderCtx` (`render.rs`) owns references to four DOM `HtmlElement`s (frame count, timestamp, delta, FPS) and has one `render_*` method per element that formats and writes a value into it. FPS is derived in `lib.rs` as `MS_PER_SEC / ctx.delta`, not tracked by the library itself.
- `macros.rs` defines two crate-local macros: `console_log!` (wraps `web_sys::console::log_1`) and `checked_cast!` (wraps `JsValue::dyn_into` with a descriptive error message naming the failed variable/type).
- JS side: `demo/index.js` imports the generated `qwarks-loop-demo` package from `demo/pkg`, constructs `RAFDemoHandler` with four DOM elements queried by id, and wires `#ctrl-start`/`#ctrl-stop` buttons to `handler.start()`/`handler.stop()`. `demo/bootstrap.js` exists solely to `import("./index.js")` asynchronously, which webpack requires for entry points that pull in wasm. `demo/pkg` is generated output from `wasm-pack build` — treat it as a build artifact, not hand-edited source.

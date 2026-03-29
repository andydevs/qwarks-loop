//! WebAssembly demo for `wasm-raf-handler`.
//!
//! Exposes [`RAFDemoHandler`] to JavaScript via `wasm-bindgen`. The handler
//! drives a `requestAnimationFrame` loop that updates four HTML elements with
//! live per-frame statistics.

#[macro_use]
mod macros;
mod render;

use macros::MS_PER_SEC;
use render::RenderCtx;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_raf_handler as raf;

/// JavaScript-facing controller for the demo animation loop.
///
/// Holds an optional [`raf::Loop`] handle (present only while running) and a
/// shared [`RenderCtx`] that writes frame statistics to the DOM.
#[wasm_bindgen]
pub struct RAFDemoHandler {
    rafloop: Option<raf::RAFLoop>,
    ctx: Rc<RenderCtx>,
}

#[wasm_bindgen]
impl RAFDemoHandler {
    /// Creates a new handler bound to four DOM elements.
    ///
    /// Each argument must be an `HtmlElement` (passed as a `JsValue` from JS).
    /// Returns an error if any value fails the cast.
    ///
    /// The loop does **not** start automatically; call [`start`](Self::start) to begin rendering.
    pub fn new(
        framecount: JsValue,
        timestamp: JsValue,
        delta: JsValue,
        fps: JsValue,
    ) -> Result<RAFDemoHandler, JsValue> {
        Ok(RAFDemoHandler {
            rafloop: None,
            ctx: Rc::new(RenderCtx::new(framecount, timestamp, delta, fps)?),
        })
    }

    /// Starts the animation loop if it is not already running.
    ///
    /// Each frame updates the frame count, timestamp, delta, and FPS elements.
    /// Calling `start` while the loop is already running has no effect.
    pub fn start(&mut self) {
        if self.rafloop.is_none() {
            let rndr = Rc::clone(&self.ctx);
            let rafloop = raf::RAFLoop::new(move |ctx: raf::FrameCtx| {
                rndr.render_frame_count(ctx.frame_count);
                rndr.render_timestamp(ctx.timestamp);
                rndr.render_delta(ctx.delta);
                let fps = MS_PER_SEC / ctx.delta;
                rndr.render_fps(fps);
            })
            .expect("Creating RAF loop");
            self.rafloop = Some(rafloop);
        }
    }

    /// Stops the animation loop by dropping the [`raf::Loop`] handle.
    ///
    /// Calling `stop` while the loop is not running has no effect.
    pub fn stop(&mut self) {
        self.rafloop.take();
    }
}

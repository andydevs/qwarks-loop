#[macro_use]
mod macros;
mod render;

use macros::MS_PER_SEC;
use render::RenderCtx;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_raf_handler as raf;

#[wasm_bindgen]
pub struct RAFDemoHandler {
    rafloop: Option<raf::Loop>,
    ctx: Rc<RenderCtx>,
}

#[wasm_bindgen]
impl RAFDemoHandler {
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

    pub fn start(&mut self) {
        if self.rafloop.is_none() {
            let rndr = Rc::clone(&self.ctx);
            let rafloop = raf::Loop::new(move |ctx: raf::Ctx| {
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

    pub fn stop(&mut self) {
        self.rafloop.take();
    }
}

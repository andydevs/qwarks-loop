use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_raf_handler as raf;
use web_sys::HtmlElement;

macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

const MS_PER_SEC: f64 = 1000.00;

struct RenderCtx {
    framecount: HtmlElement,
    timestamp: HtmlElement,
    delta: HtmlElement,
    fps: HtmlElement,
}

impl RenderCtx {
    fn render_frame_count(&self, value: u32) {
        self.framecount
            .set_inner_text(format!("{}", value).as_str());
    }

    fn render_timestamp(&self, value: f64) {
        self.timestamp
            .set_inner_text(format!("{:.3} ms", value).as_str());
    }

    fn render_delta(&self, value: f64) {
        self.delta
            .set_inner_text(format!("{:.3} ms", value).as_str());
    }

    fn render_fps(&self, value: f64) {
        self.fps
            .set_inner_text(format!("{:.3} FPS", value).as_str());
    }
}

#[wasm_bindgen]
pub struct RAFDemoHandler {
    rafloop: Option<raf::Loop>,
    ctx: Rc<RenderCtx>,
}

macro_rules! checked_cast {
    ($type:ty, $input:ident) => {{
        let err = format!(
            "Expected {} to be an {}. Got {:?}",
            stringify!($input),
            stringify!($type),
            $input
        );
        $input.dyn_into::<$type>().map_err(|_| JsValue::from(err))
    }};
}

#[wasm_bindgen]
impl RAFDemoHandler {
    pub fn new(
        framecount: JsValue,
        timestamp: JsValue,
        delta: JsValue,
        fps: JsValue,
    ) -> Result<RAFDemoHandler, JsValue> {
        let framecount = checked_cast![HtmlElement, framecount]?;
        let timestamp = checked_cast![HtmlElement, timestamp]?;
        let delta = checked_cast![HtmlElement, delta]?;
        let fps = checked_cast![HtmlElement, fps]?;
        let handler = RAFDemoHandler {
            rafloop: None,
            ctx: Rc::new(RenderCtx {
                framecount,
                timestamp,
                delta,
                fps,
            }),
        };
        Ok(handler)
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

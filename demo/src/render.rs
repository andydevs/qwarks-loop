use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::HtmlElement;

pub struct RenderCtx {
    framecount: HtmlElement,
    timestamp: HtmlElement,
    delta: HtmlElement,
    fps: HtmlElement,
}

impl RenderCtx {
    pub fn new(
        framecount: JsValue,
        timestamp: JsValue,
        delta: JsValue,
        fps: JsValue,
    ) -> Result<Self, JsValue> {
        let framecount = checked_cast![HtmlElement, framecount]?;
        let timestamp = checked_cast![HtmlElement, timestamp]?;
        let delta = checked_cast![HtmlElement, delta]?;
        let fps = checked_cast![HtmlElement, fps]?;
        Ok(Self {
            framecount,
            timestamp,
            delta,
            fps,
        })
    }

    pub fn render_frame_count(&self, value: u32) {
        self.framecount
            .set_inner_text(format!("{}", value).as_str());
    }

    pub fn render_timestamp(&self, value: f64) {
        self.timestamp
            .set_inner_text(format!("{:.2} ms", value).as_str());
    }

    pub fn render_delta(&self, value: f64) {
        self.delta
            .set_inner_text(format!("{:.2} ms", value).as_str());
    }

    pub fn render_fps(&self, value: f64) {
        self.fps
            .set_inner_text(format!("{} FPS", value.round()).as_str());
    }
}

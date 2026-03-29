use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::HtmlElement;

/// Holds references to the four DOM elements updated each animation frame.
pub struct RenderCtx {
    framecount: HtmlElement,
    timestamp: HtmlElement,
    delta: HtmlElement,
    fps: HtmlElement,
}

impl RenderCtx {
    /// Constructs a [`RenderCtx`] by casting four `JsValue`s to `HtmlElement`.
    ///
    /// Returns `Err` if any value is not an `HtmlElement`.
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

    /// Sets the frame-count element's text to `value`.
    pub fn render_frame_count(&self, value: u32) {
        self.framecount
            .set_inner_text(format!("{}", value).as_str());
    }

    /// Sets the timestamp element's text to `value` formatted as `"X.XX ms"`.
    pub fn render_timestamp(&self, value: f64) {
        self.timestamp
            .set_inner_text(format!("{:.2} ms", value).as_str());
    }

    /// Sets the delta element's text to `value` formatted as `"X.XX ms"`.
    pub fn render_delta(&self, value: f64) {
        self.delta
            .set_inner_text(format!("{:.2} ms", value).as_str());
    }

    /// Sets the FPS element's text to `value` rounded to the nearest integer, formatted as `"X FPS"`.
    pub fn render_fps(&self, value: f64) {
        self.fps
            .set_inner_text(format!("{} FPS", value.round()).as_str());
    }
}

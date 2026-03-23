use wasm_bindgen::prelude::*;
use wasm_raf_handler as raf;
use web_sys::HtmlElement;

macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

#[wasm_bindgen]
pub struct RAFDemoHandler {
    rafloop: Option<raf::Loop>,
    framecount: HtmlElement,
    timestamp: HtmlElement,
    delta: HtmlElement,
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
    ) -> Result<RAFDemoHandler, JsValue> {
        console_log!("Create RAFDemoHandler");
        let framecount = checked_cast![HtmlElement, framecount]?;
        let timestamp = checked_cast![HtmlElement, timestamp]?;
        let delta = checked_cast![HtmlElement, delta]?;
        let handler = RAFDemoHandler {
            rafloop: None,
            framecount,
            timestamp,
            delta,
        };
        Ok(handler)
    }

    pub fn start(&mut self) {
        console_log!("Start RAFDemoHandler");
        self.rafloop = Some(raf::Loop);
    }

    pub fn stop(&mut self) {
        console_log!("Stop RAFDemoHandler");
        self.rafloop.take();
    }
}

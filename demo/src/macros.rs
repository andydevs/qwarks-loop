/// Logs a formatted message to the browser console via `console.log`.
///
/// Accepts the same format string syntax as [`format!`].
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

/// Attempts to cast a `JsValue` to a concrete JS type via `dyn_into`.
///
/// Returns `Ok(T)` on success, or `Err(JsValue)` with a descriptive message
/// that includes the variable name, expected type, and the actual value received.
#[macro_export]
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

/// Milliseconds per second, used to convert delta time to an FPS value.
pub const MS_PER_SEC: f64 = 1000.00;

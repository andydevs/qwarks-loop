#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

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

pub const MS_PER_SEC: f64 = 1000.00;

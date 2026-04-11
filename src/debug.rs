pub fn input_enabled() -> bool {
    std::env::var_os("WAYDOT_DEBUG_INPUT").is_some_and(|value| !value.is_empty() && value != "0")
}

pub fn input_log(scope: &str, message: impl AsRef<str>) {
    if input_enabled() {
        eprintln!("[waydot][input][{scope}] {}", message.as_ref());
    }
}

mod background;
mod service;
mod shortcuts;

pub use background::{register_host_app, request_background_access, set_background_status};
pub use service::activate_or_register;
pub use shortcuts::{present_window, register_shortcuts};

use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;

use super::history::ClipboardHistory;

const POLL_INTERVAL_MS: u32 = 500;

pub struct ClipboardMonitor {
    history: Rc<RefCell<ClipboardHistory>>,
    last_content: Rc<RefCell<String>>,
}

impl ClipboardMonitor {
    pub fn new(history: Rc<RefCell<ClipboardHistory>>) -> Self {
        Self {
            history,
            last_content: Rc::new(RefCell::new(String::new())),
        }
    }

    pub fn start(&self) {
        let history = self.history.clone();
        let last_content = self.last_content.clone();

        glib::timeout_add_local(
            std::time::Duration::from_millis(POLL_INTERVAL_MS as u64),
            move || {
                let Some(display) = gtk::gdk::Display::default() else {
                    return glib::ControlFlow::Continue;
                };
                let clipboard = display.clipboard();
                let history = history.clone();
                let last_content = last_content.clone();

                clipboard.read_text_async(gtk::gio::Cancellable::NONE, move |result| {
                    if let Ok(Some(text)) = result {
                        let text = text.to_string();
                        let mut last = last_content.borrow_mut();
                        if !text.is_empty() && *last != text {
                            *last = text.clone();
                            history.borrow_mut().push(text);
                        }
                    }
                });

                glib::ControlFlow::Continue
            },
        );
    }
}

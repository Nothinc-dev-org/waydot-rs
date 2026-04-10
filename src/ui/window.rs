use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib;
use libadwaita as adw;

use super::{clipboard_panel, emoji_grid};
use crate::clipboard::{ClipboardHistory, ClipboardMonitor};
use crate::emoji_history::RecentEmojiHistory;
use crate::search::SearchEngine;

thread_local! {
    static VIEW_STACK: RefCell<Option<adw::ViewStack>> = const { RefCell::new(None) };
}

pub fn switch_to_tab(name: &str) {
    VIEW_STACK.with(|stack| {
        if let Some(stack) = stack.borrow().as_ref() {
            stack.set_visible_child_name(name);
        }
    });
}

pub struct Window {
    pub window: adw::Window,
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        let engine = Rc::new(SearchEngine::new());

        let history = Rc::new(RefCell::new(ClipboardHistory::load()));
        let recent_emojis = Rc::new(RefCell::new(RecentEmojiHistory::load()));

        let view_stack = adw::ViewStack::new();

        let emoji_page = emoji_grid::EmojiPage::new(engine.clone(), recent_emojis);
        let emoji_page_handle = emoji_page.handle();
        view_stack.add_titled_with_icon(
            &emoji_page.container,
            Some("emojis"),
            "",
            "emoji-people-symbolic",
        );

        let kaomoji_page = emoji_grid::build_kaomoji_page(engine.as_ref());
        view_stack.add_titled_with_icon(&kaomoji_page, Some("kaomojis"), "", "face-smile-symbolic");

        let symbols_page = emoji_grid::build_symbols_page(engine.as_ref());
        view_stack.add_titled_with_icon(
            &symbols_page,
            Some("symbols"),
            "",
            "emoji-symbols-symbolic",
        );

        let clipboard_page = clipboard_panel::build_clipboard_page(&history);
        view_stack.add_titled_with_icon(
            &clipboard_page,
            Some("clipboard"),
            "",
            "edit-paste-symbolic",
        );

        let switcher = adw::ViewSwitcher::builder()
            .stack(&view_stack)
            .policy(adw::ViewSwitcherPolicy::Wide)
            .build();

        let search_entry = gtk::SearchEntry::builder()
            .placeholder_text("Buscar...")
            .hexpand(true)
            .build();
        search_entry.set_visible(false);

        let header = adw::HeaderBar::builder().title_widget(&switcher).build();

        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.append(&header);
        content.append(&search_entry);
        content.append(&view_stack);

        let window = adw::Window::builder()
            .application(app)
            .title("Waydot")
            .icon_name(crate::app::APPLICATION_ID)
            .hide_on_close(true)
            .default_width(380)
            .default_height(420)
            .content(&content)
            .build();

        let history_for_refresh = history.clone();
        let emoji_page_handle_for_tab_change = emoji_page_handle.clone();
        view_stack.connect_visible_child_name_notify(glib::clone!(
            #[weak]
            view_stack,
            #[weak]
            search_entry,
            #[strong]
            engine,
            move |_| {
                let query = search_entry.text();
                let visible_tab = view_stack.visible_child_name();
                search_entry.set_visible(matches!(
                    visible_tab.as_deref(),
                    Some("kaomojis") | Some("symbols")
                ));

                match visible_tab.as_deref() {
                    Some("clipboard") => {
                        if let Some(child) = view_stack.child_by_name("clipboard") {
                            if let Some(scrolled) = child.downcast_ref::<gtk::ScrolledWindow>() {
                                clipboard_panel::refresh_clipboard_list(
                                    scrolled,
                                    &history_for_refresh,
                                );
                            }
                        }
                    }
                    Some("emojis") => {
                        emoji_page_handle_for_tab_change.refresh();
                    }
                    Some("kaomojis") => {
                        let results = engine.search_kaomojis(query.as_str());
                        if let Some(child) = view_stack.child_by_name("kaomojis") {
                            let page = emoji_grid::build_label_results(&results);
                            let scrolled = child.downcast_ref::<gtk::ScrolledWindow>().unwrap();
                            scrolled.set_child(Some(&page));
                        }
                    }
                    Some("symbols") => {
                        let results = engine.search_symbols(query.as_str());
                        if let Some(child) = view_stack.child_by_name("symbols") {
                            let page = emoji_grid::build_label_results(&results);
                            let scrolled = child.downcast_ref::<gtk::ScrolledWindow>().unwrap();
                            scrolled.set_child(Some(&page));
                        }
                    }
                    _ => {}
                }
            }
        ));

        search_entry.connect_search_changed(glib::clone!(
            #[weak]
            view_stack,
            #[strong]
            engine,
            #[strong]
            emoji_page_handle,
            move |entry| {
                let query = entry.text().to_string();

                match view_stack.visible_child_name().as_deref() {
                    Some("kaomojis") => {
                        let results = engine.search_kaomojis(&query);
                        if let Some(child) = view_stack.child_by_name("kaomojis") {
                            let page = emoji_grid::build_label_results(&results);
                            let scrolled = child.downcast_ref::<gtk::ScrolledWindow>().unwrap();
                            scrolled.set_child(Some(&page));
                        }
                    }
                    Some("symbols") => {
                        let results = engine.search_symbols(&query);
                        if let Some(child) = view_stack.child_by_name("symbols") {
                            let page = emoji_grid::build_label_results(&results);
                            let scrolled = child.downcast_ref::<gtk::ScrolledWindow>().unwrap();
                            scrolled.set_child(Some(&page));
                        }
                    }
                    Some("emojis") => {
                        emoji_page_handle.refresh();
                    }
                    _ => {}
                }
            }
        ));

        let history_for_monitor = history.clone();
        let monitor = ClipboardMonitor::new(
            history,
            glib::clone!(
                #[weak]
                view_stack,
                move || {
                    if view_stack.visible_child_name().as_deref() == Some("clipboard") {
                        if let Some(child) = view_stack.child_by_name("clipboard") {
                            if let Some(scrolled) = child.downcast_ref::<gtk::ScrolledWindow>() {
                                clipboard_panel::refresh_clipboard_list(
                                    scrolled,
                                    &history_for_monitor,
                                );
                            }
                        }
                    }
                }
            ),
        );
        monitor.start();

        VIEW_STACK.with(|slot| {
            *slot.borrow_mut() = Some(view_stack);
        });

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Once;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

use crate::debug;
use crate::emoji_history::RecentEmojiHistory;
use crate::input;
use crate::search::{SearchEngine, SearchResult};

const GRID_COLUMNS: i32 = 8;
const EMOJI_BUTTON_SIZE: i32 = 40;

type EmojiSelectionHandler = Rc<dyn Fn(SearchResult)>;

#[derive(Clone)]
pub struct EmojiPage {
    pub container: gtk::Box,
    controller: EmojiPageHandle,
}

#[derive(Clone)]
pub struct EmojiPageHandle {
    all_store: gio::ListStore,
    recent_scrolled: gtk::ScrolledWindow,
    search_entry: gtk::SearchEntry,
    engine: Rc<SearchEngine>,
    recent_emojis: Rc<RefCell<RecentEmojiHistory>>,
    current_query: Rc<RefCell<String>>,
    on_select: EmojiSelectionHandler,
}

impl EmojiPage {
    pub fn new(engine: Rc<SearchEngine>, recent_emojis: Rc<RefCell<RecentEmojiHistory>>) -> Self {
        install_emoji_tab_css();

        let on_select: EmojiSelectionHandler = Rc::new({
            let recent_emojis = recent_emojis.clone();
            move |result| {
                remember_recent_emoji(&recent_emojis, result);
            }
        });

        let search_entry = gtk::SearchEntry::builder()
            .placeholder_text("Buscar emojis...")
            .hexpand(true)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .build();

        let recent_scrolled = wrap_in_scroll(build_recent_content(&[], Some(on_select.clone())));
        let (all_view, all_store) = build_emoji_grid_view(Some(on_select.clone()));
        replace_emoji_store(&all_store, &engine.search_emojis(""));
        let all_scrolled = wrap_in_scroll(all_view);
        let all_page = gtk::Box::new(gtk::Orientation::Vertical, 0);
        all_page.append(&search_entry);
        all_page.append(&all_scrolled);

        let controller = EmojiPageHandle {
            all_store,
            recent_scrolled: recent_scrolled.clone(),
            search_entry: search_entry.clone(),
            engine,
            recent_emojis,
            current_query: Rc::new(RefCell::new(String::new())),
            on_select,
        };

        let subtab_stack = gtk::Stack::builder()
            .transition_type(gtk::StackTransitionType::None)
            .vexpand(true)
            .build();
        subtab_stack.add_titled(&recent_scrolled, Some("recent"), "Recientes");
        subtab_stack.add_titled(&all_page, Some("all"), "Todos");
        subtab_stack.set_visible_child_name("recent");

        let recent_tab_button = gtk::Button::builder()
            .label("Recientes")
            .hexpand(true)
            .css_classes(["flat", "emoji-subtab-button", "active"])
            .build();
        let all_tab_button = gtk::Button::builder()
            .label("Todos")
            .hexpand(true)
            .css_classes(["flat", "emoji-subtab-button"])
            .build();

        let tab_bar = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .hexpand(true)
            .css_classes(["emoji-subtab-bar"])
            .build();
        tab_bar.append(&recent_tab_button);
        tab_bar.append(&all_tab_button);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.append(&tab_bar);
        container.append(&subtab_stack);

        let page = Self {
            container,
            controller,
        };

        let controller = page.controller.clone();
        search_entry.connect_search_changed(move |_| {
            controller.refresh();
        });

        let stack = subtab_stack.clone();
        recent_tab_button.connect_clicked(move |_| {
            stack.set_visible_child_name("recent");
        });

        let stack = subtab_stack.clone();
        all_tab_button.connect_clicked(move |_| {
            stack.set_visible_child_name("all");
        });

        let controller = page.controller.clone();
        let recent_tab_button_ref = recent_tab_button.clone();
        let all_tab_button_ref = all_tab_button.clone();
        let stack = subtab_stack.clone();
        subtab_stack.connect_visible_child_name_notify(move |_| {
            sync_tab_button_state(
                &recent_tab_button_ref,
                &all_tab_button_ref,
                stack.visible_child_name().as_deref(),
            );
            controller.refresh();
        });

        page.refresh();
        page
    }

    pub fn refresh(&self) {
        self.controller.refresh();
    }

    pub fn handle(&self) -> EmojiPageHandle {
        self.controller.clone()
    }
}

pub fn build_kaomoji_page(engine: &SearchEngine) -> gtk::ScrolledWindow {
    let results = engine.search_kaomojis("");
    let grid = build_label_results(&results);
    wrap_in_scroll(grid)
}

pub fn build_symbols_page(engine: &SearchEngine) -> gtk::ScrolledWindow {
    let results = engine.search_symbols("");
    let grid = build_label_results(&results);
    wrap_in_scroll(grid)
}

impl EmojiPageHandle {
    pub fn refresh(&self) {
        let query = self.search_entry.text();
        if self.current_query.borrow().as_str() != query.as_str() {
            let all_results = self.engine.search_emojis(query.as_str());
            replace_emoji_store(&self.all_store, &all_results);
            self.current_query.replace(query.to_string());
        }

        let recent_results = filter_recent_results(&self.engine, &self.recent_emojis.borrow());
        self.recent_scrolled.set_child(Some(&build_recent_content(
            &recent_results,
            Some(self.on_select.clone()),
        )));
    }
}

pub fn build_emoji_results(
    results: &[SearchResult],
    on_select: Option<EmojiSelectionHandler>,
) -> gtk::FlowBox {
    let flow = gtk::FlowBox::builder()
        .homogeneous(true)
        .max_children_per_line(GRID_COLUMNS as u32)
        .min_children_per_line(GRID_COLUMNS as u32)
        .selection_mode(gtk::SelectionMode::None)
        .build();

    for result in results {
        let text = result.display_text();
        let tooltip = result.label();

        let button = gtk::Button::builder()
            .label(text)
            .tooltip_text(&tooltip)
            .width_request(EMOJI_BUTTON_SIZE)
            .height_request(EMOJI_BUTTON_SIZE)
            .css_classes(["flat", "emoji-button"])
            .build();

        let display = text.to_string();
        let result = result.clone();
        let on_select = on_select.clone();
        button.connect_clicked(move |_| {
            debug::input_log(
                "ui",
                format!("click en emoji button; text={display:?}; tooltip={tooltip:?}"),
            );
            input::copy_text(&display);
            if let Some(on_select) = &on_select {
                on_select(result.clone());
            }
        });

        flow.append(&button);
    }

    flow
}

fn build_emoji_grid_view(
    on_select: Option<EmojiSelectionHandler>,
) -> (gtk::GridView, gio::ListStore) {
    let store = gio::ListStore::new::<glib::BoxedAnyObject>();
    let selection = gtk::NoSelection::new(Some(store.clone()));
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        item.set_activatable(true);

        let label = gtk::Label::builder()
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .justify(gtk::Justification::Center)
            .build();

        let cell = gtk::Box::builder()
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .css_classes(["emoji-grid-item"])
            .build();
        cell.append(&label);

        item.set_child(Some(&cell));
    });

    factory.connect_bind(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let cell = item.child().and_downcast::<gtk::Box>().unwrap();
        let label = cell.first_child().and_downcast::<gtk::Label>().unwrap();
        let emoji = item.item().and_downcast::<glib::BoxedAnyObject>().unwrap();
        let result = emoji.borrow::<SearchResult>();

        label.set_label(result.display_text());
        cell.set_tooltip_text(Some(&result.label()));
    });

    let grid = gtk::GridView::builder()
        .model(&selection)
        .factory(&factory)
        .min_columns(GRID_COLUMNS as u32)
        .max_columns(GRID_COLUMNS as u32)
        .single_click_activate(true)
        .build();

    if let Some(on_select) = on_select {
        let store = store.clone();
        grid.connect_activate(move |_, position| {
            let Some(item) = store.item(position) else {
                return;
            };
            let Ok(item) = item.downcast::<glib::BoxedAnyObject>() else {
                return;
            };
            let result = item.borrow::<SearchResult>().clone();

            let display = result.display_text().to_string();
            debug::input_log("ui", format!("activate en emoji grid; text={display:?}"));
            input::copy_text(&display);
            on_select(result);
        });
    }

    (grid, store)
}

pub fn build_label_results(results: &[SearchResult]) -> gtk::FlowBox {
    let flow = gtk::FlowBox::builder()
        .homogeneous(false)
        .max_children_per_line(4)
        .selection_mode(gtk::SelectionMode::None)
        .build();

    for result in results {
        let text = result.display_text();
        let tooltip = &result.label();

        let button = gtk::Button::builder()
            .label(text)
            .tooltip_text(tooltip)
            .css_classes(["flat"])
            .build();

        let display = text.to_string();
        button.connect_clicked(move |_| {
            debug::input_log("ui", format!("click en label result; text={display:?}"));
            input::copy_text(&display);
        });

        flow.append(&button);
    }

    flow
}

fn wrap_in_scroll(child: impl IsA<gtk::Widget>) -> gtk::ScrolledWindow {
    gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .child(&child)
        .build()
}

fn remember_recent_emoji(recent_emojis: &Rc<RefCell<RecentEmojiHistory>>, result: SearchResult) {
    let Some(glyph) = result.emoji_glyph() else {
        return;
    };

    recent_emojis.borrow_mut().push(glyph);
}

fn filter_recent_results(
    engine: &SearchEngine,
    recent_emojis: &RecentEmojiHistory,
) -> Vec<SearchResult> {
    let matching = engine
        .search_emojis("")
        .into_iter()
        .filter_map(|result| {
            let glyph = result.emoji_glyph()?.to_string();
            Some((glyph, result))
        })
        .collect::<HashMap<_, _>>();

    recent_emojis
        .entries()
        .iter()
        .filter_map(|glyph| matching.get(glyph).cloned())
        .collect()
}

fn replace_emoji_store(store: &gio::ListStore, results: &[SearchResult]) {
    store.remove_all();

    for result in results {
        store.append(&glib::BoxedAnyObject::new(result.clone()));
    }
}

fn sync_tab_button_state(
    recent_tab_button: &gtk::Button,
    all_tab_button: &gtk::Button,
    visible_child: Option<&str>,
) {
    set_tab_button_active(recent_tab_button, visible_child == Some("recent"));
    set_tab_button_active(all_tab_button, visible_child == Some("all"));
}

fn set_tab_button_active(button: &gtk::Button, active: bool) {
    if active {
        button.add_css_class("active");
    } else {
        button.remove_css_class("active");
    }
}

fn install_emoji_tab_css() {
    static INSTALLED: Once = Once::new();

    INSTALLED.call_once(|| {
        let Some(display) = gtk::gdk::Display::default() else {
            return;
        };

        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            "
            .emoji-subtab-bar {
                margin: 0;
            }

            .emoji-subtab-button {
                border-radius: 0;
                min-height: 38px;
                margin: 0;
                box-shadow: none;
            }

            .emoji-subtab-button.active {
                background: alpha(@accent_bg_color, 0.18);
                box-shadow: inset 0 -2px @accent_bg_color;
            }

            .emoji-grid-item {
                min-width: 40px;
                min-height: 40px;
                border-radius: 8px;
            }

            .emoji-grid-item:hover {
                background: alpha(@accent_bg_color, 0.12);
            }
            ",
        );

        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
}

fn build_recent_content(
    results: &[SearchResult],
    on_select: Option<EmojiSelectionHandler>,
) -> gtk::Box {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);

    if results.is_empty() {
        let empty = gtk::Label::builder()
            .label("Todavia no hay emojis recientes.")
            .wrap(true)
            .justify(gtk::Justification::Center)
            .margin_top(48)
            .margin_bottom(48)
            .margin_start(24)
            .margin_end(24)
            .build();
        content.append(&empty);
    } else {
        content.append(&build_emoji_results(results, on_select));
    }

    content
}

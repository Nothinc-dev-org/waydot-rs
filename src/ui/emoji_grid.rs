use gtk::prelude::*;

use crate::input;
use crate::search::{SearchEngine, SearchResult};

const GRID_COLUMNS: i32 = 8;
const EMOJI_BUTTON_SIZE: i32 = 40;

pub fn build_emoji_page(engine: &SearchEngine) -> gtk::ScrolledWindow {
    let results = engine.search_emojis("");
    let grid = build_emoji_results(&results);
    wrap_in_scroll(grid)
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

pub fn build_emoji_results(results: &[SearchResult]) -> gtk::FlowBox {
    let flow = gtk::FlowBox::builder()
        .homogeneous(true)
        .max_children_per_line(GRID_COLUMNS as u32)
        .min_children_per_line(GRID_COLUMNS as u32)
        .selection_mode(gtk::SelectionMode::None)
        .build();

    for result in results {
        let text = result.display_text();
        let tooltip = &result.label();

        let button = gtk::Button::builder()
            .label(text)
            .tooltip_text(tooltip)
            .width_request(EMOJI_BUTTON_SIZE)
            .height_request(EMOJI_BUTTON_SIZE)
            .css_classes(["flat", "emoji-button"])
            .build();

        let display = text.to_string();
        button.connect_clicked(move |_| {
            input::inject_text(&display);
        });

        flow.append(&button);
    }

    flow
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
            input::inject_text(&display);
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

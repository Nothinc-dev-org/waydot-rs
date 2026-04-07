use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;

use crate::clipboard::ClipboardHistory;

pub fn build_clipboard_page(history: &Rc<RefCell<ClipboardHistory>>) -> gtk::ScrolledWindow {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let toolbar = build_toolbar(history);
    container.append(&toolbar);

    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    populate_list(&list_box, history);
    container.append(&list_box);

    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .child(&container)
        .build();

    scrolled
}

pub fn refresh_clipboard_list(
    scrolled: &gtk::ScrolledWindow,
    history: &Rc<RefCell<ClipboardHistory>>,
) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let toolbar = build_toolbar(history);
    container.append(&toolbar);

    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    populate_list(&list_box, history);
    container.append(&list_box);

    scrolled.set_child(Some(&container));
}

fn build_toolbar(history: &Rc<RefCell<ClipboardHistory>>) -> gtk::Box {
    let clear_button = gtk::Button::builder()
        .label("Limpiar")
        .css_classes(["flat"])
        .tooltip_text("Eliminar entradas no ancladas")
        .halign(gtk::Align::End)
        .margin_top(4)
        .margin_bottom(4)
        .margin_end(8)
        .build();

    let history_ref = history.clone();
    clear_button.connect_clicked(move |_| {
        history_ref.borrow_mut().clear_unpinned();
    });

    let toolbar = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .build();

    let spacer = gtk::Box::builder().hexpand(true).build();
    toolbar.append(&spacer);
    toolbar.append(&clear_button);

    toolbar
}

fn populate_list(list_box: &gtk::ListBox, history: &Rc<RefCell<ClipboardHistory>>) {
    let entries = history.borrow();
    if entries.entries().is_empty() {
        let label = gtk::Label::builder()
            .label("El portapapeles esta vacio")
            .css_classes(["dim-label"])
            .margin_top(24)
            .margin_bottom(24)
            .build();
        list_box.append(&label);
        return;
    }

    for (i, entry) in entries.entries().iter().enumerate() {
        let row = build_entry_row(i, entry, history);
        list_box.append(&row);
    }
}

fn build_entry_row(
    index: usize,
    entry: &crate::clipboard::ClipboardEntry,
    history: &Rc<RefCell<ClipboardHistory>>,
) -> gtk::Box {
    let preview = truncate(&entry.content, 80);

    let label = gtk::Label::builder()
        .label(&preview)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .max_width_chars(50)
        .build();

    let pin_icon = "view-pin-symbolic";
    let pin_button = gtk::Button::builder()
        .icon_name(pin_icon)
        .css_classes(["flat", "circular"])
        .tooltip_text(if entry.pinned { "Desanclar" } else { "Anclar" })
        .build();

    let copy_button = gtk::Button::builder()
        .icon_name("edit-copy-symbolic")
        .css_classes(["flat", "circular"])
        .tooltip_text("Copiar")
        .build();

    let delete_button = gtk::Button::builder()
        .icon_name("user-trash-symbolic")
        .css_classes(["flat", "circular"])
        .tooltip_text("Eliminar")
        .build();

    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .margin_top(4)
        .margin_bottom(4)
        .margin_start(8)
        .margin_end(8)
        .build();

    row.append(&label);
    row.append(&pin_button);
    row.append(&copy_button);
    row.append(&delete_button);

    let content = entry.content.clone();
    copy_button.connect_clicked(move |_| {
        if let Some(display) = gtk::gdk::Display::default() {
            display.clipboard().set_text(&content);
        }
    });

    let history_ref = history.clone();
    pin_button.connect_clicked(move |btn| {
        history_ref.borrow_mut().toggle_pin(index);
        let pinned = history_ref.borrow().entries()[index].pinned;
        btn.set_icon_name(pin_icon);
        btn.set_tooltip_text(Some(if pinned { "Desanclar" } else { "Anclar" }));
    });

    let history_ref = history.clone();
    delete_button.connect_clicked(move |_| {
        history_ref.borrow_mut().remove(index);
    });

    row
}

fn truncate(s: &str, max: usize) -> String {
    let single_line: String = s.chars().map(|c| if c == '\n' { ' ' } else { c }).collect();
    if single_line.len() <= max {
        single_line
    } else {
        format!("{}...", &single_line[..max])
    }
}

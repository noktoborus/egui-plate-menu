//! The window of the demo: the switches on the left, what the menu answered
//! on the right, the menu over both.

use egui_plate_menu::{Beside, MenuItem, Notice, PlateMenu};

use crate::entries::{Details, LevelNotice, Set, Shape, entries};

/// What stands above the plates of the whole menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuNotice {
    None,
    Text,
    Widget,
}

/// Which call opens the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opening {
    /// `open`: no entry selected.
    Plain,
    /// `open_at`: on an entry, in the level that holds it.
    At,
    /// `open_in`: inside a level, no entry selected.
    In,
}

/// How many plates the menu shows at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Plates {
    Auto,
    Five,
    Ten,
}

pub struct Demo {
    menu: PlateMenu,
    shape: Shape,
    notice: MenuNotice,
    opening: Opening,
    beside: Beside,
    plates: Plates,
    refill: bool,
    last_tick: f64,
    chosen: Vec<String>,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            menu: PlateMenu::new(),
            shape: Shape::default(),
            notice: MenuNotice::Text,
            opening: Opening::Plain,
            beside: Beside::Answers,
            plates: Plates::Auto,
            refill: false,
            last_tick: 0.0,
            chosen: Vec::new(),
        }
    }
}

impl Demo {
    /// Opens the menu as the switches say.
    fn open(&mut self) {
        let mut menu = PlateMenu::new().with_beside(self.beside);
        menu = match self.plates {
            Plates::Auto => menu,
            Plates::Five => menu.max_plates(5),
            Plates::Ten => menu.max_plates(10),
        };
        self.menu = menu;
        let items = entries(&self.shape);
        let opened = match self.opening {
            Opening::Plain => self.menu.open(items),
            Opening::At => {
                let at = target(&items, false);
                self.menu.open_at(items, &at)
            }
            Opening::In => {
                let at = target(&items, true);
                self.menu.open_in(items, &at)
            }
        };
        if let Err(error) = opened {
            self.chosen.push(format!("the menu did not open: {error}"));
            return;
        }
        match self.notice {
            MenuNotice::None => {}
            MenuNotice::Text => self.menu.notice(format!(
                "{}: walk with the arrows, type to search, Esc to go back",
                self.shape.set.name()
            )),
            MenuNotice::Widget => self.menu.notice(Notice::widget(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("A notice that is a widget").strong());
                    ui.add(egui::Spinner::new());
                });
            })),
        }
    }

    fn panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("egui-plate-menu");
        ui.hyperlink_to(
            "the repository",
            "https://github.com/noktoborus/egui-plate-menu",
        );
        ui.add_space(8.0);
        if ui
            .add_sized(
                [ui.available_width(), 32.0],
                egui::Button::new("Open the menu"),
            )
            .clicked()
        {
            self.open();
        }
        ui.label(egui::RichText::new("or press the right button anywhere").weak());
        ui.separator();

        let shape = &mut self.shape;
        ui.label(egui::RichText::new("Entries").strong());
        for set in Set::ALL {
            ui.radio_value(&mut shape.set, set, set.name());
        }
        ui.checkbox(&mut shape.icons, "Icons before the labels");
        ui.horizontal_wrapped(|ui| {
            ui.label("Right of a label:");
            ui.radio_value(&mut shape.details, Details::None, "nothing");
            ui.radio_value(&mut shape.details, Details::Text, "a text");
            ui.radio_value(&mut shape.details, Details::Widget, "a widget");
        });
        ui.checkbox(&mut shape.hints, "Hints under the pointer");
        ui.checkbox(&mut shape.disabled, "Entries that cannot be chosen");
        ui.checkbox(&mut shape.separators, "Lines between groups");
        ui.checkbox(&mut shape.split, "Entries chosen and stepped into (zone ›)");
        ui.horizontal_wrapped(|ui| {
            ui.label("Notice of a level:");
            ui.radio_value(&mut shape.level_notice, LevelNotice::None, "none");
            ui.radio_value(&mut shape.level_notice, LevelNotice::Text, "a text");
            ui.radio_value(&mut shape.level_notice, LevelNotice::Widget, "a widget");
        });
        ui.checkbox(
            &mut shape.hidden_search,
            "Words a query finds, entries it does not",
        );

        ui.separator();
        ui.label(egui::RichText::new("The menu").strong());
        ui.horizontal_wrapped(|ui| {
            ui.label("Notice:");
            ui.radio_value(&mut self.notice, MenuNotice::None, "none");
            ui.radio_value(&mut self.notice, MenuNotice::Text, "a text");
            ui.radio_value(&mut self.notice, MenuNotice::Widget, "a widget");
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Opens:");
            ui.radio_value(&mut self.opening, Opening::Plain, "open");
            ui.radio_value(&mut self.opening, Opening::At, "open_at");
            ui.radio_value(&mut self.opening, Opening::In, "open_in");
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Plates:");
            ui.radio_value(&mut self.plates, Plates::Auto, "as fit");
            ui.radio_value(&mut self.plates, Plates::Five, "5");
            ui.radio_value(&mut self.plates, Plates::Ten, "10");
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("A click beside:");
            ui.radio_value(&mut self.beside, Beside::Closes, "closes");
            ui.radio_value(&mut self.beside, Beside::Answers, "answers");
            ui.radio_value(&mut self.beside, Beside::Ignored, "is ignored");
        });
        ui.checkbox(&mut self.refill, "Refilled every second while it stands");
    }

    fn answers(&self, ui: &mut egui::Ui) {
        ui.heading("What the menu says");
        let selected = self
            .menu
            .selected()
            .map_or("none".to_string(), |row| row.to_string());
        let level = self.menu.level();
        egui::Grid::new("state").num_columns(2).show(ui, |ui| {
            ui.label("open");
            ui.label(self.menu.is_open().to_string());
            ui.end_row();
            ui.label("selected()");
            ui.label(selected);
            ui.end_row();
            ui.label("level()");
            ui.label(if level.is_empty() {
                "the top".to_string()
            } else {
                level.join(" › ")
            });
            ui.end_row();
        });
        ui.add_space(12.0);
        ui.label(egui::RichText::new("Chosen, the last first").strong());
        for line in self.chosen.iter().rev().take(20) {
            ui.monospace(line);
        }
    }

    /// Lays a layer that takes every press under the menu while it stands,
    /// and the menu over it: nothing behind it is reached.
    fn block_under_the_menu(&self, context: &egui::Context) {
        if !self.menu.is_open() {
            return;
        }
        egui::Modal::new(egui::Id::unique("demo-blocker"))
            .backdrop_color(egui::Color32::from_black_alpha(40))
            .frame(egui::Frame::NONE)
            .show(context, |_| {});
        context.move_to_top(self.menu.layer_id());
    }
}

impl eframe::App for Demo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let context = ui.ctx().clone();
        egui::Panel::left("switches")
            .resizable(false)
            .exact_size(360.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| self.panel(ui));
            });
        egui::CentralPanel::default().show(ui, |ui| {
            self.answers(ui);
            let response = ui.interact(
                ui.max_rect(),
                egui::Id::unique("demo-ground"),
                egui::Sense::click(),
            );
            if response.secondary_clicked() && !self.menu.is_open() {
                self.open();
            }
        });

        if self.refill && self.menu.is_open() {
            let now = context.input(|input| input.time);
            if now - self.last_tick >= 1.0 {
                self.last_tick = now;
                self.shape.tick += 1;
                let _ = self.menu.refill(entries(&self.shape));
            }
            context.request_repaint_after(std::time::Duration::from_millis(250));
        }

        self.block_under_the_menu(&context);
        if let Some(chosen) = self.menu.show(&context) {
            let held = chosen.held;
            let mut keys = Vec::new();
            for (down, name) in [
                (held.ctrl, "Ctrl"),
                (held.shift, "Shift"),
                (held.alt, "Alt"),
            ] {
                if down {
                    keys.push(name);
                }
            }
            let keys = match keys.is_empty() {
                true => String::new(),
                false => format!(" with {}", keys.join("+")),
            };
            self.chosen.push(format!("{}{keys}", chosen.id));
        }
    }
}

/// The entry `open_at` opens on, or the level `open_in` opens in: the first
/// entry with entries of its own when one is asked for, the last reachable
/// entry of the top otherwise.
fn target(items: &[MenuItem], level: bool) -> String {
    let found = match level {
        true => items.iter().find(|item| item.has_children()),
        false => items.iter().rev().find(|item| item.is_reachable()),
    };
    found.map(|item| item.id.clone()).unwrap_or_default()
}

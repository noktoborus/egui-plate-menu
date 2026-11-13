//! The entries of the menu, built from the switches of the panel.

use egui_plate_menu::{Detail, MenuItem, Notice};

/// Which entries the menu holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Set {
    /// Commands of an editor, with their keys.
    Commands,
    /// Levels inside levels: settings with their values.
    Tree,
    /// More entries than the window holds.
    Long,
    /// Labels cut short, each with its whole on a plate of its own.
    Wholes,
}

impl Set {
    pub const ALL: [Set; 4] = [Set::Commands, Set::Tree, Set::Long, Set::Wholes];

    pub fn name(self) -> &'static str {
        match self {
            Set::Commands => "Commands",
            Set::Tree => "Levels",
            Set::Long => "A long list",
            Set::Wholes => "Long labels",
        }
    }
}

/// What stands right of a label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Details {
    None,
    Text,
    Widget,
}

/// What stands above the plates of a level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelNotice {
    None,
    Text,
    Widget,
}

/// The switches of the panel that shape the entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    pub set: Set,
    pub icons: bool,
    pub details: Details,
    pub hints: bool,
    pub disabled: bool,
    pub separators: bool,
    pub split: bool,
    pub level_notice: LevelNotice,
    pub hidden_search: bool,
    /// A count that changes, for the menu refilled while it stands.
    pub tick: u32,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            set: Set::Commands,
            icons: true,
            details: Details::Text,
            hints: true,
            disabled: true,
            separators: true,
            split: true,
            level_notice: LevelNotice::Text,
            hidden_search: true,
            tick: 0,
        }
    }
}

/// The entries the switches ask for.
pub fn entries(shape: &Shape) -> Vec<MenuItem> {
    match shape.set {
        Set::Commands => commands(shape),
        Set::Tree => tree(shape),
        Set::Long => long(shape),
        Set::Wholes => wholes(shape),
    }
}

/// An entry with the icon, the detail and the hint the switches ask for.
fn entry(
    shape: &Shape,
    id: &str,
    label: &str,
    icon: &str,
    detail: &str,
    colour: [u8; 3],
) -> MenuItem {
    let mut item = MenuItem::new(id, label);
    if shape.icons {
        item = item.icon(icon);
    }
    item = match shape.details {
        Details::None => item,
        Details::Text => item.detail(detail),
        Details::Widget => item.detail(swatch(colour)),
    };
    if shape.hints {
        item = item.hint(format!("{label}: the hint shown while the pointer rests"));
    }
    item
}

/// A square of a colour, a widget right of a label.
fn swatch(colour: [u8; 3]) -> Detail {
    Detail::widget(28.0, move |ui| {
        let size = egui::vec2(24.0, ui.available_height().min(14.0));
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().rect_filled(
            rect,
            3.0,
            egui::Color32::from_rgb(colour[0], colour[1], colour[2]),
        );
    })
}

fn separator(shape: &Shape, items: &mut Vec<MenuItem>) {
    if shape.separators {
        items.push(MenuItem::separator());
    }
}

fn commands(shape: &Shape) -> Vec<MenuItem> {
    let mut items = vec![
        entry(shape, "copy", "Copy", "📋", "ctrl+c", [90, 140, 220]),
        entry(shape, "paste", "Paste", "📄", "ctrl+v", [80, 180, 120]),
        entry(shape, "cut", "Cut", "✂", "ctrl+x", [220, 120, 90]),
    ];
    separator(shape, &mut items);
    items.push(
        entry(shape, "save", "Save", "💾", "ctrl+s", [200, 180, 80]).enabled(!shape.disabled),
    );
    items.push(entry(
        shape,
        "find",
        "Find",
        "🔍",
        "ctrl+f",
        [160, 120, 220],
    ));
    items.push(entry(
        shape,
        "refreshed",
        "Lines counted",
        "⟳",
        &shape.tick.to_string(),
        [120, 120, 120],
    ));
    if shape.hidden_search {
        items.push(
            entry(shape, "quit", "Quit", "⏻", "ctrl+q", [200, 80, 80]).search("exit leave close"),
        );
    } else {
        items.push(entry(shape, "quit", "Quit", "⏻", "ctrl+q", [200, 80, 80]));
    }
    items
}

/// A level: its notice, as the switch says.
fn level(shape: &Shape, item: MenuItem, says: &'static str) -> MenuItem {
    match shape.level_notice {
        LevelNotice::None => item,
        LevelNotice::Text => item.notice(says),
        LevelNotice::Widget => item.notice(Notice::widget(move |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚙").strong());
                ui.label(says);
            });
            ui.add(egui::ProgressBar::new(0.6).text("a widget of the caller"));
        })),
    }
}

fn tree(shape: &Shape) -> Vec<MenuItem> {
    let theme = level(
        shape,
        entry(shape, "theme", "Theme", "🎨", "Dark", [60, 60, 70]).children(vec![
            entry(shape, "theme.dark", "Dark", "🌙", "", [30, 30, 40]),
            entry(shape, "theme.light", "Light", "☀", "", [240, 240, 230]),
            entry(
                shape,
                "theme.system",
                "As the system",
                "🖥",
                "",
                [128, 128, 128],
            ),
        ]),
        "How the window is drawn",
    )
    .opens_at("theme.dark");

    let ports = ["ttyUSB0", "ttyUSB1", "ttyACM0"];
    let port_items: Vec<MenuItem> = ports
        .iter()
        .map(|port| {
            let item = entry(
                shape,
                &format!("port.{port}"),
                port,
                "🔌",
                "115200",
                [90, 160, 200],
            );
            if shape.split {
                level(
                    shape,
                    item.choosable(true).children(vec![
                        entry(
                            shape,
                            &format!("port.{port}.speed"),
                            "Speed",
                            "⏱",
                            "115200",
                            [90, 160, 200],
                        ),
                        entry(
                            shape,
                            &format!("port.{port}.start"),
                            "At start",
                            "▶",
                            "",
                            [120, 200, 120],
                        ),
                    ]),
                    "A port: chosen from its label, its settings behind ›",
                )
            } else {
                item
            }
        })
        .collect();
    let port = level(
        shape,
        entry(shape, "port", "Port", "🔌", "ttyUSB0", [90, 160, 200]).children(port_items),
        "Which port the window opens",
    );

    let mut font_sizes: Vec<MenuItem> = (10..=20)
        .map(|size| {
            entry(
                shape,
                &format!("size.{size}"),
                &format!("{size} points"),
                "🔤",
                "",
                [180, 180, 180],
            )
        })
        .collect();
    if shape.hidden_search {
        font_sizes.push(
            MenuItem::new("size.reset", "Back to the default")
                .searchable(false)
                .hint("not found by a query: reached through its level alone"),
        );
    }
    let font = level(
        shape,
        entry(shape, "font", "Font size", "🔤", "13", [180, 180, 180]).children(font_sizes),
        "The size of the text",
    )
    .opens_at("size.13");

    let mut items = vec![theme, port, font];
    separator(shape, &mut items);
    items.push(entry(shape, "about", "About", "ℹ", "", [120, 120, 120]).enabled(!shape.disabled));
    items
}

fn long(shape: &Shape) -> Vec<MenuItem> {
    let mut items = Vec::new();
    for index in 1..=60 {
        if index % 10 == 1 && index > 1 {
            separator(shape, &mut items);
        }
        let colour = [(index * 4) as u8, 120, (240 - index * 3) as u8];
        let item = entry(
            shape,
            &format!("line.{index}"),
            &format!("Line {index}"),
            "≡",
            &format!("{}", index * 7),
            colour,
        );
        items.push(item.enabled(!(shape.disabled && index % 7 == 0)));
    }
    items
}

fn wholes(shape: &Shape) -> Vec<MenuItem> {
    let commands = [
        "make test && cargo clippy --workspace --all-targets && cargo test --workspace && cargo audit",
        "ssh build@farm 'cd /srv/build && git pull && ./configure --prefix=/usr && make -j32 install'",
        "tmux new-session -d -s logs 'journalctl -f -u serial-getty@ttyUSB0' \\; attach -t logs",
    ];
    let mut items: Vec<MenuItem> = commands
        .iter()
        .enumerate()
        .map(|(index, command)| {
            let cut: String = command.chars().take(28).collect();
            entry(
                shape,
                &format!("command.{index}"),
                &format!("{cut} …"),
                "❯",
                "",
                [100, 200, 100],
            )
            .full(*command)
        })
        .collect();
    separator(shape, &mut items);
    items.push(
        entry(shape, "note", "A note", "✎", "", [220, 200, 100]).full(
            "A whole that is no command: what more an entry says,\non lines of its own,\nfor as long as it needs.",
        ),
    );
    items
}

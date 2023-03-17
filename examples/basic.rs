use egui::{CentralPanel, Context, Widget};
use egui_keybinds::{KeyBind, KeyBindWidget, KeyCode};

struct Gui {
    key1: KeyBind,
    key2: KeyBind,
}

impl Gui {
    fn new() -> Self {
        Self {
            key1: KeyBind::new(Some(KeyCode::A), vec![]),
            key2: KeyBind::new(Some(KeyCode::B), vec![]),
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Example keybind 1");
            KeyBindWidget::new(&mut self.key1).ui(ui);

            ui.label("Example keybind 2");
            KeyBindWidget::new(&mut self.key2).ui(ui);
        });
    }
}

fn main() {
    eframe::run_native(
        "testing",
        Default::default(),
        Box::new(|_| Box::new(Gui::new())),
    )
    .unwrap();
}

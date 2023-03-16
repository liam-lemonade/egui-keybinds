use std::time::Duration;

use egui::{vec2, CentralPanel, Context, Widget};
use egui_keybinds::{KeyBind, KeyCode, KeybindWidget};

struct Gui {
    key: KeyBind,
}

impl Gui {
    fn new() -> Self {
        Self {
            key: KeyBind::new(KeyCode::A, vec![]),
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(50)); // 20fps, fixes lazy redraw

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Example keybind");

            KeybindWidget::new(&mut self.key).ui(ui);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(650_f32, 350_f32)),
        follow_system_theme: true,
        ..Default::default()
    };

    let gui = Gui::new();
    eframe::run_native("testing", options, Box::new(|_| Box::new(gui)));
}

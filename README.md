# egui-keybinds
designed for ease-of-use, **egui-keybinds** provides keybinding functionality to [egui](https://crates.io/crates/egui)

# Example

```rust
use  egui::{CentralPanel, Context, Widget};
use  egui_keybinds::{KeyBind, KeyBindWidget, KeyCode}; 

struct  Gui {
	key1: KeyBind,
	key2: KeyBind,
} 

impl  Gui {
	fn  new() ->  Self {
		Self {
		key1: KeyBind::new(KeyCode::A, vec![]),
		key2: KeyBind::new(KeyCode::B, vec![]),
		}
	}
}
  
impl  eframe::App  for  Gui {
	fn  update(&mut  self, ctx: &Context, _frame: &mut  eframe::Frame) {
		CentralPanel::default().show(ctx, |ui| {
			ui.label("Example keybind 1");
			KeyBindWidget::new(&mut  self.key1).ui(ui);
			
			ui.label("Example keybind 2");
			KeyBindWidget::new(&mut  self.key2).ui(ui);
		});
	}
}

fn  main() {
	eframe::run_native(
		"testing", 
		Default::default(), 
		
		Box::new(
			|_| Box::new(Gui::new())
		)
	).unwrap();
}
```

# Add to your project

run `cargo add egui-keybinds` in your terminal of choice while CD'd into the root directory of your project.
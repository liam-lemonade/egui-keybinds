use crate::{keycodes::KeyModifier, *};
use device_query::{DeviceQuery, DeviceState};
use egui::{Align2, FontId, Id, Response, Sense, Ui, Widget};
use egui_modal::Modal;

pub struct KeybindWidget<'a> {
    value: &'a mut KeyBind,

    state: KeybindWidgetState,
}

#[derive(Clone)]
struct KeybindWidgetState {
    active: bool,
    device_state: DeviceState,

    escape_type: EscapeType,

    last_keys: Vec<device_query::Keycode>,
    in_escape_ui: bool,

    mods: Vec<KeyModifier>,
}

impl Default for KeybindWidgetState {
    fn default() -> Self {
        KeybindWidgetState {
            active: false,
            device_state: DeviceState::new(),

            escape_type: EscapeType::None,

            last_keys: Vec::new(),
            in_escape_ui: false,

            mods: Vec::new(),
        }
    }
}

impl<'a> KeybindWidget<'a> {
    pub fn new(value: &'a mut KeyBind) -> Self {
        Self {
            value,

            state: KeybindWidgetState {
                active: false,
                device_state: DeviceState::new(),

                escape_type: EscapeType::None,

                last_keys: Vec::new(),
                in_escape_ui: false,

                mods: Vec::new(),
            },
        }
    }
}

impl KeybindWidget<'_> {
    fn draw_binding_modal(&mut self, ui: &mut Ui) -> Modal {
        let modal = Modal::new(ui.ctx(), "_binding_modal");

        modal.show(|ui| {
            modal.frame(ui, |ui| {
                ui.label("Press any key...");
            });
        });

        modal.open();

        modal
    }

    fn draw_escape_modal(&mut self, ui: &mut Ui) -> bool {
        if self.state.escape_type == EscapeType::None {
            self.state.escape_type = EscapeType::Bind; // fix None
        }

        self.state.in_escape_ui = true; // fix None

        let modal = Modal::new(ui.ctx(), "_escape_modal");
        let mut result = false;

        modal.show(|ui| {
            modal.title(ui, "It seems you've pressed \"Escape\"");

            modal.frame(ui, |ui| {
                ui.label("This is, unfortunately, ambigious.");
                ui.label("What did you intend to do?");

                ui.add_space(10_f32);

                ui.radio_value(
                    &mut self.state.escape_type,
                    EscapeType::Bind,
                    "Set bind to \"Escape\"",
                );

                ui.radio_value(
                    &mut self.state.escape_type,
                    EscapeType::Clear,
                    "Remove keybinding",
                );

                ui.radio_value(
                    &mut self.state.escape_type,
                    EscapeType::Cancel,
                    "Keep same keybind",
                );
            });

            modal.buttons(ui, |ui| {
                if modal.button(ui, "Confirm").clicked() {
                    self.state.in_escape_ui = false;
                    result = true;
                };
            });
        });

        modal.open();
        return result;
    }

    fn handle_escape(&mut self) {
        match self.state.escape_type {
            EscapeType::Bind => {
                self.value.key = Some(KeyCode::Escape);
                self.value.modifiers = self.state.mods.clone();
            }

            EscapeType::Clear => {
                self.value.key = None;
                self.value.modifiers = Vec::new();
            }

            _ => (),
        }

        self.state.active = false;
        self.state.mods = Vec::new();
        self.state.last_keys = Vec::new();
    }

    fn assign(&mut self, key: KeyCode) {
        self.value.key = Some(key);
        self.value.modifiers = self.state.mods.clone();

        self.state.active = false;
        self.state.mods = Vec::new();
        self.state.last_keys = Vec::new();
    }

    fn run_input(&mut self, ui: &mut Ui) {
        if self.state.in_escape_ui {
            if self.draw_escape_modal(ui) {
                self.handle_escape();
            }

            return;
        }

        let binding_modal = self.draw_binding_modal(ui);
        let keys = self.state.device_state.get_keys();

        for key in &keys {
            if self.state.last_keys.contains(&key) {
                continue;
            }

            // key down
            let keycode = KeyCode::from(key.clone());

            if let Some(modifier) = keycode.as_modifier() {
                self.state.mods.push(modifier);
                continue;
            }

            if keycode == KeyCode::Escape {
                if self.state.escape_type == EscapeType::None {
                    binding_modal.close();

                    self.draw_escape_modal(ui);
                } else {
                    self.handle_escape();
                    return;
                }

                continue;
            }

            self.assign(keycode.clone());
            return;
        }

        for key in &self.state.last_keys {
            if keys.contains(&key) {
                continue;
            }

            // key up
            let keycode = KeyCode::from(key.clone());

            if keycode == KeyCode::Escape {
                continue;
            }

            if let Some(modifier) = keycode.as_modifier() {
                self.state.mods.retain(|m| *m != modifier);
            }

            self.assign(keycode.clone());
            return;
        }

        self.state.last_keys = keys;
    }
}

impl Widget for KeybindWidget<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        //let id = ui.make_persistent_id(self.id);

        let (response, painter) = ui.allocate_painter(ui.spacing().interact_size, Sense::click());

        self.state = ui.data_mut(|d| {
            d.get_temp(Id::null())
                .unwrap_or(KeybindWidgetState::default())
        });

        let visuals = ui.style().interact_selectable(&response, self.state.active); // get the current interactable style settings

        painter.rect_filled(response.rect, visuals.rounding, visuals.bg_fill); // draw a lil button :)
        painter.text(
            response.rect.center(),
            Align2::CENTER_CENTER,
            self.value.serialize(),
            FontId::default(),
            visuals.text_color(),
        );

        if self.state.active {
            self.run_input(ui);
        }

        if response.clicked() {
            self.state.active = true; // clicked, now active keybind
        }

        ui.data_mut(|d| d.insert_temp(Id::null(), self.state));

        return response;
    }
}

#[derive(PartialEq, Clone)]
enum EscapeType {
    Bind,
    Clear,
    Cancel,
    None,
}

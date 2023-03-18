use crate::{keycodes::KeyModifier, *};
use device_query::{DeviceQuery, DeviceState};

use egui::{Align2, FontId, Id, Response, Sense, Ui, Widget};
use once_cell::sync::Lazy;

pub struct KeyBindWidget<'a> {
    value: &'a mut KeyBind,
    id: Id,
}

#[derive(Clone, Hash)]
pub struct KeyBindWidgetState {
    active: bool,                     // is this keybind currently being set
    modifiers_held: Vec<KeyModifier>, // current held modifier keys
    last_keys_held: Vec<device_query::Keycode>,
    keys_held: Vec<device_query::Keycode>,
}

impl KeyBindWidgetState {
    fn new() -> Self {
        Self {
            active: false,
            modifiers_held: vec![],
            last_keys_held: vec![],
            keys_held: vec![],
        }
    }
}

impl<'a> KeyBindWidget<'a> {
    pub fn new(value: &'a mut KeyBind) -> Self {
        let addr = value as *const KeyBind as u64; // get pointer of KeyBind as a u64

        Self {
            value,
            id: Id::new(addr), // create id from KeyBind pointer (should be unique)
        }
    }
}

const DEVICE_STATE_ID: Lazy<Id> = Lazy::new(|| Id::new("_keybind_device_state"));

impl KeyBindWidget<'_> {
    // returns true if the bind was set
    fn run_input(
        &mut self,
        down: Vec<device_query::Keycode>,
        up: Vec<device_query::Keycode>,
        state: &mut KeyBindWidgetState,
    ) -> bool {
        for device_key in down {
            let key = KeyCode::from(device_key);

            if let Some(modifier) = key.as_modifier() {
                // prevent double mods, eg lctrl and rctrl
                if !state.modifiers_held.contains(&modifier) {
                    state.modifiers_held.push(modifier);
                }
            } else {
                self.value.key = Some(key);
                self.value.modifiers = state.modifiers_held.clone();

                return true;
            }
        }

        for device_key in up {
            let key = KeyCode::from(device_key);

            if let Some(modifier) = key.as_modifier() {
                state.modifiers_held.retain(|m| *m != modifier);

                self.value.key = Some(key);
                self.value.modifiers = state.modifiers_held.clone();

                return true;
            }
        }

        return false;
    }

    fn check_keys(
        &mut self,
        device: &DeviceState,
        state: &mut KeyBindWidgetState,
    ) -> (Vec<device_query::Keycode>, Vec<device_query::Keycode>) {
        state.keys_held = device.get_keys();

        let pressed_keys = helper::vec_intersection(&state.keys_held, &state.last_keys_held);

        let released_keys = helper::vec_intersection(&state.last_keys_held, &state.keys_held);

        state.last_keys_held = state.keys_held.clone();

        (pressed_keys, released_keys)
    }

    fn read_states(&mut self, ui: &mut Ui) -> (DeviceState, KeyBindWidgetState) {
        let device_state = ui.data_mut(|d| {
            d.get_temp(DEVICE_STATE_ID.clone())
                .unwrap_or(DeviceState::new())
        });

        let state = ui.data_mut(|d| d.get_temp(self.id).unwrap_or(KeyBindWidgetState::new()));

        (device_state, state)
    }

    fn save_states(&mut self, ui: &mut Ui, device: DeviceState, state: KeyBindWidgetState) {
        ui.data_mut(|d| {
            d.insert_temp(DEVICE_STATE_ID.clone(), device);
            d.insert_temp(self.id, state);
        });
    }
}

impl Widget for KeyBindWidget<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let (response, painter) = ui.allocate_painter(ui.spacing().interact_size, Sense::click());

        let (device, mut state) = self.read_states(ui);

        let visuals = ui.style().interact_selectable(&response, state.active); // get the current interactable style settings

        painter.rect_filled(response.rect, visuals.rounding, visuals.bg_fill); // draw a lil button :)

        painter.text(
            response.rect.center(),
            Align2::CENTER_CENTER,
            self.value.serialize(),
            FontId::default(),
            visuals.text_color(),
        );

        if response.clicked() {
            if state.active {
                state.active = false;
            } else {
                state.active = true;

                state.last_keys_held = device.get_keys();
                state.keys_held = state.last_keys_held.clone();
            }
        }

        if response.secondary_clicked() {
            self.value.key = None;
            self.value.modifiers = vec![];

            state.active = false;
        }

        if state.active {
            let (down, up) = self.check_keys(&device, &mut state);

            if self.run_input(down, up, &mut state) {
                state.active = false; // key was set

                ui.ctx().request_repaint();
            }
        }

        let saved_state = if state.active {
            state
        } else {
            KeyBindWidgetState::new()
        };

        self.save_states(ui, device, saved_state);
        return response;
    }
}

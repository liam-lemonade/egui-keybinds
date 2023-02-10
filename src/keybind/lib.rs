extern crate device_query;
extern crate eframe;
extern crate egui;

pub mod keycodes;
pub mod widget;

pub use keycodes::*;
pub use widget::*;

pub struct KeyBind {
    key: Option<KeyCode>,
    modifiers: Vec<KeyModifier>,
}

impl KeyBind {
    pub fn new(key: KeyCode, modifiers: Vec<KeyModifier>) -> Self {
        Self {
            key: Some(key),
            modifiers,
        }
    }

    pub fn simple_name(&self) -> String {
        match &self.key {
            Some(k) => {
                let mut prefix = String::with_capacity(self.modifiers.len());

                for modifier in &self.modifiers {
                    let char = match modifier {
                        KeyModifier::CtrlCmd => "^",
                        KeyModifier::Shift => "_",
                        KeyModifier::AltOpt => "*",
                        KeyModifier::Function => "!",
                    };

                    prefix += char;
                }

                return format!("{}{}", prefix, k.to_string());
            }

            None => "...".to_string(),
        }
    }
}

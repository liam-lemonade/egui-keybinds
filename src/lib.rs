extern crate device_query;
extern crate eframe;
extern crate egui;
extern crate egui_modal;
extern crate once_cell;

pub mod keycodes;
pub mod widget;

pub use keycodes::*;
pub use widget::*;

#[derive(Clone)]
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

    pub fn serialize(&mut self) -> String {
        match &self.key {
            Some(k) => {
                let mut prefix = String::with_capacity(self.modifiers.len());

                self.modifiers.sort();

                for modifier in &self.modifiers {
                    prefix.push(modifier.serialize());
                }

                return format!("{}{}", prefix, k.serialize());
            }

            None => "...".to_string(),
        }
    }

    pub fn deserialize(data: String) -> Result<Self, ()> {
        let mut result: Result<Self, ()> = Err(());

        let mut modifiers: Vec<KeyModifier> = vec![];

        for (i, ch) in data.chars().enumerate() {
            let deserialized_modifier = KeyModifier::deserialize(ch);

            match deserialized_modifier {
                Ok(modifier) => modifiers.push(modifier),

                Err(_) => {
                    let name_slice = &data[i..data.len()];

                    let deserialized_key = KeyCode::deserialize(name_slice.to_string());

                    match deserialized_key {
                        Ok(key) => {
                            result = Ok(Self::new(key, modifiers));
                        }

                        _ => (),
                    }

                    break;
                }
            }
        }

        result
    }
}

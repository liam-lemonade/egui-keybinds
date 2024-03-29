extern crate device_query;
extern crate eframe;
extern crate egui;
extern crate once_cell;

pub mod helper;
pub mod keycodes;
pub mod widget;

pub use keycodes::*;
pub use widget::*;

#[derive(Clone, Hash)]
pub struct KeyBind {
    pub key: Option<KeyCode>,
    pub modifiers: Vec<KeyModifier>,
}

impl KeyBind {
    pub fn new(key: Option<KeyCode>, modifiers: Vec<KeyModifier>) -> Self {
        Self { key, modifiers }
    }

    pub fn empty() -> Self {
        Self {
            key: None,
            modifiers: vec![],
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
                            let mods = if key.is_some() { modifiers } else { vec![] };

                            result = Ok(Self::new(key, mods));
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

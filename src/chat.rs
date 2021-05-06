//! This module contains the [`Chat`] type, which represents the [raw JSON text format](https://minecraft.fandom.com/wiki/Raw_JSON_text_format#Java_Edition), also [called Chat](https://wiki.vg/Chat).

use {
    std::fmt,
    serde::{
        Deserialize,
        Serialize,
    },
};

/// The text colors used in [`Chat`] messages.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)] // variants are obvious
pub enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

/// The [raw JSON text format](https://minecraft.fandom.com/wiki/Raw_JSON_text_format#Java_Edition), also [called Chat](https://wiki.vg/Chat).
///
/// Not yet fully implemented.
#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Chat {
    text: String,
    color: Option<Color>,
}

impl Chat {
    /// Sets the color of the text.
    pub fn color(&mut self, color: Color) -> &mut Chat {
        self.color = Some(color);
        self
    }
}

impl From<String> for Chat {
    fn from(text: String) -> Chat {
        Chat { text, ..Chat::default() }
    }
}

impl fmt::Display for Chat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| fmt::Error)?)
    }
}

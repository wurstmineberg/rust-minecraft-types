//! This module contains the [`Chat`] type, which represents the [raw JSON text format](https://minecraft.fandom.com/wiki/Raw_JSON_text_format#Java_Edition), also [called Chat](https://wiki.vg/Chat).

use {
    std::fmt,
    serde::{
        Deserialize,
        Serialize,
    },
    uuid::Uuid,
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

/// The events that can be performed when a [`Chat`] is clicked.
#[derive(Deserialize, Serialize)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
#[allow(missing_docs)] //TODO
pub enum ClickEvent {
    OpenUrl(String),
    OpenFile(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(String),
    CopyToClipboard(String),
}

/// The events that can be performed when a player hovers over a [`Chat`] with the mouse.
#[derive(Deserialize, Serialize)]
#[serde(tag = "action", content = "contents", rename_all = "snake_case")]
#[allow(missing_docs)] //TODO
pub enum HoverEvent {
    ShowText(Box<Chat>),
    ShowItem {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        count: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
    },
    ShowEntity {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<Box<Chat>>,
        #[serde(rename = "type")]
        entity_type: String,
        id: Uuid,
    },
}

/// The [raw JSON text format](https://minecraft.fandom.com/wiki/Raw_JSON_text_format#Java_Edition), also [called Chat](https://wiki.vg/Chat).
///
/// Not yet fully implemented.
#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Chat {
    /// The plain text of this text component.
    pub text: String,
    /// Text components displayed after the main `text`. The main formatting is inherited unless specified otherwise.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<Chat>,
    /// The text color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// Whether to render the content in boldface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// Whether to render the content in italics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// Whether to underline the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    /// Whether to strike through the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    /// Whether to render the content obfuscated, i.e. with characters randomly replaced with others of the same width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    /// The action to perform when this text component is clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    /// The action to perform when a player hovers over this text component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

impl Chat {
    /// Adds a text component to the `extra` list.
    pub fn add_extra(&mut self, extra: Chat) -> &mut Chat {
        self.extra.push(extra);
        self
    }

    /// Replaces the `extra` list with the given text components.
    pub fn set_extras(&mut self, extras: Vec<Chat>) -> &mut Chat {
        self.extra = extras;
        self
    }

    /// Sets the color of the text.
    pub fn color(&mut self, color: Color) -> &mut Chat {
        self.color = Some(color);
        self
    }

    /// Enables boldface.
    pub fn bold(&mut self) -> &mut Chat {
        self.bold = Some(true);
        self
    }

    /// Disables boldface.
    pub fn no_bold(&mut self) -> &mut Chat {
        self.bold = Some(false);
        self
    }

    /// Enables italics.
    pub fn italic(&mut self) -> &mut Chat {
        self.italic = Some(true);
        self
    }

    /// Disables italics.
    pub fn no_italic(&mut self) -> &mut Chat {
        self.italic = Some(false);
        self
    }

    /// Enables underline.
    pub fn underlined(&mut self) -> &mut Chat {
        self.underlined = Some(true);
        self
    }

    /// Disables underline.
    pub fn no_underlined(&mut self) -> &mut Chat {
        self.underlined = Some(false);
        self
    }

    /// Enables strike-through.
    pub fn strikethrough(&mut self) -> &mut Chat {
        self.strikethrough = Some(true);
        self
    }

    /// Disables strike-through.
    pub fn no_strikethrough(&mut self) -> &mut Chat {
        self.strikethrough = Some(false);
        self
    }

    /// Enables obfuscation.
    pub fn obfuscated(&mut self) -> &mut Chat {
        self.obfuscated = Some(true);
        self
    }

    /// Disables obfuscation.
    pub fn no_obfuscated(&mut self) -> &mut Chat {
        self.obfuscated = Some(false);
        self
    }

    /// Sets the action to perform when clicked.
    pub fn on_click(&mut self, event: ClickEvent) -> &mut Chat {
        self.click_event = Some(event);
        self
    }

    /// Sets the action to perform when hovered over.
    pub fn on_hover(&mut self, event: HoverEvent) -> &mut Chat {
        self.hover_event = Some(event);
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

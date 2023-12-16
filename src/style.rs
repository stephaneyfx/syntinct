use palette::Srgb;

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Style {
    pub foreground: Option<Srgb<u8>>,
    pub background: Option<Srgb<u8>>,
    pub special: Option<Srgb<u8>>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<UnderlineStyle>,
    pub reversed: bool,
}

impl Style {
    pub fn reverse() -> Self {
        Self {
            reversed: true,
            ..Self::default()
        }
    }

    pub fn foreground<C>(self, color: C) -> Self
    where
        C: Into<Srgb<u8>>,
    {
        Self {
            foreground: Some(color.into()),
            ..self
        }
    }

    pub fn no_foreground(self) -> Self {
        Self {
            foreground: None,
            ..self
        }
    }

    pub fn background<C>(self, color: C) -> Self
    where
        C: Into<Srgb<u8>>,
    {
        Self {
            background: Some(color.into()),
            ..self
        }
    }

    pub fn no_background(self) -> Self {
        Self {
            background: None,
            ..self
        }
    }

    pub fn special<C>(self, color: C) -> Self
    where
        C: Into<Srgb<u8>>,
    {
        Self {
            special: Some(color.into()),
            ..self
        }
    }

    pub fn no_special(self) -> Self {
        Self {
            special: None,
            ..self
        }
    }

    pub fn no_underline(self) -> Self {
        Self {
            underline: None,
            ..self
        }
    }

    pub fn underline(self) -> Self {
        Self {
            underline: Some(UnderlineStyle::Single),
            ..self
        }
    }

    pub fn double_underline(self) -> Self {
        Self {
            underline: Some(UnderlineStyle::Double),
            ..self
        }
    }

    pub fn curly_underline(self) -> Self {
        Self {
            underline: Some(UnderlineStyle::Curly),
            ..self
        }
    }

    pub fn dotted_underline(self) -> Self {
        Self {
            underline: Some(UnderlineStyle::Dotted),
            ..self
        }
    }

    pub fn dashed_underline(self) -> Self {
        Self {
            underline: Some(UnderlineStyle::Dashed),
            ..self
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UnderlineStyle {
    Single,
    Double,
    Curly,
    Dotted,
    Dashed,
}

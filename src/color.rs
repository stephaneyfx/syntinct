use palette::{Darken, Lighten, Srgb};

pub fn lighten(c: Srgb<u8>, factor: f32) -> Srgb<u8> {
    Srgb::from_linear(c.into_linear().lighten(factor))
}

pub fn darken(c: Srgb<u8>, factor: f32) -> Srgb<u8> {
    Srgb::from_linear(c.into_linear().darken(factor))
}

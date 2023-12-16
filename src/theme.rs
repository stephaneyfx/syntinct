use crate::{Category, DiagnosticLevel, Token};
use palette::Srgb;

pub trait Theme {
    fn category_color(&self, category: Category) -> Srgb<u8>;
    fn token_color(&self, token: Token) -> Srgb<u8>;
    fn diagnostic_level_color(&self, level: DiagnosticLevel) -> Srgb<u8>;
}

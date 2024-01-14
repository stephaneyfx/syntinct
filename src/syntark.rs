use crate::{darken, lighten, Category, DiagnosticLevel, Theme, Token};
use palette::{FromColor, Hsv, Srgb};

#[derive(Debug, Default)]
pub struct SyntarkTheme(());

impl Theme for SyntarkTheme {
    fn category_color(&self, category: Category) -> Srgb<u8> {
        match category {
            Category::ActiveSearchMatch => palette::named::CHOCOLATE,
            Category::ActiveTab => palette::named::WHITE,
            Category::ActiveTabBackground => self.category_color(Category::NormalBackground),
            Category::BadSpelling => palette::named::BROWN,
            Category::ColumnGuide => darken(self.category_color(Category::NormalBackground), 0.1),
            Category::CursorLine => lighten(self.category_color(Category::NormalBackground), 0.01),
            Category::CursorLineNumber => palette::named::WHITE,
            Category::DiffAdd => Srgb::from_color(Hsv {
                saturation: 0.3,
                value: 0.3,
                ..Hsv::from_color(palette::named::GREEN.into_format())
            })
            .into_format(),
            Category::DiffChange => Srgb::from_color(Hsv {
                saturation: 0.6,
                value: 0.15,
                ..Hsv::from_color(palette::named::GREEN.into_format())
            })
            .into_format(),
            Category::DiffDelete => Srgb::from_color(Hsv {
                saturation: 0.4,
                value: 0.3,
                ..Hsv::from_color(palette::named::RED.into_format())
            })
            .into_format(),
            Category::DiffText => self.category_color(Category::DiffAdd),
            Category::Folded => darken(self.category_color(Category::NormalBackground), 0.2),
            Category::InactiveTab => darken(self.category_color(Category::Normal), 0.75),
            Category::InactiveTabBackground => {
                darken(self.category_color(Category::NormalBackground), 0.3)
            }
            Category::LineNumber => darken(self.category_color(Category::Normal), 0.75),
            Category::MatchedBracket => palette::named::YELLOW,
            Category::MessageSeparator => self.category_color(Category::Normal),
            Category::ModeMessage => self.category_color(Category::Normal),
            Category::NonText => palette::named::DIMGRAY,
            Category::Normal => 0xd8d8d8.into(),
            Category::NormalBackground => 0x181818.into(),
            Category::Question => self.category_color(Category::Selection),
            Category::Search => self.category_color(Category::Normal),
            Category::SearchMatch => palette::named::BLUE,
            Category::Selection => palette::named::DARKSLATEGRAY,
            Category::Special => palette::named::DODGERBLUE,
            Category::StatusLine => lighten(self.category_color(Category::NormalBackground), 0.005),
            Category::TermCursor => 0xaeafad.into(),
            Category::UnfocusedTermCursor => self.category_color(Category::TermCursor),
            Category::Whitespace => darken(self.category_color(Category::Normal), 0.9),
        }
    }

    fn token_color(&self, token: Token) -> Srgb<u8> {
        match token {
            Token::Attribute => palette::named::LIGHTPINK,
            Token::Boolean => self.token_color(Token::Integer),
            Token::Character => palette::named::SEAGREEN,
            Token::Comment => palette::named::SLATEGRAY,
            Token::Constant => palette::named::LIGHTSALMON,
            Token::Delimiter => palette::named::LIGHTCORAL,
            Token::DocComment => self.token_color(Token::Comment),
            Token::Enum => self.token_color(Token::Type),
            Token::Field => palette::named::TAN,
            Token::Float => self.token_color(Token::Integer),
            Token::Function => palette::named::DEEPSKYBLUE,
            Token::Identifier => palette::named::STEELBLUE,
            Token::Integer => palette::named::GOLDENROD,
            Token::Interface => palette::named::TEAL,
            Token::Keyword => palette::named::ORCHID,
            Token::Link => palette::named::DARKCYAN,
            Token::Macro => palette::named::PINK,
            Token::Module => palette::named::AQUAMARINE,
            Token::Operator => palette::named::DODGERBLUE,
            Token::Parameter => self.token_color(Token::Variable),
            Token::String => palette::named::FORESTGREEN,
            Token::Struct => self.token_color(Token::Type),
            Token::Tag => palette::named::CADETBLUE,
            Token::Todo => palette::named::DARKORANGE,
            Token::Type => palette::named::LIGHTGREEN,
            Token::TypeParameter => self.token_color(Token::Type),
            Token::Variable => self.token_color(Token::Identifier),
            Token::Variant => palette::named::LIGHTSKYBLUE,
        }
    }

    fn diagnostic_level_color(&self, level: DiagnosticLevel) -> Srgb<u8> {
        match level {
            DiagnosticLevel::Error => palette::named::CRIMSON,
            DiagnosticLevel::Warning => palette::named::ORANGE,
            DiagnosticLevel::Info => palette::named::STEELBLUE,
            DiagnosticLevel::Hint => palette::named::AQUA,
        }
    }
}

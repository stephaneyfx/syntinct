use crate::{Category, DiagnosticLevel, Theme, Token};
use palette::Srgb;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ThematicVariant {
    Dark,
    Light,
}

#[derive(Debug)]
pub struct ThematicTheme {
    variant: ThematicVariant,
}

impl ThematicTheme {
    pub fn dark() -> Self {
        Self {
            variant: ThematicVariant::Dark,
        }
    }

    pub fn light() -> Self {
        Self {
            variant: ThematicVariant::Light,
        }
    }
}

impl Theme for ThematicTheme {
    fn category_color(&self, category: Category) -> Srgb<u8> {
        match self.variant {
            ThematicVariant::Dark => DarkThematicTheme.category_color(category),
            ThematicVariant::Light => LightThematicTheme.category_color(category),
        }
    }

    fn token_color(&self, token: Token) -> Srgb<u8> {
        match self.variant {
            ThematicVariant::Dark => DarkThematicTheme.token_color(token),
            ThematicVariant::Light => LightThematicTheme.token_color(token),
        }
    }

    fn diagnostic_level_color(&self, level: DiagnosticLevel) -> Srgb<u8> {
        match self.variant {
            ThematicVariant::Dark => DarkThematicTheme.diagnostic_level_color(level),
            ThematicVariant::Light => LightThematicTheme.diagnostic_level_color(level),
        }
    }
}

#[derive(Debug)]
struct DarkThematicTheme;

impl Theme for DarkThematicTheme {
    fn category_color(&self, category: Category) -> Srgb<u8> {
        category_color(self, category)
    }

    fn token_color(&self, token: Token) -> Srgb<u8> {
        token_color(self, token)
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

#[derive(Debug)]
struct LightThematicTheme;

impl Theme for LightThematicTheme {
    fn category_color(&self, category: Category) -> Srgb<u8> {
        category_color(self, category)
    }

    fn token_color(&self, token: Token) -> Srgb<u8> {
        token_color(self, token)
    }

    fn diagnostic_level_color(&self, level: DiagnosticLevel) -> Srgb<u8> {
        DarkThematicTheme.diagnostic_level_color(level)
    }
}

fn category_color<P>(provider: &P, category: Category) -> Srgb<u8>
where
    P: Provider,
{
    match category {
        Category::ActiveSearchMatch => provider.active_search_match(),
        Category::ActiveTab => category_color(provider, Category::Normal),
        Category::ActiveTabBackground => category_color(provider, Category::NormalBackground),
        Category::BadSpelling => provider.error(),
        Category::ColumnGuide => provider.guide(),
        Category::CursorLine => provider.selection(),
        Category::CursorLineNumber => provider.foreground(),
        Category::DiffAdd => provider.diff_add(),
        Category::DiffChange => provider.diff_change(),
        Category::DiffDelete => provider.diff_delete(),
        Category::DiffText => category_color(provider, Category::DiffAdd),
        Category::Folded => provider.secondary_background(),
        Category::InactiveTab => provider.secondary_foreground(),
        Category::InactiveTabBackground => provider.secondary_background(),
        Category::LineNumber => provider.secondary_foreground(),
        Category::MatchedBracket => provider.matched_bracket(),
        Category::MessageSeparator => category_color(provider, Category::Normal),
        Category::ModeMessage => category_color(provider, Category::Normal),
        Category::NonText => provider.secondary_foreground(),
        Category::Normal => provider.foreground(),
        Category::NormalBackground => provider.background(),
        Category::Question => category_color(provider, Category::Selection),
        Category::Search => category_color(provider, Category::Normal),
        Category::SearchMatch => provider.search_match(),
        Category::Selection => provider.selection(),
        Category::Special => palette::named::DODGERBLUE,
        Category::StatusLine => provider.secondary_background(),
        Category::TermCursor => provider.cursor(),
        Category::UnfocusedTermCursor => category_color(provider, Category::TermCursor),
        Category::Whitespace => provider.secondary_foreground(),
    }
}

fn token_color<P>(provider: &P, token: Token) -> Srgb<u8>
where
    P: Provider,
{
    match token {
        Token::Attribute => palette::named::LIGHTPINK,
        Token::Boolean => token_color(provider, Token::Variant),
        Token::Character => token_color(provider, Token::String),
        Token::Comment => provider.secondary_foreground(),
        Token::Constant => palette::named::CADETBLUE,
        Token::ConstGenericParameter => token_color(provider, Token::Constant),
        Token::Delimiter => provider.foreground(),
        Token::DocComment => token_color(provider, Token::Comment),
        Token::Enum => token_color(provider, Token::Type),
        Token::Field => token_color(provider, Token::Variable),
        Token::Float => token_color(provider, Token::Integer),
        Token::Function => palette::named::DEEPSKYBLUE,
        Token::Identifier => palette::named::STEELBLUE,
        Token::Integer => palette::named::LIGHTSALMON,
        Token::Interface => palette::named::SEAGREEN,
        Token::Keyword => 0xbb9af7.into(),
        Token::Link => palette::named::DARKCYAN,
        Token::Macro => token_color(provider, Token::Attribute),
        Token::Module => palette::named::TEAL,
        Token::Operator => token_color(provider, Token::Keyword),
        Token::Parameter => token_color(provider, Token::Variable),
        Token::Static => token_color(provider, Token::Variable),
        Token::String => palette::named::DARKKHAKI,
        Token::Struct => token_color(provider, Token::Type),
        Token::Tag => provider.foreground(),
        Token::Todo => palette::named::DARKORANGE,
        Token::Type => palette::named::DARKSEAGREEN,
        Token::TypeParameter => token_color(provider, Token::Type),
        Token::Variable => token_color(provider, Token::Identifier),
        Token::Variant => palette::named::CORNFLOWERBLUE,
    }
}

trait Provider {
    fn active_search_match(&self) -> Srgb<u8>;
    fn error(&self) -> Srgb<u8>;
    fn diff_add(&self) -> Srgb<u8>;
    fn diff_change(&self) -> Srgb<u8>;
    fn diff_delete(&self) -> Srgb<u8>;
    fn guide(&self) -> Srgb<u8>;
    fn foreground(&self) -> Srgb<u8>;
    fn background(&self) -> Srgb<u8>;
    fn secondary_background(&self) -> Srgb<u8>;
    fn secondary_foreground(&self) -> Srgb<u8>;
    fn search_match(&self) -> Srgb<u8>;
    fn selection(&self) -> Srgb<u8>;
    fn matched_bracket(&self) -> Srgb<u8>;
    fn cursor(&self) -> Srgb<u8>;
}

impl Provider for DarkThematicTheme {
    fn active_search_match(&self) -> Srgb<u8> {
        palette::named::CORAL
    }

    fn error(&self) -> Srgb<u8> {
        0xbf616a.into()
    }

    fn diff_add(&self) -> Srgb<u8> {
        0xa3be8c.into()
    }

    fn diff_change(&self) -> Srgb<u8> {
        self.secondary_background()
    }

    fn diff_delete(&self) -> Srgb<u8> {
        0xbf616a.into()
    }

    fn guide(&self) -> Srgb<u8> {
        self.secondary_background()
    }

    fn foreground(&self) -> Srgb<u8> {
        0xd8d8d8.into()
    }

    fn background(&self) -> Srgb<u8> {
        0x181818.into()
    }

    fn secondary_background(&self) -> Srgb<u8> {
        0x202020.into()
    }

    fn secondary_foreground(&self) -> Srgb<u8> {
        palette::named::SLATEGRAY
    }

    fn search_match(&self) -> Srgb<u8> {
        palette::named::ROYALBLUE
    }

    fn selection(&self) -> Srgb<u8> {
        0x282828.into()
    }

    fn matched_bracket(&self) -> Srgb<u8> {
        self.search_match()
    }

    fn cursor(&self) -> Srgb<u8> {
        0xd8dee9.into()
    }
}

impl Provider for LightThematicTheme {
    fn active_search_match(&self) -> Srgb<u8> {
        DarkThematicTheme.active_search_match()
    }

    fn error(&self) -> Srgb<u8> {
        DarkThematicTheme.error()
    }

    fn diff_add(&self) -> Srgb<u8> {
        DarkThematicTheme.diff_add()
    }

    fn diff_change(&self) -> Srgb<u8> {
        DarkThematicTheme.diff_change()
    }

    fn diff_delete(&self) -> Srgb<u8> {
        DarkThematicTheme.diff_delete()
    }

    fn guide(&self) -> Srgb<u8> {
        self.secondary_background()
    }

    fn foreground(&self) -> Srgb<u8> {
        palette::named::BLACK
    }

    fn background(&self) -> Srgb<u8> {
        palette::named::BEIGE
    }

    fn secondary_background(&self) -> Srgb<u8> {
        palette::named::IVORY
    }

    fn secondary_foreground(&self) -> Srgb<u8> {
        palette::named::LIGHTGRAY
    }

    fn search_match(&self) -> Srgb<u8> {
        DarkThematicTheme.search_match()
    }

    fn selection(&self) -> Srgb<u8> {
        palette::named::AZURE
    }

    fn matched_bracket(&self) -> Srgb<u8> {
        DarkThematicTheme.matched_bracket()
    }

    fn cursor(&self) -> Srgb<u8> {
        palette::named::SLATEGRAY
    }
}

use crate::{
    darken, lighten, style::UnderlineStyle, Category, DiagnosticLevel, Style, Theme, Token,
};
use heck::ToUpperCamelCase;
use palette::Srgb;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::{self, Write},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum HighlightName {
    // Built into Neovim
    ColorColumn,
    Conceal,
    CurSearch,
    Cursor,
    CursorIM,
    CursorColumn,
    CursorLine,
    Directory,
    DiffAdd,
    DiffChange,
    DiffDelete,
    DiffText,
    EndOfBuffer,
    TermCursor,
    TermCursorNC,
    ErrorMsg,
    WinSeparator,
    Folded,
    FoldColumn,
    SignColumn,
    IncSearch,
    Substitute,
    LineNr,
    LineNrAbove,
    LineNrBelow,
    CursorLineNr,
    CursorLineFold,
    CursorLineSign,
    MatchParen,
    ModeMsg,
    MsgArea,
    MsgSeparator,
    MoreMsg,
    NonText,
    Normal,
    NormalFloat,
    FloatBorder,
    FloatTitle,
    NormalNC,
    Pmenu,
    PmenuSel,
    PmenuKind,
    PmenuKindSel,
    PmenuExtra,
    PmenuExtraSel,
    PmenuSbar,
    PmenuThumb,
    Question,
    QuickFixLine,
    Search,
    SpecialKey,
    SpellBad,
    SpellCap,
    SpellLocal,
    SpellRare,
    StatusLine,
    StatusLineNC,
    TabLine,
    TabLineFill,
    TabLineSel,
    Title,
    Visual,
    VisualNOS,
    WarningMsg,
    WhiteSpace,
    WildMenu,
    WinBar,
    WinBarNC,
    Lsp(LspHighlightName),
    Boolean,
    Character,
    Comment,
    Conditional,
    Constant,
    Debug,
    Define,
    Delimiter,
    Error,
    Exception,
    Float,
    Function,
    Identifier,
    Include,
    Keyword,
    Label,
    Macro,
    Number,
    Operator,
    PreCondit,
    PreProc,
    Repeat,
    Special,
    SpecialChar,
    SpecialComment,
    Statement,
    StorageClass,
    String,
    Structure,
    Tag,
    Todo,
    Type,
    Typedef,
    Underlined,
    // Diagnostic
    Diagnostic(Diagnostic),
    DiagnosticDeprecated,
    DiagnosticUnnecessary,
    // Markdown
    MarkdownCode,
    MarkdownCodeBlock,
    MarkdownH1,
    MarkdownH2,
    MarkdownHeadingDelimiter,
    MarkdownLinkText,
    // nvim-cmp
    CmpItemAbbrMatch,
    CmpItemAbbrMatchFuzzy,
    CmpItemKind(LspType),
    // Telescope
    TelescopeBorder,
    TelescopeTitle,
}

impl HighlightName {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        match self {
            Self::ColorColumn => "ColorColumn".into(),
            Self::Conceal => "Conceal".into(),
            Self::CurSearch => "CurSearch".into(),
            Self::Cursor => "Cursor".into(),
            Self::CursorIM => "CursorIM".into(),
            Self::CursorColumn => "CursorColumn".into(),
            Self::CursorLine => "CursorLine".into(),
            Self::Directory => "Directory".into(),
            Self::DiffAdd => "DiffAdd".into(),
            Self::DiffChange => "DiffChange".into(),
            Self::DiffDelete => "DiffDelete".into(),
            Self::DiffText => "DiffText".into(),
            Self::EndOfBuffer => "EndOfBuffer".into(),
            Self::TermCursor => "TermCursor".into(),
            Self::TermCursorNC => "TermCursorNC".into(),
            Self::ErrorMsg => "ErrorMsg".into(),
            Self::WinSeparator => "WinSeparator".into(),
            Self::Folded => "Folded".into(),
            Self::FoldColumn => "FoldColumn".into(),
            Self::SignColumn => "SignColumn".into(),
            Self::IncSearch => "IncSearch".into(),
            Self::Substitute => "Substitute".into(),
            Self::LineNr => "LineNr".into(),
            Self::LineNrAbove => "LineNrAbove".into(),
            Self::LineNrBelow => "LineNrBelow".into(),
            Self::CursorLineNr => "CursorLineNr".into(),
            Self::CursorLineFold => "CursorLineFold".into(),
            Self::CursorLineSign => "CursorLineSign".into(),
            Self::MatchParen => "MatchParen".into(),
            Self::ModeMsg => "ModeMsg".into(),
            Self::MsgArea => "MsgArea".into(),
            Self::MsgSeparator => "MsgSeparator".into(),
            Self::MoreMsg => "MoreMsg".into(),
            Self::NonText => "NonText".into(),
            Self::Normal => "Normal".into(),
            Self::NormalFloat => "NormalFloat".into(),
            Self::FloatBorder => "FloatBorder".into(),
            Self::FloatTitle => "FloatTitle".into(),
            Self::NormalNC => "NormalNC".into(),
            Self::Pmenu => "Pmenu".into(),
            Self::PmenuSel => "PmenuSel".into(),
            Self::PmenuKind => "PmenuKind".into(),
            Self::PmenuKindSel => "PmenuKindSel".into(),
            Self::PmenuExtra => "PmenuExtra".into(),
            Self::PmenuExtraSel => "PmenuExtraSel".into(),
            Self::PmenuSbar => "PmenuSbar".into(),
            Self::PmenuThumb => "PmenuThumb".into(),
            Self::Question => "Question".into(),
            Self::QuickFixLine => "QuickFixLine".into(),
            Self::Search => "Search".into(),
            Self::SpecialKey => "SpecialKey".into(),
            Self::SpellBad => "SpellBad".into(),
            Self::SpellCap => "SpellCap".into(),
            Self::SpellLocal => "SpellLocal".into(),
            Self::SpellRare => "SpellRare".into(),
            Self::StatusLine => "StatusLine".into(),
            Self::StatusLineNC => "StatusLineNC".into(),
            Self::TabLine => "TabLine".into(),
            Self::TabLineFill => "TabLineFill".into(),
            Self::TabLineSel => "TabLineSel".into(),
            Self::Title => "Title".into(),
            Self::Visual => "Visual".into(),
            Self::VisualNOS => "VisualNOS".into(),
            Self::WarningMsg => "WarningMsg".into(),
            Self::WhiteSpace => "WhiteSpace".into(),
            Self::WildMenu => "WildMenu".into(),
            Self::WinBar => "WinBar".into(),
            Self::WinBarNC => "WinBarNC".into(),
            Self::Lsp(h) => h.to_string(),
            Self::Boolean => "Boolean".into(),
            Self::Character => "Character".into(),
            Self::Comment => "Comment".into(),
            Self::Conditional => "Conditional".into(),
            Self::Constant => "Constant".into(),
            Self::Debug => "Debug".into(),
            Self::Define => "Define".into(),
            Self::Delimiter => "Delimiter".into(),
            Self::Error => "Error".into(),
            Self::Exception => "Exception".into(),
            Self::Float => "Float".into(),
            Self::Function => "Function".into(),
            Self::Identifier => "Identifier".into(),
            Self::Include => "Include".into(),
            Self::Keyword => "Keyword".into(),
            Self::Label => "Label".into(),
            Self::Macro => "Macro".into(),
            Self::Number => "Number".into(),
            Self::Operator => "Operator".into(),
            Self::PreCondit => "PreCondit".into(),
            Self::PreProc => "PreProc".into(),
            Self::Repeat => "Repeat".into(),
            Self::Special => "Special".into(),
            Self::SpecialChar => "SpecialChar".into(),
            Self::SpecialComment => "SpecialComment".into(),
            Self::Statement => "Statement".into(),
            Self::StorageClass => "StorageClass".into(),
            Self::String => "String".into(),
            Self::Structure => "Structure".into(),
            Self::Tag => "Tag".into(),
            Self::Todo => "Todo".into(),
            Self::Type => "Type".into(),
            Self::Typedef => "Typedef".into(),
            Self::Underlined => "Underlined".into(),
            Self::Diagnostic(d) => d.to_string(),
            Self::DiagnosticDeprecated => "DiagnosticDeprecated".into(),
            Self::DiagnosticUnnecessary => "DiagnosticUnnecessary".into(),
            Self::MarkdownCode => "markdownCode".into(),
            Self::MarkdownCodeBlock => "markdownCodeBlock".into(),
            Self::MarkdownH1 => "markdownH1".into(),
            Self::MarkdownH2 => "markdownH2".into(),
            Self::MarkdownHeadingDelimiter => "markdownHeadingDelimiter".into(),
            Self::MarkdownLinkText => "markdownLinkText".into(),
            Self::CmpItemAbbrMatch => "CmpItemAbbrMatch".into(),
            Self::CmpItemAbbrMatchFuzzy => "CmpItemAbbrMatchFuzzy".into(),
            Self::CmpItemKind(k) => format!("CmpItemKind{}", k.to_string().to_upper_camel_case()),
            Self::TelescopeBorder => "TelescopeBorder".into(),
            Self::TelescopeTitle => "TelescopeTitle".into(),
        }
    }
}

impl Display for HighlightName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl From<LspTypeMod> for HighlightName {
    fn from(type_mod: LspTypeMod) -> Self {
        Self::Lsp(LspHighlightName {
            type_mod,
            lang: None,
        })
    }
}

impl From<LspType> for HighlightName {
    fn from(ty: LspType) -> Self {
        LspTypeMod::Type(ty).into()
    }
}

impl From<LspModifier> for HighlightName {
    fn from(modifier: LspModifier) -> Self {
        LspTypeMod::Modifier(modifier).into()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct LspHighlightName {
    type_mod: LspTypeMod,
    lang: Option<Language>,
}

impl LspHighlightName {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        format!(
            "@lsp.{}{}",
            self.type_mod,
            self.lang
                .as_ref()
                .map(|lang| format!(".{lang}"))
                .unwrap_or_default()
        )
    }
}

impl Display for LspHighlightName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum LspTypeMod {
    Type(LspType),
    Modifier(LspModifier),
    #[allow(dead_code)]
    Both(LspType, LspModifier),
}

impl LspTypeMod {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        match self {
            Self::Type(ty) => format!("type.{ty}"),
            Self::Modifier(m) => format!("mod.{m}"),
            Self::Both(ty, m) => format!("typemod.{ty}.{m}"),
        }
    }
}

impl Display for LspTypeMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Language {
    #[allow(dead_code)]
    Rust,
}

impl Language {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        match self {
            Self::Rust => "rust".into(),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, enum_iterator::Sequence)]
enum LspType {
    Class,
    Decorator,
    Derive,
    Enum,
    EnumMember,
    Function,
    Interface,
    Keyword,
    Macro,
    Method,
    Namespace,
    Parameter,
    Property,
    Struct,
    Type,
    TypeAlias,
    TypeParameter,
    Variable,
}

impl LspType {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        match self {
            Self::Class => "class".into(),
            Self::Decorator => "decorator".into(),
            Self::Derive => "derive".into(),
            Self::Enum => "enum".into(),
            Self::EnumMember => "enumMember".into(),
            Self::Function => "function".into(),
            Self::Interface => "interface".into(),
            Self::Keyword => "keyword".into(),
            Self::Macro => "macro".into(),
            Self::Method => "method".into(),
            Self::Namespace => "namespace".into(),
            Self::Parameter => "parameter".into(),
            Self::Property => "property".into(),
            Self::Struct => "struct".into(),
            Self::Type => "type".into(),
            Self::TypeAlias => "typeAlias".into(),
            Self::TypeParameter => "typeParameter".into(),
            Self::Variable => "variable".into(),
        }
    }
}

impl Display for LspType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum LspModifier {
    #[allow(dead_code)]
    Deprecated,
}

impl LspModifier {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        match self {
            Self::Deprecated => "deprecated".into(),
        }
    }
}

impl Display for LspModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

fn diagnostic_level_as_str(level: DiagnosticLevel) -> &'static str {
    match level {
        DiagnosticLevel::Error => "Error",
        DiagnosticLevel::Warning => "Warn",
        DiagnosticLevel::Info => "Info",
        DiagnosticLevel::Hint => "Hint",
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, enum_iterator::Sequence)]
enum DiagnosticUiKind {
    VirtualText,
    Underline,
}

impl DiagnosticUiKind {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        match self {
            Self::VirtualText => "VirtualText".into(),
            Self::Underline => "Underline".into(),
        }
    }
}

impl Display for DiagnosticUiKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Diagnostic {
    level: DiagnosticLevel,
    kind: Option<DiagnosticUiKind>,
}

impl Diagnostic {
    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
        format!(
            "Diagnostic{}{}",
            self.kind
                .as_ref()
                .map(DiagnosticUiKind::to_string)
                .unwrap_or_default(),
            diagnostic_level_as_str(self.level),
        )
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Highlight {
    Value(Style),
    Link(HighlightName),
}

impl From<Style> for Highlight {
    fn from(style: Style) -> Self {
        Self::Value(style)
    }
}

impl From<HighlightName> for Highlight {
    fn from(name: HighlightName) -> Self {
        Self::Link(name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NeovimTheme {
    highlights: HashMap<HighlightName, Highlight>,
}

impl NeovimTheme {
    pub fn new<T: Theme>(base: &T) -> Self {
        Self {
            highlights: vec![
                // Built into Neovim
                (
                    HighlightName::ColorColumn,
                    Style::default()
                        .background(base.category_color(Category::ColumnGuide))
                        .into(),
                ),
                (HighlightName::Conceal, Style::default().into()),
                (
                    HighlightName::CurSearch,
                    Style::default()
                        .foreground(base.category_color(Category::Search))
                        .background(base.category_color(Category::ActiveSearchMatch))
                        .into(),
                ),
                (HighlightName::Cursor, Style::reverse().into()),
                (HighlightName::CursorIM, HighlightName::Cursor.into()),
                (HighlightName::CursorColumn, Style::default().into()),
                (
                    HighlightName::CursorLine,
                    Style::default()
                        .background(base.category_color(Category::CursorLine))
                        .into(),
                ),
                (HighlightName::Directory, Style::default().into()),
                (
                    HighlightName::DiffAdd,
                    Style::default()
                        .background(base.category_color(Category::DiffAdd))
                        .into(),
                ),
                (
                    HighlightName::DiffChange,
                    Style::default()
                        .background(base.category_color(Category::DiffChange))
                        .into(),
                ),
                (
                    HighlightName::DiffDelete,
                    Style::default()
                        .background(base.category_color(Category::DiffDelete))
                        .into(),
                ),
                (
                    HighlightName::DiffText,
                    Style::default()
                        .background(base.category_color(Category::DiffText))
                        .into(),
                ),
                (HighlightName::EndOfBuffer, HighlightName::NonText.into()),
                (
                    HighlightName::TermCursor,
                    Style::default()
                        .background(base.category_color(Category::TermCursor))
                        .into(),
                ),
                (
                    HighlightName::TermCursorNC,
                    Style::default()
                        .background(base.category_color(Category::UnfocusedTermCursor))
                        .into(),
                ),
                (
                    HighlightName::ErrorMsg,
                    Style::default()
                        .foreground(base.diagnostic_level_color(DiagnosticLevel::Error))
                        .into(),
                ),
                (
                    HighlightName::WinSeparator,
                    Style::default()
                        .foreground(darken(base.category_color(Category::Normal), 0.95))
                        .into(),
                ),
                (
                    HighlightName::Folded,
                    Style::default()
                        .background(base.category_color(Category::Folded))
                        .into(),
                ),
                (HighlightName::FoldColumn, Style::default().into()),
                (HighlightName::SignColumn, Style::default().into()),
                (HighlightName::IncSearch, HighlightName::CurSearch.into()),
                (HighlightName::Substitute, HighlightName::IncSearch.into()),
                (
                    HighlightName::LineNr,
                    Style::default()
                        .foreground(base.category_color(Category::LineNumber))
                        .into(),
                ),
                (HighlightName::LineNrAbove, HighlightName::LineNr.into()),
                (HighlightName::LineNrBelow, HighlightName::LineNr.into()),
                (
                    HighlightName::CursorLineNr,
                    Style::default()
                        .foreground(base.category_color(Category::CursorLineNumber))
                        .into(),
                ),
                (
                    HighlightName::CursorLineFold,
                    HighlightName::FoldColumn.into(),
                ),
                (
                    HighlightName::CursorLineSign,
                    HighlightName::SignColumn.into(),
                ),
                (
                    HighlightName::MatchParen,
                    Style::default()
                        .foreground(base.category_color(Category::MatchedBracket))
                        .into(),
                ),
                (
                    HighlightName::ModeMsg,
                    Style::default()
                        .foreground(base.category_color(Category::ModeMessage))
                        .into(),
                ),
                (HighlightName::MsgArea, Style::default().into()),
                (
                    HighlightName::MsgSeparator,
                    Style::default()
                        .foreground(base.category_color(Category::MessageSeparator))
                        .into(),
                ),
                (HighlightName::MoreMsg, HighlightName::ModeMsg.into()),
                (
                    HighlightName::NonText,
                    Style::default()
                        .foreground(base.category_color(Category::NonText))
                        .into(),
                ),
                (
                    HighlightName::Normal,
                    Style::default()
                        .foreground(base.category_color(Category::Normal))
                        .background(base.category_color(Category::NormalBackground))
                        .into(),
                ),
                (HighlightName::NormalFloat, Style::default().into()),
                (
                    HighlightName::FloatBorder,
                    HighlightName::WinSeparator.into(),
                ),
                (HighlightName::FloatTitle, HighlightName::Title.into()),
                (HighlightName::NormalNC, Style::default().into()),
                (HighlightName::Pmenu, Style::default().into()),
                (HighlightName::PmenuSel, HighlightName::Visual.into()),
                (HighlightName::PmenuKind, Style::default().into()),
                (HighlightName::PmenuKindSel, HighlightName::Visual.into()),
                (HighlightName::PmenuExtra, Style::default().into()),
                (HighlightName::PmenuExtraSel, HighlightName::Visual.into()),
                (
                    HighlightName::PmenuSbar,
                    Style::default()
                        .foreground(lighten(
                            base.category_color(Category::NormalBackground),
                            0.1,
                        ))
                        .into(),
                ),
                (HighlightName::PmenuThumb, HighlightName::Pmenu.into()),
                (
                    HighlightName::Question,
                    Style::default()
                        .foreground(base.category_color(Category::Question))
                        .into(),
                ),
                (HighlightName::QuickFixLine, Style::default().into()),
                (
                    HighlightName::Search,
                    Style::default()
                        .foreground(base.category_color(Category::Search))
                        .background(base.category_color(Category::SearchMatch))
                        .into(),
                ),
                (
                    HighlightName::SpecialKey,
                    Style::default()
                        .foreground(base.category_color(Category::Special))
                        .into(),
                ),
                (
                    HighlightName::SpellBad,
                    Style::default()
                        .foreground(base.category_color(Category::BadSpelling))
                        .into(),
                ),
                (HighlightName::SpellCap, HighlightName::SpellBad.into()),
                (HighlightName::SpellLocal, HighlightName::SpellBad.into()),
                (HighlightName::SpellRare, HighlightName::SpellBad.into()),
                (
                    HighlightName::StatusLine,
                    Style::default()
                        .background(base.category_color(Category::StatusLine))
                        .into(),
                ),
                (
                    HighlightName::StatusLineNC,
                    Style::default()
                        .background(darken(base.category_color(Category::StatusLine), 0.5))
                        .into(),
                ),
                (
                    HighlightName::TabLine,
                    Style::default()
                        .foreground(base.category_color(Category::InactiveTab))
                        .background(base.category_color(Category::InactiveTabBackground))
                        .into(),
                ),
                (HighlightName::TabLineFill, Style::default().into()),
                (
                    HighlightName::TabLineSel,
                    Style::default()
                        .foreground(base.category_color(Category::ActiveTab))
                        .background(base.category_color(Category::ActiveTabBackground))
                        .into(),
                ),
                (HighlightName::Title, HighlightName::TabLineSel.into()),
                (
                    HighlightName::Visual,
                    Style::default()
                        .background(base.category_color(Category::Selection))
                        .into(),
                ),
                (HighlightName::VisualNOS, HighlightName::Visual.into()),
                (
                    HighlightName::WarningMsg,
                    Style::default()
                        .foreground(base.diagnostic_level_color(DiagnosticLevel::Warning))
                        .into(),
                ),
                (
                    HighlightName::WhiteSpace,
                    Style::default()
                        .foreground(base.category_color(Category::Whitespace))
                        .into(),
                ),
                (HighlightName::WildMenu, HighlightName::PmenuSel.into()),
                (HighlightName::WinBar, HighlightName::TabLineSel.into()),
                (HighlightName::WinBarNC, HighlightName::TabLine.into()),
                (
                    LspType::Class.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Type))
                        .into(),
                ),
                (
                    LspType::Decorator.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Attribute))
                        .into(),
                ),
                (
                    LspType::Derive.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Interface))
                        .into(),
                ),
                (
                    LspType::Enum.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Enum))
                        .into(),
                ),
                (
                    LspType::EnumMember.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Variant))
                        .into(),
                ),
                (
                    LspType::Function.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Function))
                        .into(),
                ),
                (
                    LspType::Interface.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Interface))
                        .into(),
                ),
                (
                    LspType::Keyword.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Keyword))
                        .into(),
                ),
                (
                    LspType::Macro.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Macro))
                        .into(),
                ),
                (
                    LspType::Method.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Function))
                        .into(),
                ),
                (
                    LspType::Namespace.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Module))
                        .into(),
                ),
                (
                    LspType::Parameter.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Parameter))
                        .into(),
                ),
                (
                    LspType::Property.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Field))
                        .into(),
                ),
                (
                    LspType::Struct.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Type))
                        .into(),
                ),
                (
                    LspType::Type.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Type))
                        .into(),
                ),
                (
                    LspType::TypeAlias.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Type))
                        .into(),
                ),
                (
                    LspType::TypeParameter.into(),
                    Style::default()
                        .foreground(base.token_color(Token::TypeParameter))
                        .into(),
                ),
                (
                    LspType::Variable.into(),
                    Style::default()
                        .foreground(base.token_color(Token::Variable))
                        .into(),
                ),
                (
                    HighlightName::Boolean,
                    Style::default()
                        .foreground(base.token_color(Token::Boolean))
                        .into(),
                ),
                (
                    HighlightName::Character,
                    Style::default()
                        .foreground(base.token_color(Token::Character))
                        .into(),
                ),
                (
                    HighlightName::Comment,
                    Style::default()
                        .foreground(base.token_color(Token::Comment))
                        .into(),
                ),
                (HighlightName::Conditional, HighlightName::Keyword.into()),
                (
                    HighlightName::Constant,
                    Style::default()
                        .foreground(base.token_color(Token::Constant))
                        .into(),
                ),
                (HighlightName::Debug, Style::default().into()),
                (HighlightName::Define, HighlightName::Macro.into()),
                (
                    HighlightName::Delimiter,
                    Style::default()
                        .foreground(base.token_color(Token::Delimiter))
                        .into(),
                ),
                (
                    HighlightName::Error,
                    Style::default()
                        .foreground(base.diagnostic_level_color(DiagnosticLevel::Error))
                        .into(),
                ),
                (HighlightName::Exception, HighlightName::Keyword.into()),
                (
                    HighlightName::Float,
                    Style::default()
                        .foreground(base.token_color(Token::Float))
                        .into(),
                ),
                (
                    HighlightName::Function,
                    Style::default()
                        .foreground(base.token_color(Token::Function))
                        .into(),
                ),
                (
                    HighlightName::Identifier,
                    Style::default()
                        .foreground(base.token_color(Token::Identifier))
                        .into(),
                ),
                (
                    HighlightName::Include,
                    Style::default()
                        .foreground(base.token_color(Token::Module))
                        .into(),
                ),
                (
                    HighlightName::Keyword,
                    Style::default()
                        .foreground(base.token_color(Token::Keyword))
                        .into(),
                ),
                (HighlightName::Label, HighlightName::Keyword.into()),
                (
                    HighlightName::Macro,
                    Style::default()
                        .foreground(base.token_color(Token::Macro))
                        .into(),
                ),
                (
                    HighlightName::Number,
                    Style::default()
                        .foreground(base.token_color(Token::Integer))
                        .into(),
                ),
                (
                    HighlightName::Operator,
                    Style::default()
                        .foreground(base.token_color(Token::Operator))
                        .into(),
                ),
                (HighlightName::PreCondit, HighlightName::Macro.into()),
                (HighlightName::PreProc, HighlightName::Macro.into()),
                (HighlightName::Repeat, HighlightName::Keyword.into()),
                (HighlightName::Special, HighlightName::SpecialChar.into()),
                (
                    HighlightName::SpecialChar,
                    Style::default()
                        .foreground(base.category_color(Category::Special))
                        .into(),
                ),
                (HighlightName::SpecialComment, HighlightName::Comment.into()),
                (HighlightName::Statement, HighlightName::Keyword.into()),
                (HighlightName::StorageClass, HighlightName::Keyword.into()),
                (
                    HighlightName::String,
                    Style::default()
                        .foreground(base.token_color(Token::String))
                        .into(),
                ),
                (HighlightName::Structure, HighlightName::Type.into()),
                (
                    HighlightName::Tag,
                    Style::default()
                        .foreground(base.token_color(Token::Tag))
                        .into(),
                ),
                (
                    HighlightName::Todo,
                    Style::default()
                        .foreground(base.token_color(Token::Todo))
                        .into(),
                ),
                (
                    HighlightName::Type,
                    Style::default()
                        .foreground(base.token_color(Token::Type))
                        .into(),
                ),
                (HighlightName::Typedef, HighlightName::Type.into()),
                (
                    HighlightName::Underlined,
                    Style::default()
                        .foreground(base.token_color(Token::Link))
                        .into(),
                ),
            ]
            .into_iter()
            .chain(
                enum_iterator::all::<(DiagnosticLevel, Option<DiagnosticUiKind>)>().map(
                    |(level, kind)| {
                        let color = base.diagnostic_level_color(level);
                        let style = match kind {
                            None => Style::default().foreground(color),
                            Some(DiagnosticUiKind::Underline) => {
                                Style::default().special(color).curly_underline()
                            }
                            Some(DiagnosticUiKind::VirtualText) => Style::default()
                                .foreground(color)
                                .background(darken(color, 0.95)),
                        };
                        (
                            HighlightName::Diagnostic(Diagnostic { level, kind }),
                            style.into(),
                        )
                    },
                ),
            )
            .chain([
                (HighlightName::DiagnosticDeprecated, Style::default().into()),
                (
                    HighlightName::DiagnosticUnnecessary,
                    Style::default().into(),
                ),
                (
                    HighlightName::MarkdownCode,
                    Style::default()
                        .foreground(base.token_color(Token::Identifier))
                        .into(),
                ),
                (
                    HighlightName::MarkdownCodeBlock,
                    Style::default()
                        .foreground(base.token_color(Token::String))
                        .into(),
                ),
                (
                    HighlightName::MarkdownH1,
                    Style::default()
                        .foreground(base.token_color(Token::Module))
                        .into(),
                ),
                (HighlightName::MarkdownH2, HighlightName::MarkdownH1.into()),
                (
                    HighlightName::MarkdownHeadingDelimiter,
                    HighlightName::Delimiter.into(),
                ),
                (
                    HighlightName::MarkdownLinkText,
                    Style::default()
                        .foreground(base.token_color(Token::Link))
                        .into(),
                ),
                (
                    HighlightName::CmpItemAbbrMatch,
                    HighlightName::Special.into(),
                ),
                (
                    HighlightName::CmpItemAbbrMatchFuzzy,
                    HighlightName::Special.into(),
                ),
            ])
            .chain(enum_iterator::all::<LspType>().map(|lsp_type| {
                (
                    HighlightName::CmpItemKind(lsp_type),
                    HighlightName::from(lsp_type).into(),
                )
            }))
            .chain([
                (
                    HighlightName::TelescopeBorder,
                    HighlightName::FloatBorder.into(),
                ),
                (HighlightName::TelescopeTitle, HighlightName::Title.into()),
            ])
            .collect(),
        }
    }

    pub fn to_lua_module(&self) -> String {
        let mut buffer = Vec::new();
        self.write(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        writeln!(writer, "local highlights = {{")?;
        let indent = "    ";
        let mut highlights = self
            .highlights
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        highlights.sort_by_cached_key(|(a, _)| a.to_string());
        for (key, h) in &highlights {
            writeln!(writer, r#"  ["{key}"] = {{"#)?;
            match h {
                Highlight::Value(style) => {
                    if let Some(fg) = style.foreground {
                        writeln!(writer, r#"{indent}fg = "{}","#, CssColor(fg))?;
                    }
                    if let Some(bg) = style.background {
                        writeln!(writer, r#"{indent}bg = "{}","#, CssColor(bg))?;
                    }
                    if let Some(c) = style.special {
                        writeln!(writer, r#"{indent}sp = "{}","#, CssColor(c))?;
                    }
                    if let Some(bold) = style.bold {
                        writeln!(writer, "{indent}bold = {bold},")?;
                    }
                    if let Some(underline) = style.underline {
                        const EFFECTS: &[(UnderlineStyle, &str)] = &[
                            (UnderlineStyle::Single, "underline"),
                            (UnderlineStyle::Double, "underdouble"),
                            (UnderlineStyle::Curly, "undercurl"),
                            (UnderlineStyle::Dotted, "underdotted"),
                            (UnderlineStyle::Dashed, "underdashed"),
                        ];
                        let key = EFFECTS
                            .iter()
                            .find_map(|&(kind, key)| (kind == underline).then_some(key));
                        if let Some(key) = key {
                            writeln!(writer, "{indent}{key} = true,")?;
                        }
                    }
                    if let Some(strikethrough) = style.strikethrough {
                        writeln!(writer, "{indent}strikethrough = {strikethrough},")?;
                    }
                    if let Some(italic) = style.italic {
                        writeln!(writer, "{indent}italic = {italic},")?;
                    }
                    writeln!(writer, "{indent}reverse = {},", style.reversed)?;
                }
                Highlight::Link(link) => writeln!(writer, r#"{indent}link = "{link}","#)?,
            }
            writeln!(writer, "  }},")?;
        }
        writeln!(writer, "}}")?;
        writeln!(writer)?;
        const CODE: &str = include_str!("theme.lua");
        write!(writer, "{CODE}")?;
        Ok(())
    }
}

#[derive(Debug)]
struct CssColor(Srgb<u8>);

impl Display for CssColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Srgb {
            red, green, blue, ..
        } = self.0;
        write!(f, "#{red:02x}{green:02x}{blue:02x}")
    }
}

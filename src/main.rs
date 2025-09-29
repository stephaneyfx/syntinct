use clap::{builder::styling::AnsiColor, Args, Parser, Subcommand, ValueEnum};

fn terminal_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(AnsiColor::Green.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Blue.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default().bold())
}

#[derive(Debug, Parser)]
#[clap(author, styles = terminal_styles(), version)]
struct AppArgs {
    #[clap(subcommand)]
    cmd: AppCommand,
}

#[derive(Debug, Subcommand)]
enum AppCommand {
    Generate(GenerateCommand),
}

#[derive(Args, Debug)]
struct GenerateCommand {
    theme: ThemeName,
}

#[derive(Clone, Debug, ValueEnum)]
enum ThemeName {
    Syntark,
    Thematic,
}

fn main() {
    let args = AppArgs::parse();
    match args.cmd {
        AppCommand::Generate(cmd) => generate_theme(cmd),
    }
}

fn generate_theme(cmd: GenerateCommand) {
    let neovim_theme = match cmd.theme {
        ThemeName::Syntark => syntinct::NeovimTheme::new(
            "syntark",
            &syntinct::SyntarkTheme::default(),
            &syntinct::SyntarkTheme::default(),
        ),
        ThemeName::Thematic => syntinct::NeovimTheme::new(
            "thematic",
            &syntinct::ThematicTheme::dark(),
            &syntinct::ThematicTheme::light(),
        ),
    };
    println!("{}", neovim_theme.to_lua_module());
}

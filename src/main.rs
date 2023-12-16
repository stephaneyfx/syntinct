fn main() {
    let base_theme = syntinct::SyntarkTheme::default();
    let neovim_theme = syntinct::NeovimTheme::new(&base_theme);
    println!("{}", neovim_theme.to_lua_module());
}

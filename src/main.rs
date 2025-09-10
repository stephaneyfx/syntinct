fn main() {
    let neovim_theme = syntinct::NeovimTheme::new(
        "thematic",
        &syntinct::ThematicTheme::dark(),
        &syntinct::ThematicTheme::light(),
    );
    println!("{}", neovim_theme.to_lua_module());
}

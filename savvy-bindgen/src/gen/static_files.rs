pub fn generate_makevars(crate_name: &str) -> String {
    format!(include_str!("./templates/Makevars"), crate_name, crate_name)
}

pub fn generate_makevars_win(crate_name: &str) -> String {
    format!(
        include_str!("./templates/Makevars.win"),
        crate_name, crate_name
    )
}

pub fn generate_gitignore() -> String {
    include_str!("./templates/gitignore").to_string()
}

pub fn generate_makevars_in(crate_name: &str) -> String {
    format!(
        include_str!("./templates/Makevars.in"),
        crate_name, crate_name
    )
}

pub fn generate_configure() -> String {
    include_str!("./templates/configure").to_string()
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

pub fn generate_cargo_toml(crate_name: &str) -> String {
    format!(include_str!("./templates/Cargo_toml"), crate_name)
}

pub fn generate_config_toml() -> String {
    include_str!("./templates/config_toml").to_string()
}

pub fn generate_example_lib_rs() -> String {
    include_str!("./templates/lib_rs").to_string()
}

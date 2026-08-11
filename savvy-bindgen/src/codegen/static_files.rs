pub fn generate_makevars_in(crate_name: &str) -> String {
    format!(
        include_str!("./templates/Makevars.in"),
        crate_name, crate_name
    )
}

pub fn generate_configure() -> String {
    include_str!("./templates/configure").to_string()
}

pub fn generate_cleanup() -> String {
    include_str!("./templates/cleanup").to_string()
}

pub fn generate_makevars_win_in(crate_name: &str) -> String {
    format!(
        include_str!("./templates/Makevars.win.in"),
        crate_name, crate_name
    )
}

pub fn generate_configure_win() -> String {
    include_str!("./templates/configure.win").to_string()
}

pub fn generate_cleanup_win() -> String {
    include_str!("./templates/cleanup.win").to_string()
}

pub fn generate_win_def(crate_name: &str) -> String {
    format!(include_str!("./templates/dllname-win.def"), crate_name)
}

pub fn generate_gitignore() -> String {
    include_str!("./templates/gitignore").to_string()
}

pub fn generate_cargo_toml(crate_name: &str, dependencies: &str) -> String {
    format!(
        include_str!("./templates/Cargo_toml"),
        crate_name, dependencies
    )
}

pub fn generate_config_toml() -> String {
    include_str!("./templates/config_toml").to_string()
}

pub fn generate_example_lib_rs() -> String {
    include_str!("./templates/lib_rs").to_string()
}

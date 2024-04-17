use std::path::Path;

pub(crate) fn to_snake_case(x: &str) -> String {
    let mut out = String::with_capacity(x.len() + 3);
    for c in x.chars() {
        if c.is_uppercase() {
            // first character doesn't need _
            if !out.is_empty() {
                out.push('_');
            }

            out.push_str(&c.to_lowercase().to_string());
        } else {
            out.push(c)
        }
    }
    out
}

// Parse Cargo.toml and get the crate name in a dirty way
pub(crate) fn parse_cargo_toml(path: &Path) -> (String, String) {
    let content = savvy_bindgen::read_file(path);

    let mut dev_dependencies = vec![];
    let mut crate_name = "";

    let mut section = "";
    for line in content.lines() {
        if line.trim_start().starts_with('[') {
            section = line.trim();
            continue;
        }

        match section {
            // find crate name
            "[package]" => {
                let mut s = line.split('=');

                // if the line contains = and the key is "name", return the
                // value as crate_name otherwise, skip the line.
                match s.next() {
                    Some(key) if key.trim() == "name" => {
                        if let Some(value) = s.next() {
                            crate_name = value.trim().trim_matches(['"', '\'']);
                        }
                    }
                    _ => {}
                }
            }
            "[dev-dependencies]" => {
                dev_dependencies.push(line);
            }
            _ => {}
        }
    }

    if crate_name.is_empty() {
        eprintln!("Cargo.toml doesn't have package name!");
        std::process::exit(10);
    }

    (crate_name.to_string(), dev_dependencies.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snake_case() {
        assert_eq!(&to_snake_case("foo"), "foo");
        assert_eq!(&to_snake_case("Foo"), "foo");
        assert_eq!(&to_snake_case("fooBar"), "foo_bar");
        assert_eq!(&to_snake_case("FooBar"), "foo_bar");
        assert_eq!(&to_snake_case("fooBarBaz"), "foo_bar_baz");
    }
}

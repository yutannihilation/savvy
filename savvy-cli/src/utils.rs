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

pub(crate) fn dot_containing_to_camel_case(x: &str) -> String {
    x.split('.')
        .enumerate()
        .map(|(i_split, split)| {
            if i_split == 0 {
                return split.to_string();
            }
            split
                .chars()
                .enumerate()
                .map(|(i_char, c)| {
                    if i_char == 0 {
                        c.to_uppercase().next().unwrap()
                    } else {
                        c.to_lowercase().next().unwrap()
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("")
}

pub(crate) fn canonicalize(path: &Path) -> Result<String, std::io::Error> {
    let crate_dir_abs = path.canonicalize()?;
    let crate_dir_abs = crate_dir_abs.to_string_lossy();
    #[cfg(windows)]
    let crate_dir_abs = if crate_dir_abs.starts_with(r#"\\?\"#) {
        crate_dir_abs.get(4..).unwrap().replace('\\', "/")
    } else {
        crate_dir_abs.replace('\\', "/")
    };
    Ok(crate_dir_abs.to_string())
}

// Parse Cargo.toml and get the crate name in a dirty way

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

    #[test]
    fn test_camel_case() {
        assert_eq!(&dot_containing_to_camel_case("foo.bar"), "fooBar");
        assert_eq!(&dot_containing_to_camel_case("foo.bar.baz"), "fooBarBaz");
        assert_eq!(&dot_containing_to_camel_case("foo.BAR.baz"), "fooBarBaz");
    }
}

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

pub(crate) fn canonicalize(path: &Path) -> Result<String, std::io::Error> {
    let crate_dir_abs = path.canonicalize()?;
    let crate_dir_abs = crate_dir_abs.to_string_lossy();
    #[cfg(windows)]
    let crate_dir_abs = if crate_dir_abs.starts_with(r#"\\?\"#) {
        crate_dir_abs.get(4..).unwrap().replace('\\', "/")
    } else {
        crate_dir_abs.replace('\\', "/")
    };
    Ok(crate_dir_abs)
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
}

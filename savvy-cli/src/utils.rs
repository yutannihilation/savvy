pub(crate) fn to_snake_case(x: &str) -> String {
    let mut out = String::with_capacity(x.len());
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

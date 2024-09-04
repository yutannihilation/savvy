use savvy::savvy;

#[savvy]
fn r#fn(r#struct: bool) -> savvy::Result<()> {
    Ok(())
}

#[savvy]
struct r#struct {
    r#fn: bool,
}

#[savvy]
impl r#struct {
    fn r#new() -> Self {
        Self { r#fn: true }
    }

    fn r#fn(r#fn: bool) -> savvy::Result<()> {
        Ok(())
    }
}

#[savvy]
enum Enum {
    r#enum,
}

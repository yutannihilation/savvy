use savvy::{savvy, IntegerSexp, Sexp};

#[savvy]
fn default_value_scalar(x: Option<i32>) -> savvy::Result<Sexp> {
    x.unwrap_or(-1).try_into()
}

#[savvy]
fn default_value_vec(x: Option<IntegerSexp>) -> savvy::Result<Sexp> {
    if let Some(x) = x {
        x.iter().sum::<i32>().try_into()
    } else {
        (-1).try_into()
    }
}

#[savvy]
struct FooWithDefault {
    default_value: i32,
}

#[savvy]
impl FooWithDefault {
    fn new(default_value: i32) -> Self {
        Self { default_value }
    }

    fn default_value_method(&self, x: Option<i32>) -> savvy::Result<Sexp> {
        x.unwrap_or(self.default_value).try_into()
    }

    fn default_value_associated_fn(x: Option<i32>) -> savvy::Result<Sexp> {
        x.unwrap_or(-1).try_into()
    }
}

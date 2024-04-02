use savvy::{r_println, savvy, Sexp};

#[savvy]
#[derive(Clone, Debug)]
pub(crate) struct Value(pub(crate) i32);

#[savvy]
impl Value {
    fn new(x: i32) -> Self {
        Self(x)
    }

    fn pair(self, b: Value) -> savvy::Result<ValuePair> {
        Ok(ValuePair { a: self, b })
    }

    fn get(&self) -> savvy::Result<Sexp> {
        self.0.try_into()
    }
}

#[allow(dead_code)]
#[savvy]
#[derive(Debug)]
struct ValuePair {
    a: Value,
    b: Value,
}

#[savvy]
impl ValuePair {
    fn new(a: Value, b: Value) -> Self {
        Self { a, b }
    }

    fn new_copy(a: &Value, b: &Value) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
        }
    }

    fn print(&self) -> savvy::Result<()> {
        r_println!("{:?}", self);
        Ok(())
    }
}

#[savvy]
fn new_value_pair(a: Value, b: Value) -> savvy::Result<ValuePair> {
    Ok(ValuePair { a, b })
}

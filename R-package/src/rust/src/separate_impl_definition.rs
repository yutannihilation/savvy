use savvy::{savvy, Sexp};

#[savvy]
impl crate::consuming_type::Value {
    fn get2(&self) -> savvy::Result<Sexp> {
        self.0.try_into()
    }
}

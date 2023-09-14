use crate::sexp::Sxp;

// impl TryFrom<Sxp> for i32 {
//     type Error = crate::error::Error;

//     fn try_from(value: Sxp) -> crate::error::Result<Self> {
//         if !value.is_integer() {
//             let type_name = value.get_human_readable_type_name();
//             let msg = format!("Cannot convert {type_name} to integer");
//             return Err(crate::error::Error::UnexpectedType(msg));
//         }
//         Ok(Self(value))
//     }
// }

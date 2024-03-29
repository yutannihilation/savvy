pub fn assert_len(len: usize, i: usize) -> crate::error::Result<()> {
    if i >= len {
        Err(crate::error::Error::new(&format!(
            "index out of bounds: the length is {} but the index is {}",
            len, i
        )))
    } else {
        Ok(())
    }
}

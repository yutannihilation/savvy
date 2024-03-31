# Advanced Topics

## "External" external pointers

As described in Struct section, a struct marked with `#[savvy]` is
transparently converted from and into an SEXP of an external pointer. So,
usually, you don't need to think about external pointers.

However, in some cases, you might need to deal with an external pointer created
by another R package. For example, you might want to access an Apache Arrow data
created by nanoarrow R package. In such caes, you can use unsafe methods
`.cast_unchecked()` or `.cast_mut_unchecked()`.

```rust
let foo: &Foo = unsafe { &*ext_ptr_sexp.cast_unchecked::<Foo>() };
```

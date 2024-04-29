# Initialization Routine

A special convention of `#[savvy]` is that, if `#[savvy]` is placed on a
function that takes `*mut DllInfo` as its argument, the function is called when
the package is loaded, which is what [Writing R Extension][wre] calls
"initialization routine".

[wre]: https://cran.r-project.org/doc/manuals/r-release/R-exts.html#dyn_002eload-and-dyn_002eunload

For example, if you write such a Rust function like this,

``` rust
#[savvy]
fn init_foo(_dll_info: *mut savvy::ffi::DllInfo) -> savvy::Result<()> {
    r_eprintln!("Initialized!");
    Ok(())
}
```

You'll see the following message on your R session when you load the package.

```r
library(yourPackage)
#> Initialized!
```

Under the hood, `savvy-cli update .` inserts the following line in a C function
`R_init_*()`, which is called when the DLL is loaded.

``` c
void R_init_yourPackage(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    savvy_init_foo__impl(dll); // added!
}
```

This is useful for initializing resources. For example, you can initialize a
global variable.

``` rust
use std::sync::OnceLock;

static GLOBAL_FOO: OnceLock<Foo> = OnceLock::new();

#[savvy]
fn init_global_foo(dll_info: *mut savvy::ffi::DllInfo) -> savvy::Result<()> {
    let foo = Foo::new();
    GLOBAL_FOO.set(foo);

    Ok(())
}
```

You can also register an ALTREP class using this mechanism.

(TBD)
# Linkage

Savvy compiles the Rust code into a static library and then use it to generate a
DLL for the R package. There's one tricky thing about static library. [The
Rust's official document about linkage][linkage] says

[linkage]: https://doc.rust-lang.org/reference/linkage.html

> Note that any dynamic dependencies that the static library may have (such as
> dependencies on system libraries, or dependencies on Rust libraries that are
> compiled as dynamic libraries) will have to be specified manually when linking
> that static library from somewhere.

What does this mean? If some of the dependency crate needs linking to a native
library, the necessary compiler flags are added by `cargo`. But, after creating
the static library, `cargo`'s turn is over. It's you who have to tell the linker
the necessary flags because there's no automatic mechanism.

If some of the flags are missing, you'll see a "symbol not found" error. For
example, this is what I got on macOS. Some dependency of my package uses the
[objc2](https://github.com/madsmtm/objc2) crate, and it needs to be linked
against Apple's Objective-C frameworks.

```
 unable to load shared object '.../foo.so':
  dlopen(../foo.so, 0x0006): symbol not found in flat namespace '_NSAppKitVersionNumber'
Execution halted
```

So, how can we know the necessary flags? The official document provides a
pro-tip!

> The `--print=native-static-libs` flag may help with this.

You can add this option to `src/Makevars.in` and `src/Makevars.win.in` via
`RUSTFLAGS` envvar. Please edit this line.

``` diff
  # Add flags if necessary
- RUSTFLAGS = 
+ RUSTFLAGS = --print=native-static-libs
```

Then, you'll find this note in the installation log.

```sh
   Compiling ahash v0.8.11
   Compiling serde v1.0.210
   Compiling zerocopy v0.7.35

...snip...

note: Link against the following native artifacts when linking against this static library. The order and any duplication can be significant on some platforms.

note: native-static-libs: -framework CoreText -framework CoreGraphics -framework CoreFoundation -framework Foundation -lobjc -liconv -lSystem -lc -lm

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.17s
   gcc -shared -L/usr/lib64/R/lib -Wl,-O1 -Wl,--sort-common -Wl,...
   installing to /tmp/RtmpvQv8Ur/devtools_install_...
   ** checking absolute paths in shared objects and dynamic libraries
```

You can copy these flags to `cargo build`. Please be aware that this differs on
platforms, so you probably need to run this command on CI, not on your local.
Also, since Linux and macOS requires different options, you need to tweak it in
the configure script.

For example, here's my setup on [the vellogd package](https://github.com/yutannihilation/vellogd-r).

`./configure`:

```sh
if [ "$(uname)" = "Darwin" ]; then
  FEATURES=""
  # result of --print=native-static-libs
  ADDITIONAL_PKG_LIBS="-framework CoreText -framework CoreGraphics -framework CoreFoundation -framework Foundation -lobjc -liconv -lSystem -lc -lm"
else
  FEATURES="--features use_winit"
fi
```

`src/Makevars.in`:

```make
PKG_LIBS = -L$(LIBDIR) -lvellogd @ADDITIONAL_PKG_LIBS@
```
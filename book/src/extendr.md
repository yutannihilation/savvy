# Comparison with extendr

## Why does savvy exist alongside extendr?

[extendr](https://extendr.github.io/) is a great framework with a larger community
and a longer history. savvy was created as a simpler, more explicit alternative.
It is not meant to be a complete replacement for extendr — rather, it offers a
different set of trade-offs that may or may not suit your needs.

Both frameworks are actively maintained and used in production. Choose whichever
fits your project and preferences better.

### Comparison

#### Design philosophy

extendr prioritizes **ergonomics**. It automatically converts between R objects and
native Rust types at function boundaries, so your Rust code can look clean and
dependency-free (e.g., `fn foo(x: &[f64]) -> f64`). It also provides a rich set of
helper macros (`r!()`, `R!()`, `list!()`, `data_frame!()`, etc.) to reduce
boilerplate.

savvy prioritizes **explicitness**. Functions receive and return R-specific wrapper
types (`IntegerSexp`, `OwnedRealSexp`, etc.), and no implicit type conversions happen.
This can feel verbose, but it makes the data flow between R and Rust transparent
and predictable.

#### API surface

extendr has a broader API surface, covering more R constructs out of the box
(data frames, environments, formulas, language objects, pairlists, etc.) and offering
optional integrations with `ndarray`, `serde`, and `faer`.

savvy intentionally provides a smaller API. Complex R constructs like data frames
are delegated to R wrapper code. This keeps the Rust side simple but means you write
more R glue code.

#### Stability and maintenance

extendr has a larger community (~500 GitHub stars, 33+ contributors, published in
JOSS). It is actively maintained with regular releases.

savvy is maintained by a single developer with limited bandwidth. However, the
minimal design means there is less surface area for bugs, and the framework has been
stable without much active development. It is used in production by notable projects
including polars and SedonaDB.

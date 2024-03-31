# Comparison with extendr

## What the hell is this?? Why do you need another framework when there's extendr?

[extendr](https://extendr.github.io/) is great and ready to use, but it's not
perfect in some points (e.g., [error handling][error]) and it's a kind of stuck;
extendr is too feature-rich and complex that no one can introduce a big breaking
change easily. So, I needed to create a new, simple framework to experiment
with. The main goal of savvy is to provide a simpler option other than extendr,
not to be a complete alternative to extendr.

[error]: https://github.com/extendr/extendr/issues/278

### Pros and cons compared to extendr

Pros:

* You can use `Result` for error handling instead of `panic!`
* You can compile your package for webR (I hope extendr gets webR-ready soon)

Cos:

* savvy prefers explicitness over ergonomics
* savvy provides limited amount of APIs and might not fit for complex usages

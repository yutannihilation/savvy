// From r83513 (R 4.3), R defines the `NORET` macro differently depending on the
// C/C++ standard the compiler uses. It matters when the header is used in C/C++
// libraries, but all we want to do here is to make bindgen interpret `NOREP` to
// `!`. However, for some reason, bindgen doesn't handle other no-return
// attributes like `_Noreturn` (for C11) and `[[noreturn]]` (for C++ and C23),
// so we define it here.
#define NORET __attribute__((__noreturn__))

// Currently, I'm adding these on as-needed basis
// but we may simply throw the whole lot in in the future.
#include <Rinternals.h>

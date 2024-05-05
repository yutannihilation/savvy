# error handling works

    Code
      safe_stop()
    Output
      Foo is Dropped!
    Condition
      Error:
      ! This is an error from inside unwind_protect()!

---

    Code
      safe_warn()
    Output
      Foo is Dropped!
    Condition
      Error:
      ! (converted from warning) foo


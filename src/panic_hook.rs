// PanicInfo is deprecated since 1.82 (PanicHookInfo exists since 1.81)
// cf. https://github.com/rust-lang/rust/pull/115974/
#[rustversion::since(1.81)]
type PanicHookInfo<'a> = std::panic::PanicHookInfo<'a>;
#[rustversion::before(1.81)]
type PanicHookInfo<'a> = std::panic::PanicInfo<'a>;

pub fn panic_hook(panic_info: &PanicHookInfo) {
    // Add indent
    let panic_info_indented = format!("{panic_info}")
        .lines()
        .map(|x| format!("{:indent$}{x}", "", indent = 4))
        .collect::<Vec<String>>()
        .join("\n");

    // Backtrace is available only when the debug info is available.
    #[cfg(debug_assertions)]
    let bt = get_backtrace();
    #[cfg(not(debug_assertions))]
    let bt = "    (Backtrace is not available on the release build)";

    crate::io::r_eprint(
        &format!(
            "panic occured!

Original message:
{panic_info_indented}

Backtrace:
{bt}
"
        ),
        true,
    );
}

// Since savvy generates many wrappers, the backtrace is boringly deep. Try
// cutting the uninteresting part.
#[cfg(debug_assertions)]
fn get_backtrace() -> String {
    let show_full = if let Ok(v) = std::env::var("RUST_BACKTRACE") {
        &v == "1"
    } else {
        false
    };

    // Forcibly captures a full backtrace regardless of RUST_BACKTRACE
    let bt = std::backtrace::Backtrace::force_capture().to_string();

    // try to shorten if the user doesn't require the full backtrace
    if !show_full {
        let bt_short = bt
            .lines()
            .skip_while(|line| !line.contains("std::panic::catch_unwind"))
            // C stacks are not visible from Rust's side and shown as `<unknown>`.
            .take_while(|line| !line.contains("<unknown>"))
            // Add indent
            .map(|x| format!("{:indent$}{x}", "", indent = 4))
            .collect::<Vec<String>>()
            .join("\n");

        if !bt_short.is_empty() {
            return format!(
                "    ...
{bt_short}
    ...

note: Run with `RUST_BACKTRACE=1` for a full backtrace.
"
            );
        }
    }

    // if the user require the full backtrace or the shortened backtrace became mistakenly empty string, show the full backtrace.
    bt.lines()
        .map(|x| format!("{:indent$}{x}", "", indent = 4))
        .collect::<Vec<String>>()
        .join("\n")
}

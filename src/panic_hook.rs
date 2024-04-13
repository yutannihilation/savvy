pub fn panic_hook(panic_info: &std::panic::PanicInfo) {
    // Forcibly captures a full backtrace regardless of RUST_BACKTRACE
    let bt = std::backtrace::Backtrace::force_capture().to_string();

    // Since savvy generates many wrappers, the backtrace is boringly deep. Try
    // cutting the uninteresting part.
    let bt_short = bt
        .lines()
        .skip_while(|line| !line.contains("std::panic::catch_unwind"))
        // C stacks are not visible from Rust's side and shown as `<unknown>`.
        .take_while(|line| !line.contains("<unknown>"))
        .collect::<Vec<&str>>()
        .join("\n");

    let bt = if bt_short.is_empty() { bt } else { bt_short };

    crate::io::r_eprint(
        &format!(
            "panic occured!

Original message:
{panic_info}

Backtrace:

...snip... (frames of savvy framework)

{bt}

...snip... (frames of R)
"
        ),
        true,
    );
}

mod panic_info;
mod panic_location;

use std::backtrace::Backtrace;
use std::fs;
use rfd::MessageButtons;
pub use panic_location::*;
pub use panic_info::*;

pub fn capture_panic() {
    std::env::set_var("RUST_LIB_BACKTRACE", "1");
    std::env::set_var("RUST_BACKTRACE", "full");
    
    std::panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::capture();
        let mut info = PanicInfo::from_hook(panic_info);
        info.set_backtrace(backtrace);
        let file_name = format!("dump_{}.json", chrono::Local::now().format("%Y%m%d_%H%M%S"));
        match fs::write(&file_name, info.to_string()) {
            Ok(_) => {
                rfd::MessageDialog::new()
                    .set_title("Crash Report")
                    .set_description(format!("Crash dump written at {file_name}"))
                    .set_buttons(MessageButtons::Ok)
                    .show();
            },

            Err(e) => {
                rfd::MessageDialog::new()
                    .set_title("Crash Report")
                    .set_description(format!("Could not write crash report!\n{e}"))
                    .set_buttons(MessageButtons::Ok)
                    .show();
            }
        }
    }))
}
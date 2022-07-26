use std::str;
use crate::prelude::{PRETTY_PACKAGE_NAME, Config};


pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub struct Logger ();

impl Logger {
    pub fn info(info: &str, to_add: Option<&str>, config: &Config) {
        let mut to_print = PRETTY_PACKAGE_NAME.to_string();
        to_print.push_str(": ");

        if to_add.is_some() {
            to_print.push_str( &str::replace(info, "{}}", to_add.unwrap()));
        }
        else {
            to_print.push_str(info);
        }

        if config.run_in_wasm {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from(to_print));
        }
        else {
            println!("{:#?}", to_print);

        }
    }

    pub fn error(error: &str, to_add: Option<&str>, config: &Config) {
        let mut to_print = PRETTY_PACKAGE_NAME.to_string();
        to_print.push_str(": ");

        if to_add.is_some() {
            to_print.push_str( &str::replace(error, "{}}", to_add.unwrap()));
        }
        else {
            to_print.push_str(error);
        }

        if config.run_in_wasm {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from(to_print));
            std::process::exit(1);
        }
        else {
            eprintln!("{:#?}", to_print);
        }
    }
}
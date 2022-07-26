pub mod helpers;
pub mod types;
pub mod utils;
pub mod parser;

mod prelude {
    pub use crate::helpers::*;
    pub use crate::types::*;
    pub use crate::utils::*;
    pub use crate::parser::*;
}

extern crate serde_json;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use crate::prelude::{set_panic_hook, parse_schema_from_string, parse_schema_from_file, Config};



// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Since wasm_bindgen's default target of wasm32-unknown-unknown can't be used to read files, we will 
// leave that to TS and TS will provide the file converted to string to us, skipping having to read the file in Rust 

#[wasm_bindgen]
pub fn parse_schema_for_typescript(raw_schema: &str) -> JsValue {
    set_panic_hook();

    let config: Config = Config {
        parallel: false,
        run_in_wasm: true
    };

    let data = parse_schema_from_string(raw_schema, &config);

    return JsValue::from_serde(&data).unwrap()
}


pub fn parse_schema_from_file_and_print(schema_file_path: &str, config: &Config) {
    set_panic_hook();

    parse_schema_from_file(schema_file_path, config);

    // Notice the :?. This tells println how to format the data being passed to print
    // println!("Data: {:#?}", data);
}
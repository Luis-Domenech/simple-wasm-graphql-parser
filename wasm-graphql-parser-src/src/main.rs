use wasm_graphql_parser::{parse_schema_from_file_and_print, types::Config};

fn main() {
    let config: Config = Config {
        parallel: false,
        run_in_wasm: false
    };

    parse_schema_from_file_and_print("./benches/data/schema.graphql", &config);
}
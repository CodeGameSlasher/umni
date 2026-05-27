use clap::Parser;
use std::path::PathBuf;
use umni::{data_processor::SymbolicData, generate, string_helper::Title};

#[derive(Parser, Debug)]
struct Options {
    // Start with text
    #[structopt(default_value = "")]
    input: String,

    // File with data
    #[structopt(short = 'f', long = "file", default_value = "data.txt")]
    data_path: PathBuf,

    // Generation max of characters
    #[structopt(short, long, default_value = "1000")]
    tokens: usize,
}

fn main() {
    let options = Options::parse();
    let mut values = SymbolicData::default();
    let data_path = std::path::absolute(&options.data_path).unwrap_or_default();
    let cache_path = data_path.with_extension("umni");

    let mut original_content = String::new();
    if data_path != cache_path {
        original_content = String::from_utf8(std::fs::read(&data_path).unwrap_or_else(|e| {
            panic!(
                r#"Error {} while reading "{}" file"#,
                e,
                data_path.to_str().unwrap_or("Undefined")
            );
        }))
        .unwrap_or_else(|e| {
            panic!("Can't parse content due {} error", e);
        });
    }

    let _ = std::fs::read(&cache_path).map(|content| match String::from_utf8(content) {
        Ok(content) => {
            values = serde_json::from_str::<SymbolicData>(&content).unwrap_or_default();

            if !original_content.trim().is_empty() && values != original_content {
                println!("Hash is different");
                values.data.clear();
            };
        }
        Err(e) => {
            eprintln!(
                r#"Cache file "{}" is empty or can't be read due {} error"#,
                cache_path.to_str().unwrap_or("Undefined"),
                e
            );
        }
    });

    if values.data.is_empty() && !original_content.trim().is_empty() {
        values = SymbolicData::process_data(&original_content);

        if let Err(e) = std::fs::write(
            &cache_path,
            serde_json::to_string(&values).unwrap_or_default(),
        ) {
            eprintln!(
                r#"Error {} while writing to cache file "{}""#,
                e,
                cache_path.to_str().unwrap_or("Undefined")
            );
        }
    }

    if values.data.is_empty() {
        panic!("There's no data to produce text");
    }

    let result = generate(&options.input, &values.data, options.tokens);

    print!("{}", result.title());
}

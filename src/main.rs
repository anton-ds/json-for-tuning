use std::fs::{self, DirEntry, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use serde_json::json;
use serde_json::Value;

const DIRECTORY: &str = "/***/data";
const OUTPUT_FILE: &str = "/***/data/result.jsonl";
const PICTURE_URL_TEMPLATE: &str = "https://***/{name}.png";
const TRUSTED_EXTENSIONS: [&str; 1] = ["txt"];
const USER_MESSAGE: &str = "***";
const SYSTEM_MESSAGE: &str = "***";

fn main() -> io::Result<()> {
    let mut messages = Vec::new();

    fs::read_dir(DIRECTORY)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| is_correct_extension(entry))
        .for_each(|entry| {
            match process_file(&entry.path()) {
                Ok(training_example) => messages.push(training_example),
                Err(e) => eprintln!("Error processing file: {}", e),
            }
        });

    save_messages_to_json(OUTPUT_FILE, &messages)?;

    Ok(())
}

/// Checks that extension is correct.
fn is_correct_extension(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| TRUSTED_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

/// Processes data-file from directory and returns JSON-item.
fn process_file(path: &Path) -> io::Result<Value> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let file_stem = path.file_stem().and_then(|name| name.to_str()).unwrap_or("unknown");
    let image_url = PICTURE_URL_TEMPLATE.replace("{name}", file_stem);

    let training_example = json!({
        "messages": [
            {
                "role": "system",
                "content": SYSTEM_MESSAGE,
            },
            {
                "role": "user",
                "content": USER_MESSAGE,
            },
            {
                "role": "user",
                "content": [
                    {
                        "type": "image_url",
                        "image_url": { "url": image_url }
                    }
                ]
            },
            {
                "role": "assistant",
                "content": contents.trim()
            }
        ]
    });

    Ok(training_example)
}

/// Saves array of messages to a JSONL-file.
fn save_messages_to_json(file_path: &str, messages: &[Value]) -> io::Result<()> {
    let output_file = File::create(file_path)?;
    let mut writer = BufWriter::new(output_file);

    for message in messages {
        writeln!(writer, "{}", message.to_string())?;
    }

    Ok(())
}

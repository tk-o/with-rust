use std::{io::prelude::*, env, path::Path, fs::{create_dir,File}};
use serde::{Deserialize,Serialize};
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cwd = env::current_dir().expect("cwd is defined");

    let settings_path = cwd.join(&"input/settings.json");

    let mut settings = String::new();

    let mut settings_file = File::open(&settings_path).expect("Load settings file");

    settings_file.read_to_string(&mut settings);

    let settings: Settings = serde_json::from_str(&settings).unwrap();

    println!("Hello, Mr {:?}, greetings from version: {:?}", settings_path, &settings);

    let cwd = env::current_dir().expect("cwd is defined");

    let local_output_path = match settings {
        Settings::Output { path ,} => path,
        _ => panic!("Unknown settings type"),
    };

    let output_path = cwd.join(&local_output_path);

    if !&output_path.exists() {
        create_dir(&output_path);
    }

    let mut output_file = File::create(
        cwd.join(&output_path).join("result.md")
    ).expect("Output file is created");

    output_file.write_all(b"# my two bytes");
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Settings {
    Output {
        path: String,
    },
}
use clap::{Args, Parser, Subcommand};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde_derive::{Deserialize, Serialize};
use std::{fs};

/// Simple CLI to create and manage notes using your favorite text editor.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initializes your note-taking app
    Init,
    /// Creates a new note
    Create(Create),
    List,
}

#[derive(Args, Debug)]
struct Create {
    /// The path to the file to create
    name: String,
}

fn create_file(args: Create) {
    let cfg: Config = confy::load("ukato", None).unwrap();
    let path = std::path::Path::new(&cfg.directory);
    let extension = ".md".to_string();

    if !std::path::Path::is_dir(path) {
        std::fs::create_dir(path).unwrap();
    }

    std::process::Command::new(cfg.editor)
        .arg(path.join([args.name, extension].join("")))
        .status()
        .expect("Something went wrong.");
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    directory: String,
    editor: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            directory: ".".to_string(),
            editor: "vim".to_string(),
        }
    }
}

fn init_config() {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");

    let directory = Input::with_theme(&theme)
        .with_prompt("Directory")
        .interact()
        .unwrap();

    let items = &["vim", "nano", "emacs"];

    let editor_index = Select::with_theme(&theme)
        .with_prompt("Select your preferred editor")
        .default(0)
        .items(&items[..])
        .interact()
        .unwrap();

    let my_config = Config {
        directory: directory,
        editor: items[editor_index].to_string(),
    };

    confy::store("ukato", None, my_config).unwrap();
}

fn list_notes() {
    let cfg: Config = confy::load("ukato", None).unwrap();
    let dir = std::path::Path::new(&cfg.directory);

   let paths = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    let note_index = Select::with_theme(&theme)
        .with_prompt("Your notes:")
        .default(0)
        .items(&paths[..])
        .interact()
        .unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_config(),
        Commands::Create(args) => create_file(args),
        Commands::List => list_notes(),
    }
}

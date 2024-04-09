use clap::{Args, Parser, Subcommand};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde_derive::{Deserialize, Serialize};
use dirs;

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



fn ensure_dir(path: &String){
    let path = std::path::Path::new(&path);
    if !std::path::Path::is_dir(path){
        match std::fs::create_dir(path) {
            Ok(_) => {},
            Err(err) => println!("Error creating directory: {}", err),
        }
    }
}

fn expand_path(path: &String) -> String {
    let expanded_path = if path.contains('~') {
        let home_dir = dirs::home_dir().unwrap();
        path.replace("~", home_dir.to_str().unwrap())
    } else {
        path.to_owned()
    };
    return expanded_path
}

fn validate_config(config: &Config){
    ensure_dir(&config.directory);
}

fn init_config() {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");

    let mut directory = Input::with_theme(&theme)
        .with_prompt("Directory")
        .interact()
        .unwrap();
    directory = expand_path(&directory);

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
    validate_config(&my_config);
    confy::store("ukato", None, my_config).unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_config(),
        Commands::Create(args) => create_file(args),
    }
}

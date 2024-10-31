use clap::{Args, Parser, Subcommand};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::fs;

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
    /// Get list of notes
    List,
    /// Get most recent note
    Recent,
}

#[derive(Args, Debug)]
struct Create {
    /// The path to the file to create
    name: String,
}

fn create_or_open_file(args: Create) {
    let cfg: Config = confy::load("ukato", None).unwrap();
    let path = std::path::Path::new(&cfg.directory);
    let full_path;

    if args.name.ends_with(".md") {
        full_path = path.join(args.name);
    } else {
        let extension = ".md".to_string();
        full_path = path.join([args.name, extension].join(""));
        std::fs::File::create(&full_path).unwrap();
    }

    if !std::path::Path::is_dir(path) {
        std::fs::create_dir(path).unwrap();
    }

    let mut viewer_handle = std::process::Command::new("inlyne")
        .arg(full_path.clone())
        .spawn()
        .unwrap();

    std::process::Command::new(cfg.editor)
        .arg(full_path.clone())
        .status()
        .expect("Something went wrong.");

    let _ = viewer_handle.kill();
}

fn open_recent_file() {
    let cfg: Config = confy::load("ukato", None).unwrap();
    let path = std::path::Path::new(&cfg.directory);

    let last_modified_file = std::fs::read_dir(path)
        .expect("Couldn't access local directory")
        .flatten() // Remove failed
        .filter(|f| f.metadata().unwrap().is_file()) // Filter out directories (only consider files)
        .max_by_key(|x| x.metadata().unwrap().modified().unwrap()) // Get the most recently modified file
        .unwrap()
        .file_name()
        .to_string_lossy()
        .into_owned();

    create_or_open_file(Create {
        name: last_modified_file,
    })
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
            directory: "~/notes".to_string(),
            editor: "vim".to_string(),
        }
    }
}

fn ensure_dir(path: &String) {
    let path = std::path::Path::new(&path);
    if !std::path::Path::is_dir(path) {
        match std::fs::create_dir(path) {
            Ok(_) => {}
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
    return expanded_path;
}

fn validate_config(config: &Config) {
    ensure_dir(&config.directory);
}

fn init_config() {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let cfg: Config = confy::load("ukato", None).unwrap();

    println!("Welcome to the setup wizard");

    let mut directory = Input::with_theme(&theme)
        .with_prompt("Notes directory")
        .with_initial_text(cfg.directory)
        .interact_text()
        .unwrap();
    directory = expand_path(&directory);

    let items = &["vim", "nano", "emacs", "micro"];

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

    create_or_open_file(Create {
        name: paths[note_index].clone(),
    })
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_config(),
        Commands::Create(args) => create_or_open_file(args),
        Commands::List => list_notes(),
        Commands::Recent => open_recent_file(),
    }
}

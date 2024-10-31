use clap::{Args, Parser, Subcommand};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

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
    Create(CreateSubcommand),
    /// Create new template
    Template(Create),
    /// Get list of notes
    ListNotes,
    /// Get list of templates
    ListTemplates,
    /// Get most recent note
    Recent,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    directory: String,
    editor: String,
}

#[derive(Args, Debug)]
struct Create {
    /// The path to the file to create
    name: String,
    template: Option<String>,
}

#[derive(Args, Debug)]
struct CreateSubcommand {
    name: String,
    /// template flag for creating the note
    #[clap(short = 't', long = "template")]
    template: Option<String>,
}

fn create_template(args: Create) {
    let cfg = match confy::load::<Config>("ukato", None) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!(
                "Error: No config found, did you run the init function? {}",
                err
            );
            return;
        }
    };
    let path = std::path::Path::new(&cfg.directory).join("templates");
    let full_path;

    if args.name.ends_with(".md") {
        full_path = path.join(args.name);
    } else {
        let extension = ".md".to_string();
        full_path = path.join([args.name, extension].join(""));
        let _ = fs::File::create(&full_path);
    }

    if !std::path::Path::is_dir(&path) {
        match std::fs::create_dir(&path) {
            Ok(_) => {} // if directory is created, all good
            Err(err) => {
                eprintln!("Error: {}. Did you run the init function?", err);
                return;
            }
        }
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

fn create_or_open_file(args: Create) {
    // let cfg: Config = confy::load("ukato", None).expect("Unable to load Ukato config");

    let cfg = match confy::load::<Config>("ukato", None) {
        Ok(config) => config,
        Err(err) => {
            eprintln!(
                "Error: No config found, did you run the init function? {}",
                err
            );
            return;
        }
    };

    let path = std::path::Path::new(&cfg.directory);
    let full_path;
    let title = args.name.clone();

    // Define template path based on flag
    let template_path = match args.template {
        Some(template) => {
            let full_template_path = path.join("templates").join(format!("{}.md", template));
            if std::path::Path::exists(&full_template_path) {
                full_template_path
            } else {
                eprintln!("Warning: Template '{}' not found. Using default.", template);
                path.join("templates/basic.md")
            }
        }
        None => path.join("templates/basic.md"),
    };

    // Read template
    let mut source_content = String::new();
    match fs::File::open(template_path) {
        Ok(mut file) => {
            file.read_to_string(&mut source_content)
                .expect("Error reading source file");
        }
        Err(_) => {
            eprintln!("Error: Template file not found");
        }
    }

    if args.name.ends_with(".md") {
        full_path = path.join(args.name);
    } else {
        let extension = ".md".to_string();
        full_path = path.join([args.name, extension].join(""));

        if !std::path::Path::exists(&full_path) {
            // Replace placeholders in template
            let mut content_with_title =
                source_content.replace("_TITLE_", format!("# {}", title).as_str());
            let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
            content_with_title = content_with_title.replace("_CREATION_DATE_", &current_date);

            // Write content to new file
            match fs::File::create(&full_path) {
                Ok(_new_file) => {
                    let mut new_file = fs::File::create(&full_path).unwrap();
                    new_file
                        .write_all(content_with_title.as_bytes())
                        .expect("Error writing to file")
                }
                Err(_error) => {
                    eprintln!("Error: No path was found, did you run the init function?",);
                    return;
                }
            }
        } else {
            println!(
                "File '{}' already exists. Skipping creation and opening existing file.",
                full_path.display()
            );
        }
    }

    if !std::path::Path::is_dir(&path) {
        match std::fs::create_dir(&path) {
            Ok(_) => {} // if directory is created, all good
            Err(err) => {
                eprintln!("Error: Error creating directory {}", err);
                return;
            }
        }
    }

    let mut viewer_handle = std::process::Command::new("inlyne")
        .arg(full_path.clone())
        .spawn()
        .unwrap();

    let editor_status = std::process::Command::new(cfg.editor)
        .arg(full_path.clone())
        .status();

    match editor_status {
        Ok(status) => {
            if !status.success() {
                eprintln!("Error opening editor: {}", status);
                // Offer alternative approach (e.g., print file path)
            }
        }
        Err(err) => {
            eprintln!("Error: Failed to spawn editor: {}", err);
            // Offer alternative approach (e.g., print file path)
        }
    }
    let _ = viewer_handle.kill();
}

fn open_recent_file() {
    let cfg: Config = confy::load("ukato", None).expect("Unable to load Ukato config");
    let path = std::path::Path::new(&cfg.directory);

    let last_modified_file = std::fs::read_dir(path)
        .expect("Couldn't access local directory")
        .flatten() // Remove failed
        .filter(|f| f.metadata().unwrap().is_file()) // Filter out directories (only consider files)
        .max_by_key(|x| {
            x.metadata()
                .expect("Failed to get metadata")
                .modified()
                .expect("Failed to get modified time")
        }) // Get the most recently modified file
        .expect("No files found in directory")
        .file_name()
        .to_string_lossy()
        .into_owned();

    create_or_open_file(Create {
        name: last_modified_file,
        template: None,
    })
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

fn ensure_dir(path: &str) {
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

    let cfg: Config = confy::load("ukato", None).expect("Unable to load Ukato config");

    println!("Welcome to the setup wizard");

    let mut directory = Input::with_theme(&theme)
        .with_prompt("Notes directory")
        .with_initial_text(&cfg.directory)
        .interact_text()
        .expect("Failed to select directory");

    directory = expand_path(&directory);

    let items = &["vim", "nano", "emacs", "micro"];

    let editor_index = Select::with_theme(&theme)
        .with_prompt("Select your preferred editor")
        .default(0)
        .items(&items[..])
        .interact()
        .expect("Failed to select editor");

    let my_config = Config {
        directory: directory,
        editor: items[editor_index].to_string(),
    };

    validate_config(&my_config);
    confy::store("ukato", None, my_config).expect("Failed to store config");

    // Create a dir for templates
    let template_dir = std::path::Path::new(&cfg.directory).join("templates");

    if !std::path::Path::is_dir(&template_dir) {
        std::fs::create_dir_all(&template_dir).expect("failed to create template dir");
    }

    // Get the current working directory
    let current_dir = env::current_dir().expect("Failed to get the current directory");

    // Path to where templates are stored in ukato
    let template_source_path = &current_dir.join("src/templates");

    // Copy templates to local dir
    match copy_templates_to_local(&template_source_path, &template_dir) {
        Ok(_) => println!("Templates copied successfully. You are ready to start using Ukato!"),
        Err(err) => println!("Error copying templates: {}", err),
    }
}

fn copy_templates_to_local(source_path: &Path, destination_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut destination_path = PathBuf::from(destination_dir);
            destination_path.push(path.file_name().unwrap().to_str().unwrap());
            fs::copy(path, destination_path)?;
        }
    }
    Ok(())
}

fn list_notes(show_templates: bool) {
    let cfg: Config = confy::load("ukato", None).expect("Unable to load Ukato config");
    let dir = std::path::Path::new(&cfg.directory);

    let folder_path = if show_templates {
        dir.join("templates")   // Path to templates directory
    } else {
        dir.to_path_buf() // Path to notes directory (base_dir itself)
    };

    let paths = match fs::read_dir(folder_path) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|entry| {
                let binding = entry.file_name();
                let path = binding.to_string_lossy();
                if show_templates {
                    entry.file_type().is_ok() && entry.file_type().unwrap().is_dir()
                        || path.ends_with(".md")
                } else {
                    path.ends_with(".md")
                }
            })
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!(
                "Error reading directory: {}. Did you run the init function?",
                e
            );
            return;
        }
    };

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let prompt_text = if show_templates {
        "Your templates:"
    } else {
        "Your notes:"
    };

    if paths.is_empty() {
        eprintln!(
            "Error: No notes found, you can create a note by running 'ukato create <note-name>'"
        );
        return;
    }

    let note_index = Select::with_theme(&theme)
        .with_prompt(prompt_text)
        .default(0)
        .items(&paths[..])
        .interact()
        .expect("Failed to get notes");


    let note_path = if show_templates {
            format!("templates/{}", paths[note_index])
        } else {
            paths[note_index].clone()
        };
    
    create_or_open_file(Create {
        name: note_path,
        template: None,
    });
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_config(),
        Commands::Create(create_subcommand) => create_or_open_file(Create {
            name: create_subcommand.name,
            template: create_subcommand.template,
        }),
        Commands::ListNotes => list_notes(false),
        Commands::ListTemplates => list_notes(true),
        Commands::Recent => open_recent_file(),
        Commands::Template(args) => create_template(args),
    }
}

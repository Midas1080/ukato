# Ukato

Ukato is a simple command-line application for creating and managing notes using your favorite text editor. It provides an easy way to organize your notes and templates efficiently.

Also, in the endangered Kwazinian language of the people of Rondônia, Brazil, 'Ukato' is the word for the [Rusty tree frog](https://en.wikipedia.org/wiki/Rusty_tree_frog).

## Features

- **Note Creation:** Create new notes with customizable templates.
- **Template Management:** Manage and customize templates for your notes.
- **List Notes:** View a list of existing notes and templates.
- **Recent Note Access:** Quickly access the most recently modified note.

## Installation

To install Ukato, follow these steps:

0. Install dependency
   ```bash
   cargo install inlyne
   ```

1. Clone the repository:

    ```bash
    git clone https://github.com/Midas1080/ukato.git
    ```

2. Navigate to the project directory:

    ```bash
    cd ukato
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

4. Install the binary:

    ```bash
    cargo install --path .
    ```

## Usage

After installation, you can use Ukato from the command line. Here are some common commands:

- `ukato init`: Initialize Ukato and configure your notes directory and preferred text editor.
- `ukato create <note-name>`: Create a new note with the specified name.
- `ukato template <template-name>`: Create a new template with the specified name.
- `ukato list-notes`: List all existing notes.
- `ukato list-templates`: List all available templates.
- `ukato recent`: Open the most recently modified note.

For more detailed usage instructions and options, run `ukato --help`.

## Configuration

Ukato allows you to configure your notes directory and preferred text editor. You can modify the configuration using the `ukato init` command.

## Contributing

We are Rust noobs on our way to become full fledged, coconut-oiled, blue-haired Rustaceans, so contributions to Ukato are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request.

## Next steps

- Implement a category system using YAML frontmatter so that notes can be organised into categories
- Support more editors, currently we only tested with vim
- Improve error handling
- Split functionality into modules


## License

Ukato is licensed under the MIT License.

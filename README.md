# er - Environment Runner

A command-line tool that collects environment variables from `.env` files in the current directory and all parent directories, then executes a command with those environment variables.

## Features

- Automatically loads environment variables from `.env` files
- Searches for `.env` files in the current directory and all parent directories
- Variables in child directories override those in parent directories
- Executes commands with the collected environment variables

## Usage

```bash
er <command> [args...]
```

For example:
```bash
er npm start
er cargo run
er python script.py
```
## Installation

### Using Nix Flakes from GitHub

You can run this tool directly from GitHub without cloning the repository:

```bash
# Run directly from GitHub
nix run github:reonokiy/er -- <command>
```

## License

[MIT](LICENSE)

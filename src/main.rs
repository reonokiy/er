use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::process::{Command, ExitStatus};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        anyhow::bail!("No command specified");
    }

    // Collect environment variables from .env files
    let env_vars = collect_env_vars()?;

    // Execute the command with the collected environment variables
    let status = execute_command(&args[1..], &env_vars)?;

    // Exit with the same status code as the command
    std::process::exit(status.code().unwrap_or(1));
}

fn parse_env_file(content: &str) -> Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            vars.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    Ok(vars)
}

/// Collects environment variables from .env files in the current directory
/// and all parent directories.
fn collect_env_vars() -> Result<HashMap<String, String>> {
    let mut env_files = Vec::new();
    let mut dir = env::current_dir().expect("Failed to get current directory");
    loop {
        let env_path = dir.join(".env");
        if env_path.is_file() {
            env_files.push(env_path.clone());
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }

    let mut env_vars: HashMap<String, String> = HashMap::new();
    for env_file in env_files.iter().rev() {
        let content = std::fs::read_to_string(&env_file)
            .with_context(|| format!("Failed to read .env file: {:?}", env_file))?;
        let vars = parse_env_file(&content)?;
        env_vars.extend(vars);
    }
    Ok(env_vars)
}

/// Executes the given command with the specified environment variables.
fn execute_command(
    command_args: &[String],
    env_vars: &HashMap<String, String>,
) -> Result<ExitStatus> {
    let (program, args) = command_args.split_first().unwrap();
    let mut cmd = Command::new(program);
    cmd.args(args);

    for (key, value) in env_vars {
        cmd.env(key, value);
    }

    let status = cmd
        .status()
        .context(format!("Failed to execute command: {}", program))?;

    Ok(status)
}

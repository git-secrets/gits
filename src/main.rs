use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{debug, info, warn};

#[derive(Parser)]
#[command(name = "gits")]
#[command(about = "A Git wrapper for sensitive files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Arguments to pass directly to git
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new .gits repository
    Init,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => init_gits_repo()?,
        None => {
            // Pass all arguments directly to git with environment variables set
            let status = run_git_command(&cli.args)?;
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    Ok(())
}

/// Initialize a new .gits repository in the current directory
fn init_gits_repo() -> Result<()> {
    let current_dir = env::current_dir()?;
    let gits_dir = current_dir.join(".gits");
    
    // Check if .git exists (we're in a git repo)
    let git_dir = current_dir.join(".git");
    if !git_dir.exists() {
        return Err(anyhow::anyhow!("Not in a git repository. Initialize a git repository first."));
    }
    
    // Create .gits directory if it doesn't exist
    if !gits_dir.exists() {
        fs::create_dir(&gits_dir)
            .context("Failed to create .gits directory")?;
        info!("Created .gits directory");
    }
    
    // Initialize git repository in .gits
    let status = Command::new("git")
        .env("GIT_DIR", &gits_dir)
        .args(["init"])
        .status()
        .context("Failed to initialize git repository in .gits")?;
    
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to initialize git repository in .gits"));
    }
    
    // Add .gits to .git/info/exclude
    let exclude_file = git_dir.join("info").join("exclude");
    ensure_directory_exists(exclude_file.parent().unwrap())?;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&exclude_file)
        .context("Failed to open .git/info/exclude file")?;
    
    // Check if .gits is already in exclude file
    let exclude_content = fs::read_to_string(&exclude_file)
        .context("Failed to read .git/info/exclude file")?;
    
    if !exclude_content.lines().any(|line| line.trim() == ".gits") {
        writeln!(file, ".gits")
            .context("Failed to write to .git/info/exclude file")?;
        info!("Added .gits to .git/info/exclude");
    }
    
    println!("Initialized empty gits repository in .gits/");
    println!("\nRecommendation: Add your sensitive files to your main .gitignore as well for extra safety.");
    
    Ok(())
}

/// Run a git command with GIT_DIR set to .gits and GIT_WORK_TREE set to the current directory
fn run_git_command(args: &[String]) -> Result<ExitStatus> {
    let current_dir = env::current_dir()?;
    let gits_dir = current_dir.join(".gits");
    
    if !gits_dir.exists() {
        return Err(anyhow::anyhow!(".gits directory not found. Run 'gits init' first."));
    }
    
    debug!("Running git command with args: {:?}", args);
    
    let status = Command::new("git")
        .env("GIT_DIR", &gits_dir)
        .env("GIT_WORK_TREE", &current_dir)
        .args(args)
        .status()
        .context("Failed to execute git command")?;
    
    Ok(status)
}

/// Ensure a directory exists, creating it if necessary
fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .context(format!("Failed to create directory: {}", path.display()))?;
    }
    Ok()
}

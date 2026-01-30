//! Peta CLI - Command-line interface for the static site generator

use clap::Parser;
use anyhow::Result;
use peta::cli::{Cli, Commands, commands, OutputFormatter};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut output = OutputFormatter::new();
    
    match cli.command {
        Commands::New { name, theme } => {
            commands::init_site(&name, &theme, &mut output)?;
        }
        Commands::Init { action } => {
            match action {
                peta::cli::args::InitAction::Site { name, theme } => {
                    commands::init_site(&name, &theme, &mut output)?;
                }
                peta::cli::args::InitAction::Content { r#type, title } => {
                    commands::init_content(&r#type, &title, &mut output)?;
                }
            }
        }
        Commands::Build { output: output_dir, theme, draft } => {
            commands::build_site(output_dir, theme, draft, &mut output).await?;
        }
        Commands::Serve { port, host, open, draft } => {
            commands::serve_site(port, &host, open, draft, &mut output).await?;
        }
        Commands::Deploy { target } => {
            commands::deploy_site(&target, &mut output).await?;
        }
        Commands::Clean { all } => {
            commands::clean_site(all, &mut output)?;
        }
        Commands::Theme { action } => {
            match action {
                peta::cli::args::ThemeAction::List => {
                    commands::theme::list_themes(&mut output)?;
                }
                peta::cli::args::ThemeAction::Create { name, base } => {
                    commands::theme::create_theme(&name, base, &mut output)?;
                }
                peta::cli::args::ThemeAction::Validate { name } => {
                    commands::theme::validate_theme(&name, &mut output)?;
                }
                peta::cli::args::ThemeAction::Info { name } => {
                    commands::theme::theme_info(&name, &mut output)?;
                }
                peta::cli::args::ThemeAction::Install { source, name } => {
                    commands::theme::install_theme(&source, name, &mut output)?;
                }
            }
        }
    }
    
    Ok(())
}
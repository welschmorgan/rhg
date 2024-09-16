use std::path::PathBuf;

use clap::{builder::ValueParser, command, Parser, Subcommand};

use crate::{parse_filter, Filter};

#[derive(Parser, Debug)]
#[command(author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"), about, long_about)]
/// Command-line resource packing utility
pub struct CliOptions {
  /// Main application mode
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Parser, Debug)]
pub struct AddCommandOptions {
  /// Path of the archive to write
  pub archive: PathBuf,
  /// Files to add to the archive
  #[arg(num_args = 1..)]
  pub files: Vec<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct UpdateCommandOptions {
  /// Path of the archive to write
  pub archive: PathBuf,
  /// Files to add to the archive
  #[arg(num_args = 1..)]
  pub files: Vec<PathBuf>,
}

const DEFAULT_LIST_TEMPLATE: &'static str = "%offset %archived_at %name";

#[derive(Parser, Debug)]
pub struct ListCommandOptions {
  /// Path of the archive to write
  pub archive: PathBuf,

  /// Filter listed files
  #[arg(value_parser = ValueParser::new(parse_filter))]
  pub filters: Vec<Filter>,
  
  /// Change output columns
  #[arg(short, long, default_value = DEFAULT_LIST_TEMPLATE)]
  pub template: String,
  
  /// Show possible template variables
  #[arg(short, long)]
  pub show_template_vars: bool,
}

#[derive(Parser, Debug)]
pub struct RemoveCommandOptions {
  /// Path of the archive to write
  pub archive: PathBuf,
  /// Files to remove from the archive
  #[arg(num_args = 1.., value_parser = ValueParser::new(parse_filter))]
  pub filter: Vec<Filter>,
}

#[derive(Parser, Debug)]
pub struct ExtractCommandOptions {
  /// Path of the archive to write
  pub archive: PathBuf,
  
  /// Files to extract from the archive
  #[arg(num_args = 1.., value_parser = ValueParser::new(parse_filter))]
  pub filter: Vec<Filter>,

  /// Optional output dir
  #[arg(short, long)]
  pub output_dir: Option<PathBuf>
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// Add files to the archive
  Add(AddCommandOptions),
  /// Update files inside the archive
  Update(UpdateCommandOptions),
  /// Remove files from the archive
  Remove(RemoveCommandOptions),
  /// Remove files from the archive
  Extract(ExtractCommandOptions),
  /// List all files contained within the archive
  List(ListCommandOptions),
}

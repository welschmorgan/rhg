use chrono::{DateTime, Utc};
use clap::Parser;
use rhg_pack::{
  AddCommandOptions, CliOptions, Command, ExtractCommandOptions, Filter, ListCommandOptions,
  RemoveCommandOptions, UpdateCommandOptions,
};
use std::{
  io::{stdout, Stdout},
  ops::{Deref, DerefMut},
  path::{Path, PathBuf},
  process::{exit, ExitCode, ExitStatus},
  str::FromStr,
  time::SystemTime,
};

use rhg_engine_core::{err, here, Archive, ArchiveFile, Error, ErrorKind};

fn add(opt: &AddCommandOptions) -> rhg_engine_core::Result<()> {
  let mut a = Archive::default();
  for f in &opt.files {
    a.add(ArchiveFile::load(f)?)?;
  }
  a.save_file(&opt.archive)?;
  Ok(())
}

fn update(opt: &UpdateCommandOptions) -> rhg_engine_core::Result<()> {
  let mut a = match opt.archive.exists() {
    true => Archive::load_file(&opt.archive)?,
    false => Archive::default(),
  };
  for f in &opt.files {
    let _ = a.remove_file(f);
    a.add(ArchiveFile::load(f)?)?;
  }
  a.save_file(&opt.archive)?;
  Ok(())
}

fn remove(opt: &RemoveCommandOptions) -> rhg_engine_core::Result<()> {
  let mut a = Archive::load_file(&opt.archive)?;
  let mut modified = false;
  if let Some(files) = filter_files(&a, &opt.filter)
    .map(|files| files.iter().map(|file| (*file).clone()).collect::<Vec<_>>())
  {
    for file in files {
      if let Some(_) = a.remove_file(file.path()) {
        modified = true;
      }
    }
  }
  if modified {
    a.save_file(&opt.archive)?;
  } else {
    eprintln!("\x1b[0;33mwarn\x1b[0m: archive left untouched")
  }
  Ok(())
}

fn extract(opt: &ExtractCommandOptions) -> rhg_engine_core::Result<()> {
  let a = Archive::load_file(&opt.archive)?;
  let output_dir = opt
    .output_dir
    .as_ref()
    .map(|v| v.clone())
    .unwrap_or_else(|| PathBuf::from("."));
  if !output_dir.exists() {
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
      return err!(
        ErrorKind::IO,
        format!(
          "failed to create directory '{}', {}",
          output_dir.display(),
          e
        )
      );
    }
  }
  if let Some(files) = filter_files(&a, &opt.filter) {
    for file in files {
      let out_path = output_dir.join(file.path());
      if let Err(e) = std::fs::write(&out_path, file.content()) {
        return err!(
          ErrorKind::IO,
          format!("failed to write file '{}', {}", out_path.display(), e)
        );
      }
      println!("write {} - {}B", out_path.display(), file.content_len());
    }
  }
  Ok(())
}

fn filter_files<'a>(a: &'a Archive, filters: &[Filter]) -> Option<Vec<&'a ArchiveFile>> {
  let filtered = a
    .files()
    .iter()
    .filter(|file| {
      let matches = filters.is_empty()
        || filters.iter().any(|filter| {
          file
            .name()
            .map(|ref name| filter.matches(&name))
            .unwrap_or_default()
        });
      return matches;
    })
    .collect::<Vec<_>>();
  if filtered.is_empty() {
    eprintln!(
      "\x1b[0;33mwarn\x1b[0m: no files match filters: {}",
      filters
        .iter()
        .map(|filter| filter.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
    return None;
  }
  Some(filtered)
}

type Getter<'a> = fn(&'a ArchiveFile) -> Option<String>;

fn print_sys_time(st: &SystemTime) -> String {
  let datetime: DateTime<Utc> = (*st).into();
  format!("{}", datetime.format("%d/%m/%Y %T"))
}

fn list(opt: &ListCommandOptions) -> rhg_engine_core::Result<()> {
  let mut tpl_vars: Vec<(&str, Getter<'_>)> = vec![
    ("offset", |file| Some(format!("0x{:08x}", file.offset()))),
    ("path", |file| Some(format!("{}", file.path().display()))),
    ("name", |file| file.name()),
    ("created_at", |file| {
      file.created_at().map(|st| print_sys_time(st))
    }),
    ("modified_at", |file| {
      file.modified_at().map(|st| print_sys_time(st))
    }),
    ("archived_at", |file| {
      file.archived_at().map(|st| print_sys_time(st))
    }),
  ];
  if opt.show_template_vars {
    println!("List of template variables:");
    for (name, _getter) in &tpl_vars {
      println!("{}", name);
    }
    exit(0);
  }
  let a = Archive::load_file(&opt.archive)?;
  {
    if let Some(filtered) = filter_files(&a, &opt.filters) {
      for file in filtered {
        // println!("{}", &["Offset", "Created at", "Modified at", ""]);
        let tpl_vals = tpl_vars
          .iter()
          .map(|(key, getter)| (format!("%{}", key), getter(file)))
          .collect::<Vec<_>>();
        let mut tpl_out = opt.template.clone();
        for (tpl_key, tpl_val) in &tpl_vals {
          tpl_out = tpl_out.replace(
            tpl_key,
            &tpl_val.as_ref().map(|v| v.clone()).unwrap_or_default(),
          );
        }
        println!("{}", tpl_out)
      }
    }
  }
  Ok(())
}

fn main() -> ExitCode {
  let options = CliOptions::parse();
  let e = match options.command {
    Command::Add(opts) => add(&opts),
    Command::Update(opts) => update(&opts),
    Command::Remove(opts) => remove(&opts),
    Command::List(opts) => list(&opts),
    Command::Extract(opts) => extract(&opts),
  };
  if let Err(e) = e {
    eprintln!("\x1b[0;31merror\x1b[0m: {}", e);
    return ExitCode::FAILURE;
  }
  ExitCode::SUCCESS
}

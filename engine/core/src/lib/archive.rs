use std::{
  collections::HashMap,
  io::{BufReader, Cursor, Read, Seek as _},
  path::{Path, PathBuf},
  time::{Duration, Instant, SystemTime},
};

use crate::{err, here, Error, ErrorKind};

pub const ARCHIVE_MAGIC_NUMBER: u64 = 0xdeadbeef;
pub const ARCHIVE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Debug, Clone)]
pub struct ArchiveFile {
  path: PathBuf,
  offset: u64,
  created_at: Option<SystemTime>,
  modified_at: Option<SystemTime>,
  archived_at: Option<SystemTime>,
  content: Vec<u8>,
}

impl ArchiveFile {
  pub fn new<P: AsRef<Path>>(path: P, content: &[u8]) -> Self {
    Self {
      path: path.as_ref().to_path_buf(),
      content: content.to_vec(),
      created_at: None,
      modified_at: None,
      archived_at: None,
      offset: 0,
    }
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn path_mut(&mut self) -> &mut PathBuf {
    &mut self.path
  }

  pub fn content_len(&self) -> usize {
    self.content.len()
  }

  pub fn content(&self) -> &Vec<u8> {
    &self.content
  }

  pub fn content_mut(&mut self) -> &mut Vec<u8> {
    &mut self.content
  }

  pub fn offset_mut(&mut self) -> &mut u64 {
    &mut self.offset
  }

  pub fn offset(&self) -> u64 {
    self.offset
  }

  pub fn created_at(&self) -> Option<&SystemTime> {
    self.created_at.as_ref()
  }

  pub fn modified_at(&self) -> Option<&SystemTime> {
    self.modified_at.as_ref()
  }

  pub fn archived_at(&self) -> Option<&SystemTime> {
    self.archived_at.as_ref()
  }

  pub fn name(&self) -> Option<String> {
    self
      .path
      .file_name()
      .and_then(|name| name.to_str())
      .map(|name| name.to_string())
  }

  pub fn extension(&self) -> Option<String> {
    self
      .path
      .extension()
      .and_then(|ext| ext.to_str())
      .map(|name| name.to_string())
  }

  pub fn matches<P: AsRef<Path>>(&self, path: P) -> bool {
    if self.path.eq(path.as_ref()) {
      return true;
    }
    let f_name = self.path.file_name().and_then(|oss| oss.to_str());
    let i_name = path.as_ref().file_name().and_then(|oss| oss.to_str());
    if let (Some(f_name), Some(i_name)) = (f_name, i_name) {
      if f_name.eq(i_name) {
        return true;
      }
    }
    false
  }

  pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<ArchiveFile> {  
    let md = std::fs::metadata(&path)?;
    let content = std::fs::read(&path).map_err(|e| {
      Error::new(
        ErrorKind::IO,
        format!("{}: {}", path.as_ref().display(), e),
        None,
        here!(),
      )
    })?;
    let mut f = Self::new(path, &content);
    f.modified_at = Some(md.modified()?);
    f.created_at = Some(md.created()?);
    Ok(f)
  }
}

#[derive(Default, Debug, Clone)]
pub struct Archive {
  path: Option<PathBuf>,
  files: Vec<ArchiveFile>,
}

impl Archive {
  pub fn contains_file<P: AsRef<Path>>(&self, path: P) -> bool {
    return self.get_file(path).is_some();
  }

  pub fn get_file<P: AsRef<Path>>(&self, path: P) -> Option<&ArchiveFile> {
    for f in &self.files {
      if f.matches(path.as_ref()) {
        return Some(f);
      }
    }
    return None;
  }

  pub fn add(&mut self, f: ArchiveFile) -> crate::Result<&mut ArchiveFile> {
    if let Some(_) = self.get_file(f.path()) {
      return err!(
        ErrorKind::IO,
        format!("file '{}' already exists", f.path().display())
      );
    }
    self.files.push(f);
    Ok(self.files.last_mut().unwrap())
  }
  
  pub fn add_file<P: AsRef<Path>>(
    &mut self,
    path: P,
    content: &[u8],
  ) -> crate::Result<&mut ArchiveFile> {
    self.add(ArchiveFile::new(path, content))
  }

  pub fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Option<ArchiveFile> {
    for (i, f) in self.files.iter().enumerate() {
      if f.matches(path.as_ref()) {
        return Some(self.files.remove(i));
      }
    }
    return None;
  }

  pub fn files(&self) -> &Vec<ArchiveFile> {
    &self.files
  }

  pub fn files_mut(&mut self) -> &mut Vec<ArchiveFile> {
    &mut self.files
  }

  pub fn save_file<P: AsRef<Path>>(&mut self, path: P) -> crate::Result<()> {
    let mut f = std::fs::File::create(&path)?;
    self.save(Some(path.as_ref().to_path_buf()), &mut f)?;
    Ok(())
  }

  pub fn save<W: std::io::Write>(&mut self, path: Option<PathBuf>, w: &mut W) -> crate::Result<()> {
    if let Some(path) = path {
      self.path = Some(path.to_path_buf());
    }
    let mut nwritten = w.write(&(ARCHIVE_MAGIC_NUMBER as u64).to_le_bytes())?;
    nwritten += w.write(&(ARCHIVE_VERSION.len() as u64).to_le_bytes())?;
    nwritten += w.write(ARCHIVE_VERSION.as_bytes())?;
    nwritten += w.write(&(self.files.len() as u64).to_le_bytes())?;
    let mut buf: Vec<u8> = vec![];
    let mut header_len = nwritten;
    for f in &self.files {
      let path = format!("{}", f.path.display()).to_string();
      // path len
      header_len += (u64::BITS / 8) as usize;
      // path
      header_len += path.as_bytes().len();
      // content len
      header_len += (u64::BITS / 8) as usize;
      // offset
      header_len += (u64::BITS / 8) as usize;
      // created at
      header_len += (u64::BITS / 8) as usize;
      // modified at
      header_len += (u64::BITS / 8) as usize;
      // archived at
      header_len += (u64::BITS / 8) as usize;
    }
    let mut offset: usize = header_len;
    for f in &mut self.files {
      let path = format!("{}", f.path.display()).to_string();
      let modified_at = f
        .modified_at
        .map(|mt| {
          mt.duration_since(SystemTime::UNIX_EPOCH)
            .expect("invalid epoch")
            .as_secs()
        })
        .unwrap_or_default();
      let created_at = f
        .created_at
        .map(|mt| {
          mt.duration_since(SystemTime::UNIX_EPOCH)
            .expect("invalid epoch")
            .as_secs()
        })
        .unwrap_or_default();
      f.archived_at = Some(SystemTime::now());
      let archived_at = f
        .archived_at
        .map(|mt| {
          mt.duration_since(SystemTime::UNIX_EPOCH)
            .expect("invalid epoch")
            .as_secs()
        })
        .unwrap_or_default();
      header_len += w.write(&(path.len() as u64).to_le_bytes())?;
      header_len += w.write(path.as_bytes())?;
      header_len += w.write(&(f.content.len() as u64).to_le_bytes())?;
      header_len += w.write(&(offset as u64).to_le_bytes())?;
      header_len += w.write(&(created_at as u64).to_le_bytes())?;
      header_len += w.write(&(modified_at as u64).to_le_bytes())?;
      header_len += w.write(&(archived_at as u64).to_le_bytes())?;
      f.offset = offset as u64;
      buf.extend_from_slice(&f.content);
      println!("write '{}' at 0x{:04x}", path, offset);
      offset += f.content.len()
    }
    let _ = w.write(&buf)?;
    Ok(())
  }

  pub fn load_file<P: AsRef<Path>>(path: P) -> crate::Result<Archive> {
    let mut f = std::fs::File::open(&path)?;
    Self::load(path.as_ref(), &mut f)
  }

  pub fn load<P: AsRef<Path>, R: std::io::Read>(path: P, r: &mut R) -> crate::Result<Archive> {
    let mut bytes: Vec<u8> = vec![];
    let _ = r.read_to_end(&mut bytes).map_err(|e| {
      Error::new(
        ErrorKind::IO,
        format!(
          "failed to load archive file '{}', {}",
          path.as_ref().display(),
          e
        ),
        None,
        here!(),
      )
    })?;
    let mut u64_buf: [u8; 8] = [0; 8];
    let mut curs = Cursor::new(bytes);

    let _ = curs.read(&mut u64_buf)?;
    let _magic = u64::from_le_bytes(u64_buf);
    let _ = curs.read(&mut u64_buf)?;
    let _version_len = u64::from_le_bytes(u64_buf);
    let mut _version = vec![0; _version_len as usize];
    let _ = curs.read_exact(&mut _version)?;
    let _version_str = String::from_utf8_lossy(&_version);

    let _ = curs.read(&mut u64_buf)?;
    let num_files = u64::from_le_bytes(u64_buf);

    if _magic != ARCHIVE_MAGIC_NUMBER {
      return err!(ErrorKind::IO, "corrupted archive");
    } else if !_version_str.eq(ARCHIVE_VERSION) {
      eprintln!(
        "\x1b[0;33mwarn\x1b[0m archive created using packer v{} but archiver is v{}",
        _version_str, ARCHIVE_VERSION
      )
    }

    let mut f_path_len: usize;
    let mut f_path: Vec<u8>;
    let mut f_content_len: usize;
    let mut a = Archive::default();
    a.path = Some(path.as_ref().to_path_buf());
    for _ in 0..num_files {
      let _ = curs.read_exact(&mut u64_buf)?;
      f_path_len = u64::from_le_bytes(u64_buf) as usize;

      f_path = vec![0; f_path_len];
      let _ = curs.read_exact(&mut f_path)?;

      let _ = curs.read_exact(&mut u64_buf)?;
      let content_len = u64::from_le_bytes(u64_buf);

      let _ = curs.read_exact(&mut u64_buf)?;
      let offset = u64::from_le_bytes(u64_buf);
      
      let _ = curs.read_exact(&mut u64_buf)?;
      let created_at = u64::from_le_bytes(u64_buf);

      let _ = curs.read_exact(&mut u64_buf)?;
      let modified_at = u64::from_le_bytes(u64_buf);

      let _ = curs.read_exact(&mut u64_buf)?;
      let archived_at = u64::from_le_bytes(u64_buf);

      let mut content: Vec<u8> = vec![0; content_len as usize];
      let prev_pos = curs.stream_position()?;
      curs.seek(std::io::SeekFrom::Start(offset as u64))?;
      let _ = curs.read_exact(&mut content)?;
      curs.seek(std::io::SeekFrom::Start(prev_pos))?;

      let mut f = ArchiveFile::new(
        PathBuf::from(String::from_utf8_lossy(&f_path).to_string()),
        &content,
      );
      f.offset = offset;
      if let Some(time) = SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(modified_at)) {
        f.modified_at = Some(time);
      }
      if let Some(time) = SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(created_at)) {
        f.created_at = Some(time);
      }
      if let Some(time) = SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(archived_at)) {
        f.archived_at = Some(time);
      }
      a.files.push(f);
    }
    Ok(a)
  }
}

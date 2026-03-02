use anyhow::{Context, Result};
#[cfg(feature = "memmap2")]
use memmap2::Mmap;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Resolved input text — either memory-mapped or owned
pub enum InputText {
    #[cfg(feature = "memmap2")]
    Mapped(Mmap),
    Owned(String),
}

impl InputText {
    pub fn as_str(&self) -> Result<&str> {
        match self {
            #[cfg(feature = "memmap2")]
            InputText::Mapped(mmap) => {
                std::str::from_utf8(mmap).context("input is not valid UTF-8")
            }
            InputText::Owned(s) => Ok(s.as_str()),
        }
    }
}

/// Read input from a file path (memory-mapped for performance)
pub fn read_file(path: &Path) -> Result<InputText> {
    let file = File::open(path)
        .with_context(|| format!("could not open '{}'", path.display()))?;
    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        return Ok(InputText::Owned(String::new()));
    }
    #[cfg(feature = "memmap2")]
    {
        let mmap = unsafe { Mmap::map(&file) }
            .with_context(|| format!("could not mmap '{}'", path.display()))?;
        Ok(InputText::Mapped(mmap))
    }
    #[cfg(not(feature = "memmap2"))]
    {
        use std::io::Read as _;
        let mut buf = String::new();
        std::io::BufReader::new(file).read_to_string(&mut buf)?;
        Ok(InputText::Owned(buf))
    }
}

/// Read all of stdin into a string
pub fn read_stdin() -> Result<InputText> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).context("could not read stdin")?;
    Ok(InputText::Owned(buf))
}

/// Collect file paths from a directory (optionally recursive)
pub fn collect_files(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_files_inner(dir, recursive, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_inner(dir: &Path, recursive: bool, out: &mut Vec<PathBuf>) -> Result<()> {
    let entries = std::fs::read_dir(dir)
        .with_context(|| format!("could not read directory '{}'", dir.display()))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            out.push(path);
        } else if path.is_dir() && recursive {
            collect_files_inner(&path, recursive, out)?;
        }
    }
    Ok(())
}

/// Resolve input: if path is Some, read file(s); otherwise read stdin
pub fn resolve_input(path: Option<&PathBuf>, recursive: bool) -> Result<Vec<(String, InputText)>> {
    match path {
        Some(p) if p.is_dir() => {
            let files = collect_files(p, recursive)?;
            anyhow::ensure!(!files.is_empty(), "no files found in '{}'", p.display());
            files
                .into_iter()
                .map(|f| {
                    let name = f.display().to_string();
                    read_file(&f).map(|text| (name, text))
                })
                .collect()
        }
        Some(p) => {
            let name = p.display().to_string();
            Ok(vec![(name, read_file(p)?)])
        }
        None => Ok(vec![("<stdin>".to_string(), read_stdin()?)]),
    }
}

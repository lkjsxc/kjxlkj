use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const SOURCE_EXTENSIONS: &[&str] = &[
    "c", "cc", "cpp", "cxx", "go", "h", "hpp", "java", "js", "jsx", "kt", "kts", "php", "py", "rb",
    "rs", "sh", "swift", "ts", "tsx",
];

pub fn collect_files<F>(
    root: &Path,
    include_path: F,
    allow_missing_root: bool,
) -> io::Result<Vec<PathBuf>>
where
    F: Fn(&Path) -> bool + Copy,
{
    if !root.exists() {
        if allow_missing_root {
            return Ok(Vec::new());
        }
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("directory not found: {}", root.display()),
        ));
    }
    if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path is not a directory: {}", root.display()),
        ));
    }
    let mut files = Vec::new();
    collect_files_recursive(root, include_path, &mut files)?;
    files.sort();
    Ok(files)
}

pub fn count_lines(path: &Path) -> io::Result<usize> {
    Ok(fs::read_to_string(path)?.lines().count())
}

pub fn is_markdown_file(path: &Path) -> bool {
    path.extension().and_then(OsStr::to_str) == Some("md")
}

pub fn is_source_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(OsStr::to_str) else {
        return false;
    };
    SOURCE_EXTENSIONS
        .iter()
        .any(|allowed| extension.eq_ignore_ascii_case(allowed))
}

fn collect_files_recursive<F>(
    directory: &Path,
    include_path: F,
    files: &mut Vec<PathBuf>,
) -> io::Result<()>
where
    F: Fn(&Path) -> bool + Copy,
{
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files_recursive(&path, include_path, files)?;
        } else if file_type.is_file() && include_path(&path) {
            files.push(path);
        }
    }
    Ok(())
}

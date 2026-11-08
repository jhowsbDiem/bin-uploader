use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::Deserialize;

pub struct Build {
    pub folder: PathBuf,
    pub sketch: PathBuf,
    pub name: String,
    pub fqbn: String,
    pub date: SystemTime,
}

#[derive(Debug)]
pub enum FindError {
    NoSketch { start: PathBuf },
    NoCacheDir,
    NoBuild { sketch: PathBuf, folder: PathBuf },
    BadBuild { folder: PathBuf, reason: String },
}

impl fmt::Display for FindError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FindError::NoSketch { start } => write!(
                f,
                "no sketch found in {} or its parent folders: a sketch folder contains a .ino file named after the folder",
                start.display()
            ),
            FindError::NoCacheDir => write!(
                f,
                "cannot find the Arduino build cache: set ARDUINO_BUILD_CACHE_PATH or use --build-path"
            ),
            FindError::NoBuild { sketch, folder } => write!(
                f,
                "{} has not been compiled yet ({} does not exist): compile it in the Arduino IDE or with arduino-cli first",
                sketch.display(),
                folder.display()
            ),
            FindError::BadBuild { folder, reason } => {
                write!(
                    f,
                    "{} is not a complete Arduino build: {reason}",
                    folder.display()
                )
            }
        }
    }
}

impl std::error::Error for FindError {}

// Arduino names each build folder after the MD5 of the sketch path, so the
// same sketch always compiles into the same folder.
fn sketch_hash(sketch: &Path) -> String {
    format!("{:X}", md5::compute(sketch.as_os_str().as_encoded_bytes()))
}

fn find_sketch(start: &Path) -> Result<PathBuf, FindError> {
    for folder in start.ancestors() {
        if let Some(name) = folder.file_name() {
            // Appended by hand because Path::with_extension would turn `my.sketch` into `my.ino`.
            let mut main_file = name.to_os_string();
            main_file.push(".ino");
            if folder.join(main_file).is_file() {
                return Ok(folder.to_path_buf());
            }
        }
    }
    Err(FindError::NoSketch {
        start: start.to_path_buf(),
    })
}

fn cache_dir(override_dir: Option<PathBuf>, user_cache: Option<PathBuf>) -> Option<PathBuf> {
    match override_dir {
        Some(dir) => Some(dir),
        None => user_cache.map(|dir| dir.join("arduino")),
    }
}

// Only the fields bin-uploader needs; serde ignores the rest of the file.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildOptions {
    sketch_location: PathBuf,
    fqbn: String,
}

fn read_build(folder: &Path) -> Result<Build, FindError> {
    let bad = |reason: String| FindError::BadBuild {
        folder: folder.to_path_buf(),
        reason,
    };

    let text = fs::read_to_string(folder.join("build.options.json"))
        .map_err(|error| bad(format!("cannot read build.options.json: {error}")))?;
    let options: BuildOptions = serde_json::from_str(&text)
        .map_err(|error| bad(format!("invalid build.options.json: {error}")))?;

    let name = options
        .sketch_location
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let binary = format!("{name}.ino.bin");
    let date = fs::metadata(folder.join(&binary))
        .and_then(|metadata| metadata.modified())
        .map_err(|error| bad(format!("cannot read {binary}: {error}")))?;

    Ok(Build {
        folder: folder.to_path_buf(),
        sketch: options.sketch_location,
        name,
        fqbn: options.fqbn,
        date,
    })
}

pub fn find_build(
    start: &Path,
    build_path: Option<&Path>,
    cache: Option<&Path>,
) -> Result<Build, FindError> {
    if let Some(folder) = build_path {
        return read_build(folder);
    }

    let sketch = find_sketch(start)?;
    let cache = cache.ok_or(FindError::NoCacheDir)?;
    let folder = cache.join("sketches").join(sketch_hash(&sketch));
    if !folder.is_dir() {
        return Err(FindError::NoBuild { sketch, folder });
    }
    read_build(&folder)
}

pub fn build_cache() -> Option<PathBuf> {
    cache_dir(env_dir("ARDUINO_BUILD_CACHE_PATH"), user_cache_dir())
}

// An empty variable is treated as unset, matching Go's os.UserCacheDir.
fn env_dir(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

// Mirrors Go's os.UserCacheDir, under which arduino-cli keeps its cache.
#[cfg(windows)]
fn user_cache_dir() -> Option<PathBuf> {
    env_dir("LOCALAPPDATA")
}

#[cfg(not(windows))]
fn user_cache_dir() -> Option<PathBuf> {
    env_dir("XDG_CACHE_HOME").or_else(|| env_dir("HOME").map(|home| home.join(".cache")))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    use serde_json::json;
    use tempfile::TempDir;

    fn write(path: &Path, contents: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn make_sketch(root: &Path, name: &str) -> PathBuf {
        let sketch = root.join(name);
        write(&sketch.join(format!("{name}.ino")), "");
        sketch
    }

    fn make_build(folder: &Path, sketch: &Path) {
        let name = sketch.file_name().unwrap().to_str().unwrap();
        let options = json!({ "fqbn": "esp32:esp32:esp32", "sketchLocation": sketch });
        write(&folder.join("build.options.json"), &options.to_string());
        write(&folder.join(format!("{name}.ino.bin")), "");
    }

    #[test]
    fn hash_matches_a_real_arduino_cli_build() {
        let hash = sketch_hash(Path::new("/home/jonathan/Arduino/Blink"));
        assert_eq!(hash, "FE14F9E05127B11E60A3426A7847B4CD");
    }

    #[test]
    fn finds_the_sketch_from_a_subfolder() {
        let root = TempDir::new().unwrap();
        let sketch = make_sketch(root.path(), "Blink");

        let found = find_sketch(&sketch.join("src").join("utils")).unwrap();
        assert_eq!(found, sketch);
    }

    #[test]
    fn finds_a_sketch_with_a_dot_in_its_name() {
        let root = TempDir::new().unwrap();
        let sketch = make_sketch(root.path(), "my.sketch");

        assert_eq!(find_sketch(&sketch).unwrap(), sketch);
    }

    #[test]
    fn a_folder_outside_any_sketch_is_an_error() {
        let root = TempDir::new().unwrap();

        let result = find_sketch(root.path());
        assert!(matches!(result, Err(FindError::NoSketch { .. })));
    }

    #[test]
    fn arduino_build_cache_path_wins_over_the_user_cache() {
        let cache = cache_dir(
            Some(PathBuf::from("/custom")),
            Some(PathBuf::from("/home/u/.cache")),
        );
        assert_eq!(cache, Some(PathBuf::from("/custom")));
    }

    #[test]
    fn the_default_cache_is_arduino_inside_the_user_cache() {
        let cache = cache_dir(None, Some(PathBuf::from("/home/u/.cache")));
        assert_eq!(cache, Some(PathBuf::from("/home/u/.cache/arduino")));
    }

    #[test]
    fn reads_the_sketch_and_the_fqbn_of_a_build() {
        let root = TempDir::new().unwrap();
        let sketch = root.path().join("Blink");
        let folder = root.path().join("build");
        make_build(&folder, &sketch);

        let build = read_build(&folder).unwrap();
        assert_eq!(build.folder, folder);
        assert_eq!(build.sketch, sketch);
        assert_eq!(build.name, "Blink");
        assert_eq!(build.fqbn, "esp32:esp32:esp32");
    }

    #[test]
    fn finds_the_build_of_the_sketch_in_the_cache() {
        let root = TempDir::new().unwrap();
        let sketch = make_sketch(root.path(), "Blink");
        let cache = root.path().join("cache");
        let folder = cache.join("sketches").join(sketch_hash(&sketch));
        make_build(&folder, &sketch);

        let build = find_build(&sketch.join("src"), None, Some(&cache)).unwrap();
        assert_eq!(build.folder, folder);
    }

    #[test]
    fn build_path_needs_neither_the_sketch_nor_the_cache() {
        let root = TempDir::new().unwrap();
        let folder = root.path().join("elsewhere");
        make_build(&folder, &root.path().join("Blink"));

        let build = find_build(root.path(), Some(&folder), None).unwrap();
        assert_eq!(build.folder, folder);
    }

    #[test]
    fn a_sketch_never_compiled_is_an_error() {
        let root = TempDir::new().unwrap();
        let sketch = make_sketch(root.path(), "Blink");
        let cache = root.path().join("cache");

        let result = find_build(&sketch, None, Some(&cache));
        assert!(matches!(result, Err(FindError::NoBuild { .. })));
    }
}

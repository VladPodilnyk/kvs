use serde::{Deserialize, Serialize};

use crate::{KvsError, Result};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are persisted to disk in log files. Log files are named after
/// monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
///
/// ```rust
/// # use kvs::{KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
pub struct KvStore {
    /// Directory for the log and other data.
    path: PathBuf,
    file: File,
    cache: HashMap<String, String>,
}

impl KvStore {
    /// Opens a `KvStore` with the given path.
    ///
    /// This will create a new directory if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        if !fs::exists(&path)? {
            fs::create_dir_all(&path)?;
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&path)?;

            return Ok(KvStore {
                file,
                path,
                cache: HashMap::new(),
            });
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?;

        let mut cache = HashMap::new();
        load(&file, &mut cache);
        Ok(KvStore { path, file, cache })
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    ///
    /// # Errors
    ///
    /// It propagates I/O or serialization errors during writing the log.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        panic!("Not implemented");
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::UnexpectedCommandType` if the given command type unexpected.
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        panic!("Not implmented");
    }

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the log.
    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!("Not implemented");
    }

    /// Clears stale entries in the log.
    pub fn compact(&mut self) -> Result<()> {
        panic!("Not implemented");
    }
}

fn load(file: &File, data: &mut HashMap<String, String>) -> Result<()> {
    let lines = BufReader::new(file).lines();
    for line in lines.map_while(std::io::Result::ok) {
        let cmd: Command = serde_json::from_str(&line)?;
        match cmd {
            Command::Set { key, value } => {
                data.insert(key, value);
            }
            Command::Remove { key } => {
                data.remove(&key);
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set {
            key: key,
            value: value,
        }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key: key }
    }
}

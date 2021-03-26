use crate::Result;

/// Abstraction over the storage operations that the key-value store is able to do.
///
/// The storage trait contains the methods necessary for representing a Storage engine
/// while abstract enough to not bind it to a specific implementation.
// TODO: Maybe should add iterator implementation? Maybe it does not matter for the demo...?
pub trait Storage {
    /// Opens the underlying storage, and initializes necessary datastructures.
    ///
    /// This method is intended to be the first call after the creating an instance that implements
    /// the Storage trait.
    fn open(&mut self, dir: String, options: StorageOptions) -> Result<()>;

    /// Assings the value to the `key` string.
    /// Assinging a key that exists already will override the existing value.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Get the value identified by the given key.
    fn get(&self, key: &String) -> Result<Option<&String>>;

    /// Unset the value assigned to the given key and return it.
    /// A `None` option is returned if no value is assigned to the given key.
    fn unset(&mut self, key: &String) -> Result<Option<String>>;

    /// Flushes any pending writes and cleans up the internal datastructures.
    fn close(self) -> Result<()>;
}

/// Groups all the options to be used by the storage engine to tweak the way it will use and create
/// it's underlying datastructures.
pub struct StorageOptions {}

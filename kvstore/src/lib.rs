pub type Err = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Err>;

mod handler;
pub use handler::ConnectionHandler;

pub mod protocol;
pub use protocol::{Command, Parser, Response, Writer};

pub mod client;
pub use client::{create, Client};

pub mod server;

pub mod storage;
pub use storage::{Storage, StorageOptions};

mod executor;

pub const DEFAULT_PORT: &str = "6555";

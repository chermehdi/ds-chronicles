use crate::handler::ConnectionHandler;
use crate::Command;
use crate::Storage;
use crate::{Response, Result};
use std::sync::{Arc, Mutex};

/// StorageEngine is a type alias to help reduce the verbosity of the storage interface type.
type StorageEngine = Arc<Mutex<Box<dyn Storage + Send + Sync>>>;

/// Executor is created per client, and it will handle the flow of oupdating the underlying storage
/// for the whole duration of lifetime of the client.
///
/// The executor is ran in a continuous loop in the `run` method, until the client's connection is
/// closed, or an unexpected command is sent over the wire.
///
/// The executor's loop is blocking, and it does not spawn any tokio tasks, it's the caller's
/// responsability to manage and create tasks as needed.
pub(crate) struct Executor {
    /// The handler represents the client's connection that this executor will manage.
    handler: ConnectionHandler,

    /// The shared storage between all of the executor instances and it's safe to access from
    /// multiple threads.
    store: Arc<Mutex<Box<dyn Storage + Send + Sync>>>,
}

/// Execute comand dispatches the correct method to execute the command
/// And writes the resulting `Response` to the handler's output.
async fn execute_cmd(
    store: &mut StorageEngine,
    handler: &mut ConnectionHandler,
    cmd: Command,
) -> Result<()> {
    let result = match cmd {
        Command::Ping(key) => handle_ping(key),
        Command::Set(key, value) => handle_set(store, key, value),
        Command::Get(key) => handle_get(store, key),
        Command::Clear(key) => handle_unset(store, key),
    };
    handler.write_response(&result).await?;
    Ok(())
}

fn handle_ping(key: String) -> Response {
    if key.is_empty() {
        // Default to a ping.
        return Response::Ok("PONG".into());
    } else {
        return Response::Ok(key);
    }
}

fn handle_set(store: &mut StorageEngine, key: String, value: String) -> Response {
    let mut guard = store.lock().unwrap();
    return match guard.set(key.clone(), value) {
        Ok(_) => Response::Ok(key),
        Err(_) => Response::Error(String::from("Error happened while setting the key")),
    };
}

fn handle_get(store: &mut StorageEngine, key: String) -> Response {
    let guard = store.lock().unwrap();
    match guard.get(&key) {
        Ok(Some(val)) => {
            return Response::Ok(val.clone());
        }
        _ => {
            return Response::Error(String::from("Key not found"));
        }
    }
}

fn handle_unset(store: &mut StorageEngine, key: String) -> Response {
    let mut guard = store.lock().unwrap();
    match guard.unset(&key) {
        Ok(Some(value)) => {
            return Response::Ok(value);
        }
        _ => {
            return Response::Ok(String::new());
        }
    }
}

impl Executor {
    pub(crate) fn new(handler: ConnectionHandler, store: StorageEngine) -> Self {
        return Executor { handler, store };
    }

    pub(crate) async fn run(&mut self) -> Result<()> {
        loop {
            let cmd = match self.handler.read_command().await {
                Ok(val) => val,
                Err(_msg) => None,
            };
            if let Some(cmd) = cmd {
                execute_cmd(&mut self.store, &mut self.handler, cmd).await?;
            } else {
                // Either there was an error trying to parse a command from the input stream,
                // Or the client closed the socket. eitherway we won't be processing further
                // requests for simplicity.
                return Err("Connection closed / Poisened message".into());
            }
        }
    }
}

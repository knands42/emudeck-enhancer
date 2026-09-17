mod listen;
pub use listen::Listener;

#[derive(Debug)]
pub enum ListenerError {
    Io(std::io::Error),
    Notify(notify::Error),
}

impl From<notify::Error> for ListenerError {
    fn from(value: notify::Error) -> Self {
        ListenerError::Notify(value)
    }
}

impl From<std::io::Error> for ListenerError {
    fn from(value: std::io::Error) -> Self {
        ListenerError::Io(value)
    }
}

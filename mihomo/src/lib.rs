use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub use mihomo_sys;

static STARTED: AtomicBool = AtomicBool::new(false);
static LIFECYCLE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

/// Safe wrapper for the in-process mihomo kernel.
///
/// Note: mihomo core is effectively process-global; this wrapper enforces a single
/// start/stop lifecycle per process.
pub struct Mihomo {
    _private: (),
}

impl Mihomo {
    pub fn start(config_yaml: &str) -> Result<Self, Error> {
        let _guard = LIFECYCLE_LOCK
            .lock()
            .expect("Mihomo lifecycle lock poisoned");

        if STARTED.swap(true, Ordering::SeqCst) {
            return Err(Error::new("mihomo already started in this process"));
        }

        let code = unsafe { mihomo_sys::start(config_yaml.as_bytes()) };
        if code != 0 {
            STARTED.store(false, Ordering::SeqCst);
            return Err(Error::new(mihomo_sys::last_error()));
        }

        Ok(Self { _private: () })
    }
}

impl Drop for Mihomo {
    fn drop(&mut self) {
        let _guard = LIFECYCLE_LOCK
            .lock()
            .expect("Mihomo lifecycle lock poisoned");

        if !STARTED.swap(false, Ordering::SeqCst) {
            return;
        }
        let _ = unsafe { mihomo_sys::stop() };
    }
}

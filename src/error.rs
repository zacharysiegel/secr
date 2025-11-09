use chacha20poly1305::aead;
use std::backtrace::{Backtrace, BacktraceStatus};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{env, fmt, io, string};

macro_rules! impl_from_error {
    ($error_type:ty) => {
        impl From<$error_type> for $crate::error::AppError {
            fn from(value: $error_type) -> Self {
                Self::from_error_default(::std::boxed::Box::new(value))
            }
        }

        impl From<$error_type> for $crate::error::AppErrorStatic {
            fn from(value: $error_type) -> Self {
                Self::new(&value.to_string())
            }
        }
    };
}

/// Should be initialized lazily (e.g. [Option::ok_or_else]) for captured backtraces to make sense.
pub struct AppError {
    pub message: String,
    pub sub_error: Option<Box<dyn Error>>,
    pub backtrace: Backtrace,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AppError [{}]", self.message)?;
        if let Some(sub_error) = &self.sub_error {
            write!(f, "\n[{}]", sub_error)?;
        }
        match self.backtrace.status() {
            BacktraceStatus::Unsupported | BacktraceStatus::Disabled => Ok(()),
            BacktraceStatus::Captured => write!(f, "\n{}", self.backtrace),
            _ => Ok(()),
        }
    }
}

// When main exits with Result::Err, it prints the Debug formatting of the Error
impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for AppError {}

impl Default for AppError {
    fn default() -> Self {
        Self::new(Self::DEFAULT_MESSAGE)
    }
}

impl From<AppErrorStatic> for AppError {
    fn from(value: AppErrorStatic) -> Self {
        AppError {
            message: value.message,
            sub_error: None,
            backtrace: value.backtrace,
        }
    }
}

impl AppError {
    const DEFAULT_MESSAGE: &'static str = "unspecified";

    pub fn new(message: &str) -> AppError {
        Self::_new(message, None)
    }

    pub fn from_error(message: &str, error: Box<dyn Error>) -> AppError {
        Self::_new(message, Some(error))
    }

    pub fn from_error_default(error: Box<dyn Error>) -> AppError {
        Self::_new(&error.to_string(), Some(error))
    }

    fn _new(message: &str, error: Option<Box<dyn Error>>) -> AppError {
        let backtrace: Backtrace = Backtrace::force_capture();
        let app_error = AppError {
            message: format!("Error: {}", message),
            sub_error: error,
            backtrace,
        };
        app_error
    }
}

/// Like [AppError], but cannot include a sub error (in order to be dyn-compatible)
/// Should be initialized lazily (e.g. [Option::ok_or_else]) for captured backtraces to make sense.
pub struct AppErrorStatic {
    pub message: String,
    pub backtrace: Backtrace,
}

impl Display for AppErrorStatic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AppError [{}]", self.message)?;
        match self.backtrace.status() {
            BacktraceStatus::Unsupported | BacktraceStatus::Disabled => Ok(()),
            BacktraceStatus::Captured => write!(f, "\n{}", self.backtrace),
            _ => Ok(()),
        }
    }
}

// When main exits with Result::Err, it prints the Debug formatting of the Error
impl Debug for AppErrorStatic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for AppErrorStatic {}

impl Default for AppErrorStatic {
    fn default() -> Self {
        Self::new(Self::DEFAULT_MESSAGE)
    }
}

impl From<AppError> for AppErrorStatic {
    fn from(value: AppError) -> Self {
        AppErrorStatic {
            message: value.message,
            backtrace: value.backtrace,
        }
    }
}

impl AppErrorStatic {
    const DEFAULT_MESSAGE: &'static str = "unspecified";

    pub fn new(message: &str) -> AppErrorStatic {
        let backtrace: Backtrace = Backtrace::force_capture();
        let app_error = AppErrorStatic {
            message: format!("Error: {}", message),
            backtrace,
        };
        app_error
    }
}

impl_from_error!(env::VarError);
impl_from_error!(base64::DecodeError);
impl_from_error!(string::FromUtf8Error);
impl_from_error!(aead::Error);
impl_from_error!(io::Error);
impl_from_error!(serde_yaml_ng::Error);

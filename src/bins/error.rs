use config;
use std::io;

error_chain! {
  // The type defined for this error. These are the conventional
  // and recommended names, but they can be arbitrarily chosen.
  types {
    Error, ErrorKind, ChainErr, Result;
  }

  // Automatic conversions between this error chain and other
  // error chains. In this case, it will e.g. generate an
  // `ErrorKind` variant called `Dist` which in turn contains
  // the `rustup_dist::ErrorKind`, with conversions from
  // `rustup_dist::Error`.
  //
  // This section can be empty.
  links {
  }

  // Automatic conversions between this error chain and other
  // error types not defined by the `error_chain!`. These will be
  // boxed as the error cause and wrapped in a new error with,
  // in this case, the `ErrorKind::Temp` variant.
  //
  // This section can be empty.
  foreign_links {
    config::error::ConfigError, ConfigError, "configuration error";
    io::Error, IoError, "I/O error";
  }

  // Define additional `ErrorKind` variants. The syntax here is
  // the same as `quick_error!`, but the `from()` and `cause()`
  // syntax is not supported.
  errors {
  }
}

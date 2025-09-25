//! Logging utilities.

mod formatter;

pub(crate) use formatter::Formatter;

macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "tracing")]
        tracing::trace!(target: $target, $($arg)*);

        #[cfg(feature = "log")]
        log::trace!(target: $target, $($arg)*);

        #[cfg(feature = "defmt")]
        {
            _ = $target;
            defmt::trace!($($arg)*);
        }

    };
}

macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: $target, $($arg)*);

        #[cfg(feature = "log")]
        log::debug!(target: $target, $($arg)*);

        #[cfg(feature = "defmt")]
        {
            _ = $target;
            defmt::debug!($($arg)*);
        }
    };
}

macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "tracing")]
        tracing::error!(target: $target, $($arg)*);

        #[cfg(feature = "log")]
        log::error!(target: $target, $($arg)*);

        #[cfg(feature = "defmt")]
        {
            _ = $target;
            defmt::error!($($arg)*);
        }
    };
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use trace;

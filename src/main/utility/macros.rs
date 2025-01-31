/// Log a warning, and if a debug build then panic.
macro_rules! debug_panic {
    ($($x:tt)+) => {
        log::warn!($($x)+);
        #[cfg(debug_assertions)]
        panic!($($x)+);
    };
}

/// Log a message once at level `lvl_once`, and any later log messages from this line at level
/// `lvl_remaining`. A log target is not supported. It is recommended to also prepend "(LOG_ONCE)"
/// to the log message to indicate that it will not be logged again at that level, but will be
/// logged at a different level.
///
/// ```
/// # use log::Level;
/// # use shadow_rs::log_once_at_level;
/// log_once_at_level!(Level::Warn, Level::Debug, "(LOG_ONCE) Unexpected flag {}", 10);
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! log_once_at_level {
    ($lvl_once:expr, $lvl_remaining:expr, $($x:tt)+) => {
        // don't do atomic operations if this log statement isn't enabled
        if log::log_enabled!($lvl_once) || log::log_enabled!($lvl_remaining) {
            static HAS_LOGGED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);

            // TODO: doing just a `load()` might be faster in the typical case, but would need to
            // have performance metrics to back that up
            match HAS_LOGGED.compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => log::log!($lvl_once, $($x)+),
                Err(_) => log::log!($lvl_remaining, $($x)+),
            }
        }
    };
}

/// Log a message once at warn level, and any later log messages from this line at debug level. A
/// log target is not supported. It is recommended to also prepend "(LOG_ONCE)" to the log message
/// to indicate that it will not be logged again at that level, but will be logged at a different
/// level.
///
/// ```ignore
/// warn_once_then_debug!("(LOG_ONCE) Unexpected flag {}", 10);
/// ```
#[allow(unused_macros)]
macro_rules! warn_once_then_debug {
    ($($x:tt)+) => {
        log_once_at_level!(log::Level::Warn, log::Level::Debug, $($x)+);
    };
}

/// Log a message once at warn level, and any later log messages from this line at trace level. A
/// log target is not supported. It is recommended to also prepend "(LOG_ONCE)" to the log message
/// to indicate that it will not be logged again at that level, but will be logged at a different
/// level.
///
/// ```ignore
/// warn_once_then_trace!("(LOG_ONCE) Unexpected flag {}", 10);
/// ```
#[allow(unused_macros)]
macro_rules! warn_once_then_trace {
    ($($x:tt)+) => {
        log_once_at_level!(log::Level::Warn, log::Level::Trace, $($x)+);
    };
}

#[cfg(test)]
mod tests {
    // will panic in debug mode
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn debug_panic_macro() {
        debug_panic!("Hello {}", "World");
    }

    // will *not* panic in release mode
    #[test]
    #[cfg(not(debug_assertions))]
    fn debug_panic_macro() {
        debug_panic!("Hello {}", "World");
    }

    #[test]
    fn log_once_at_level() {
        // we don't have a logger set up so we can't actually inspect the log output (well we
        // probably could with a custom logger), so instead we just make sure it compiles
        for x in 0..10 {
            log_once_at_level!(log::Level::Warn, log::Level::Debug, "{x}");
        }

        log_once_at_level!(log::Level::Warn, log::Level::Debug, "A");
        log_once_at_level!(log::Level::Warn, log::Level::Debug, "A");

        // expected log output is:
        // Warn: 0
        // Debug: 1
        // Debug: 2
        // ...
        // Warn: A
        // Warn: A
    }

    #[test]
    fn warn_once() {
        warn_once_then_trace!("A");
        warn_once_then_debug!("A");
    }
}

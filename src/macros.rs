/// Our custom `try!` macro aka `?`, to get rid of [`std::convert::From`]/[`std::convert::Into`] used by the `?` operator.
#[macro_export]
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => value,
            core::result::Result::Err(err) => {
                return core::result::Result::Err(err);
            }
        }
    };
}

#[macro_export]
macro_rules! tri_decode {
    ($e:expr, $s:ident, $f:ident $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(target: "rusmpp::decode", Struct=stringify!($s), Field=stringify!($f), ?value);

                value
            }
            core::result::Result::Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!(target: "rusmpp::decode", Struct=stringify!($s), Field=stringify!($f), ?err);

                return core::result::Result::Err(err);
            }
        }
    };
}

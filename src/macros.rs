/// Our custom `try!` macro aka `?`, to get rid of [`std::convert::From`]/[`std::convert::Into`] used by the `?` operator
#[macro_export]
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

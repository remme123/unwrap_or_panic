//! Unwrap a `Result<T, E>` without the trait bound `E: Debug` for the Error.
//!
//! Unlike [`unwrap`](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap),
//! this does not format the error with `fmt::Debug`.
//!
//! # Usage
//! ```should_panic
//! use unwrap_or_panic::UnwrapOrPanic;
//!
//! Err::<i32, i32>(-1).unwrap_or_panic();    // panic with msg `Panic at <FILE>:<LINE>:<COLUME>`
//! ```

#![cfg_attr(not(test), no_std)]

pub trait UnwrapOrPanic<T> {
    /// Return the contained value or panic
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn unwrap_or_panic(self) -> T;
}

impl<T, E> UnwrapOrPanic<T> for Result<T, E> {
    #[cfg_attr(feature = "track_caller", inline(never))]
    fn unwrap_or_panic(self) -> T {
        if let Ok(x) = self {
            x
        } else {
            #[cfg(not(feature = "panic_location"))]
            {
                panic!();
            }
            #[cfg(feature = "panic_location")]
            {
                let caller = core::panic::Location::caller();
                panic!("paniced at {}:{}:{}", caller.file(), caller.line(), caller.column());
            }
        }
    }
}

impl<T> UnwrapOrPanic<T> for Option<T> {
    #[cfg_attr(feature = "track_caller", inline(never))]
    fn unwrap_or_panic(self) -> T {
        if let Some(x) = self {
            x
        } else {
            #[cfg(not(feature = "panic_location"))]
            {
                panic!();
            }
            #[cfg(feature = "panic_location")]
            {
                let caller = core::panic::Location::caller();
                panic!("paniced at {}:{}:{}", caller.file(), caller.line(), caller.column());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwrap_ok() {
        assert_eq!(Ok::<i32, i32>(1).unwrap_or_panic(), 1);
    }

    #[test]
    #[should_panic]
    fn unwrap_err() {
        Err::<i32, i32>(-1).unwrap_or_panic();
    }
}

#![cfg_attr(not(any(feature = "std", test)), no_std)]

pub trait UnwrapOrPanic<T> {
    #[track_caller]
    fn unwrap_or_panic(self) -> T;
}

impl<T, E> UnwrapOrPanic<T> for Result<T, E> {
    #[inline(never)]
    fn unwrap_or_panic(self) -> T {
        if let Ok(x) = self {
            x
        } else {
            // let caller = core::panic::Location::caller();
            // let tx = unsafe {DEBUG_SERIAL_TX.get_mut().unwrap_or_panic()};
            // uwriteln!(tx, "Panic @ {}:{}:{}", caller.file(), caller.line(), caller.column()).ok();
            panic!();
        }
    }
}

impl<T> UnwrapOrPanic<T> for Option<T> {
    #[inline(never)]
    fn unwrap_or_panic(self) -> T {
        if let Some(x) = self {
            x
        } else {
            // let caller = core::panic::Location::caller();
            // let tx = unsafe {DEBUG_SERIAL_TX.get_mut().unwrap_or_panic()};
            // uwriteln!(tx, "Panic @ {}:{}:{}", caller.file(), caller.line(), caller.column()).ok();
            panic!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}

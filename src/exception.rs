//! Exceptions

use ctxt::Context;

/// The default exception handler
///
pub extern "msp430-interrupt" fn default_handler<T>(_token: T)
where
    T: Context,
{
    match () {
        #[cfg(target_arch = "msp430")]
        () => {
            ::interrupt::disable();
            loop {}
        }
        #[cfg(not(target_arch = "msp430"))]
        () => {}
    }
}

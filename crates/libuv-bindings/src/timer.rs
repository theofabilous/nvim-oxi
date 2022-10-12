use std::error::Error as StdError;
use std::time::Duration;

use libuv_sys2::{self as ffi, uv_timer_t};

use crate::{Error, Handle};

pub(crate) type Callback = Box<
    dyn FnMut(&mut TimerHandle) -> Result<(), Box<dyn StdError>> + 'static,
>;

pub struct TimerHandle {
    handle: Handle<uv_timer_t, Callback>,
}

impl TimerHandle {
    /// TODO: docs
    fn new() -> Result<Self, crate::Error> {
        let handle = Handle::new(|uv_loop, handle| unsafe {
            ffi::uv_timer_init(uv_loop, handle.as_mut_ptr())
        })?;

        Ok(Self { handle })
    }

    /// TODO: docs
    pub fn start<E, Cb>(
        timeout: Duration,
        repeat: Duration,
        mut callback: Cb,
    ) -> Result<Self, Error>
    where
        E: StdError + 'static,
        Cb: FnMut(&mut Self) -> Result<(), E> + 'static,
    {
        let mut timer = Self::new()?;

        let callback: Callback = Box::new(move |timer| {
            // Type erase the callback by boxing its error.
            callback(timer).map_err(|err| Box::new(err) as Box<dyn StdError>)
        });

        unsafe { timer.handle.set_data(callback) };

        let retv = unsafe {
            ffi::uv_timer_start(
                timer.handle.as_mut_ptr(),
                Some(timer_cb as _),
                timeout.as_millis() as u64,
                repeat.as_millis() as u64,
            )
        };

        if retv < 0 {
            return Err(crate::Error::TimerStart);
        }

        Ok(timer)
    }

    /// TODO: docs
    pub fn once<E, Cb>(
        timeout: Duration,
        callback: Cb,
    ) -> Result<Self, crate::Error>
    where
        E: StdError + 'static,
        Cb: FnOnce() -> Result<(), E> + 'static,
    {
        let mut callback = Some(callback);

        Self::start(timeout, Duration::from_millis(0), move |timer| {
            let res = callback.take().unwrap()();
            timer.stop().unwrap();
            res
        })
    }

    /// TODO: docs
    pub fn stop(&mut self) -> Result<(), Error> {
        let retv = unsafe { ffi::uv_timer_stop(self.handle.as_mut_ptr()) };

        if retv < 0 {
            return Err(Error::TimerStop);
        }

        Ok(())
    }
}

extern "C" fn timer_cb(ptr: *mut uv_timer_t) {
    let handle: Handle<_, Callback> = unsafe { Handle::from_raw(ptr) };

    let callback = unsafe { handle.get_data() };

    if !callback.is_null() {
        let mut handle = TimerHandle { handle };
        let callback = unsafe { &mut *callback };

        if let Err(_err) = callback(&mut handle) {
            // TODO: what now?
        }
    }
}

#![no_std]

pub mod error;
pub mod format;
pub mod params;
pub mod raw;

pub use error::Error;

use core::ffi::c_uint;
use core::{marker::PhantomData, ptr::null};
use core::ptr::null_mut;

use libsoxr_sys as sys;

use format::IoFormat;
use params::{QualitySpec, RuntimeSpec};
use raw::SoxrPtr;

pub type ChannelCount = usize;

pub struct Soxr<Format: IoFormat> {
    soxr: SoxrPtr,
    _phantom: PhantomData<Format>,
}

impl<Format: IoFormat> Soxr<Format> {
    /// Creates a new resampler instance using default values for quality
    /// and runtime parameters
    pub fn new(input_rate: f64, output_rate: f64) -> Result<Self, Error> {
        Self::new_with_params(
            input_rate,
            output_rate,
            QualitySpec::default(),
            RuntimeSpec::default(),
        )
    }

    /// Creates a new resampler instance with the specified quality and
    /// runtime parameters
    pub fn new_with_params(
        input_rate: f64,
        output_rate: f64,
        quality: QualitySpec,
        runtime: RuntimeSpec,
    ) -> Result<Self, Error> {
        let io = Format::io_spec(1.0);

        let channels = c_uint::try_from(Format::channels())
            .map_err(|_| error::CHANNEL_COUNT_TOO_LARGE)?;

        let soxr = unsafe {
            let mut error = null();

            let ptr = sys::soxr_create(
                input_rate,
                output_rate,
                channels,
                &mut error,
                &io,
                quality.as_raw(),
                runtime.as_raw(),
            );

            if ptr == null_mut() {
                return Err(Error::from_raw(error));
            }

            SoxrPtr::from_raw(ptr)
        };

        Ok(Soxr {
            soxr,
            _phantom: PhantomData,
        })
    }

    pub fn as_ptr(&self) -> sys::soxr_t {
        self.soxr.as_ptr()
    }

    /// Process audio through the sampler. Once finished, call `drain` until
    /// it returns `0``.
    pub fn process(&mut self, input: &Format::Buffer, output: &mut Format::Buffer)
        -> Result<Processed, Error>
    {
        let input_len = Format::frame_count(input);
        let output_len = Format::frame_count(output);

        let mut input_consumed = 0;
        let mut output_produced = 0;

        unsafe {
            let input_ptr = Format::buffer_ptr(input);
            let output_ptr = Format::buffer_mut_ptr(output);

            Error::check(sys::soxr_process(
                self.as_ptr(),
                input_ptr,
                input_len,
                &mut input_consumed,
                output_ptr,
                output_len,
                &mut output_produced,
            ))?;
        }

        Ok(Processed {
            input_frames: input_consumed,
            output_frames: output_produced,
        })
    }

    /// Indicate to the resampler that the input stream has finished, and
    /// read remaining buffered data out of resampler
    pub fn drain(&mut self, output: &mut Format::Buffer) -> Result<usize, Error> {
        let output_len = Format::frame_count(output);
        let mut output_produced = 0;

        unsafe {
            let output_ptr = Format::buffer_mut_ptr(output);

            Error::check(sys::soxr_process(
                self.as_ptr(),
                null(),
                0,
                null_mut(),
                output_ptr,
                output_len,
                &mut output_produced,
            ))?;
        }

        Ok(output_produced)
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        unsafe { Error::check(sys::soxr_clear(self.as_ptr())) }
    }

    /// Change the resampler's input and output sample rates, smoothly
    /// changing over `slew_len` frames. Set `slew_len` to 0 to change
    /// rates immediately.
    pub fn set_rates(&mut self, input_rate: f64, output_rate: f64, slew_len: usize)
        -> Result<(), Error>
    {
        self.set_io_ratio(input_rate / output_rate, slew_len)
    }

    /// Change the resampler's input/output sample ratio, smoothly changing
    /// over `slew_len` frames. Set `slew_len` to 0 to change rates
    /// immediately.
    pub fn set_io_ratio(&mut self, ratio: f64, slew_len: usize)
        -> Result<(), Error>
    {
        unsafe {
            Error::check(sys::soxr_set_io_ratio(
                self.as_ptr(),
                ratio,
                slew_len,
            ))
        }
    }
}

pub struct Processed {
    pub input_frames: usize,
    pub output_frames: usize,
}

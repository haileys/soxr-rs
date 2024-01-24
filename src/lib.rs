#![no_std]

pub mod error;
pub mod format;
pub mod params;
pub mod raw;

pub use error::Error;

use core::ffi::{c_uint, c_void};
use core::{marker::PhantomData, ptr::null};
use core::ptr::null_mut;

use libsoxr_sys as sys;

use format::{Sample, SampleFormat};
use params::{QualitySpec, RuntimeSpec};
use raw::SoxrPtr;

pub type ChannelCount = usize;

pub struct Soxr<In: Sample, Out: Sample, const N: ChannelCount> {
    soxr: SoxrPtr,
    _phantom: PhantomData<(In, Out)>,
}

impl<In: Sample, Out: Sample, const N: ChannelCount> Soxr<In, Out, N> {
    fn io_spec() -> sys::soxr_io_spec {
        sys::soxr_io_spec {
            itype: interleaved::<In>(),
            otype: interleaved::<Out>(),
            scale: 1.0,
            e: null_mut(),
            flags: 0,
        }
    }

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
        let io = Self::io_spec();
        let quality = quality.to_raw();
        let runtime = runtime.to_raw();
        let channels = c_uint::try_from(N)
            .map_err(|_| error::CHANNEL_COUNT_TOO_LARGE)?;

        let soxr = unsafe {
            let mut error = null();

            let ptr = sys::soxr_create(
                input_rate,
                output_rate,
                channels,
                &mut error,
                &io,
                &quality,
                &runtime,
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
    pub fn process(&mut self, input: &[[In; N]], output: &mut [[Out; N]])
        -> Result<Processed, Error>
    {
        // soxr API uses frame count, so take len from slices before casting
        // down to a flat slice:
        let input_len = input.len();
        let output_len = output.len();

        let input: &[In] = bytemuck::must_cast_slice(input);
        let output: &mut [Out] = bytemuck::must_cast_slice_mut(output);

        let mut input_consumed = 0;
        let mut output_produced = 0;

        unsafe {
            let input_ptr: *const c_void = input.as_ptr().cast();
            let output_ptr: *mut c_void = output.as_mut_ptr().cast();

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
    pub fn drain(&mut self, output: &mut [[Out; N]]) -> Result<usize, Error> {
        let output_len = output.len();
        let output: &mut [Out] = bytemuck::must_cast_slice_mut(output);
        let mut output_produced = 0;

        unsafe {
            let output_ptr: *mut c_void = output.as_mut_ptr().cast();

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
}

pub struct Processed {
    pub input_frames: usize,
    pub output_frames: usize,
}

fn interleaved<S: Sample>() -> sys::soxr_datatype_t {
    match S::FORMAT {
        SampleFormat::Int16 => sys::SOXR_INT16_I,
        SampleFormat::Int32 => sys::SOXR_INT32_I,
        SampleFormat::Float32 => sys::SOXR_FLOAT32_I,
        SampleFormat::Float64 => sys::SOXR_FLOAT64_I,
    }
}

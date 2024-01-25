use core::ffi::c_void;
use core::marker::PhantomData;

use bytemuck::Pod;
use libsoxr_sys as sys;

use crate::buffer::{PlanarBuf, PlanarMut};

pub enum SampleFormat {
    Int16,
    Int32,
    Float32,
    Float64,
}

pub unsafe trait Sample: Pod {
    const FORMAT: SampleFormat;
}

unsafe impl Sample for i16 {
    const FORMAT: SampleFormat = SampleFormat::Int16;
}

unsafe impl Sample for i32 {
    const FORMAT: SampleFormat = SampleFormat::Int32;
}

unsafe impl Sample for f32 {
    const FORMAT: SampleFormat = SampleFormat::Float32;
}

unsafe impl Sample for f64 {
    const FORMAT: SampleFormat = SampleFormat::Float64;
}

pub unsafe trait IoFormat {
    type Sample: Sample;
    type Input<'a>: ?Sized;
    type Output<'a>: ?Sized;

    fn channels() -> usize;
    fn datatype() -> sys::soxr_datatype_t;

    fn input_len<'a>(input: &Self::Input<'a>) -> usize;
    fn input_ptr<'a>(input: &Self::Input<'a>) -> *const c_void;

    fn output_len<'a>(output: &Self::Output<'a>) -> usize;
    fn output_ptr<'a>(output: &mut Self::Output<'a>) -> *mut c_void;
}

/// Mono audio samples
pub struct Mono<S: Sample>(PhantomData<S>);

unsafe impl<S: Sample> IoFormat for Mono<S> {
    type Sample = S;
    type Input<'a> = [S];
    type Output<'a> = [S];

    fn channels() -> usize { 1 }
    fn datatype() -> sys::soxr_datatype_t { interleaved::<S>() }

    fn input_len<'a>(input: &Self::Input<'a>) -> usize { input.len() }
    fn input_ptr<'a>(input: &Self::Input<'a>) -> *const c_void { input.as_ptr().cast() }

    fn output_len<'a>(output: &Self::Output<'a>) -> usize { output.len() }
    fn output_ptr<'a>(output: &mut Self::Output<'a>) -> *mut c_void { output.as_mut_ptr().cast() }
}

/// Stereo interleaved audio samples
pub struct Stereo<S: Sample>(PhantomData<S>);

unsafe impl<S: Sample> IoFormat for Stereo<S> {
    type Sample = S;
    type Input<'a> = [[S; 2]];
    type Output<'a> = [[S; 2]];

    fn channels() -> usize { 2 }
    fn datatype() -> sys::soxr_datatype_t { interleaved::<S>() }

    fn input_len<'a>(input: &Self::Input<'a>) -> usize { input.len() }
    fn input_ptr<'a>(input: &Self::Input<'a>) -> *const c_void { input.as_ptr().cast() }

    fn output_len<'a>(output: &Self::Output<'a>) -> usize { output.len() }
    fn output_ptr<'a>(output: &mut Self::Output<'a>) -> *mut c_void { output.as_mut_ptr().cast() }
}

/// N-channel interleaved audio samples
pub struct Interleaved<S: Sample, const CHANNELS: usize>(PhantomData<S>);

unsafe impl<S: Sample, const CHANNELS: usize> IoFormat for Interleaved<S, CHANNELS> {
    type Sample = S;
    type Input<'a> = [[S; CHANNELS]];
    type Output<'a> = [[S; CHANNELS]];

    fn channels() -> usize { CHANNELS }
    fn datatype() -> sys::soxr_datatype_t { interleaved::<S>() }

    fn input_len<'a>(input: &Self::Input<'a>) -> usize { input.len() }
    fn input_ptr<'a>(input: &Self::Input<'a>) -> *const c_void { input.as_ptr().cast() }

    fn output_len<'a>(output: &Self::Output<'a>) -> usize { output.len() }
    fn output_ptr<'a>(output: &mut Self::Output<'a>) -> *mut c_void { output.as_mut_ptr().cast() }
}

/// N-channel audio samples in planar buffers
pub struct Planar<S: Sample, const CHANNELS: usize>(PhantomData<S>);

unsafe impl<S: Sample, const CHANNELS: usize> IoFormat for Planar<S, CHANNELS> {
    type Sample = S;
    type Input<'a> = PlanarBuf<'a, S, CHANNELS>;
    type Output<'a> = PlanarMut<'a, S, CHANNELS>;

    fn channels() -> usize { CHANNELS }
    fn datatype() -> sys::soxr_datatype_t { planar::<S>() }

    fn input_len<'a>(input: &Self::Input<'a>) -> usize { input.frames() }
    fn input_ptr<'a>(input: &Self::Input<'a>) -> *const c_void { input.as_ptr() }

    fn output_len<'a>(output: &Self::Output<'a>) -> usize { output.frames() }
    fn output_ptr<'a>(output: &mut Self::Output<'a>) -> *mut c_void { output.as_ptr() }
}

fn interleaved<S: Sample>() -> sys::soxr_datatype_t {
    match S::FORMAT {
        SampleFormat::Int16 => sys::SOXR_INT16_I,
        SampleFormat::Int32 => sys::SOXR_INT32_I,
        SampleFormat::Float32 => sys::SOXR_FLOAT32_I,
        SampleFormat::Float64 => sys::SOXR_FLOAT64_I,
    }
}

fn planar<S: Sample>() -> sys::soxr_datatype_t {
    match S::FORMAT {
        SampleFormat::Int16 => sys::SOXR_INT16_S,
        SampleFormat::Int32 => sys::SOXR_INT32_S,
        SampleFormat::Float32 => sys::SOXR_FLOAT32_S,
        SampleFormat::Float64 => sys::SOXR_FLOAT64_S,
    }
}

use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::null_mut;

use bytemuck::Pod;
use libsoxr_sys as sys;

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
    const FORMAT: SampleFormat = SampleFormat::Int16;
}

unsafe impl Sample for f64 {
    const FORMAT: SampleFormat = SampleFormat::Int16;
}

pub unsafe trait IoFormat {
    type Sample: Sample;
    type Buffer: ?Sized;

    fn channels() -> usize;
    fn io_spec(scale: f64) -> sys::soxr_io_spec;

    fn frame_count(buffer: &Self::Buffer) -> usize;
    fn buffer_ptr(buffer: &Self::Buffer) -> *const c_void;
    fn buffer_mut_ptr(buffer: &mut Self::Buffer) -> *mut c_void;
}

/// Mono audio samples
pub struct Mono<S: Sample>(PhantomData<S>);

unsafe impl<S: Sample> IoFormat for Mono<S> {
    type Sample = S;
    type Buffer = [S];

    fn channels() -> usize { 1 }

    fn io_spec(scale: f64) -> sys::soxr_io_spec {
        sys::soxr_io_spec {
            itype: interleaved::<S>(),
            otype: interleaved::<S>(),
            scale,
            e: null_mut(),
            flags: 0,
        }
    }

    fn frame_count(buffer: &Self::Buffer) -> usize {
        buffer.len()
    }

    fn buffer_ptr(buffer: &Self::Buffer) -> *const c_void {
        buffer.as_ptr().cast()
    }

    fn buffer_mut_ptr(buffer: &mut Self::Buffer) -> *mut c_void {
        buffer.as_mut_ptr().cast()
    }
}

/// Stereo interleaved audio samples
pub struct Stereo<S: Sample>(PhantomData<S>);

unsafe impl<S: Sample> IoFormat for Stereo<S> {
    type Sample = S;
    type Buffer = [[S; 2]];

    fn channels() -> usize { 2 }

    fn io_spec(scale: f64) -> sys::soxr_io_spec {
        sys::soxr_io_spec {
            itype: interleaved::<S>(),
            otype: interleaved::<S>(),
            scale,
            e: null_mut(),
            flags: 0,
        }
    }

    fn frame_count(buffer: &Self::Buffer) -> usize {
        buffer.len()
    }

    fn buffer_ptr(buffer: &Self::Buffer) -> *const c_void {
        buffer.as_ptr().cast()
    }

    fn buffer_mut_ptr(buffer: &mut Self::Buffer) -> *mut c_void {
        buffer.as_mut_ptr().cast()
    }
}

/// N-channel interleaved audio samples
pub struct Interleaved<S: Sample, const CHANNELS: usize>(PhantomData<S>);

unsafe impl<S: Sample, const CHANNELS: usize> IoFormat for Interleaved<S, CHANNELS> {
    type Sample = S;
    type Buffer = [[S; CHANNELS]];

    fn channels() -> usize { CHANNELS }

    fn io_spec(scale: f64) -> sys::soxr_io_spec {
        sys::soxr_io_spec {
            itype: interleaved::<S>(),
            otype: interleaved::<S>(),
            scale,
            e: null_mut(),
            flags: 0,
        }
    }

    fn frame_count(buffer: &Self::Buffer) -> usize {
        buffer.len()
    }

    fn buffer_ptr(buffer: &Self::Buffer) -> *const c_void {
        buffer.as_ptr().cast()
    }

    fn buffer_mut_ptr(buffer: &mut Self::Buffer) -> *mut c_void {
        buffer.as_mut_ptr().cast()
    }
}

fn interleaved<S: Sample>() -> sys::soxr_datatype_t {
    match S::FORMAT {
        SampleFormat::Int16 => sys::SOXR_INT16_I,
        SampleFormat::Int32 => sys::SOXR_INT32_I,
        SampleFormat::Float32 => sys::SOXR_FLOAT32_I,
        SampleFormat::Float64 => sys::SOXR_FLOAT64_I,
    }
}

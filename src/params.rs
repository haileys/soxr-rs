use core::ptr::null_mut;
use core::ffi::{c_uint, c_ulong};

use libsoxr_sys as sys;

pub struct QualitySpec {
    /// Conversion precision (in bits); typically 20.0
    pub precision: f64,
    /// 0=minimum, ... 50=linear, ... 100=maximum; typically 50.0
    pub phase_response: f64,
    /// 0dB pt. bandwidth to preserve; nyquist=1; typically 0.913
    pub passband_end: f64,
    /// Aliasing/imaging control; > passband_end; typically 1.0
    pub stopband_begin: f64,
    /// SOXR_ROLLOFF_* options, typically `Rolloff::Small`
    pub rolloff: Rolloff,
    /// Increase `irrational' ratio accuracy.
    pub high_precision_clock: bool,
    /// Use D.P. calcs even if precision <= 20.
    pub double_precision: bool,
    /// Variable-rate resampling.
    pub variable_rate: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Rolloff {
    /// <= 0.01 dB
    Small = 0,
    /// <= 0.35 dB
    Medium = 1,
    /// For Chebyshev bandwidth
    None = 2,
}

impl QualitySpec {
    pub const fn to_raw(&self) -> sys::soxr_quality_spec {
        let mut flags = self.rolloff as c_ulong;

        if self.high_precision_clock {
            flags |= sys::SOXR_HI_PREC_CLOCK as c_ulong;
        }

        if self.double_precision {
            flags |= sys::SOXR_DOUBLE_PRECISION as c_ulong;
        }

        if self.variable_rate {
            flags |= sys::SOXR_VR as c_ulong;
        }

        sys::soxr_quality_spec {
            precision: self.precision,
            phase_response: self.phase_response,
            passband_end: self.passband_end,
            stopband_begin: self.stopband_begin,
            e: null_mut(),
            flags,
        }
    }
}

impl Default for QualitySpec {
    fn default() -> Self {
        // typical values from soxr.h
        QualitySpec {
            precision: 20.0,
            phase_response: 50.0,
            passband_end: 0.913,
            stopband_begin: 1.0,
            rolloff: Rolloff::Small,
            high_precision_clock: false,
            double_precision: false,
            variable_rate: false,
        }
    }
}

pub struct RuntimeSpec {
    /// For DFT efficiency. [8,15]; typically 10
    pub log2_min_dft_size: c_uint,
    /// For DFT efficiency. [8,20]; typically 17
    pub log2_large_dft_size: c_uint,
    /// For CoefficientInterpolation::Auto, typically 400
    pub coef_size_kbytes: c_uint,
    /// 0: per OMP_NUM_THREADS; 1: 1 thread; typically 1
    pub num_threads: c_uint,
    /// For `irrational' ratios only
    pub interpolation: CoefficientInterpolation,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CoefficientInterpolation {
    /// Auto select coef. interpolation.
    Auto = 0,
    /// Man. select: less CPU, more memory.
    Low = 2,
    /// Man. select: more CPU, less memory.
    High = 3,
}

impl RuntimeSpec {
    pub fn to_raw(&self) -> sys::soxr_runtime_spec {
        sys::soxr_runtime_spec {
            log2_min_dft_size: self.log2_min_dft_size,
            log2_large_dft_size: self.log2_large_dft_size,
            coef_size_kbytes: self.coef_size_kbytes,
            num_threads: self.num_threads,
            e: null_mut(),
            flags: self.interpolation as u8 as c_ulong,
        }
    }
}

impl Default for RuntimeSpec {
    fn default() -> Self {
        // typical values from soxr.h
        RuntimeSpec {
            log2_min_dft_size: 10,
            log2_large_dft_size: 17,
            coef_size_kbytes: 400,
            num_threads: 1,
            interpolation: CoefficientInterpolation::Auto,
        }
    }
}

use core::ffi::{c_uint, c_ulong};

use libsoxr_sys as sys;

#[derive(Debug, Clone)]
pub struct QualitySpec {
    raw: sys::soxr_quality_spec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QualityRecipe {
    /// `SOXR_QQ` - 'Quick' cubic interpolation.
    Quick = 0,
    /// `SOXR_LQ` - 'Low' 16-bit with larger rolloff.
    Low = 1,
    /// `SOXR_MQ` - 'Medium' 16-bit with medium rolloff.
    Medium = 2,
    /// `SOXR_16_BITQ`
    Bits16 = 3,
    Bits20 = 4,
    Bits24 = 5,
    Bits28 = 6,
    Bits32 = 7,
}

impl QualityRecipe {
    /// High quality. alias for `Bits20`
    pub const fn high() -> Self {
        QualityRecipe::Bits20
    }

    /// High quality. alias for `Bits28`
    pub const fn very_high() -> Self {
        QualityRecipe::Bits28
    }

    pub const fn to_raw(self) -> c_ulong {
        self as u8 as c_ulong
    }
}

impl Default for QualityRecipe {
    fn default() -> Self {
        QualityRecipe::high()
    }
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

impl Default for Rolloff {
    fn default() -> Self {
        Rolloff::Small
    }
}

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct QualityFlags: u8 {
        /// `SOXR_HI_PREC_CLOCK` - Increase `irrational' ratio accuracy.
        const HighPrecisionClock = 8;
        /// `SOXR_DOUBLE_PRECISION` - Use D.P. calcs even if precision <= 20.
        const DoublePrecision = 16;
        /// `SOXR_VR` - Variable-rate resampling.
        const VariableRate = 32;
    }
}

impl QualitySpec {
    /// Construct a new resampler with given [`QualityRecipe`]
    pub fn new(recipe: QualityRecipe) -> Self {
        Self::configure(recipe, Rolloff::default(), QualityFlags::default())
    }

    /// Construct a new variable rate resampler with given [`QualityRecipe`]
    pub fn variable_rate(recipe: QualityRecipe) -> Self {
        Self::configure(recipe, Rolloff::default(), QualityFlags::VariableRate)
    }

    /// Construct a new `QualitySpec` with all available configuration options
    pub fn configure(recipe: QualityRecipe, rolloff: Rolloff, flags: QualityFlags) -> Self {
        let flags = flags.bits();
        let flags = flags | rolloff as u8;
        let flags = flags as c_ulong;

        unsafe { Self::from_raw(sys::soxr_quality_spec(recipe.to_raw(), flags)) }
    }

    /// Conversion precision (in bits); typically 20.0
    pub fn precision(&self) -> f64 {
        self.raw.precision
    }

    /// Set conversion precision
    pub fn set_precision(&mut self, precision: f64) {
        self.raw.precision = precision;
    }

    /// Chainable convenience method to set conversion precision
    pub fn with_precision(mut self, precision: f64) -> Self {
        self.set_precision(precision);
        self
    }


    /// 0=minimum, ... 50=linear, ... 100=maximum; typically 50.0
    pub fn phase_response(&self) -> f64 {
        self.raw.phase_response
    }

    /// Set phase response
    pub fn set_phase_response(&mut self, phase_response: f64) {
        self.raw.phase_response = phase_response;
    }

    /// Chainable convenience method to set phase response
    pub fn with_phase_response(mut self, phase_response: f64) -> Self {
        self.set_phase_response(phase_response);
        self
    }


    /// 0dB pt. bandwidth to preserve; nyquist=1; typically 0.913
    pub fn passband_end(&self) -> f64 {
        self.raw.passband_end
    }

    /// Set passband end
    pub fn set_passband_end(&mut self, passband_end: f64) {
        self.raw.passband_end = passband_end;
    }

    /// Chainable convenience method to set passband end
    pub fn with_passband_end(mut self, passband_end: f64) -> Self {
        self.set_passband_end(passband_end);
        self
    }


    /// Aliasing/imaging control; > passband_end; typically 1.0
    pub fn stopband_begin(&self) -> f64 {
        self.raw.stopband_begin
    }

    /// Set stopband begin
    pub fn set_stopband_begin(&mut self, stopband_begin: f64) {
        self.raw.stopband_begin = stopband_begin;
    }

    /// Chainable convenience method to set stopband begin
    pub fn with_stopband_begin(mut self, stopband_begin: f64) -> Self {
        self.set_stopband_begin(stopband_begin);
        self
    }


    pub const fn as_raw(&self) -> &sys::soxr_quality_spec {
        &self.raw
    }

    pub const unsafe fn from_raw(raw: sys::soxr_quality_spec) -> Self {
        QualitySpec { raw }
    }
}

impl Default for QualitySpec {
    fn default() -> Self {
        QualitySpec::new(QualityRecipe::high())
    }
}

pub struct RuntimeSpec {
    raw: sys::soxr_runtime_spec,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Interpolation {
    /// Auto select coef. interpolation.
    Auto = 0,
    /// Man. select: less CPU, more memory.
    Low = 2,
    /// Man. select: more CPU, less memory.
    High = 3,
}

impl RuntimeSpec {
    /// Construct a new `RuntimeSpec` with the specified number of threads
    pub fn new(num_threads: c_uint) -> Self {
        unsafe { Self::from_raw(sys::soxr_runtime_spec(num_threads)) }
    }


    /// For DFT efficiency. [8,15]; typically 10
    pub fn log2_min_dft_size(&self) -> c_uint {
        self.raw.log2_min_dft_size
    }

    /// Set `log2_min_dft_size`
    pub fn set_log2_min_dft_size(&mut self, log2_min_dft_size: c_uint) {
        self.raw.log2_min_dft_size = log2_min_dft_size;
    }

    /// Chainable convenience method to set `log2_min_dft_size`
    pub fn with_log2_min_dft_size(mut self, log2_min_dft_size: c_uint) -> Self {
        self.set_log2_min_dft_size(log2_min_dft_size);
        self
    }


    /// For DFT efficiency. [8,20]; typically 17
    pub fn log2_large_dft_size(&self) -> c_uint {
        self.raw.log2_large_dft_size
    }

    /// Set `log2_large_dft_size`
    pub fn set_log2_large_dft_size(&mut self, log2_large_dft_size: c_uint) {
        self.raw.log2_large_dft_size = log2_large_dft_size;
    }

    /// Chainable convenience method to set `log2_large_dft_size`
    pub fn with_log2_large_dft_size(mut self, log2_large_dft_size: c_uint) -> Self {
        self.set_log2_large_dft_size(log2_large_dft_size);
        self
    }


    /// For `Interpolation::Auto`, typically 400
    pub fn coef_size_kbytes(&self) -> c_uint {
        self.raw.log2_large_dft_size
    }

    /// Set `coef_size_kbytes`
    pub fn set_coef_size_kbytes(&mut self, coef_size_kbytes: c_uint) {
        self.raw.coef_size_kbytes = coef_size_kbytes;
    }

    /// Chainable convenience method to set `coef_size_kbytes`
    pub fn with_coef_size_kbytes(mut self, coef_size_kbytes: c_uint) -> Self {
        self.set_coef_size_kbytes(coef_size_kbytes);
        self
    }


    /// 0: per OMP_NUM_THREADS; 1: 1 thread; typically 1
    pub fn num_threads(&self) -> c_uint {
        self.raw.num_threads
    }

    /// Set `num_threads`
    pub fn set_num_threads(&mut self, num_threads: c_uint) {
        self.raw.num_threads = num_threads;
    }

    /// Chainable convenience method to set `num_threads`
    pub fn with_num_threads(mut self, num_threads: c_uint) -> Self {
        self.set_num_threads(num_threads);
        self
    }


    /// For `irrational' ratios only
    pub fn interpolation(&self) -> Interpolation {
        let interp = self.raw.flags & 3;
        match interp {
            0 => Interpolation::Auto,
            2 => Interpolation::Low,
            3 => Interpolation::High,
            _ => {
                // 1 is unspecified... FIXME what to do here
                // for now just claim its Auto
                Interpolation::Auto
            }
        }
    }

    /// Set interpolation
    pub fn set_interpolation(&mut self, interpolation: Interpolation) {
        self.raw.flags &= !3;
        self.raw.flags |= interpolation as u8 as c_ulong;
    }

    /// Chainable convenience method to set interpolation
    pub fn with_interpolation(mut self, interpolation: Interpolation) -> Self {
        self.set_interpolation(interpolation);
        self
    }


    pub const fn as_raw(&self) -> &sys::soxr_runtime_spec {
        &self.raw
    }

    pub const unsafe fn from_raw(raw: sys::soxr_runtime_spec) -> Self {
        RuntimeSpec { raw }
    }
}

impl Default for RuntimeSpec {
    fn default() -> Self {
        Self::new(1)
    }
}

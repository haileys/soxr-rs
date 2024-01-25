use core::array;
use core::ffi::c_void;
use core::marker::PhantomData;

use crate::format::Sample;

pub struct PlanarBuf<'a, S: Sample, const CHANNELS: usize> {
    frames: usize,
    planes: [*const S; CHANNELS],
    _phantom: PhantomData<&'a [S]>,
}

impl<'a, S: Sample, const CHANNELS: usize> PlanarBuf<'a, S, CHANNELS> {
    /// Create new `PlanarBuf` from array of plane slices
    ///
    /// # Panics
    ///
    /// Panics if all plane slices are not of same length
    pub fn new(planes: [&'a [S]; CHANNELS]) -> Self {
        let frames = planes.get(0).unwrap_or(&[].as_slice()).len();

        let planes = array::from_fn(|index| {
            let plane = &planes[index];

            // validate plane length
            let length = plane.len();
            if length != frames {
                panic!("plane at index {index} of different length to previous planes: len={length}, expected={frames}");
            }

            plane.as_ptr()
        });

        PlanarBuf { frames, planes, _phantom: PhantomData }
    }

    pub unsafe fn new_unchecked(frames: usize, planes: [*const S; CHANNELS]) -> Self {
        PlanarBuf { frames, planes, _phantom: PhantomData }
    }

    pub fn frames(&self) -> usize {
        self.frames
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.planes.as_ptr().cast()
    }
}

pub struct PlanarMut<'a, S: Sample, const CHANNELS: usize> {
    frames: usize,
    planes: [*mut S; CHANNELS],
    _phantom: PhantomData<&'a mut [S]>,
}

impl<'a, S: Sample, const CHANNELS: usize> PlanarMut<'a, S, CHANNELS> {
    /// Create new `PlanarMut` from array of plane slices
    ///
    /// # Panics
    ///
    /// Panics if all plane slices are not of same length
    pub fn new(mut planes: [&'a mut [S]; CHANNELS]) -> Self {
        let frames = planes.get(0).unwrap_or(&[].as_mut_slice()).len();

        let planes = array::from_fn(|index| {
            let plane = &mut planes[index];

            // validate plane length
            let length = plane.len();
            if length != frames {
                panic!("plane at index {index} of different length to previous planes: len={length}, expected={frames}");
            }

            plane.as_mut_ptr()
        });

        PlanarMut { frames, planes, _phantom: PhantomData }
    }

    pub unsafe fn new_unchecked(frames: usize, planes: [*mut S; CHANNELS]) -> Self {
        PlanarMut { frames, planes, _phantom: PhantomData }
    }

    pub fn frames(&self) -> usize {
        self.frames
    }

    pub fn as_ptr(&mut self) -> *mut c_void {
        self.planes.as_mut_ptr().cast()
    }
}

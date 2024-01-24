use bytemuck::Pod;

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

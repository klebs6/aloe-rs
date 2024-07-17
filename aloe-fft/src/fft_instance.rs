crate::ix!();

pub trait FftEngineInterface {

    fn create(&self, order: i32) -> *mut dyn FftInstance;
}

pub trait FftInstance {

    fn perform(&self, 
        input:   *const Complex<f32>,
        output:  *mut Complex<f32>,
        inverse: bool);

    fn perform_real_only_forward_transform(&self, 
        _0: *mut f32,
        _1: bool);

    fn perform_real_only_inverse_transform(&self, _0: *mut f32);
}

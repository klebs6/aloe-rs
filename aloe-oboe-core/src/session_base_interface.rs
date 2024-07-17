crate::ix!();

pub trait OboeSessionBaseInterface {

    fn start(&mut self);

    fn stop(&mut self);

    fn get_output_latency_in_samples(&mut self) -> i32;

    fn get_input_latency_in_samples(&mut self) -> i32;
}


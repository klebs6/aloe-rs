crate::ix!();

pub trait GetLatencyInSamples<SampleType> {

    fn get_latency_in_samples(&self) -> SampleType;
}

pub trait ProcessSamplesUp<SampleType: FloatSample> {

    fn process_samples_up(&mut self, _0: &AudioBlock<SampleType>);
}

pub trait ProcessSamplesDown<SampleType: FloatSample> {

    fn process_samples_down(&mut self, _0: &mut AudioBlock<SampleType>);
}

pub trait InitProcessing {

    fn init_processing(&mut self, maximum_number_of_samples_before_oversampling: usize);
}

pub trait Reset {

    fn reset(&mut self);
}

pub trait OversamplingStageInterface<SampleType: FloatSample>
: GetLatencyInSamples<SampleType>
+ ProcessSamplesUp<SampleType>
+ ProcessSamplesDown<SampleType>
+ InitProcessing
+ Reset
{ }

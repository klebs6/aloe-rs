crate::ix!();

#[derive(Default)]
pub struct VstTempBuffers<FloatType> {

    channels:            HeapBlock<*mut FloatType>,

    /**
      | see note in processReplacing()
      |
      */
    temp_channels:       Vec<*mut FloatType>,

    process_temp_buffer: AudioBuffer<FloatType>,
}

impl<FloatType> Drop for VstTempBuffers<FloatType> {

    fn drop(&mut self) {
        todo!();
        /*
            release();
        */
    }
}

impl<FloatType> VstTempBuffers<FloatType> {

    pub fn release(&mut self)  {
        
        todo!();
        /*
            for (auto* c : tempChannels)
                    delete[] c;

                tempChannels.clear();
        */
    }
}

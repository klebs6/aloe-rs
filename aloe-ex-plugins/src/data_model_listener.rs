crate::ix!();

pub trait DataModelListener {

    fn sample_reader_changed(&mut self, _0: Arc<dyn AudioFormatReaderFactory>)  {
        
        todo!();
        /*
        
        */
    }

    fn centre_frequency_hz_changed(&mut self, _0: f64)  {
        
        todo!();
        /*
        
        */
    }

    fn loop_mode_changed(&mut self, _0: LoopMode)  {
        
        todo!();
        /*
        
        */
    }

    fn loop_points_seconds_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
        
        */
    }
}

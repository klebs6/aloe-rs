crate::ix!();

pub trait GetResponseCurve {

    fn get_response_curve(
        &self, 
        curve_type: AudioProcessorCurveDataType
    ) 
        -> AudioProcessorCurveData 
    {
        todo!();
        /*
            return {};
        */
    }
}

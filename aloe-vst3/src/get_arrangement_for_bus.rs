crate::ix!();

#[inline] pub fn get_arrangement_for_bus(
    processor: *mut dyn AudioProcessorInterface,
    is_input:  bool,
    bus_index: i32

) -> SpeakerArrangement {
    
    todo!();
        /*
            typename VstSpeakerArrangement arrangement = typename VstSpeakerArr::kEmpty;

        if (processor != nullptr)
            processor->getBusArrangement (isInput ? typename VstkInput : typename VstkOutput,
                                          (typename i32) busIndex, arrangement);

        return arrangement;
        */
}

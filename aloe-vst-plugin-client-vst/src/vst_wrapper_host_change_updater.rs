crate::ix!();

pub struct AloeVSTWrapperHostChangeUpdater<'a> {
    base:          AsyncUpdater<'a>,
    owner:         &'a mut AloeVSTWrapper<'a>,
    callback_bits: Atomic<i32>, // default = { 0  }
}

pub const ALOE_VST_WRAPPER_HOST_CHANGE_UPDATER_AUDIO_MASTER_UPDATE_DISPLAY_BIT: usize = 1 << 0;
pub const ALOE_VST_WRAPPER_HOST_CHANGE_UPDATER_AUDIO_MASTER_IO_CHANGED_BIT:     usize = 1 << 1;

impl<'a> Drop for AloeVSTWrapperHostChangeUpdater<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            cancelPendingUpdate();
        */
    }
}

impl<'a> AloeVSTWrapperHostChangeUpdater<'a> {

    pub fn new(o: &mut AloeVSTWrapper) -> Self {
    
        todo!();
        /*
        : owner(o),

        
        */
    }
    
    pub fn update(&mut self, details: &AudioProcessorChangeDetails)  {
        
        todo!();
        /*
            if (details.latencyChanged)
                {
                    owner.vstEffect.initialDelay = owner.processor->getLatencySamples();
                    callbackBits |= audioMasterIOChangedBit;
                }

                if (details.parameterInfoChanged || details.programChanged)
                    callbackBits |= audioMasterUpdateDisplayBit;

                triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            const auto callbacksToFire = callbackBits.exchange (0);

                if (auto* callback = owner.hostCallback)
                {
                    struct FlagPair
                    {
                        Vst2::AudioMasterOpcodesX opcode;
                        int bit;
                    };

                    constexpr FlagPair pairs[] { { Vst2::audioMasterUpdateDisplay, audioMasterUpdateDisplayBit },
                                                 { Vst2::audioMasterIOChanged,     audioMasterIOChangedBit } };

                    for (const auto& pair : pairs)
                        if ((callbacksToFire & pair.bit) != 0)
                            callback (&owner.vstEffect, pair.opcode, 0, 0, nullptr, 0);
                }
        */
    }
}

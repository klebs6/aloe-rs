crate::ix!();

impl AUInputElement {

    #[inline] pub fn pull_input_with_buffer_list(&mut self, 
        io_action_flags: &mut AudioUnitRenderActionFlags,
        in_time_stamp:   &AudioTimeStamp,
        in_element:      AudioUnitElement,
        n_frames:        u32,
        in_buffer_list:  *mut AudioBufferList) -> OSStatus {
        
        todo!();
        /*
            OSStatus theResult;

        if (HasConnection()) {
                // only support connections for V2 audio units
    #if !CA_USE_AUDIO_PLUGIN_ONLY
            if (mConnRenderProc != NULL)
                theResult = reinterpret_cast<AudioUnitRenderProc>(mConnRenderProc)(
                                mConnInstanceStorage, &ioActionFlags, &inTimeStamp, mConnection.sourceOutputNumber, nFrames, inBufferList);
            else
    #endif
                theResult = AudioUnitRender(
                                mConnection.sourceAudioUnit, &ioActionFlags, &inTimeStamp, mConnection.sourceOutputNumber, nFrames, inBufferList);
        } else {
            // kFromCallback:
                theResult = (mInputProc)(
                                mInputProcRefCon, &ioActionFlags, &inTimeStamp, inElement, nFrames, inBufferList);
        }

        if (mInputType == kNoInput) // defense: the guy upstream could have disconnected
                                    // it's a horrible thing to do, but may happen!
            return kAudioUnitErr_NoConnection;

        return theResult;
        */
    }
}


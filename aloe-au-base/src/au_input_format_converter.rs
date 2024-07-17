crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUInputFormatConverter.h]

// TODO
pub struct MissingLink {}
pub type FormatConverterClient = MissingLink;

/**
  | AUInputFormatConverter
  |
  | Subclass of FormatConverterClient that applies
  | a format conversion to an input of an
  | AudioUnit.
  */
pub struct AUInputFormatConverter {
    base:                   FormatConverterClient,
    host:                   *mut AUBase,
    host_bus:               i32,
    timestamp_generator:    AUTimestampGenerator,
    is_pcm_topcm:           bool,
    hassrc:                 bool,
    silent_output:          bool,
    previous_silent_frames: u32,
}

impl AUInputFormatConverter {
    
    pub fn new(
        hostau:    *mut AUBase,
        input_bus: i32) -> Self {
    
        todo!();
        /*


            :
            mHost(hostAU),
            mHostBus(inputBus),
            mPreviousSilentFrames(0x1000)

    #if DEBUG
            mTimestampGenerator.mVerbosity = 0;
            strcpy(mTimestampGenerator.mDebugName, "AUConverter");
    #endif
        */
    }

    /*
       need to subsequently call Initialize, with
       the desired formats
      */
    
    pub fn set_start_input_time_at_zero(&mut self, b: bool)  {
        
        todo!();
        /*
            mTimestampGenerator.SetStartInputAtZero(b);
        */
    }
    
    pub fn au_fill_complex_buffer(&mut self, 
        in_time_stamp:              &AudioTimeStamp,
        io_output_data_packet_size: &mut u32,
        out_output_data:            &mut AudioBufferList,
        out_packet_description:     *mut AudioStreamPacketDescription,
        out_silence:                &mut bool) -> OSStatus {
        
        todo!();
        /*
            mTimestampGenerator.AddOutputTime(inTimeStamp, ioOutputDataPacketSize, mOutputFormat.mSampleRate);
            mSilentOutput = true;
            OSStatus err = FillComplexBuffer(ioOutputDataPacketSize, outOutputData, outPacketDescription);
            if (mSilentOutput) {
                if (!mIsPCMToPCM || (mHasSRC && mPreviousSilentFrames < 32))
                    mSilentOutput = false;
                mPreviousSilentFrames += ioOutputDataPacketSize;
            } else
                mPreviousSilentFrames = 0;
            outSilence = mSilentOutput;
            return err;
        */
    }
}


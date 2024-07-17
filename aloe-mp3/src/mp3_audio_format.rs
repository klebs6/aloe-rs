#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_MP3AudioFormat.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_MP3AudioFormat.cpp]

/**
  | Software-based MP3 decoding format
  | (doesn't currently provide an encoder).
  | 
  | IMPORTANT DISCLAIMER: By choosing
  | to enable the ALOE_USE_MP3AUDIOFORMAT
  | flag and to compile the MP3 code into
  | your software, you do so AT YOUR OWN RISK!
  | By doing so, you are agreeing that Raw
  | Material Software Limited is in no way
  | responsible for any patent, copyright,
  | or other legal issues that you may suffer
  | as a result.
  | 
  | The code in aloe_MP3AudioFormat.cpp
  | is NOT guaranteed to be free from infringements
  | of 3rd-party intellectual property.
  | If you wish to use it, please seek your
  | own independent advice about the legality
  | of doing so. If you are not willing to
  | accept full responsibility for the
  | consequences of using this code, then
  | do not enable the ALOE_USE_MP3AUDIOFORMAT
  | setting.
  | 
  | @tags{Audio}
  |
  */
pub struct MP3AudioFormat {

    base: AudioFormat,
}

impl Default for MP3AudioFormat {

    fn default() -> Self {
    
        todo!();
        /*
        : audio_format(MP3Decoder::mp3FormatName, ".mp3"),
        */
    }
}

impl MP3AudioFormat {
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn can_do_stereo(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn can_do_mono(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_compressed(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_quality_options(&mut self) -> StringArray {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn create_reader_for(
        &mut self, 
        source_stream:                  *mut dyn Read,
        delete_stream_if_opening_fails: bool

    ) -> *mut AudioFormatReader {
        
        todo!();
        /*
            std::unique_ptr<MP3Decoder::MP3Reader> r (new MP3Decoder::MP3Reader (sourceStream));

        if (r->lengthInSamples > 0)
            return r.release();

        if (! deleteStreamIfOpeningFails)
            r->input = nullptr;

        return nullptr;
        */
    }
    
    pub fn create_writer_for(
        &mut self, 
        _0:                   *mut dyn Write,
        sample_rate_to_use:   f64,
        number_of_channels:   u32,
        bits_per_sample:      i32,
        metadata_values:      &StringPairArray,
        quality_option_index: i32

    ) -> *mut AudioFormatWriter<Box<dyn Write>> {
        
        todo!();
        /*
            jassertfalse; // not yet implemented!
        return nullptr;
        */
    }
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatReaderSource.h]

/**
  | A type of AudioSource that will read
  | from an AudioFormatReader.
  | 
  | @see PositionableAudioSource, AudioTransportSource,
  | BufferingAudioSource
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioFormatReaderSource<'a> {
    reader:        OptionalScopedPointer<AudioFormatReader<'a>>,
    next_play_pos: i64,
    looping:       bool,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatReaderSource.cpp]
impl<'a> AudioSource for AudioFormatReaderSource<'a> {}

impl<'a> PrepareToPlayAudioSource for AudioFormatReaderSource<'a> {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn prepare_to_play(
        &mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64

    ) {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> ReleaseResources for AudioFormatReaderSource<'a> {
    
    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> GetNextAudioBlock for AudioFormatReaderSource<'a> {
    
    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn get_next_audio_block(&mut self, info: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            if (info.numSamples > 0)
        {
            const int64 start = nextPlayPos;

            if (looping)
            {
                const int64 newStart = start % reader->lengthInSamples;
                const int64 newEnd = (start + info.numSamples) % reader->lengthInSamples;

                if (newEnd > newStart)
                {
                    reader->read (info.buffer, info.startSample,
                                  (int) (newEnd - newStart), newStart, true, true);
                }
                else
                {
                    const int endSamps = (int) (reader->lengthInSamples - newStart);

                    reader->read (info.buffer, info.startSample,
                                  endSamps, newStart, true, true);

                    reader->read (info.buffer, info.startSample + endSamps,
                                  (int) newEnd, 0, true, true);
                }

                nextPlayPos = newEnd;
            }
            else
            {
                reader->read (info.buffer, info.startSample,
                              info.numSamples, start, true, true);
                nextPlayPos += info.numSamples;
            }
        }
        */
    }
}

impl<'a> PositionableAudioSource for AudioFormatReaderSource<'a> {

    /**
      | Returns whether loop-mode is turned
      | on or not.
      |
      */
    fn is_looping(&self) -> bool {
        
        todo!();
        /*
            return looping;
        */
    }

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn get_total_length(&self) -> i64 {
        
        todo!();
        /*
            return reader->lengthInSamples;
        */
    }
    
    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn set_next_read_position(&mut self, new_position: i64)  {
        
        todo!();
        /*
            nextPlayPos = newPosition;
        */
    }
    
    /**
      | Toggles loop-mode.
      | 
      | If set to true, it will continuously
      | loop the input source. If false, it will
      | just emit silence after the source has
      | finished.
      | 
      | @see isLooping
      |
      */
    fn set_looping(&mut self, should_loop: bool)  {
        
        todo!();
        /*
            looping = shouldLoop;
        */
    }
    
    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn get_next_read_position(&self) -> i64 {
        
        todo!();
        /*
            return looping ? nextPlayPos % reader->lengthInSamples
                       : nextPlayPos;
        */
    }
}

impl<'a> AudioFormatReaderSource<'a> {

    /**
      | Returns the reader that's being used.
      |
      */
    pub fn get_audio_format_reader(&self) -> *mut AudioFormatReader<'a> {
        
        todo!();
        /*
            return reader;
        */
    }
    
    /**
      | Creates an AudioFormatReaderSource
      | for a given reader.
      | 
      | -----------
      | @param sourceReader
      | 
      | the reader to use as the data source -
      | this must not be null
      | ----------
      | @param deleteReaderWhenThisIsDeleted
      | 
      | if true, the reader passed-in will be
      | deleted when this object is deleted;
      | if false it will be left up to the caller
      | to manage its lifetime
      |
      */
    pub fn new(
        r:                                  *mut AudioFormatReader<'a>,
        delete_reader_when_this_is_deleted: bool

    ) -> Self {
    
        todo!();
        /*
        : reader(r, deleteReaderWhenThisIsDeleted),
        : next_play_pos(0),
        : looping(false),

            jassert (reader != nullptr);
        */
    }
}

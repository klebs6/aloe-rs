crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioThumbnailBase.h]

/**
  | Provides a base for classes that can
  | store and draw scaled views of an audio
  | waveform.
  | 
  | Typically, you'll want to use the derived
  | class AudioThumbnail, which provides
  | a concrete implementation.
  | 
  | @see AudioThumbnail, AudioThumbnailCache
  | 
  | @tags{Audio}
  |
  */
pub trait AudioThumbnailBase<'a, R: Read>
//: ChangeBroadcasterInterface 
: GetChangeBroadcaster<'a>
+ Clear
+ DrawChannel
+ DrawChannels
+ GetNumChannels
+ GetNumSamplesFinished
+ GetTotalLength
+ IncomingDataReceiver
+ IsFullyLoaded
+ LoadFrom
+ SaveTo
+ SetReader 
+ SetSource 
{ }

pub trait GetChangeBroadcaster<'a> {

    fn get_change_broadcaster(&'a self) -> &'a ChangeBroadcaster<'a>;
}

pub trait Clear {

    /**
      | Clears and resets the thumbnail.
      |
      */
    fn clear(&mut self);
}

pub trait SetSource {

    /**
      | Specifies the file or stream that contains
      | the audio file.
      | 
      | For a file, just call @code setSource
      | (new FileInputSource (file)) @endcode
      | 
      | You can pass a nullptr in here to clear
      | the thumbnail. The source that is passed
      | in will be deleted by this object when
      | it is no longer needed.
      | 
      | -----------
      | @return
      | 
      | true if the source could be opened as
      | a valid audio file, false if this failed
      | for some reason.
      |
      */
    fn set_source(&mut self, new_source: &mut dyn Read) -> bool;
}

pub trait SetReader {

    /**
      | Gives the thumbnail an AudioFormatReader
      | to use directly. This will start parsing
      | the audio in a background thread (unless
      | the hash code can be looked-up successfully
      | in the thumbnail cache). Note that the
      | reader object will be held by the thumbnail
      | and deleted later when no longer needed.
      | The thumbnail will actually keep hold
      | of this reader until you clear the thumbnail
      | or change the input source, so the file
      | will be held open for all this time. If
      | you don't want the thumbnail to keep
      | a file handle open continuously, you
      | should use the setSource() method instead,
      | which will only open the file when it
      | needs to.
      |
      */
    fn set_reader(
        &mut self, 
        new_reader: *mut AudioFormatReader,
        hash_code:  i64
    );
}

pub trait LoadFrom {

    /**
      | Reloads the low res thumbnail data from
      | an input stream.
      | 
      | This is not an audio file stream! It takes
      | a stream of thumbnail data that would
      | previously have been created by the
      | saveTo() method. @see saveTo
      |
      */
    fn load_from(&mut self, input: &mut dyn Read) -> bool;
}

pub trait SaveTo {

    /**
      | Saves the low res thumbnail data to an
      | output stream.
      | 
      | The data that is written can later be
      | reloaded using loadFrom(). @see loadFrom
      |
      */
    fn save_to(&self, output: &mut dyn Write);
}

pub trait GetNumChannels {

    /**
      | Returns the number of channels in the
      | file.
      |
      */
    fn get_num_channels(&self) -> i32;
}

pub trait GetTotalLength {

    /**
      | Returns the length of the audio file,
      | in seconds.
      |
      */
    fn get_total_length(&self) -> f64;
}

pub trait DrawChannel {

    /**
      | Draws the waveform for a channel.
      | 
      | The waveform will be drawn within the
      | specified rectangle, where startTime
      | and endTime specify the times within
      | the audio file that should be positioned
      | at the left and right edges of the rectangle.
      | 
      | The waveform will be scaled vertically
      | so that a full-volume sample will fill
      | the rectangle vertically, but you can
      | also specify an extra vertical scale
      | factor with the verticalZoomFactor
      | parameter.
      |
      */
    fn draw_channel(&mut self, 
            g:                    &mut Graphics,
            area:                 &Rectangle<i32>,
            start_time_seconds:   f64,
            end_time_seconds:     f64,
            channel_num:          i32,
            vertical_zoom_factor: f32);
}

pub trait DrawChannels {

    /**
      | Draws the waveforms for all channels
      | in the thumbnail.
      | 
      | This will call drawChannel() to render
      | each of the thumbnail's channels, stacked
      | above each other within the specified
      | area.
      | 
      | @see drawChannel
      |
      */
    fn draw_channels(
        &mut self, 
        g:                    &mut Graphics,
        area:                 &Rectangle<i32>,
        start_time_seconds:   f64,
        end_time_seconds:     f64,
        vertical_zoom_factor: f32
    );
}

pub trait IsFullyLoaded {

    /**
      | Returns true if the low res preview is
      | fully generated.
      |
      */
    fn is_fully_loaded(&self) -> bool;
}

pub trait GetNumSamplesFinished {

    /**
      | Returns the number of samples that have
      | been set in the thumbnail.
      |
      */
    fn get_num_samples_finished(&self) -> i64;
}

pub trait GetApproximatePeak {

    /**
      | Returns the highest level in the thumbnail.
      | 
      | Note that because the thumb only stores
      | low-resolution data, this isn't an
      | accurate representation of the highest
      | value, it's only a rough approximation.
      |
      */
    fn get_approximate_peak(&self) -> f32;
}

pub trait GetApproximateMinMax {

    /**
      | Reads the approximate min and max levels
      | from a section of the thumbnail.
      | 
      | The lowest and highest samples are returned
      | in minValue and maxValue, but obviously
      | because the thumb only stores low-resolution
      | data, these numbers will only be a rough
      | approximation of the true values.
      |
      */
    fn get_approximate_min_max(&self, 
            start_time:    f64,
            end_time:      f64,
            channel_index: i32,
            min_value:     &mut f32,
            max_value:     &mut f32);
}

pub trait GetHashCode {

    /**
      | Returns the hash code that was set by
      | setSource() or setReader().
      |
      */
    fn get_hash_code(&self) -> i64;
}

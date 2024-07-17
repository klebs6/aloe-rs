crate::ix!();

pub trait AudioFormatReaderInterface: 
ReadMaxLevels 
+ GetChannelLayout 
+ ReadSamples { }

pub trait CreateReaderFor {

    /**
      | Searches through the known formats
      | to try to create a suitable reader for
      | this file.
      | 
      | If none of the registered formats can
      | open the file, it'll return nullptr.
      | 
      | It's the caller's responsibility to
      | delete the reader that is returned.
      |
      */
    fn create_reader_for(&mut self, file: &File) -> Box<dyn AudioFormatReaderInterface>;

    /**
      | Searches through the known formats
      | to try to create a suitable reader for
      | this stream.
      | 
      | The stream object that is passed-in
      | will be deleted by this method or by the
      | reader that is returned, so the caller
      | should not keep any references to it.
      | 
      | The stream that is passed-in must be
      | capable of being repositioned so that
      | all the formats can have a go at opening
      | it.
      | 
      | If none of the registered formats can
      | open the stream, it'll return nullptr.
      | 
      | If it returns a reader, it's the caller's
      | responsibility to delete the reader.
      |
      */
    fn create_reader_for_file_stream(&mut self, audio_file_stream: Box<dyn Read>) -> Box<dyn AudioFormatReaderInterface>;
}

crate::ix!();

pub trait SampleToFilePos {

    /**
      | Converts a sample index to a byte position
      | in the file.
      |
      */
    fn sample_to_file_pos(&self, sample: i64) -> i64;
}

pub trait FilePosToSample {

    /**
      | Converts a byte position in the file
      | to a sample index.
      |
      */
    fn file_pos_to_sample(&self, file_pos: i64) -> i64;
}

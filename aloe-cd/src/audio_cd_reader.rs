crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/audio_cd/aloe_AudioCDReader.h]

/**
  | A type of AudioFormatReader that reads
  | from an audio CD.
  | 
  | One of these can be used to read a CD as
  | if it's one big audio stream. Use the
  | getPositionOfTrackStart() method
  | to find where the individual tracks
  | are within the stream.
  | 
  | @see AudioFormatReader
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
#[cfg(target_os="windows")]
#[cfg(feature = "cd-reader")]
pub struct AudioCDReader<'a> {

    base:                  AudioFormatReader,
    track_start_samples:   Vec<i32>,
    audio_tracks:          [bool; 100],
    handle:                *mut c_void,
    buffer:                MemoryBlock,
    indexing_enabled:      bool,
    last_index:            i32,
    first_frame_in_buffer: i32,
    samples_in_buffer:     i32,
    _p0:                   PhantomData<&'a i32>,
}

#[no_copy]
#[leak_detector]
#[cfg(target_os="macos")]
#[cfg(feature = "cd-reader")]
pub struct AudioCDReader<'a> {

    base:                 AudioFormatReader<'a>,
    track_start_samples:  Vec<i32>,
    volume_dir:           File,
    tracks:               Vec<File>,
    current_reader_track: i32,
    reader:               Box<AudioFormatReader<'a>>,
}

pub const AUDIO_CD_READER_FRAMES_PER_SECOND: f32 = 75.0;
pub const AUDIO_CD_READER_SAMPLES_PER_FRAME: f32 = 44100.0 / AUDIO_CD_READER_FRAMES_PER_SECOND;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/audio_cd/aloe_AudioCDReader.cpp]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/native/aloe_linux_AudioCDReader.cpp]
#[cfg(feature = "cd-reader")]
impl<'a> AudioCDReader<'a> {
    
    #[cfg(target_os="macos")]
    pub fn new(volume: &File) -> Self {
    
        todo!();
        /*


        
        */
    }

    #[cfg(target_os="windows")]
    pub fn new(handle: *mut c_void) -> Self {
    
        todo!();
        /*


        
        */
    }
    
    #[cfg(target_os="windows")]
    pub fn get_index_at(&mut self, sample_pos: i32) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Tries to eject the disk. Ejecting the
      | disk might not actually be possible,
      | e.g. if some other process is using it.
      |
      */
    pub fn eject_disk(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the total number of tracks (audio
      | + data).
      |
      */
    pub fn get_num_tracks(&self) -> i32 {
        
        todo!();
        /*
            return trackStartSamples.size() - 1;
        */
    }
    
    /**
      | Finds the sample offset of the start
      | of a track.
      | 
      | -----------
      | @param trackNum
      | 
      | the track number, where trackNum = 0
      | is the first track and trackNum = getNumTracks()
      | means the end of the CD.
      |
      */
    pub fn get_position_of_track_start(&self, track_num: i32) -> i32 {
        
        todo!();
        /*
            return trackStartSamples [trackNum];
        */
    }
    
    /**
      | Returns an array of sample offsets for
      | the start of each track, followed by
      | the sample position of the end of the
      | CD.
      |
      */
    pub fn get_track_offsets(&self) -> &[i32] {
        
        todo!();
        /*
            return trackStartSamples;
        */
    }
    
    /**
      | Returns the CDDB id number for the CD.
      | It's not a great way of identifying a
      | disc, but it's traditional.
      |
      */
    pub fn get_cddb_id(&mut self) -> i32 {
        
        todo!();
        /*
            int checksum = 0;
        const int numTracks = getNumTracks();

        for (int i = 0; i < numTracks; ++i)
            for (int offset = (trackStartSamples.getUnchecked(i) + 88200) / 44100; offset > 0; offset /= 10)
                checksum += offset % 10;

        const int length = (trackStartSamples.getLast() - trackStartSamples.getFirst()) / 44100;

        // CCLLLLTT: checksum, length, tracks
        return ((checksum & 0xff) << 24) | (length << 8) | numTracks;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn new() -> Self {
    
        todo!();
        /*
        : audio_format_reader(0, "CD Audio"),

        
        */
    }
    
    /**
      | Returns a list of names of Audio CDs currently
      | available for reading.
      | 
      | If there's a CD drive but no CD in it, this
      | might return an empty list, or possibly
      | a device that can be opened but which
      | has no tracks, depending on the platform.
      | 
      | @see createReaderForCD
      |
      */
    #[cfg(target_os="linux")]
    pub fn get_available_cd_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> names;
        return names;
        */
    }
    
    /**
      | Tries to create an AudioFormatReader
      | that can read from an Audio CD.
      | 
      | -----------
      | @param index
      | 
      | the index of one of the available CDs
      | - use getAvailableCDNames() to find
      | out how many there are.
      | 
      | -----------
      | @return
      | 
      | a new AudioCDReader object, or nullptr
      | if it couldn't be created. The caller
      | will be responsible for deleting the
      | object returned.
      |
      */
    #[cfg(target_os="linux")]
    pub fn create_reader_forcd(&mut self, _0: i32) -> *mut AudioCDReader {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    /**
      | Refreshes the object's table of contents.
      | 
      | If the disc has been ejected and a different
      | one put in since this object was created,
      | this will cause it to update its idea
      | of how many tracks there are, etc.
      |
      */
    #[cfg(target_os="linux")]
    pub fn refresh_track_lengths(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Implementation of the AudioFormatReader
      | method.
      |
      */
    #[cfg(target_os="linux")]
    pub fn read_samples(&mut self, 
        dest_samples:                *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 i32) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    /**
      | Checks whether the CD has been removed
      | from the drive.
      |
      */
    #[cfg(target_os="linux")]
    pub fn is_cd_still_present(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    /**
      | Returns true if a given track is an audio
      | track.
      | 
      | -----------
      | @param trackNum
      | 
      | the track number, where 0 is the first
      | track.
      |
      */
    #[cfg(target_os="linux")]
    pub fn is_track_audio(&self, _0: i32) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    /**
      | Enables scanning for indexes within
      | tracks. @see getLastIndex
      |
      */
    #[cfg(target_os="linux")]
    pub fn enable_index_scanning(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the index number found during
      | the last read() call.
      | 
      | Index scanning is turned off by default
      | - turn it on with enableIndexScanning().
      | 
      | Then when the read() method is called,
      | if it comes across an index within that
      | block, the index number is stored and
      | returned by this method.
      | 
      | Some devices might not support indexes,
      | of course.
      | 
      | (If you don't know what CD indexes are,
      | it's unlikely you'll ever need them).
      | 
      | @see enableIndexScanning
      |
      */
    #[cfg(target_os="linux")]
    pub fn get_last_index(&self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    /**
      | Scans a track to find the position of
      | any indexes within it.
      | 
      | -----------
      | @param trackNumber
      | 
      | the track to look in, where 0 is the first
      | track on the disc
      | 
      | -----------
      | @return
      | 
      | an array of sample positions of any index
      | points found (not including the index
      | that marks the start of the track)
      |
      */
    #[cfg(target_os="linux")]
    pub fn find_indexes_in_track(&mut self, _0: i32) -> Vec<i32> {
        
        todo!();
        /*
            return {};
        */
    }
}

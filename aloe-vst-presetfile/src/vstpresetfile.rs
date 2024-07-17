/**
  |     Vst 3 Preset File Format Definition
  |    ===================================
  |
  | 0   +---------------------------+
  |     | HEADER                    |
  |     | header id ('Vst3')        |       4 Bytes
  |     | version                   |       4 Bytes (int32)
  |     | ASCII-encoded class id    |       32 Bytes 
  |  +--| offset to chunk list      |       8 Bytes (int64)
  |  |  +---------------------------+
  |  |  | DATA AREA                 |<-+
  |  |  | data of chunks 1..n       |  |
  |  |  ...                       ...  |
  |  |  |                           |  |
  |  +->+---------------------------+  |
  |     | CHUNK LIST                |  |
  |     | list id ('List')          |  |    4 Bytes
  |     | entry count               |  |    4 Bytes (int32)
  |     +---------------------------+  |
  |     |  1..n                     |  |
  |     |  +----------------------+ |  |
  |     |  | chunk id             | |  |    4 Bytes
  |     |  | offset to chunk data |----+    8 Bytes (int64)
  |     |  | size of chunk data   | |       8 Bytes (int64)
  |     |  +----------------------+ |
  | EOF +---------------------------+
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstpresetfile.h]

pub type ChunkID = [u8; 4];

pub enum ChunkType
{
    Header,
    ComponentState,
    ControllerState,
    ProgramData,
    MetaInfo,
    ChunkList,
    NumPresetChunks
}

lazy_static!{
    /*
    extern const ChunkID& getChunkID (ChunkType type);
    */
}

#[inline] pub fn is_equalid(
        id1: ChunkID,
        id2: ChunkID) -> bool {
    
    todo!();
        /*
            return memcmp (id1, id2, sizeof (ChunkID)) == 0;
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstpresetfile.cpp]

/**
   Preset Chunk IDs
  */
lazy_static!{
    /*
    static const ChunkID commonChunks[kNumPresetChunks] = {
        {'V', 'S', 'T', '3'},   // kHeader
        {'C', 'o', 'm', 'p'},   // kComponentState
        {'C', 'o', 'n', 't'},   // kControllerState
        {'P', 'r', 'o', 'g'},   // kProgramData
        {'I', 'n', 'f', 'o'},   // kMetaInfo
        {'L', 'i', 's', 't'}    // kChunkList
    };
    */
}

/**
   Preset Header: header id + version + class id
   + list offset
  */
pub const FORMAT_VERSION:  i32 = 1;
pub const CLASS_ID_SIZE:   usize = 32; // ASCII-encoded FUID

pub const HEADER_SIZE: usize = {
    size_of::<ChunkID>() + size_of::<i32>() + CLASS_ID_SIZE + size_of::<TSize>()
};

pub const LIST_OFFSET_POS: usize = HEADER_SIZE - size_of::<TSize>();

pub fn get_chunkid<'a>(ty: ChunkType) -> &'a ChunkID {
    
    todo!();
        /*
            return commonChunks[type];
        */
}

#[inline] pub fn verify(result: tresult) -> bool {
    
    todo!();
        /*
            return result == kResultOk || result == kNotImplemented;
        */
}

pub fn copy_stream(
    in_stream:  *mut dyn IBStream,
    out_stream: *mut dyn IBStream

) -> bool {
    
    todo!();
        /*
            if (!inStream || !outStream)
            return false;

        int8 buffer[8192];
        int32 read = 0;
        int32 written = 0;
        while (inStream->read (buffer, 8192, &read) == kResultTrue && read > 0)
        {
            if (outStream->write (buffer, read, &written) != kResultTrue)
            {
                return false;
            }
        }
        return true;
        */
}

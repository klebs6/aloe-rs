crate::ix!();

pub struct ZipEntryHolder {
    entry:           ZipEntry,
    stream_offset:   i64,
    compressed_size: i64,
    is_compressed:   bool,
}

impl ZipEntryHolder {

    pub fn new(
        buffer:        *const u8,
        file_name_len: i32) -> Self {
    
        todo!();
        /*


            isCompressed           = readUnalignedLittleEndianShort (buffer + 10) != 0;
                entry.fileTime         = parseFileTime (readUnalignedLittleEndianShort (buffer + 12),
                                                        readUnalignedLittleEndianShort (buffer + 14));
                compressedSize         = (int64) readUnalignedLittleEndianInt (buffer + 20);
                entry.uncompressedSize = (int64) readUnalignedLittleEndianInt (buffer + 24);
                streamOffset           = (int64) readUnalignedLittleEndianInt (buffer + 42);

                entry.externalFileAttributes = readUnalignedLittleEndianInt (buffer + 38);
                auto fileType = (entry.externalFileAttributes >> 28) & 0xf;
                entry.isSymbolicLink = (fileType == 0xA);

                entry.filename = String::fromUTF8 (buffer + 46, fileNameLen);
        */
    }
    
    pub fn parse_file_time(
        time: u32,
        date: u32) -> Time {
        
        todo!();
        /*
            auto year      = (int) (1980 + (date >> 9));
                auto month     = (int) (((date >> 5) & 15) - 1);
                auto day       = (int) (date & 31);
                auto hours     = (int) time >> 11;
                auto minutes   = (int) ((time >> 5) & 63);
                auto seconds   = (int) ((time & 31) << 1);

                return { year, month, day, hours, minutes, seconds };
        */
    }
}

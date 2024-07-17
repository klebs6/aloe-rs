crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ZipFileBuilderItem {
    file:              File,
    stream:            Box<dyn Read>,
    stored_pathname:   String,
    file_time:         Time,
    compressed_size:   i64, // default = 0
    uncompressed_size: i64, // default = 0
    header_start:      i64, // default = 0
    compression_level: i32, // default = 0
    checksum:          u64, // default = 0
    symbolic_link:     bool, // default = false
}

impl ZipFileBuilderItem {

    pub fn new(
        f:           &File,
        s:           *mut dyn Read,
        compression: i32,
        stored_path: &String,
        time:        Time) -> Self {
    
        todo!();
        /*
        : file(f),
        : stream(s),
        : stored_pathname(storedPath),
        : file_time(time),
        : compression_level(compression),

            symbolicLink = (file.exists() && file.isSymbolicLink());
        */
    }
    
    pub fn write_data(
        &mut self, 
        target:                 &mut dyn Write,
        overall_start_position: i64

    ) -> bool {
        
        todo!();
        /*
            MemoryOutputStream compressedData ((size_t) file.getSize());

                    if (symbolicLink)
                    {
                        auto relativePath = file.getNativeLinkedTarget().replaceCharacter (File::getSeparatorChar(), L'/');

                        uncompressedSize = relativePath.length();

                        checksum = zlib::crc32 (0, (uint8_t*) relativePath.toRawUTF8(), (unsigned int) uncompressedSize);
                        compressedData << relativePath;
                    }
                    else if (compressionLevel > 0)
                    {
                        GZIPCompressorOutputStream compressor (compressedData, compressionLevel,
                                                               GZIPCompressorOutputStream::windowBitsRaw);
                        if (! writeSource (compressor))
                            return false;
                    }
                    else
                    {
                        if (! writeSource (compressedData))
                            return false;
                    }

                    compressedSize = (int64) compressedData.getDataSize();
                    headerStart = target.getPosition() - overallStartPosition;

                    target.writeInt (0x04034b50);
                    writeFlagsAndSizes (target);
                    target << storedPathname
                           << compressedData;

                    return true;
        */
    }
    
    pub fn write_directory_entry(&mut self, target: &mut dyn Write) -> bool {
        
        todo!();
        /*
            target.writeInt (0x02014b50);
                    target.writeShort (symbolicLink ? 0x0314 : 0x0014);
                    writeFlagsAndSizes (target);
                    target.writeShort (0); // comment length
                    target.writeShort (0); // start disk num
                    target.writeShort (0); // internal attributes
                    target.writeInt ((int) (symbolicLink ? 0xA1ED0000 : 0)); // external attributes
                    target.writeInt ((int) (uint32) headerStart);
                    target << storedPathname;

                    return true;
        */
    }
    
    pub fn write_time_and_date(
        target: &mut dyn Write,
        t:      Time

    ) {
        
        todo!();
        /*
            target.writeShort ((short) (t.getSeconds() + (t.getMinutes() << 5) + (t.getHours() << 11)));
                    target.writeShort ((short) (t.getDayOfMonth() + ((t.getMonth() + 1) << 5) + ((t.getYear() - 1980) << 9)));
        */
    }
    
    pub fn write_source(&mut self, target: &mut dyn Write) -> bool {
        
        todo!();
        /*
            if (stream == nullptr)
                    {
                        stream = file.createInputStream();

                        if (stream == nullptr)
                            return false;
                    }

                    checksum = 0;
                    uncompressedSize = 0;
                    const int bufferSize = 4096;
                    HeapBlock<unsigned char> buffer (bufferSize);

                    while (! stream->isExhausted())
                    {
                        auto bytesRead = stream->read (buffer, bufferSize);

                        if (bytesRead < 0)
                            return false;

                        checksum = zlib::crc32 (checksum, buffer, (unsigned int) bytesRead);
                        target.write (buffer, (size_t) bytesRead);
                        uncompressedSize += bytesRead;
                    }

                    stream.reset();
                    return true;
        */
    }
    
    pub fn write_flags_and_sizes(&self, target: &mut dyn Write)  {
        
        todo!();
        /*
            target.writeShort (10); // version needed
                    target.writeShort ((short) (1 << 11)); // this flag indicates UTF-8 filename encoding
                    target.writeShort ((! symbolicLink && compressionLevel > 0) ? (short) 8 : (short) 0); //symlink target path is not compressed
                    writeTimeAndDate (target, fileTime);
                    target.writeInt ((int) checksum);
                    target.writeInt ((int) (uint32) compressedSize);
                    target.writeInt ((int) (uint32) uncompressedSize);
                    target.writeShort (static_cast<short> (storedPathname.toUTF8().sizeInBytes() - 1));
                    target.writeShort (0); // extra field length
        */
    }
}

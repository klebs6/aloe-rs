crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ZipInputStream {
    file:             Rc<RefCell<ZipFile>>,
    zip_entry_holder: ZipEntryHolder,
    pos:              i64, // default = 0
    header_size:      i32, // default = 0
    input_stream:     *mut dyn Read,
    stream_to_delete: Box<dyn Read>,
}

impl Read for ZipInputStream {

    fn read(&mut self, _: &mut [u8]) -> Result<usize, std::io::Error> { 
        todo!() 
    }
}

impl Drop for ZipInputStream {

    fn drop(&mut self) {
        todo!();
        /* 
               #if ALOE_DEBUG
                if (inputStream != nullptr && inputStream == file.inputStream)
                    file.streamCounter.numOpenStreams--;
               #endif
             */
    }
}

impl ZipInputStream {

    pub fn new(
        zf:  &mut ZipFile,
        zei: &ZipEntryHolder

    ) -> Self {
    
        todo!();
        /*
        : file(zf),
        : zip_entry_holder(zei),
        : input_stream(zf.inputStream),

            if (zf.inputSource != nullptr)
                {
                    streamToDelete.reset (file.inputSource->createInputStream());
                    inputStream = streamToDelete.get();
                }
                else
                {
                   #if ALOE_DEBUG
                    zf.streamCounter.numOpenStreams++;
                   #endif
                }

                char buffer[30];

                if (inputStream != nullptr
                     && inputStream->setPosition (zei.streamOffset)
                     && inputStream->read (buffer, 30) == 30
                     && ByteOrder::littleEndianInt (buffer) == 0x04034b50)
                {
                    headerSize = 30 + ByteOrder::littleEndianShort (buffer + 26)
                                    + ByteOrder::littleEndianShort (buffer + 28);
                }
        */
    }
    
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return zipEntryHolder.compressedSize;
        */
    }
    
    pub fn read(&mut self, 
        buffer:   *mut c_void,
        how_many: i32) -> i32 {
        
        todo!();
        /*
            if (headerSize <= 0)
                    return 0;

                howMany = (int) jmin ((int64) howMany, zipEntryHolder.compressedSize - pos);

                if (inputStream == nullptr)
                    return 0;

                int num;

                if (inputStream == file.inputStream)
                {
                    const ScopedLock sl (file.lock);
                    inputStream->setPosition (pos + zipEntryHolder.streamOffset + headerSize);
                    num = inputStream->read (buffer, howMany);
                }
                else
                {
                    inputStream->setPosition (pos + zipEntryHolder.streamOffset + headerSize);
                    num = inputStream->read (buffer, howMany);
                }

                pos += num;
                return num;
        */
    }
    
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return headerSize <= 0 || pos >= zipEntryHolder.compressedSize;
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return pos;
        */
    }
    
    pub fn set_position(&mut self, new_pos: i64) -> bool {
        
        todo!();
        /*
            pos = jlimit ((int64) 0, zipEntryHolder.compressedSize, newPos);
                return true;
        */
    }
}

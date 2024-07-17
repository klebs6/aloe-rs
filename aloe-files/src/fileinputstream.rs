crate::ix!();

#[cfg(feature = "aloe_posix")]
pub fn aloe_file_set_position(
        handle: *mut c_void,
        pos:    i64) -> i64 {
    
    todo!();
    /*
        if (handle != nullptr && lseek (getFD (handle), (off_t) pos, SEEK_SET) == pos)
            return pos;

        return -1;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileInputStream.h]

/**
  | An input stream that reads from a local
  | file.
  | 
  | @see InputStream, FileOutputStream,
  | File::createInputStream
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileInputStream {
    file:             File,
    file_handle:      *mut c_void, // default = nullptr
    current_position: i64, // default = 0
    status:           Result<(),()>, // { Result::ok() };
}

impl Read for FileInputStream {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize,std::io::Error> {

        todo!();

        /*
        self.file.read(buf)
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileInputStream.cpp]
impl FileInputStream {

    /**
      | Returns the file that this stream is
      | reading from.
      |
      */
    pub fn get_file(&self) -> &File {
        
        todo!();
        /*
            return file;
        */
    }

    /**
      | Returns the status of the file stream.
      | 
      | The result will be ok if the file opened
      | successfully. If an error occurs while
      | opening or reading from the file, this
      | will contain an error message.
      |
      */
    pub fn get_status(&self) -> &Result<(),()> {
        
        todo!();
        /*
            return status;
        */
    }

    /**
      | Returns true if the stream couldn't
      | be opened for some reason. @see getResult()
      |
      */
    pub fn failed_to_open(&self) -> bool {
        
        todo!();
        /*
            return status.failed();
        */
    }

    /**
      | Returns true if the stream opened without
      | problems. @see getResult()
      |
      */
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return status.wasOk();
        */
    }
    
    /**
      | Creates a FileInputStream to read from
      | the given file.
      | 
      | After creating a FileInputStream,
      | you should use openedOk() or failedToOpen()
      | to make sure that it's OK before trying
      | to read from it! If it failed, you can
      | call getStatus() to get more error information.
      |
      */
    pub fn new(f: &File) -> Self {
    
        todo!();
        /*
        : file(f),

            openHandle();
        */
    }
    
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            // You should always check that a stream opened successfully before using it!
        jassert (openedOk());

        return file.getSize();
        */
    }
    
    pub fn read(&mut self, 
        buffer:        *mut c_void,
        bytes_to_read: i32) -> i32 {
        
        todo!();
        /*
            // You should always check that a stream opened successfully before using it!
        jassert (openedOk());

        // The buffer should never be null, and a negative size is probably a
        // sign that something is broken!
        jassert (buffer != nullptr && bytesToRead >= 0);

        auto num = readInternal (buffer, (size_t) bytesToRead);
        currentPosition += (int64) num;

        return (int) num;
        */
    }
    
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return currentPosition >= getTotalLength();
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return currentPosition;
        */
    }
    
    pub fn set_position(&mut self, pos: i64) -> bool {
        
        todo!();
        /*
            // You should always check that a stream opened successfully before using it!
        jassert (openedOk());

        if (pos != currentPosition)
            currentPosition = aloe_fileSetPosition (fileHandle, pos);

        return currentPosition == pos;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct FileInputStreamTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for FileInputStreamTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("FileInputStream", UnitTestCategories::streams
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl FileInputStreamTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Open stream non-existent file");
            {
                auto tempFile = File::createTempFile (".txt");
                expect (! tempFile.exists());

                FileInputStream stream (tempFile);
                expect (stream.failedToOpen());
            }

            beginTest ("Open stream existing file");
            {
                auto tempFile = File::createTempFile (".txt");
                tempFile.create();
                expect (tempFile.exists());

                FileInputStream stream (tempFile);
                expect (stream.openedOk());
            }

            const MemoryBlock data ("abcdefghijklmnopqrstuvwxyz", 26);
            File f (File::createTempFile (".txt"));
            f.appendData (data.getData(), data.getSize());
            FileInputStream stream (f);

            beginTest ("Read");
            {
                expectEquals (stream.getPosition(), (int64) 0);
                expectEquals (stream.getTotalLength(), (int64) data.getSize());
                expectEquals (stream.getNumBytesRemaining(), stream.getTotalLength());
                expect (! stream.isExhausted());

                size_t numBytesRead = 0;
                MemoryBlock readBuffer (data.getSize());

                while (numBytesRead < data.getSize())
                {
                    numBytesRead += (size_t) stream.read (&readBuffer[numBytesRead], 3);

                    expectEquals (stream.getPosition(), (int64) numBytesRead);
                    expectEquals (stream.getNumBytesRemaining(), (int64) (data.getSize() - numBytesRead));
                    expect (stream.isExhausted() == (numBytesRead == data.getSize()));
                }

                expectEquals (stream.getPosition(), (int64) data.getSize());
                expectEquals (stream.getNumBytesRemaining(), (int64) 0);
                expect (stream.isExhausted());

                expect (readBuffer == data);
            }

            beginTest ("Skip");
            {
                stream.setPosition (0);
                expectEquals (stream.getPosition(), (int64) 0);
                expectEquals (stream.getTotalLength(), (int64) data.getSize());
                expectEquals (stream.getNumBytesRemaining(), stream.getTotalLength());
                expect (! stream.isExhausted());

                size_t numBytesRead = 0;
                const int numBytesToSkip = 5;

                while (numBytesRead < data.getSize())
                {
                    stream.skipNextBytes (numBytesToSkip);
                    numBytesRead += numBytesToSkip;
                    numBytesRead = std::min (numBytesRead, data.getSize());

                    expectEquals (stream.getPosition(), (int64) numBytesRead);
                    expectEquals (stream.getNumBytesRemaining(), (int64) (data.getSize() - numBytesRead));
                    expect (stream.isExhausted() == (numBytesRead == data.getSize()));
                }

                expectEquals (stream.getPosition(), (int64) data.getSize());
                expectEquals (stream.getNumBytesRemaining(), (int64) 0);
                expect (stream.isExhausted());

                f.deleteFile();
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static FileInputStreamTests fileInputStreamTests;
    */
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileOutputStream.h]

/**
  | An output stream that writes into a local
  | file.
  | 
  | @see OutputStream, FileInputStream,
  | File::createOutputStream
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileOutputStream {
    file:             File,
    file_handle:      *mut c_void, // default = nullptr
    status:           Result<(),()>,    // { Result::ok() };
    current_position: i64,       // default = 0
    buffer_size:      usize,
    bytes_in_buffer:  usize,     // default = 0
    buffer:           HeapBlock<u8>,
}

impl Write for FileOutputStream {

    fn write(&mut self, buf: &[u8]) -> Result<usize,std::io::Error> {

        todo!();

        /*

        let remaining_space = self.buffer_size - self.buffer.len();
        let to_buffer = &buf[..remaining_space.min(buf.len())];

        self.buffer.extend_from_slice(to_buffer);

        if self.buffer.len() == self.buffer_size {
            self.flush()?;
        }

        // Return the number of bytes written. This could be less than buf.len() if we don't auto-flush
        // in this simple example. For a real implementation, consider looping to handle remaining data.
        Ok(to_buffer.len())
        */
    }

    fn flush(&mut self) -> Result<(),std::io::Error> {

        todo!();

        /*
        self.file.write_all(&self.buffer)?;
        self.buffer.clear();
        Ok(())
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/files/aloe_FileOutputStream.cpp]
impl Drop for FileOutputStream {

    fn drop(&mut self) {
        todo!();
        /* 
        flushBuffer();
        closeHandle();
         */
    }
}

impl FileOutputStream {

    /**
      | Returns the file that this stream is
      | writing to.
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
      | opening or writing to the file, this
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
      | Attempts to truncate the file to the
      | current write position.
      | 
      | To truncate a file to a specific size,
      | first use setPosition() to seek to the
      | appropriate location, and then call
      | this method.
      |
      */
    pub fn truncate(&mut self) -> Result<(),()> {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Creates a FileOutputStream.
      | 
      | If the file doesn't exist, it will first
      | be created. If the file can't be created
      | or opened (for example, because the
      | parent directory of the file does not
      | exist), the failedToOpen() method
      | will return true.
      | 
      | If the file already exists when opened,
      | the stream's write-position will be
      | set to the end of the file. To overwrite
      | an existing file, you can truncate it
      | like this:
      | 
      | -----------
      | @code
      | 
      | FileOutputStream stream (file);
      | 
      | if (stream.openedOk())
      | {
      |     stream.setPosition (0);
      |     stream.truncate();
      |     ...
      | }
      | 
      | Destroying a FileOutputStream object
      | does not force the operating system
      | to write the buffered data to disk immediately.
      | If this is required you should call flush()
      | before triggering the destructor.
      | 
      | @see TemporaryFile
      |
      */
    pub fn new(
        f:                  &File,
        buffer_size_to_use: Option<usize>
    ) -> Self {

        let buffer_size_to_use: usize =
            buffer_size_to_use.unwrap_or(16384);
    
        todo!();
        /*
            : file (f),
          bufferSize (bufferSizeToUse),
          buffer (jmax (bufferSizeToUse, (size_t) 16))

        openHandle();
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return currentPosition;
        */
    }
    
    pub fn set_position(&mut self, new_position: i64) -> bool {
        
        todo!();
        /*
            if (newPosition != currentPosition)
        {
            flushBuffer();
            currentPosition = aloe_fileSetPosition (fileHandle, newPosition);
        }

        return newPosition == currentPosition;
        */
    }
    
    pub fn flush_buffer(&mut self) -> bool {
        
        todo!();
        /*
            bool ok = true;

        if (bytesInBuffer > 0)
        {
            ok = (writeInternal (buffer, bytesInBuffer) == (ssize_t) bytesInBuffer);
            bytesInBuffer = 0;
        }

        return ok;
        */
    }
    
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            flushBuffer();
        flushInternal();
        */
    }
    
    pub fn write(&mut self, 
        src:       *const c_void,
        num_bytes: usize) -> bool {
        
        todo!();
        /*
            jassert (src != nullptr && ((ssize_t) numBytes) >= 0);

        if (! openedOk())
            return false;

        if (bytesInBuffer + numBytes < bufferSize)
        {
            memcpy (buffer + bytesInBuffer, src, numBytes);
            bytesInBuffer += numBytes;
            currentPosition += (int64) numBytes;
        }
        else
        {
            if (! flushBuffer())
                return false;

            if (numBytes < bufferSize)
            {
                memcpy (buffer + bytesInBuffer, src, numBytes);
                bytesInBuffer += numBytes;
                currentPosition += (int64) numBytes;
            }
            else
            {
                auto bytesWritten = writeInternal (src, numBytes);

                if (bytesWritten < 0)
                    return false;

                currentPosition += (int64) bytesWritten;
                return bytesWritten == (ssize_t) numBytes;
            }
        }

        return true;
        */
    }
    
    pub fn write_repeated_byte(&mut self, 
        byte:      u8,
        num_bytes: usize) -> bool {
        
        todo!();
        /*
            jassert (((ssize_t) numBytes) >= 0);

        if (bytesInBuffer + numBytes < bufferSize)
        {
            memset (buffer + bytesInBuffer, byte, numBytes);
            bytesInBuffer += numBytes;
            currentPosition += (int64) numBytes;
            return true;
        }

        return OutputStream::writeRepeatedByte (byte, numBytes);
        */
    }
}

//#[cfg(target_os="android")]
impl FileOutputStream {
    
    pub fn flush_internal(&mut self)  {
        
        todo!();
        /*
            if (fileHandle != nullptr)
        {
            if (fsync (getFD (fileHandle)) == -1)
                status = getResultForErrno();

            // This stuff tells the OS to asynchronously update the metadata
            // that the OS has cached about the file - this metadata is used
            // when the device is acting as a USB drive, and unless it's explicitly
            // refreshed, it'll get out of step with the real file.
            new SingleMediaScanner (file.getFullPathName());
        }
        */
    }
}

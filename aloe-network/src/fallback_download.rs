crate::ix!();

#[no_copy]
#[leak_detector]
pub struct FallbackDownloadTask<'a> {
    base:        UrlDownloadTask,
    base2:       Thread,
    file_stream: Box<FileOutputStream>,
    stream:      Box<WebInputStream<'a>>,
    buffer_size: usize,
    buffer:      HeapBlock<u8>,
    listener:    *const dyn UrlDownloadTaskListener,
}

impl<'a> Drop for FallbackDownloadTask<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            signalThreadShouldExit();
            stream->cancel();
            waitForThreadToExit (-1);
         */
    }
}

impl<'a> FallbackDownloadTask<'a> {

    pub fn new(
        output_stream_to_use: Box<FileOutputStream>,
        buffer_size_to_use:   usize,
        stream_to_use:        Box<WebInputStream>,
        listener_to_use:      *mut dyn UrlDownloadTaskListener) -> Self {
    
        todo!();
        /*

            : Thread ("DownloadTask thread"),
              fileStream (std::move (outputStreamToUse)),
              stream (std::move (streamToUse)),
              bufferSize (bufferSizeToUse),
              buffer (bufferSize),
              listener (listenerToUse)

            jassert (fileStream != nullptr);
            jassert (stream != nullptr);

            targetLocation = fileStream->getFile();
            contentLength  = stream->getTotalLength();
            httpCode       = stream->getStatusCode();

            startThread();
        */
    }

    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! (stream->isExhausted() || stream->isError() || threadShouldExit()))
            {
                if (listener != nullptr)
                    listener->progress (this, downloaded, contentLength);

                auto max = (int) jmin ((int64) bufferSize, contentLength < 0 ? std::numeric_limits<int64>::max()
                                                                             : static_cast<int64> (contentLength - downloaded));

                auto actual = stream->read (buffer.get(), max);

                if (actual < 0 || threadShouldExit() || stream->isError())
                    break;

                if (! fileStream->write (buffer.get(), static_cast<size_t> (actual)))
                {
                    error = true;
                    break;
                }

                downloaded += actual;

                if (downloaded == contentLength)
                    break;
            }

            fileStream.reset();

            if (threadShouldExit() || stream->isError())
                error = true;

            if (contentLength > 0 && downloaded < contentLength)
                error = true;

            finished = true;

            if (listener != nullptr && ! threadShouldExit())
                listener->finished (this, ! error);
        */
    }
}


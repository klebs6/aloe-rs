crate::ix!();

#[cfg(ALOE_CONTENT_SHARING)]
pub struct ContentSharerPrepareDataThread {
    base:  Thread,
    owner: &mut ContentSharer,
    data:  MemoryBlock,
}

#[cfg(ALOE_CONTENT_SHARING)]
impl Drop for ContentSharerPrepareDataThread {
    fn drop(&mut self) {
        todo!();
        /* 
                signalThreadShouldExit();
                waitForThreadToExit (10000);
             */
    }
}

#[cfg(ALOE_CONTENT_SHARING)]
impl ContentSharerPrepareDataThread {

    pub fn new(
        cs: &mut ContentSharer,
        mb: &MemoryBlock) -> Self {
    
        todo!();
        /*


            : Thread ("ContentSharer::ContentSharerPrepareDataThread"),
                  owner (cs),
                  data (mb)

                startThread();
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            File tempFile = File::createTempFile ("data");

                if (tempFile.create().wasOk())
                {
                    if (auto outputStream = std::unique_ptr<FileOutputStream> (tempFile.createOutputStream()))
                    {
                        size_t pos = 0;
                        size_t totalSize = data.getSize();

                        while (pos < totalSize)
                        {
                            if (threadShouldExit())
                                return;

                            size_t numToWrite = std::min ((size_t) 8192, totalSize - pos);

                            outputStream->write (data.begin() + pos, numToWrite);

                            pos += numToWrite;
                        }

                        owner.temporaryFiles.add (tempFile);
                    }
                }

                finish();
        */
    }
    
    pub fn finish(&mut self)  {
        
        todo!();
        /*
            MessageManager::callAsync ([this]() { owner.filesToSharePrepared(); });
        */
    }
}

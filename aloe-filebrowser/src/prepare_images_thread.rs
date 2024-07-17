crate::ix!();

#[cfg(ALOE_CONTENT_SHARING)]
pub struct ContentSharerPrepareImagesThread {
    base:              Thread,
    owner:             &mut ContentSharer,
    images:            Vec<Image>,
    image_file_format: Box<ImageFileFormat>,
    extension:         String,
}

#[cfg(ALOE_CONTENT_SHARING)]
impl Drop for ContentSharerPrepareImagesThread {
    fn drop(&mut self) {
        todo!();
        /* 
                signalThreadShouldExit();
                waitForThreadToExit (10000);
             */
    }
}

#[cfg(ALOE_CONTENT_SHARING)]
impl ContentSharerPrepareImagesThread {

    pub fn new(
        cs:                       &mut ContentSharer,
        images_to_use:            &[Image],
        image_file_format_to_use: *mut ImageFileFormat) -> Self {
    
        todo!();
        /*


            : Thread ("ContentSharer::ContentSharerPrepareImagesThread"),
                  owner (cs),
                  images (imagesToUse),
                  imageFileFormat (imageFileFormatToUse == nullptr ? new PNGImageFormat()
                                                                   : imageFileFormatToUse),
                  extension (imageFileFormat->getFormatName().toLowerCase())

                startThread();
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            for (const auto& image : images)
                {
                    if (threadShouldExit())
                        return;

                    File tempFile = File::createTempFile (extension);

                    if (! tempFile.create().wasOk())
                        break;

                    std::unique_ptr<FileOutputStream> outputStream (tempFile.createOutputStream());

                    if (outputStream == nullptr)
                        break;

                    if (imageFileFormat->writeImageToStream (image, *outputStream))
                        owner.temporaryFiles.add (tempFile);
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


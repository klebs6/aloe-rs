crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_ImagePreviewComponent.h]

/**
  | A simple preview component that shows
  | thumbnails of image files.
  | 
  | @see FileChooserDialogBox, FilePreviewComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ImagePreviewComponent<'a> {
    base:              FilePreviewComponent<'a>,
    base2:             Timer,
    file_to_load:      File,
    current_thumbnail: Image,
    current_details:   String,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_ImagePreviewComponent.cpp]
impl<'a> ImagePreviewComponent<'a> {

    pub fn get_thumb_size(&self, 
        w: &mut i32,
        h: &mut i32)  {
        
        todo!();
        /*
            auto availableW = proportionOfWidth (0.97f);
        auto availableH = getHeight() - 13 * 4;

        auto scale = jmin (1.0,
                           availableW / (double) w,
                           availableH / (double) h);

        w = roundToInt (scale * w);
        h = roundToInt (scale * h);
        */
    }
    
    pub fn selected_file_changed(&mut self, file: &File)  {
        
        todo!();
        /*
            if (fileToLoad != file)
        {
            fileToLoad = file;
            startTimer (100);
        }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

        currentThumbnail = Image();
        currentDetails.clear();
        repaint();

        FileInputStream in (fileToLoad);

        if (in.openedOk() && fileToLoad.existsAsFile())
        {
            if (auto format = ImageFileFormat::findImageFormatForStream (in))
            {
                currentThumbnail = format->decodeImage (in);

                if (currentThumbnail.isValid())
                {
                    auto w = currentThumbnail.getWidth();
                    auto h = currentThumbnail.getHeight();

                    currentDetails
                        << fileToLoad.getFileName() << "\n"
                        << format->getFormatName() << "\n"
                        << w << " x " << h << " pixels\n"
                        << File::descriptionOfSizeInBytes (fileToLoad.getSize());

                    getThumbSize (w, h);

                    currentThumbnail = currentThumbnail.rescaled (w, h);
                }
            }
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (currentThumbnail.isValid())
        {
            g.setFont (13.0f);

            auto w = currentThumbnail.getWidth();
            auto h = currentThumbnail.getHeight();
            getThumbSize (w, h);

            const int numLines = 4;
            auto totalH = 13 * numLines + h + 4;
            auto y = (getHeight() - totalH) / 2;

            g.drawImageWithin (currentThumbnail,
                               (getWidth() - w) / 2, y, w, h,
                               RectanglePlacement::centred | RectanglePlacement::onlyReduceInSize,
                               false);

            g.drawFittedText (currentDetails,
                              0, y + h + 4, getWidth(), 100,
                              Justification::centredTop, numLines);
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::image);
        */
    }
}

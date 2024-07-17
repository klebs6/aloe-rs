crate::ix!();

#[no_copy]
#[leak_detector]
pub struct RecordingThumbnail<'a> {

    base:  Component<'a>,

    format_manager:     AudioFormatManager,
    thumbnail_cache:    AudioThumbnailCache, // default = 10 
    thumbnail:          AudioThumbnail<'a>,      // { 512, formatManager, thumbnailCache };
    display_full_thumb: bool, // default = false
}

impl<'a> ChangeListener for RecordingThumbnail<'a> {

    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (source == &thumbnail)
                repaint();
        */
    }
}

impl<'a> Default for RecordingThumbnail<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            formatManager.registerBasicFormats();
            thumbnail.addChangeListener (this)
        */
    }
}

impl<'a> Drop for RecordingThumbnail<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            thumbnail.removeChangeListener (this);
         */
    }
}

impl<'a> RecordingThumbnail<'a> {

    pub fn get_audio_thumbnail(&mut self) -> &mut AudioThumbnail {
        
        todo!();
        /*
            return thumbnail;
        */
    }
    
    pub fn set_display_full_thumbnail(&mut self, display_full: bool)  {
        
        todo!();
        /*
            displayFullThumb = displayFull;
            repaint();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::darkgrey);
            g.setColour (Colours::lightgrey);

            if (thumbnail.getTotalLength() > 0.0)
            {
                auto endTime = displayFullThumb ? thumbnail.getTotalLength()
                                                : jmax (30.0, thumbnail.getTotalLength());

                auto thumbArea = getLocalBounds();
                thumbnail.drawChannels (g, thumbArea.reduced (2), 0.0, endTime, 1.0f);
            }
            else
            {
                g.setFont (14.0f);
                g.drawFittedText ("(No file recorded)", getLocalBounds(), Justification::centred, 2);
            }
        */
    }
}

crate::ix!();

pub struct WaveformView<'a> {
    base:              Component<'a>,
    data_model:        DataModel<'a>,
    visible_range:     VisibleRangeDataModel<'a>,
    thumbnail_cache:   AudioThumbnailCache,
    thumbnail:         AudioThumbnail<'a>,
    current_hash_code: i64, // default = 0
}

impl<'a> VisibleRangeDataModelListener for WaveformView<'a> {

}

impl<'a> DataModelListener for WaveformView<'a> {

}

impl<'a> ChangeListener for WaveformView<'a> {
    
    fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (source == &thumbnail)
                repaint();
        */
    }
}

impl<'a> WaveformView<'a> {

    pub fn new(
        model: &DataModel,
        vr:    &VisibleRangeDataModel) -> Self {
    
        todo!();
        /*


            : dataModel (model),
              visibleRange (vr),
              thumbnailCache (4),
              thumbnail (4, dataModel.getAudioFormatManager(), thumbnailCache)

            dataModel   .addListener (*this);
            visibleRange.addListener (*this);
            thumbnail   .addChangeListener (this);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // Draw the waveforms
            g.fillAll (Colours::black);
            auto numChannels = thumbnail.getNumChannels();

            if (numChannels == 0)
            {
                g.setColour (Colours::white);
                g.drawFittedText ("No File Loaded", getLocalBounds(), Justification::centred, 1);
                return;
            }

            auto bounds = getLocalBounds();
            auto channelHeight = bounds.getHeight() / numChannels;

            for (auto i = 0; i != numChannels; ++i)
            {
                drawChannel (g, i, bounds.removeFromTop (channelHeight));
            }
        */
    }
    
    pub fn sample_reader_changed(&mut self, value: Arc<dyn AudioFormatReaderFactory>)  {
        
        todo!();
        /*
            if (value != nullptr)
            {
                if (auto reader = value->make (dataModel.getAudioFormatManager()))
                {
                    thumbnail.setReader (reader.release(), currentHashCode);
                    currentHashCode += 1;

                    return;
                }
            }

            thumbnail.clear();
        */
    }
    
    pub fn visible_range_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn draw_channel(&mut self, 
        g:       &mut Graphics,
        channel: i32,
        bounds:  Rectangle<i32>)  {
        
        todo!();
        /*
            g.setGradientFill (ColourGradient (Colours::lightblue,
                                               bounds.getTopLeft().toFloat(),
                                               Colours::darkgrey,
                                               bounds.getBottomLeft().toFloat(),
                                               false));
            thumbnail.drawChannel (g,
                                   bounds,
                                   visibleRange.getVisibleRange().getStart(),
                                   visibleRange.getVisibleRange().getEnd(),
                                   channel,
                                   1.0f);
        */
    }
}

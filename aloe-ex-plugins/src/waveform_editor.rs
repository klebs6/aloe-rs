crate::ix!();

pub struct WaveformEditor<'a> {
    base:             Component<'a>,
    data_model:       DataModel<'a>,
    visible_range:    VisibleRangeDataModel<'a>,
    waveform_view:    WaveformView<'a>,
    playback_overlay: PlaybackPositionOverlay<'a>,
    loop_points:      LoopPointsOverlay<'a>,
    ruler:            Ruler<'a>,
}

impl<'a> DataModelListener for WaveformEditor<'a> {

}

impl<'a> WaveformEditor<'a> {

    pub fn new(
        model:        &DataModel,
        provider:     PlaybackPositionOverlayProvider,
        undo_manager: &mut UndoManager

    ) -> Self {
    
        todo!();
        /*


            : dataModel (model),
              waveformView (model, visibleRange),
              playbackOverlay (visibleRange, move (provider)),
              loopPoints (dataModel, visibleRange, undoManager),
              ruler (visibleRange)

            dataModel.addListener (*this);

            addAndMakeVisible (waveformView);
            addAndMakeVisible (playbackOverlay);
            addChildComponent (loopPoints);
            loopPoints.setAlwaysOnTop (true);

            waveformView.toBack();

            addAndMakeVisible (ruler);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();
            ruler          .setBounds (bounds.removeFromTop (25));
            waveformView   .setBounds (bounds);
            playbackOverlay.setBounds (bounds);
            loopPoints     .setBounds (bounds);
        */
    }
    
    pub fn loop_mode_changed(&mut self, value: LoopMode)  {
        
        todo!();
        /*
            loopPoints.setVisible (value != LoopMode::none);
        */
    }
    
    pub fn sample_reader_changed(&mut self, _0: Arc<dyn AudioFormatReaderFactory>)  {
        
        todo!();
        /*
            auto lengthInSeconds = dataModel.getSampleLengthSeconds();
            visibleRange.setTotalRange   (Range<double> (0, lengthInSeconds), nullptr);
            visibleRange.setVisibleRange (Range<double> (0, lengthInSeconds), nullptr);
        */
    }
}

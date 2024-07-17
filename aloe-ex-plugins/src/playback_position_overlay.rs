crate::ix!();

pub type PlaybackPositionOverlayProvider = fn() -> Vec<f32>;

pub struct PlaybackPositionOverlay<'a> {
    base:          Component<'a>,
    base2:         Timer,
    visible_range: VisibleRangeDataModel<'a>,
    provider:      PlaybackPositionOverlayProvider,
}

impl<'a> VisibleRangeDataModelListener for PlaybackPositionOverlay<'a> {

}

impl<'a> PlaybackPositionOverlay<'a> {

    pub fn new(
        model:       &VisibleRangeDataModel,
        provider_in: PlaybackPositionOverlayProvider

    ) -> Self {
    
        todo!();
        /*
        : visible_range(model),
        : provider(std::move (providerIn)),

            visibleRange.addListener (*this);
            startTimer (16);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::red);

            for (auto position : provider())
            {
                g.drawVerticalLine (roundToInt (timeToXPosition (position)), 0.0f, (float) getHeight());
            }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn visible_range_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn time_to_xposition(&self, time: f64) -> f64 {
        
        todo!();
        /*
            return (time - visibleRange.getVisibleRange().getStart()) * getWidth()
                         / visibleRange.getVisibleRange().getLength();
        */
    }
}

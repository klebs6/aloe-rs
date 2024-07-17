crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SlidersComponent<'a> {
    base:              Component<'a>,
    horizontal_slider: Slider<'a>,
    inc_dec_slider:    Slider<'a>,
    rotary_sliders:    [Slider<'a>; 3],
}

impl<'a> Default for SlidersComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            auto setUpSlider = [this] (Slider& slider, Slider::SliderStyle style,
                                           double start, double end, double interval)
                {
                    slider.setSliderStyle (style);
                    slider.setRange (start, end, interval);

                    if (style != Slider::IncDecButtons)
                        slider.setTextBoxStyle (Slider::NoTextBox, false, 0, 0);

                    slider.setValue (start + (end - start) * Random::getSystemRandom().nextDouble());

                    addAndMakeVisible (slider);
                };

                setUpSlider (horizontalSlider, Slider::LinearHorizontal, 1.0, 100.0, 1.0);
                setUpSlider (incDecSlider, Slider::IncDecButtons, 1.0, 10.0, 1.0);

                for (auto& rotary : rotarySliders)
                    setUpSlider (rotary, Slider::Rotary, 1.0, 10.0, 1.0)
        */
    }
}

impl<'a> Resized for SlidersComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (2)) };

                grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)),
                                         Grid::TrackInfo (Grid::Fr (1)),
                                         Grid::TrackInfo (Grid::Fr (1)) };

                grid.items = { GridItem (horizontalSlider).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }),
                               GridItem (incDecSlider).withMargin ({ 2 }) };

                for (auto& rotary : rotarySliders)
                    grid.items.add (GridItem (rotary).withMargin ({ 2 }));

                grid.performLayout (getLocalBounds());
        */
    }
}

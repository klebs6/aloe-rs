crate::ix!();

pub enum RangedValueComponentValueType { 
    numeric, 
    ranged, 
    text 
}

#[no_copy]
#[leak_detector]
pub struct RangedValueComponent<'a> {
    base:                    Component<'a>,
    min_label:               Label<'a>, // default = "Min" 
    max_label:               Label<'a>, // default = "Max" 
    interval_label:          Label<'a>, // default = "Interval" 
    min_value_editor:        TextEditor<'a>,
    max_value_editor:        TextEditor<'a>,
    interval_value_editor:   TextEditor<'a>,
    value_slider:            Slider<'a>,
    custom_widget_component: &'a mut CustomWidgetComponent<'a>,
    value_type_box:          ComboBox<'a>,
    read_only_toggle:        ToggleButton<'a>, // default = "Read-Only" 
    numeric_value_editor:    TextEditor<'a>,
    text_value_editor:       TextEditor<'a>,
    ranged_value_component:  Box<RangedValueComponent<'a>>,
}

impl<'a> Default for RangedValueComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            const auto setUpNumericTextEditor = [this] (TextEditor& ed, double initialValue)
                    {
                        ed.setInputRestrictions (10, "0123456789.");
                        ed.setText (String (initialValue));
                        ed.onReturnKey = [this] { updateSliderRange(); };

                        addAndMakeVisible (ed);
                    };

                    setUpNumericTextEditor (minValueEditor,      0.0);
                    setUpNumericTextEditor (maxValueEditor,      10.0);
                    setUpNumericTextEditor (intervalValueEditor, 0.1);

                    addAndMakeVisible (minLabel);
                    addAndMakeVisible (maxLabel);
                    addAndMakeVisible (intervalLabel);

                    valueSlider.setSliderStyle (Slider::SliderStyle::LinearHorizontal);
                    addAndMakeVisible (valueSlider);
                    updateSliderRange()
        */
    }
}

impl<'a> Resized for RangedValueComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                    grid.templateRows = { Grid::TrackInfo (Grid::Fr (2)), Grid::TrackInfo (Grid::Fr (3)), Grid::TrackInfo (Grid::Fr (3)) };
                    grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)) };

                    grid.items = { GridItem (minLabel).withMargin (2),       GridItem (maxLabel).withMargin (2),       GridItem (intervalLabel).withMargin (2),
                                   GridItem (minValueEditor).withMargin (2), GridItem (maxValueEditor).withMargin (2), GridItem (intervalValueEditor).withMargin (2),
                                   GridItem (valueSlider).withMargin (2).withColumn ({ GridItem::Span (3) }) };

                    grid.performLayout (getLocalBounds());
        */
    }
}
    
impl<'a> RangedValueComponent<'a> {

    pub fn update_slider_range(&mut self)  {
        
        todo!();
        /*
            auto minValue = minValueEditor.getText().getDoubleValue();
                    auto maxValue = maxValueEditor.getText().getDoubleValue();
                    const auto intervalValue = jmax (intervalValueEditor.getText().getDoubleValue(), 0.0001);

                    if (maxValue <= minValue)
                    {
                        maxValue = minValue + intervalValue;
                        maxValueEditor.setText (String (maxValue));
                    }
                    else if (minValue >= maxValue)
                    {
                        minValue = maxValue - intervalValue;
                        minValueEditor.setText (String (minValue));
                    }

                    valueSlider.setRange (minValue, maxValue, intervalValue);
        */
    }
    
    pub fn index_to_value_type(index: i32) -> RangedValueComponentValueType {
        
        todo!();
        /*
            if (index == 1) return RangedValueComponentValueType::numeric;
                if (index == 2) return RangedValueComponentValueType::ranged;
                if (index == 3) return RangedValueComponentValueType::text;

                jassertfalse;
                return RangedValueComponentValueType::numeric;
        */
    }
    
    pub fn update_valueui(&mut self)  {
        
        todo!();
        /*
            const auto valueType = indexToValueType (valueTypeBox.getSelectedId());

                numericValueEditor.setVisible (valueType == RangedValueComponentValueType::numeric);
                textValueEditor.setVisible (valueType == RangedValueComponentValueType::text);
                rangedValueComponent.setVisible (valueType == RangedValueComponentValueType::ranged);

                resized();
        */
    }
}

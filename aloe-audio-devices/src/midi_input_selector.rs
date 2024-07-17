crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MidiInputSelectorComponentListBox<'a> {
    base:             ListBox<'a>,
    base2:            ListBoxModel,
    device_manager:   &'a mut AudioDeviceManager<'a>,
    no_items_message: String,
    items:            Vec<MidiDeviceInfo>,
}

impl<'a> MidiInputSelectorComponentListBox<'a> {

    pub fn new(
        dm:       &mut AudioDeviceManager,
        no_items: &String) -> Self {
    
        todo!();
        /*


            : ListBox ({}, nullptr),
              deviceManager (dm),
              noItemsMessage (noItems)
            updateDevices();
            setModel (this);
            setOutlineThickness (1);
        */
    }
    
    pub fn update_devices(&mut self)  {
        
        todo!();
        /*
            items = MidiInput::getAvailableDevices();
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return items.size();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row:             i32,
        g:               &mut Graphics,
        width:           i32,
        height:          i32,
        row_is_selected: bool)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (row, items.size()))
            {
                if (rowIsSelected)
                    g.fillAll (findColour (TextEditor::highlightColourId)
                                   .withMultipliedAlpha (0.3f));

                auto item = items[row];
                bool enabled = deviceManager.isMidiInputDeviceEnabled (item.identifier);

                auto x = getTickX();
                auto tickW = (float) height * 0.75f;

                getLookAndFeel().drawTickBox (g, *this, (float) x - tickW, ((float) height - tickW) * 0.5f, tickW, tickW,
                                              enabled, true, true, false);

                drawTextLayout (g, *this, item.name, { x + 5, 0, width - x - 5, height }, enabled);
            }
        */
    }
    
    pub fn list_box_item_clicked(&mut self, 
        row: i32,
        e:   &MouseEvent)  {
        
        todo!();
        /*
            selectRow (row);

            if (e.x < getTickX())
                flipEnablement (row);
        */
    }
    
    pub fn list_box_item_double_clicked(&mut self, 
        row: i32,
        _1:  &MouseEvent)  {
        
        todo!();
        /*
            flipEnablement (row);
        */
    }
    
    pub fn return_key_pressed(&mut self, row: i32)  {
        
        todo!();
        /*
            flipEnablement (row);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            ListBox::paint (g);

            if (items.isEmpty())
            {
                g.setColour (Colours::grey);
                g.setFont (0.5f * (float) getRowHeight());
                g.drawText (noItemsMessage,
                            0, 0, getWidth(), getHeight() / 2,
                            Justification::centred, true);
            }
        */
    }
    
    pub fn get_best_height(&mut self, preferred_height: i32) -> i32 {
        
        todo!();
        /*
            auto extra = getOutlineThickness() * 2;

            return jmax (getRowHeight() * 2 + extra,
                         jmin (getRowHeight() * getNumRows() + extra,
                               preferredHeight));
        */
    }
    
    pub fn flip_enablement(&mut self, row: i32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (row, items.size()))
            {
                auto identifier = items[row].identifier;
                deviceManager.setMidiInputDeviceEnabled (identifier, ! deviceManager.isMidiInputDeviceEnabled (identifier));
            }
        */
    }
    
    pub fn get_tickx(&self) -> i32 {
        
        todo!();
        /*
            return getRowHeight();
        */
    }
}

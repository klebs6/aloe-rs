crate::ix!();

pub struct MidiDeviceListBox<'a> {
    base:                ListBox<'a>,
    base2:               ListBoxModel,
    parent:              &'a mut MidiDemo<'a>,
    is_input:            bool,
    last_selected_items: SparseSet<i32>,
}

impl<'a> MidiDeviceListBox<'a> {
    
    pub fn new(
        name:                 &String,
        content_component:    &mut MidiDemo,
        is_input_device_list: bool) -> Self {
    
        todo!();
        /*
        : list_box(name, this),
        : parent(contentComponent),
        : is_input(isInputDeviceList),

            setOutlineThickness (1);
                setMultipleSelectionEnabled (true);
                setClickingTogglesRowSelection (true);
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return isInput ? parent.getNumMidiInputs()
                               : parent.getNumMidiOutputs();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row_number:      i32,
        g:               &mut Graphics,
        width:           i32,
        height:          i32,
        row_is_selected: bool)  {
        
        todo!();
        /*
            auto textColour = getLookAndFeel().findColour (ListBox::textColourId);

                if (rowIsSelected)
                    g.fillAll (textColour.interpolatedWith (getLookAndFeel().findColour (ListBox::backgroundColourId), 0.5));


                g.setColour (textColour);
                g.setFont ((float) height * 0.7f);

                if (isInput)
                {
                    if (rowNumber < parent.getNumMidiInputs())
                        g.drawText (parent.getMidiDevice (rowNumber, true)->deviceInfo.name,
                                    5, 0, width, height,
                                    Justification::centredLeft, true);
                }
                else
                {
                    if (rowNumber < parent.getNumMidiOutputs())
                        g.drawText (parent.getMidiDevice (rowNumber, false)->deviceInfo.name,
                                    5, 0, width, height,
                                    Justification::centredLeft, true);
                }
        */
    }
    
    pub fn selected_rows_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
            auto newSelectedItems = getSelectedRows();
                if (newSelectedItems != lastSelectedItems)
                {
                    for (auto i = 0; i < lastSelectedItems.size(); ++i)
                    {
                        if (! newSelectedItems.contains (lastSelectedItems[i]))
                            parent.closeDevice (isInput, lastSelectedItems[i]);
                    }

                    for (auto i = 0; i < newSelectedItems.size(); ++i)
                    {
                        if (! lastSelectedItems.contains (newSelectedItems[i]))
                            parent.openDevice (isInput, newSelectedItems[i]);
                    }

                    lastSelectedItems = newSelectedItems;
                }
        */
    }
    
    pub fn sync_selected_items_with_device_list(&mut self, midi_devices: &ReferenceCountedArray<MidiDeviceListEntry>)  {
        
        todo!();
        /*
            SparseSet<int> selectedRows;
                for (auto i = 0; i < midiDevices.size(); ++i)
                    if (midiDevices[i]->inDevice.get() != nullptr || midiDevices[i]->outDevice.get() != nullptr)
                        selectedRows.addRange (Range<int> (i, i + 1));

                lastSelectedItems = selectedRows;
                updateContent();
                setSelectedRows (selectedRows, dontSendNotification);
        */
    }
}

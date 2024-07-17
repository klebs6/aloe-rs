crate::ix!();

pub struct MidiTable<'a> {
    base:     Component<'a>,
    messages: &'a mut MidiListModel,
    table:    TableListBox<'a>,
}

impl<'a> TableListBoxModel for MidiTable<'a> {

}

impl<'a> TableListBoxSelectedRowsChanged for MidiTable<'a> {

}

impl<'a> TableListBoxGetCellTooltip for MidiTable<'a> {

}

impl<'a> TableListBoxGetColumnAutoSizeWidth for MidiTable<'a> {

}

impl<'a> TableListBoxSortOrderChanged for MidiTable<'a> {

}

impl<'a> TableListBoxBackgroundClicked for MidiTable<'a> {

}

impl<'a> TableListBoxCellDoubleClicked for MidiTable<'a> {

}

impl<'a> TableListBoxCellClicked for MidiTable<'a> {

}

impl<'a> TableListBoxRefreshComponentForCell for MidiTable<'a> {

}

impl<'a> TableListBoxPaintRowBackground for MidiTable<'a> {

    fn paint_row_background(&mut self, 
        _0: &mut Graphics,
        _1: i32,
        _2: i32,
        _3: i32,
        _4: bool)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> TableListBoxGetNumRows for MidiTable<'a> {

    fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return (int) messages.size();
        */
    }
}

impl<'a> TableListBoxReturnKeyPressed for MidiTable<'a> {

}

impl<'a> TableListBoxDeleteKeyPressed for MidiTable<'a> {

}

impl<'a> TableListBoxGetDragSourceDescription for MidiTable<'a> {

}

impl<'a> TableListBoxListWasScrolled for MidiTable<'a> {

}

impl<'a> TableListBoxPaintCell for MidiTable<'a> {

    fn paint_cell(&mut self, 
        _0: &mut Graphics,
        _1: i32,
        _2: i32,
        _3: i32,
        _4: i32,
        _5: bool)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for MidiTable<'a> {

    fn drop(&mut self) {
        todo!();
        /*      messages.onChange = nullptr;  */
    }
}

pub const MIDI_TABLE_MESSAGE_COLUMN: usize = 1;
pub const MIDI_TABLE_CHANNEL_COLUMN: usize = 2;
pub const MIDI_TABLE_DATA_COLUMN:    usize = 3;

impl<'a> MidiTable<'a> {
    
    pub fn new(m: &mut MidiListModel) -> Self {
    
        todo!();
        /*
        : messages(m),

            addAndMakeVisible (table);

            table.setModel (this);
            table.setClickingTogglesRowSelection (false);
            table.setHeader ([&]
            {
                auto header = std::make_unique<TableHeaderComponent>();
                header->addColumn ("Message", messageColumn, 200, 30, -1, TableHeaderComponent::notSortable);
                header->addColumn ("Channel", channelColumn, 100, 30, -1, TableHeaderComponent::notSortable);
                header->addColumn ("Data",    dataColumn,    200, 30, -1, TableHeaderComponent::notSortable);
                return header;
            }());

            messages.onChange = [&] { table.updateContent(); };
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            table.setBounds (getLocalBounds());
        */
    }
    
    pub fn refresh_component_for_cell(&mut self, 
        row_number:                   i32,
        column_id:                    i32,
        _2:                           bool,
        existing_component_to_update: *mut Component) -> *mut Component {
        
        todo!();
        /*
            delete existingComponentToUpdate;

            const auto index = (int) messages.size() - 1 - rowNumber;
            const auto message = messages[(size_t) index];

            return new Label ({}, [&]
            {
                switch (columnId)
                {
                    case messageColumn: return getEventString (message);
                    case channelColumn: return String (message.getChannel());
                    case dataColumn:    return getDataString (message);
                    default:            break;
                }

                jassertfalse;
                return String();
            }());
        */
    }
    
    pub fn get_event_string(m: &MidiMessage) -> String {
        
        todo!();
        /*
            if (m.isNoteOn())           return "Note on";
            if (m.isNoteOff())          return "Note off";
            if (m.isProgramChange())    return "Program change";
            if (m.isPitchWheel())       return "Pitch wheel";
            if (m.isAftertouch())       return "Aftertouch";
            if (m.isChannelPressure())  return "Channel pressure";
            if (m.isAllNotesOff())      return "All notes off";
            if (m.isAllSoundOff())      return "All sound off";
            if (m.isMetaEvent())        return "Meta event";

            if (m.isController())
            {
                const auto* name = MidiMessage::getControllerName (m.getControllerNumber());
                return "Controller " + (name == nullptr ? String (m.getControllerNumber()) : String (name));
            }

            return String::toHexString (m.getRawData(), m.getRawDataSize());
        */
    }
    
    pub fn get_data_string(m: &MidiMessage) -> String {
        
        todo!();
        /*
            if (m.isNoteOn())           return MidiMessage::getMidiNoteName (m.getNoteNumber(), true, true, 3) + " Velocity " + String (m.getVelocity());
            if (m.isNoteOff())          return MidiMessage::getMidiNoteName (m.getNoteNumber(), true, true, 3) + " Velocity " + String (m.getVelocity());
            if (m.isProgramChange())    return String (m.getProgramChangeNumber());
            if (m.isPitchWheel())       return String (m.getPitchWheelValue());
            if (m.isAftertouch())       return MidiMessage::getMidiNoteName (m.getNoteNumber(), true, true, 3) +  ": " + String (m.getAfterTouchValue());
            if (m.isChannelPressure())  return String (m.getChannelPressureValue());
            if (m.isController())       return String (m.getControllerValue());

            return {};
        */
    }
}

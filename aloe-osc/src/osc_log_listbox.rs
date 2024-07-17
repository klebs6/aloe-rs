crate::ix!();

#[no_copy]
#[leak_detector]
pub struct OSCLogListBox<'a> {
    base:         ListBox<'a>,
    base2:        ListBoxModel,
    base3:        AsyncUpdater<'a>,
    osc_log_list: StringArray,
}

impl<'a> Default for OSCLogListBox<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setModel (this)
        */
    }
}

impl<'a> OSCLogListBox<'a> {

    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return oscLogList.size();
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
            ignoreUnused (rowIsSelected);

            if (isPositiveAndBelow (row, oscLogList.size()))
            {
                g.setColour (Colours::white);

                g.drawText (oscLogList[row],
                            Rectangle<int> (width, height).reduced (4, 0),
                            Justification::centredLeft, true);
            }
        */
    }
    
    pub fn add_osc_message(
        &mut self, 
        message: &OSCMessage,
        level:   Option<i32>
    ) {

        let level: i32 = level.unwrap_or(0);

        todo!();
        /*
            oscLogList.add (getIndentationString (level)
                            + "- osc message, address = '"
                            + message.getAddressPattern().toString()
                            + "', "
                            + String (message.size())
                            + " argument(s)");

            if (! message.isEmpty())
            {
                for (auto& arg : message)
                    addOSCMessageArgument (arg, level + 1);
            }

            triggerAsyncUpdate();
        */
    }
    
    pub fn add_osc_bundle(
        &mut self, 
        bundle: &OSCBundle,
        level:  Option<i32>
    ) {

        let level: i32 = level.unwrap_or(0);

        todo!();
        /*
            OSCTimeTag timeTag = bundle.getTimeTag();

            oscLogList.add (getIndentationString (level)
                            + "- osc bundle, time tag = "
                            + timeTag.toTime().toString (true, true, true, true));

            for (auto& element : bundle)
            {
                if (element.isMessage())
                    addOSCMessage (element.getMessage(), level + 1);
                else if (element.isBundle())
                    addOSCBundle (element.getBundle(), level + 1);
            }

            triggerAsyncUpdate();
        */
    }
    
    pub fn add_osc_message_argument(&mut self, 
        arg:   &OSCArgument,
        level: i32)  {
        
        todo!();
        /*
            String typeAsString;
            String valueAsString;

            if (arg.isFloat32())
            {
                typeAsString = "float32";
                valueAsString = String (arg.getFloat32());
            }
            else if (arg.isInt32())
            {
                typeAsString = "int32";
                valueAsString = String (arg.getInt32());
            }
            else if (arg.isString())
            {
                typeAsString = "string";
                valueAsString = arg.getString();
            }
            else if (arg.isBlob())
            {
                typeAsString = "blob";
                auto& blob = arg.getBlob();
                valueAsString = String::fromUTF8 ((const char*) blob.getData(), (int) blob.getSize());
            }
            else
            {
                typeAsString = "(unknown)";
            }

            oscLogList.add (getIndentationString (level + 1) + "- " + typeAsString.paddedRight(' ', 12) + valueAsString);
        */
    }
    
    pub fn add_invalid_osc_packet(&mut self, 
        data:      *const u8,
        data_size: i32)  {
        
        todo!();
        /*
            oscLogList.add ("- (" + String(dataSize) + "bytes with invalid format)");
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            oscLogList.clear();
            triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            updateContent();
            scrollToEnsureRowIsOnscreen (oscLogList.size() - 1);
            repaint();
        */
    }
    
    pub fn get_indentation_string(level: i32) -> String {
        
        todo!();
        /*
            return String().paddedRight (' ', 2 * level);
        */
    }
}


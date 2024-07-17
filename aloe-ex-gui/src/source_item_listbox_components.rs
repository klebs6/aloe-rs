crate::ix!();

pub struct SourceItemListboxContents {
    base: ListBoxModel,
}

impl SourceItemListboxContents {

    /**
      | The following methods implement the
      | necessary virtual functions from
      | ListBoxModel, telling the listbox how many
      | rows there are, painting them, etc.
      */
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return 30;
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
            if (rowIsSelected)
                    g.fillAll (Colours::lightblue);

                g.setColour (LookAndFeel::getDefaultLookAndFeel().findColour (Label::textColourId));
                g.setFont ((float) height * 0.7f);

                g.drawText ("Draggable Thing #" + String (rowNumber + 1),
                            5, 0, width, height,
                            Justification::centredLeft, true);
        */
    }
    
    pub fn get_drag_source_description(&mut self, selected_rows: &SparseSet<i32>) -> Var {
        
        todo!();
        /*
            // for our drag description, we'll just make a comma-separated list of the selected row
                // numbers - this will be picked up by the drag target and displayed in its box.
                StringArray rows;

                for (int i = 0; i < selectedRows.size(); ++i)
                    rows.add (String (selectedRows[i] + 1));

                return rows.joinIntoString (", ");
        */
    }
}

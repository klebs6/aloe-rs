crate::ix!();

/**
  | This is a custom Label component, which
  | we use for the table's editable text
  | columns.
  |
  */
pub struct EditableTextCustomComponent<'a> {
    base:        Label<'a>,
    owner:       &'a mut TableDemoComponent<'a>,
    row:         i32,
    column_id:   i32,
    text_colour: Colour,
}

impl<'a> EditableTextCustomComponent<'a> {

    pub fn new(td: &mut TableDemoComponent) -> Self {
    
        todo!();
        /*
        : owner(td),

            // double click to edit the label text; single click handled below
                setEditable (false, true, false);
        */
    }
    
    pub fn mouse_down(&mut self, event: &MouseEvent)  {
        
        todo!();
        /*
            // single click on the label should simply select the row
                owner.table.selectRowsBasedOnModifierKeys (row, event.mods, false);

                Label::mouseDown (event);
        */
    }
    
    pub fn text_was_edited(&mut self)  {
        
        todo!();
        /*
            owner.setText (columnId, row, getText());
        */
    }

    /**
      | Our demo code will call this when we may
      | need to update our contents
      |
      */
    pub fn set_row_and_column(&mut self, 
        new_row:    i32,
        new_column: i32)  {
        
        todo!();
        /*
            row = newRow;
                columnId = newColumn;
                setText (owner.getText(columnId, row), dontSendNotification);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();
                if (! dynamic_cast<LookAndFeel_V4*> (&lf))
                    lf.setColour (textColourId, Colours::black);

                Label::paint (g);
        */
    }
}

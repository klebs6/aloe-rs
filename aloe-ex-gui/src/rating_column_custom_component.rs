crate::ix!();

/**
  | This is a custom component containing
  | a combo box, which we're going to put
  | inside our table's "rating" column.
  */
pub struct RatingColumnCustomComponent<'a> {
    base:      Component<'a>,
    owner:     &'a mut TableDemoComponent<'a>,
    combo_box: ComboBox<'a>,
    row:       i32,
    column_id: i32,
}

impl<'a> RatingColumnCustomComponent<'a> {

    pub fn new(td: &mut TableDemoComponent) -> Self {
    
        todo!();
        /*
        : owner(td),

            // just put a combo box inside this component
                addAndMakeVisible (comboBox);
                comboBox.addItem ("fab",        1);
                comboBox.addItem ("groovy",     2);
                comboBox.addItem ("hep",        3);
                comboBox.addItem ("mad for it", 4);
                comboBox.addItem ("neat",       5);
                comboBox.addItem ("swingin",    6);
                comboBox.addItem ("wild",       7);

                comboBox.onChange = [this] { owner.setRating (row, comboBox.getSelectedId()); };
                comboBox.setWantsKeyboardFocus (false);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            comboBox.setBoundsInset (BorderSize<int> (2));
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
                comboBox.setSelectedId (owner.getRating (row), dontSendNotification);
        */
    }
}

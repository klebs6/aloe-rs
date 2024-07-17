crate::ix!();

/**
  | Demonstrates how to put a custom component
  | into a toolbar - this one contains a ComboBox.
  |
  */
pub struct CustomToolbarComboBox<'a> {
    combo_box: ComboBox<'a>, // default = { "demo toolbar combo box"  }
}

impl<'a> ToolbarItemComponent for CustomToolbarComboBox<'a> {

    fn create_item(&mut self, _: i32) -> *mut (dyn aloe_toolbar::ToolbarItemComponent + 'static) { todo!() }
}

impl<'a> CustomToolbarComboBox<'a> {

    pub fn new(toolbar_item_id: i32) -> Self {
    
        todo!();
        /*
            : ToolbarItemComponent (toolbarItemId, "Custom Toolbar Item", false)

                    addAndMakeVisible (comboBox);

                    for (int i = 1; i < 20; ++i)
                        comboBox.addItem ("Toolbar ComboBox item " + String (i), i);

                    comboBox.setSelectedId (1);
                    comboBox.setEditableText (true);
        */
    }
    
    pub fn get_toolbar_item_sizes(&mut self, 
        toolbar_depth:  i32,
        is_vertical:    bool,
        preferred_size: &mut i32,
        min_size:       &mut i32,
        max_size:       &mut i32) -> bool {
        
        todo!();
        /*
            if (isVertical)
                        return false;

                    preferredSize = 250;
                    minSize = 80;
                    maxSize = 300;
                    return true;
        */
    }
    
    pub fn paint_button_area(&mut self, 
        _0: &mut Graphics,
        _1: i32,
        _2: i32,
        _3: bool,
        _4: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn content_area_changed(&mut self, new_area: &Rectangle<i32>)  {
        
        todo!();
        /*
            comboBox.setSize (newArea.getWidth() - 2,
                                      jmin (newArea.getHeight() - 2, 22));

                    comboBox.setCentrePosition (newArea.getCentreX(), newArea.getCentreY());
        */
    }
}

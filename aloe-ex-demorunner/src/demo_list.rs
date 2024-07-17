crate::ix!();

pub struct DemoList<'a> {
    base:              Component<'a>,
    base2:             ListBoxModel,
    selected_category: String,
    demo_holder:       &'a mut DemoContentComponent<'a>,
    demos:             ListBox<'a>,
}

impl<'a> DemoList<'a> {

    pub fn new(holder: &mut DemoContentComponent) -> Self {
    
        todo!();
        /*
        : demo_holder(holder),

            addAndMakeVisible (demos);
            demos.setModel (this);
            demos.setRowHeight (40);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            demos.setBounds (getLocalBounds());
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
            Rectangle<int> bounds (0, 0, width, height);

            auto textColour = findColour (Label::textColourId);

            g.setColour (textColour.withAlpha (0.4f));

            if (rowNumber == 0)
                g.fillRect (bounds.removeFromTop (2).reduced (7, 0));

            g.fillRect (bounds.removeFromBottom (2).reduced (7, 0));

            if (rowIsSelected)
            {
                g.setColour (findColour (TextEditor::highlightColourId).withAlpha (0.4f));
                g.fillRect (bounds);
                textColour = findColour (TextEditor::highlightedTextColourId);
            }

            g.setColour (textColour);
            g.drawFittedText (getNameForRow (rowNumber), bounds, Justification::centred, 1);
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return (int) (selectedCategory.isEmpty() ? ALOEDemos::getCategories().size()
                                                     : ALOEDemos::getCategory (selectedCategory).demos.size());
        */
    }
    
    pub fn get_name_for_row(&mut self, row_number: i32) -> String {
        
        todo!();
        /*
            if (selectedCategory.isEmpty())
            {
                if (isPositiveAndBelow (rowNumber, ALOEDemos::getCategories().size()))
                    return ALOEDemos::getCategories()[(size_t) rowNumber].name;
            }
            else
            {
                auto& category = ALOEDemos::getCategory (selectedCategory);

                if (isPositiveAndBelow (rowNumber, category.demos.size()))
                    return category.demos[(size_t) rowNumber].demoFile.getFileName();
            }

            return {};
        */
    }
    
    pub fn return_key_pressed(&mut self, row: i32)  {
        
        todo!();
        /*
            selectRow (row);
        */
    }
    
    pub fn list_box_item_clicked(&mut self, 
        row: i32,
        _1:  &MouseEvent)  {
        
        todo!();
        /*
            selectRow (row);
        */
    }
    
    pub fn show_category(&mut self, category_name: &String)  {
        
        todo!();
        /*
            selectedCategory = categoryName;

            demos.deselectAllRows();
            demos.setHeaderComponent (categoryName.isEmpty() ? nullptr
                                                             : std::make_unique<CategoryListHeaderComponent> (*this));
            demos.updateContent();
        */
    }
    
    pub fn select_row(&mut self, row: i32)  {
        
        todo!();
        /*
            if (row < 0)
                return;

            if (selectedCategory.isEmpty())
                showCategory (ALOEDemos::getCategories()[(size_t) row].name);
            else
                demoHolder.setDemo (selectedCategory, row);

            if (demos.isShowing())
                selectFirstRow();
        */
    }
    
    pub fn select_first_row(&mut self)  {
        
        todo!();
        /*
            if (auto* handler = demos.getAccessibilityHandler())
            {
                for (auto* child : handler->getChildren())
                {
                    if (child->getRole() == AccessibilityRole::listItem)
                    {
                        child->grabFocus();
                        break;
                    }
                }
            }
        */
    }
}

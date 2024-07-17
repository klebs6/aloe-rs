crate::ix!();

/**
  | This class shows how to implement a TableListBoxModel
  | to show in a TableListBox.
  |
  */
#[no_copy]
#[leak_detector]
pub struct TableDemoComponent<'a> {
    base:        Component<'a>,

    /**
      | the table component itself
      |
      */
    table:       TableListBox<'a>,

    font:        Font, // default = { 14.0f  }

    /**
      | This is the XML document loaded from
      | the embedded file "demo table data.xml"
      |
      */
    demo_data:   Box<XmlElement>,

    /**
      | A pointer to the sub-node of demoData
      | that contains the list of columns
      |
      */
    column_list: *mut XmlElement, // default = nullptr

    /**
      | A pointer to the sub-node of demoData
      | that contains the list of data rows
      |
      */
    data_list:   *mut XmlElement, // default = nullptr

    /**
      | The number of rows of data we've got
      |
      */
    num_rows:    i32,
}

impl<'a> TableListBoxModel for TableDemoComponent<'a> {

}

impl<'a> TableListBoxGetNumRows for TableDemoComponent<'a> {

    /**
      | This is overloaded from TableListBoxModel,
      | and must return the total number of rows
      | in our table
      |
      */
    fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return numRows;
        */
    }
}

impl<'a> TableListBoxPaintRowBackground for TableDemoComponent<'a> {

    /**
      | This is overloaded from TableListBoxModel,
      | and should fill in the background of
      | the whole row
      |
      */
    fn paint_row_background(&mut self, 
        g:               &mut Graphics,
        row_number:      i32,
        width:           i32,
        height:          i32,
        row_is_selected: bool)  {
        
        todo!();
        /*
            auto alternateColour = getLookAndFeel().findColour (ListBox::backgroundColourId)
                                                   .interpolatedWith (getLookAndFeel().findColour (ListBox::textColourId), 0.03f);
            if (rowIsSelected)
                g.fillAll (Colours::lightblue);
            else if (rowNumber % 2)
                g.fillAll (alternateColour);
        */
    }
}

impl<'a> TableListBoxRefreshComponentForCell for TableDemoComponent<'a> {

}

impl<'a> TableListBoxCellClicked for TableDemoComponent<'a> {

}

impl<'a> TableListBoxCellDoubleClicked for TableDemoComponent<'a> {

}

impl<'a> TableListBoxBackgroundClicked for TableDemoComponent<'a> {

}

impl<'a> TableListBoxSortOrderChanged for TableDemoComponent<'a> {

}

impl<'a> TableListBoxGetCellTooltip for TableDemoComponent<'a> {

}

impl<'a> TableListBoxGetColumnAutoSizeWidth for TableDemoComponent<'a> {

}

impl<'a> TableListBoxSelectedRowsChanged for TableDemoComponent<'a> {

}

impl<'a> TableListBoxDeleteKeyPressed for TableDemoComponent<'a> {

}

impl<'a> TableListBoxReturnKeyPressed for TableDemoComponent<'a> {

}

impl<'a> TableListBoxListWasScrolled for TableDemoComponent<'a> {

}

impl<'a> TableListBoxGetDragSourceDescription for TableDemoComponent<'a> {

}

impl<'a> TableListBoxPaintCell for TableDemoComponent<'a> {

    /**
      | This is overloaded from TableListBoxModel,
      | and must paint any cells that aren't
      | using custom components.
      |
      */
    fn paint_cell(
        &mut self, 
        g:               &mut Graphics,
        row_number:      i32,
        column_id:       i32,
        width:           i32,
        height:          i32,
        row_is_selected: bool
    ) {
        
        todo!();
        /*
            g.setColour (getLookAndFeel().findColour (ListBox::textColourId));
            g.setFont (font);

            if (auto* rowElement = dataList->getChildElement (rowNumber))
            {
                auto text = rowElement->getStringAttribute (getAttributeNameForColumnId (columnId));

                g.drawText (text, 2, 0, width - 4, height, Justification::centredLeft, true);
            }

            g.setColour (getLookAndFeel().findColour (ListBox::backgroundColourId));
            g.fillRect (width - 1, 0, 1, height);
        */
    }
}

impl<'a> Default for TableDemoComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            // Load some data from an embedded XML file..
            loadData();

            // Create our table component and add it to this component..
            addAndMakeVisible (table);
            table.setModel (this);

            // give it a border
            table.setColour (ListBox::outlineColourId, Colours::grey);
            table.setOutlineThickness (1);

            // Add some columns to the table header, based on the column list in our database..
            for (auto* columnXml : columnList->getChildIterator())
            {
                table.getHeader().addColumn (columnXml->getStringAttribute ("name"),
                                             columnXml->getIntAttribute ("columnId"),
                                             columnXml->getIntAttribute ("width"),
                                             50, 400,
                                             TableHeaderComponent::defaultFlags);
            }

            // we could now change some initial settings..
            table.getHeader().setSortColumnId (1, true); // sort forwards by the ID column
            table.getHeader().setColumnVisible (7, false); // hide the "length" column until the user shows it

            // un-comment this line to have a go of stretch-to-fit mode
            // table.getHeader().setStretchToFitActive (true);

            table.setMultipleSelectionEnabled (true)
        */
    }
}

impl<'a> TableDemoComponent<'a> {

    /**
      | This is overloaded from TableListBoxModel,
      | and tells us that the user has clicked
      | a table header to change the sort order.
      |
      */
    pub fn sort_order_changed(&mut self, 
        new_sort_column_id: i32,
        is_forwards:        bool)  {
        
        todo!();
        /*
            if (newSortColumnId != 0)
            {
                DemoDataSorter sorter (getAttributeNameForColumnId (newSortColumnId), isForwards);
                dataList->sortChildElements (sorter);

                table.updateContent();
            }
        */
    }

    /**
      | This is overloaded from TableListBoxModel,
      | and must update any custom components
      | that we're using
      |
      */
    pub fn refresh_component_for_cell(&mut self, 
        row_number:                   i32,
        column_id:                    i32,
        is_row_selected:              bool,
        existing_component_to_update: *mut Component) -> *mut Component {
        
        todo!();
        /*
            if (columnId == 1 || columnId == 7) // The ID and Length columns do not have a custom component
            {
                jassert (existingComponentToUpdate == nullptr);
                return nullptr;
            }

            if (columnId == 5) // For the ratings column, we return the custom combobox component
            {
                auto* ratingsBox = static_cast<RatingColumnCustomComponent*> (existingComponentToUpdate);

                // If an existing component is being passed-in for updating, we'll re-use it, but
                // if not, we'll have to create one.
                if (ratingsBox == nullptr)
                    ratingsBox = new RatingColumnCustomComponent (*this);

                ratingsBox->setRowAndColumn (rowNumber, columnId);
                return ratingsBox;
            }

            // The other columns are editable text columns, for which we use the custom Label component
            auto* textLabel = static_cast<EditableTextCustomComponent*> (existingComponentToUpdate);

            // same as above...
            if (textLabel == nullptr)
                textLabel = new EditableTextCustomComponent (*this);

            textLabel->setRowAndColumn (rowNumber, columnId);
            return textLabel;
        */
    }

    /**
      | This is overloaded from TableListBoxModel,
      | and should choose the best width for
      | the specified column.
      |
      */
    pub fn get_column_auto_size_width(&mut self, column_id: i32) -> i32 {
        
        todo!();
        /*
            if (columnId == 5)
                return 100; // (this is the ratings column, containing a custom combobox component)

            int widest = 32;

            // find the widest bit of text in this column..
            for (int i = getNumRows(); --i >= 0;)
            {
                if (auto* rowElement = dataList->getChildElement (i))
                {
                    auto text = rowElement->getStringAttribute (getAttributeNameForColumnId (columnId));

                    widest = jmax (widest, font.getStringWidth (text));
                }
            }

            return widest + 8;
        */
    }

    /**
      | A couple of quick methods to set and get
      | cell values when the user changes them
      |
      */
    pub fn get_rating(&self, row_number: i32) -> i32 {
        
        todo!();
        /*
            return dataList->getChildElement (rowNumber)->getIntAttribute ("Rating");
        */
    }
    
    pub fn set_rating(&mut self, 
        row_number: i32,
        new_rating: i32)  {
        
        todo!();
        /*
            dataList->getChildElement (rowNumber)->setAttribute ("Rating", newRating);
        */
    }
    
    pub fn get_text(&self, 
        column_number: i32,
        row_number:    i32) -> String {
        
        todo!();
        /*
            return dataList->getChildElement (rowNumber)->getStringAttribute ( getAttributeNameForColumnId(columnNumber));
        */
    }
    
    pub fn set_text(&mut self, 
        column_number: i32,
        row_number:    i32,
        new_text:      &String)  {
        
        todo!();
        /*
            auto columnName = table.getHeader().getColumnName (columnNumber);
            dataList->getChildElement (rowNumber)->setAttribute (columnName, newText);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // position our table with a gap around its edge
            table.setBoundsInset (BorderSize<int> (8));
        */
    }

    /**
      | this loads the embedded database XML
      | file into memory
      |
      */
    pub fn load_data(&mut self)  {
        
        todo!();
        /*
            demoData = parseXML (loadEntireAssetIntoString ("demo table data.xml"));

            dataList   = demoData->getChildByName ("DATA");
            columnList = demoData->getChildByName ("COLUMNS");

            numRows = dataList->getNumChildElements();
        */
    }

    /**
      | (a utility method to search our XML for
      | the attribute that matches a column
      | ID)
      |
      */
    pub fn get_attribute_name_for_column_id(&self, column_id: i32) -> String {
        
        todo!();
        /*
            for (auto* columnXml : columnList->getChildIterator())
            {
                if (columnXml->getIntAttribute ("columnId") == columnId)
                    return columnXml->getStringAttribute ("name");
            }

            return {};
        */
    }
}

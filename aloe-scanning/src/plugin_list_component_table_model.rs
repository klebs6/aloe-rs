crate::ix!();

pub const PLUGIN_LIST_COMPONENT_TABLE_MODEL_NAME_COL:         usize = 1;
pub const PLUGIN_LIST_COMPONENT_TABLE_MODEL_TYPE_COL:         usize = 2;
pub const PLUGIN_LIST_COMPONENT_TABLE_MODEL_CATEGORY_COL:     usize = 3;
pub const PLUGIN_LIST_COMPONENT_TABLE_MODEL_MANUFACTURER_COL: usize = 4;
pub const PLUGIN_LIST_COMPONENT_TABLE_MODEL_DESC_COL:         usize = 5;

///-------------------
#[no_copy]
#[leak_detector]
pub struct PluginListComponentTableModel<'a> {
    owner: &'a mut PluginListComponent<'a>,
    list:  &'a mut KnownPluginList<'a>,
}

impl<'a> TableListBoxModel for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxPaintCell for PluginListComponentTableModel<'a> {

    fn paint_cell(
        &mut self, 
        g:               &mut Graphics,
        row:             i32,
        column_id:       i32,
        width:           i32,
        height:          i32,
        row_is_selected: bool

    ) {
        
        todo!();
        /*
            String text;
                bool isBlacklisted = row >= list.getNumTypes();

                if (isBlacklisted)
                {
                    if (columnId == nameCol)
                        text = list.getBlacklistedFiles() [row - list.getNumTypes()];
                    else if (columnId == descCol)
                        text = TRANS("Deactivated after failing to initialise correctly");
                }
                else
                {
                    auto desc = list.getTypes()[row];

                    switch (columnId)
                    {
                        case nameCol:         text = desc.name; break;
                        case typeCol:         text = desc.pluginFormatName; break;
                        case categoryCol:     text = desc.category.isNotEmpty() ? desc.category : "-"; break;
                        case manufacturerCol: text = desc.manufacturerName; break;
                        case descCol:         text = getPluginDescription (desc); break;

                        default: jassertfalse; break;
                    }
                }

                if (text.isNotEmpty())
                {
                    const auto defaultTextColour = owner.findColour (ListBox::textColourId);
                    g.setColour (isBlacklisted ? Colours::red
                                               : columnId == nameCol ? defaultTextColour
                                                                     : defaultTextColour.interpolatedWith (Colours::transparentBlack, 0.3f));
                    g.setFont (Font ((float) height * 0.7f, Font::bold));
                    g.drawFittedText (text, 4, 0, width - 6, height, Justification::centredLeft, 1, 0.9f);
                }
        */
    }
}

impl<'a> TableListBoxGetDragSourceDescription for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxListWasScrolled for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxReturnKeyPressed for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxDeleteKeyPressed for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxSelectedRowsChanged for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxGetCellTooltip for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxGetColumnAutoSizeWidth for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxSortOrderChanged for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxBackgroundClicked for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxCellDoubleClicked for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxCellClicked for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxRefreshComponentForCell for PluginListComponentTableModel<'a> {

}

impl<'a> TableListBoxPaintRowBackground for PluginListComponentTableModel<'a> {

    fn paint_row_background(
        &mut self, 
        g:               &mut Graphics,
        row_number:      i32,
        width:           i32,
        height:          i32,
        row_is_selected: bool

    ) {
        
        todo!();
        /*
            const auto defaultColour = owner.findColour (ListBox::backgroundColourId);
                const auto c = rowIsSelected ? defaultColour.interpolatedWith (owner.findColour (ListBox::textColourId), 0.5f)
                                             : defaultColour;

                g.fillAll (c);
        */
    }
}

impl<'a> TableListBoxGetNumRows for PluginListComponentTableModel<'a> {

    fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return list.getNumTypes() + list.getBlacklistedFiles().size();
        */
    }
}

impl<'a> PluginListComponentTableModel<'a> {

    pub fn new(
        c: &mut PluginListComponent,
        l: &mut KnownPluginList) -> Self {
    
        todo!();
        /*
        : owner(c),
        : list(l),

        
        */
    }
    
    pub fn cell_clicked(&mut self, 
        row_number: i32,
        column_id:  i32,
        e:          &MouseEvent)  {
        
        todo!();
        /*
            TableListBoxModel::cellClicked (rowNumber, columnId, e);

                if (rowNumber >= 0 && rowNumber < getNumRows() && e.mods.isPopupMenu())
                    owner.createMenuForRow (rowNumber).showMenuAsync (typename PopupMenu::Options().withDeletionCheck (owner));
        */
    }
    
    pub fn delete_key_pressed(&mut self, _0: i32)  {
        
        todo!();
        /*
            owner.removeSelectedPlugins();
        */
    }
    
    pub fn sort_order_changed(&mut self, 
        new_sort_column_id: i32,
        is_forwards:        bool)  {
        
        todo!();
        /*
            switch (newSortColumnId)
                {
                    case nameCol:         list.sort (KnownPluginList::sortAlphabetically, isForwards); break;
                    case typeCol:         list.sort (KnownPluginList::sortByFormat, isForwards); break;
                    case categoryCol:     list.sort (KnownPluginList::sortByCategory, isForwards); break;
                    case manufacturerCol: list.sort (KnownPluginList::sortByManufacturer, isForwards); break;
                    case descCol:         break;

                    default: jassertfalse; break;
                }
        */
    }
    
    pub fn get_plugin_description(desc: &PluginDescription) -> String {
        
        todo!();
        /*
            StringArray items;

                if (desc.descriptiveName != desc.name)
                    items.add (desc.descriptiveName);

                items.add (desc.version);

                items.removeEmptyStrings();
                return items.joinIntoString (" - ");
        */
    }
}

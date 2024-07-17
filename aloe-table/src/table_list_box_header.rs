crate::ix!();

pub const HEADER_AUTO_SIZE_COLUMN_ID: usize = 0xf836743; 
pub const HEADER_AUTO_SIZE_ALL_ID:    usize = 0xf836744;

#[no_copy]
#[leak_detector]
pub struct TableListBoxHeader<'a> {
    base:  TableHeaderComponent<'a>,
    owner: &'a mut TableListBox<'a>,
}

impl<'a> TableListBoxHeader<'a> {

    pub fn new(tlb: &mut TableListBox) -> Self {
    
        todo!();
        /*
        : owner(tlb),
        */
    }
    
    pub fn add_menu_items(&mut self, 
        menu:              &mut PopupMenu,
        column_id_clicked: i32)  {
        
        todo!();
        /*
            if (owner.isAutoSizeMenuOptionShown())
            {
                menu.addItem (autoSizeColumnId, TRANS("Auto-size this column"), columnIdClicked != 0);
                menu.addItem (autoSizeAllId, TRANS("Auto-size all columns"), owner.getHeader().getNumColumns (true) > 0);
                menu.addSeparator();
            }

            TableHeaderComponent::addMenuItems (menu, columnIdClicked);
        */
    }
    
    pub fn react_to_menu_item(&mut self, 
        menu_return_id:    i32,
        column_id_clicked: i32)  {
        
        todo!();
        /*
            switch (menuReturnId)
            {
                case autoSizeColumnId:      owner.autoSizeColumn (columnIdClicked); break;
                case autoSizeAllId:         owner.autoSizeAllColumns(); break;
                default:                    TableHeaderComponent::reactToMenuItem (menuReturnId, columnIdClicked); break;
            }
        */
    }
}


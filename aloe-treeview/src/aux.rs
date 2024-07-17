crate::ix!();

pub fn escape_slashes_in_tree_view_item_name(s: &String) -> String {
    
    todo!();
        /*
            return s.replaceCharacter ('/', '\\');
        */
}

pub fn add_all_selected_item_ids(
        item:   *mut TreeViewItem,
        parent: &mut XmlElement)  {
    
    todo!();
        /*
            if (item->isSelected())
            parent.createNewChildElement ("SELECTED")->setAttribute ("id", item->getItemIdentifierString());

        auto numSubItems = item->getNumSubItems();

        for (int i = 0; i < numSubItems; ++i)
            addAllSelectedItemIds (item->getSubItem (i), parent);
        */
}

pub fn get_item_depth(item: *const TreeViewItem) -> i32 {
    
    todo!();
        /*
            if (item == nullptr || item->getOwnerView() == nullptr)
            return 0;

        auto depth = item->getOwnerView()->isRootItemVisible() ? 0 : -1;

        for (auto* parent = item->getParentItem(); parent != nullptr; parent = parent->getParentItem())
            ++depth;

        return depth;
        */
}

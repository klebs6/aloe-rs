crate::ix!();

/**
  | This handy class takes a copy of a TreeViewItem's
  | openness when you create it, and restores
  | that openness state when its destructor
  | is called.
  | 
  | This can very handy when you're refreshing
  | sub-items - e.g.
  | 
  | -----------
  | @code
  | 
  | void MyTreeViewItem::updateChildItems()
  | {
  |     TreeViewItemOpennessRestorer openness (*this);  //  saves the openness state here..
  | 
  |     clearSubItems();
  | 
  |     // add a bunch of sub-items here which may or may not be the same as the ones that
  |     // were previously there
  |     addSubItem (...
  | 
  |     // ..and at this point, the old openness is restored, so any items that haven't
  |     // changed will have their old openness retained.
  | }
  |
  */
#[no_copy]
#[leak_detector]
pub struct TreeViewItemOpennessRestorer<'a> {
    tree_view_item: &'a mut TreeViewItem<'a>,
    old_openness:   Box<XmlElement>,
}

impl<'a> Drop for TreeViewItemOpennessRestorer<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (oldOpenness != nullptr)
            treeViewItem.restoreOpennessState (*oldOpenness);
        */
    }
}

impl<'a> TreeViewItemOpennessRestorer<'a> {
    
    pub fn new(item: &mut TreeViewItem) -> Self {
    
        todo!();
        /*
        : tree_view_item(item),
        : old_openness(item.getOpennessState()),

        
        */
    }
}

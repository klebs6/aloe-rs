crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/ValueTreesDemo.h]

#[no_copy]
#[leak_detector]
pub struct ValueTreeItem<'a> {
    base:         TreeViewItem<'a>,
    tree:         ValueTree,
    undo_manager: &'a mut UndoManager<'a>,
}

impl<'a> ValueTreeListener for ValueTreeItem<'a> {

}

impl<'a> ValueTreeItem<'a> {

    pub fn new(
        v:  &ValueTree,
        um: &mut UndoManager) -> Self {
    
        todo!();
        /*
        : tree(v),
        : undo_manager(um),

            tree.addListener (this);
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            return tree["name"].toString();
        */
    }
    
    pub fn might_contain_sub_items(&mut self) -> bool {
        
        todo!();
        /*
            return tree.getNumChildren() > 0;
        */
    }
    
    pub fn paint_item(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            if (isSelected())
                g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::highlightedFill,
                                                   Colours::teal));

            g.setColour (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::defaultText,
                                                 Colours::black));
            g.setFont (15.0f);

            g.drawText (tree["name"].toString(),
                        4, 0, width - 4, height,
                        Justification::centredLeft, true);
        */
    }
    
    pub fn item_openness_changed(&mut self, is_now_open: bool)  {
        
        todo!();
        /*
            if (isNowOpen && getNumSubItems() == 0)
                refreshSubItems();
            else
                clearSubItems();
        */
    }
    
    pub fn get_drag_source_description(&mut self) -> Var {
        
        todo!();
        /*
            return "Drag Demo";
        */
    }
    
    pub fn is_interested_in_drag_source(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) -> bool {
        
        todo!();
        /*
            return dragSourceDetails.description == "Drag Demo";
        */
    }
    
    pub fn item_dropped(
        &mut self, 
        _0:           &DragAndDropTargetSourceDetails,
        insert_index: i32
    ) {

        todo!();
        /*
            Vec<Box<ValueTree>> selectedTrees;
            getSelectedTreeViewItems (*getOwnerView(), selectedTrees);

            moveItems (*getOwnerView(), selectedTrees, tree, insertIndex, undoManager);
        */
    }
    
    pub fn move_items(
        tree_view:    &mut TreeView,
        items:        &Vec<Box<ValueTree>>,
        new_parent:   ValueTree,
        insert_index: i32,
        undo_manager: &mut UndoManager)  {
        
        todo!();
        /*
            if (items.size() > 0)
            {
                std::unique_ptr<XmlElement> oldOpenness (treeView.getOpennessState (false));

                for (auto* v : items)
                {
                    if (v->getParent().isValid() && newParent != *v && ! newParent.isAChildOf (*v))
                    {
                        if (v->getParent() == newParent && newParent.indexOf (*v) < insertIndex)
                            --insertIndex;

                        v->getParent().removeChild (*v, &undoManager);
                        newParent.addChild (*v, insertIndex, &undoManager);
                    }
                }

                if (oldOpenness.get() != nullptr)
                    treeView.restoreOpennessState (*oldOpenness, false);
            }
        */
    }
    
    pub fn get_selected_tree_view_items(
        tree_view: &mut TreeView,
        items:     &mut Vec<Box<ValueTree>>)  {
        
        todo!();
        /*
            auto numSelected = treeView.getNumSelectedItems();

            for (int i = 0; i < numSelected; ++i)
                if (auto* vti = dynamic_cast<ValueTreeItem*> (treeView.getSelectedItem (i)))
                    items.add (new ValueTree (vti->tree));
        */
    }
    
    pub fn refresh_sub_items(&mut self)  {
        
        todo!();
        /*
            clearSubItems();

            for (int i = 0; i < tree.getNumChildren(); ++i)
                addSubItem (new ValueTreeItem (tree.getChild (i), undoManager));
        */
    }
    
    pub fn value_tree_property_changed(&mut self, 
        _0: &mut ValueTree,
        _1: &Identifier)  {
        
        todo!();
        /*
            repaintItem();
        */
    }
    
    pub fn value_tree_child_added(&mut self, 
        parent_tree: &mut ValueTree,
        _1:          &mut ValueTree)  {
        
        todo!();
        /*
            treeChildrenChanged (parentTree);
        */
    }
    
    pub fn value_tree_child_removed(&mut self, 
        parent_tree: &mut ValueTree,
        _1:          &mut ValueTree,
        _2:          i32)  {
        
        todo!();
        /*
            treeChildrenChanged (parentTree);
        */
    }
    
    pub fn value_tree_child_order_changed(&mut self, 
        parent_tree: &mut ValueTree,
        _1:          i32,
        _2:          i32)  {
        
        todo!();
        /*
            treeChildrenChanged (parentTree);
        */
    }
    
    pub fn value_tree_parent_changed(&mut self, _0: &mut ValueTree)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn tree_children_changed(&mut self, parent_tree: &ValueTree)  {
        
        todo!();
        /*
            if (parentTree == tree)
            {
                refreshSubItems();
                treeHasChanged();
                setOpen (true);
            }
        */
    }
}

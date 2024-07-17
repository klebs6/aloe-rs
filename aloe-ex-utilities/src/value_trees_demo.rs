crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ValueTreesDemo<'a> {
    base:         Component<'a>,
    base2:        DragAndDropContainer<'a>,
    base3:        Timer,
    tree:         TreeView<'a>,
    undo_button:  TextButton<'a>, // default = { "Undo"  }
    redo_button:  TextButton<'a>, // default = { "Redo"  }
    root_item:    Box<ValueTreeItem<'a>>,
    undo_manager: UndoManager<'a>,
}

impl<'a> Default for ValueTreesDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (tree);

            tree.setTitle ("ValueTree");
            tree.setDefaultOpenness (true);
            tree.setMultiSelectEnabled (true);
            rootItem.reset (new ValueTreeItem (createRootValueTree(), undoManager));
            tree.setRootItem (rootItem.get());

            addAndMakeVisible (undoButton);
            addAndMakeVisible (redoButton);
            undoButton.onClick = [this] { undoManager.undo(); };
            redoButton.onClick = [this] { undoManager.redo(); };

            startTimer (500);

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for ValueTreesDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            tree.setRootItem (nullptr);
         */
    }
}

impl<'a> ValueTreesDemo<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (8);

            auto buttons = r.removeFromBottom (22);
            undoButton.setBounds (buttons.removeFromLeft (100));
            buttons.removeFromLeft (6);
            redoButton.setBounds (buttons.removeFromLeft (100));

            r.removeFromBottom (4);
            tree.setBounds (r);
        */
    }
    
    pub fn create_tree(desc: &String) -> ValueTree {
        
        todo!();
        /*
            ValueTree t ("Item");
            t.setProperty ("name", desc, nullptr);
            return t;
        */
    }
    
    pub fn create_root_value_tree() -> ValueTree {
        
        todo!();
        /*
            auto vt = createTree ("This demo displays a ValueTree as a treeview.");
            vt.appendChild (createTree ("You can drag around the nodes to rearrange them"),               nullptr);
            vt.appendChild (createTree ("..and press 'delete' or 'backspace' to delete them"),            nullptr);
            vt.appendChild (createTree ("Then, you can use the undo/redo buttons to undo these changes"), nullptr);

            int n = 1;
            vt.appendChild (createRandomTree (n, 0), nullptr);

            return vt;
        */
    }
    
    pub fn create_random_tree(
        counter: &mut i32,
        depth:   i32) -> ValueTree {
        
        todo!();
        /*
            auto t = createTree ("Item " + String (counter++));

            if (depth < 3)
                for (int i = 1 + Random::getSystemRandom().nextInt (7); --i >= 0;)
                    t.appendChild (createRandomTree (counter, depth + 1), nullptr);

            return t;
        */
    }
    
    pub fn delete_selected_items(&mut self)  {
        
        todo!();
        /*
            Vec<Box<ValueTree>> selectedItems;
            ValueTreeItem::getSelectedTreeViewItems (tree, selectedItems);

            for (auto* v : selectedItems)
            {
                if (v->getParent().isValid())
                    v->getParent().removeChild (*v, &undoManager);
            }
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (key == KeyPress::deleteKey || key == KeyPress::backspaceKey)
            {
                deleteSelectedItems();
                return true;
            }

            if (key == KeyPress ('z', ModifierKeys::commandModifier, 0))
            {
                undoManager.undo();
                return true;
            }

            if (key == KeyPress ('z', ModifierKeys::commandModifier | ModifierKeys::shiftModifier, 0))
            {
                undoManager.redo();
                return true;
            }

            return Component::keyPressed (key);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            undoManager.beginNewTransaction();
        */
    }
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileTreeComponent.h]

/**
  | A component that displays the files
  | in a directory as a treeview.
  | 
  | This implements the DirectoryContentsDisplayComponent
  | base class so that it can be used in a FileBrowserComponent.
  | 
  | To attach a listener to it, use its DirectoryContentsDisplayComponent
  | base class and the FileBrowserListener
  | class.
  | 
  | @see DirectoryContentsList, FileListComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileTreeComponent<'a> {
    base:                      TreeView<'a>,
    base2:                     DirectoryContentsDisplayComponent<'a>,
    drag_and_drop_description: String,
    item_height:               i32,
}

impl<'a> Drop for FileTreeComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        deleteRootItem();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileTreeComponent.cpp]
impl<'a> FileTreeComponent<'a> {
    
    /**
      | Returns the number of files the user
      | has got selected. @see getSelectedFile
      |
      */
    pub fn get_num_selected_files(&self) -> i32 {
        
        todo!();
        /*
            return TreeView::getNumSelectedItems();
        */
    }

    /**
      | Returns the last value that was set by
      | setDragAndDropDescription().
      |
      */
    pub fn get_drag_and_drop_description(&self) -> &String {
        
        todo!();
        /*
            return dragAndDropDescription;
        */
    }

    /**
      | Returns the height of the treeview items.
      |
      */
    pub fn get_item_height(&self) -> i32 {
        
        todo!();
        /*
            return itemHeight;
        */
    }
    
    /**
      | Creates a listbox to show the contents
      | of a specified directory.
      |
      */
    pub fn new(list_to_show: &mut DirectoryContentsList) -> Self {
    
        todo!();
        /*
        : directory_contents_display_component(listToShow),
        : item_height(22),

            setRootItemVisible (false);
        refresh();
        */
    }
    
    /**
      | Updates the files in the list.
      |
      */
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            deleteRootItem();

        auto root = new FileListTreeItem (*this, nullptr, 0, directoryContentsList.getDirectory(),
                                          directoryContentsList.getTimeSliceThread());

        root->setSubContentsList (&directoryContentsList, false);
        setRootItem (root);
        */
    }
    
    /**
      | Returns one of the files that the user
      | has currently selected.
      | 
      | The index should be in the range 0 to (getNumSelectedFiles()
      | - 1). @see getNumSelectedFiles
      |
      */
    pub fn get_selected_file(&self, index: Option<i32>) -> File {

        let index: i32 = index.unwrap_or(0);
        
        todo!();
        /*
            if (auto* item = dynamic_cast<const FileListTreeItem*> (getSelectedItem (index)))
            return item->file;

        return {};
        */
    }
    
    /**
      | Deselects any files that are currently
      | selected.
      |
      */
    pub fn deselect_all_files(&mut self)  {
        
        todo!();
        /*
            clearSelectedItems();
        */
    }
    
    /**
      | Scrolls the list to the top.
      |
      */
    pub fn scroll_to_top(&mut self)  {
        
        todo!();
        /*
            getViewport()->getVerticalScrollBar().setCurrentRangeStart (0);
        */
    }
    
    /**
      | Setting a name for this allows tree items
      | to be dragged.
      | 
      | The string that you pass in here will
      | be returned by the getDragSourceDescription()
      | of the items in the tree. For more info,
      | see TreeViewItem::getDragSourceDescription().
      |
      */
    pub fn set_drag_and_drop_description(&mut self, description: &String)  {
        
        todo!();
        /*
            dragAndDropDescription = description;
        */
    }
    
    /**
      | If the specified file is in the list,
      | it will become the only selected item
      | (and if the file isn't in the list, all
      | other items will be deselected).
      |
      */
    pub fn set_selected_file(&mut self, target: &File)  {
        
        todo!();
        /*
            if (auto* t = dynamic_cast<FileListTreeItem*> (getRootItem()))
            if (! t->selectFile (target))
                clearSelectedItems();
        */
    }
    
    /**
      | Changes the height of the treeview items.
      |
      */
    pub fn set_item_height(&mut self, new_height: i32)  {
        
        todo!();
        /*
            if (itemHeight != newHeight)
        {
            itemHeight = newHeight;

            if (auto* root = getRootItem())
                root->treeHasChanged();
        }
        */
    }
}

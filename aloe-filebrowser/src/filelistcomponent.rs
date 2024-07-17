crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileListComponent.h]

pub fn aloe_create_icon_for_file(file: &File) -> Image {
    
    todo!();
    /*
    
    */
}

/**
  | A component that displays the files
  | in a directory as a listbox.
  | 
  | This implements the DirectoryContentsDisplayComponent
  | base class so that it can be used in a FileBrowserComponent.
  | 
  | To attach a listener to it, use its DirectoryContentsDisplayComponent
  | base class and the FileBrowserListener
  | class.
  | 
  | @see DirectoryContentsList, FileTreeComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileListComponent<'a> {
    base:                        ListBox<'a>,
    base2:                       DirectoryContentsDisplayComponent<'a>,
    base3:                       ListBoxModel,
    last_directory:              File,
    file_waiting_to_be_selected: File,
}

impl<'a> ChangeListener for FileListComponent<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            updateContent();

        if (lastDirectory != directoryContentsList.getDirectory())
        {
            fileWaitingToBeSelected = File();
            lastDirectory = directoryContentsList.getDirectory();
            deselectAllRows();
        }

        if (fileWaitingToBeSelected != File())
            setSelectedFile (fileWaitingToBeSelected);
        */
    }
}

impl<'a> Drop for FileListComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        directoryContentsList.removeChangeListener (this);
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileListComponent.cpp]
impl<'a> FileListComponent<'a> {

    /**
      | Creates a listbox to show the contents
      | of a specified directory.
      |
      */
    pub fn new(list_to_show: &mut DirectoryContentsList) -> Self {
    
        todo!();
        /*


            : ListBox ({}, nullptr),
          DirectoryContentsDisplayComponent (listToShow),
          lastDirectory (listToShow.getDirectory())

        setTitle ("Files");
        setModel (this);
        directoryContentsList.addChangeListener (this);
        */
    }
    
    /**
      | Returns the number of files the user
      | has got selected. @see getSelectedFile
      |
      */
    pub fn get_num_selected_files(&self) -> i32 {
        
        todo!();
        /*
            return getNumSelectedRows();
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
            return directoryContentsList.getFile (getSelectedRow (index));
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
            deselectAllRows();
        */
    }
    
    /**
      | Scrolls to the top of the list.
      |
      */
    pub fn scroll_to_top(&mut self)  {
        
        todo!();
        /*
            getVerticalScrollBar().setCurrentRangeStart (0);
        */
    }
    
    /**
      | If the specified file is in the list,
      | it will become the only selected item
      | (and if the file isn't in the list, all
      | other items will be deselected).
      |
      */
    pub fn set_selected_file(&mut self, f: &File)  {
        
        todo!();
        /*
            for (int i = directoryContentsList.getNumFiles(); --i >= 0;)
        {
            if (directoryContentsList.getFile (i) == f)
            {
                fileWaitingToBeSelected = File();

                selectRow (i);
                return;
            }
        }

        deselectAllRows();
        fileWaitingToBeSelected = f;
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return directoryContentsList.getNumFiles();
        */
    }
    
    pub fn get_name_for_row(&mut self, row_number: i32) -> String {
        
        todo!();
        /*
            return directoryContentsList.getFile (rowNumber).getFileName();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        _0: i32,
        _1: &mut Graphics,
        _2: i32,
        _3: i32,
        _4: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn refresh_component_for_row(&mut self, 
        row:                          i32,
        is_selected:                  bool,
        existing_component_to_update: *mut Component) -> *mut Component {
        
        todo!();
        /*
            jassert (existingComponentToUpdate == nullptr || dynamic_cast<FileListComponentItemComponent*> (existingComponentToUpdate) != nullptr);

        auto comp = static_cast<FileListComponentItemComponent*> (existingComponentToUpdate);

        if (comp == nullptr)
            comp = new FileListComponentItemComponent (*this, directoryContentsList.getTimeSliceThread());

        DirectoryContentsList::FileInfo fileInfo;
        comp->update (directoryContentsList.getDirectory(),
                      directoryContentsList.getFileInfo (row, fileInfo) ? &fileInfo : nullptr,
                      row, isSelected);

        return comp;
        */
    }
    
    pub fn selected_rows_changed(&mut self, last_row_selected: i32)  {
        
        todo!();
        /*
            sendSelectionChangeMessage();
        */
    }
    
    pub fn delete_key_pressed(&mut self, current_selected_row: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn return_key_pressed(&mut self, current_selected_row: i32)  {
        
        todo!();
        /*
            sendDoubleClickMessage (directoryContentsList.getFile (currentSelectedRow));
        */
    }
}

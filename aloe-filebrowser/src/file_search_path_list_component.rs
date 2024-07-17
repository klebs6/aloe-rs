crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileSearchPathListComponent.h]

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the label.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum FileSearchPathListComponentColourIds
{
    /**
      | The background colour to fill the component
      | with. Make this transparent if you don't
      | want the background to be filled.
      |
      */
    backgroundColourId      = 0x1004100, 
}

/**
  | Shows a set of file paths in a list, allowing
  | them to be added, removed or re-ordered.
  | 
  | @see FileSearchPath
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileSearchPathListComponent<'a> {
    base:                  Component<'a>,
    base2:                 SettableTooltipClient,
    base4:                 ListBoxModel,
    path:                  FileSearchPath,
    default_browse_target: File,
    chooser:               Box<FileChooser<'a>>,
    list_box:              ListBox<'a>,
    add_button:            TextButton<'a>,
    remove_button:         TextButton<'a>,
    change_button:         TextButton<'a>,
    up_button:             DrawableButton<'a>,
    down_button:           DrawableButton<'a>,
}

impl<'a> FileDragAndDropTarget for FileSearchPathListComponent<'a> {

}

impl<'a> ShouldDrawDragImageWhenOver for FileSearchPathListComponent<'a> {

}

impl<'a> ItemDropped for FileSearchPathListComponent<'a> {

    fn item_dropped(&mut self, _: &DragAndDropTargetSourceDetails<'_>) 
    { 
        todo!() 
    }
}

impl<'a> ItemDragExit for FileSearchPathListComponent<'a> {

}

impl<'a> ItemDragMove for FileSearchPathListComponent<'a> {

}

impl<'a> ItemDragEnter for FileSearchPathListComponent<'a> {

}

impl<'a> IsInterestedInDragSource for FileSearchPathListComponent<'a> {

    fn is_interested_in_drag_source(
        &mut self, 
        _: &DragAndDropTargetSourceDetails<'_>

    ) -> bool { 

        todo!() 
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileSearchPathListComponent.cpp]
impl<'a> FileSearchPathListComponent<'a> {

    /**
      | Returns the path as it is currently shown.
      |
      */
    pub fn get_path(&self) -> &FileSearchPath {
        
        todo!();
        /*
            return path;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*


            : addButton ("+"),
          removeButton ("-"),
          changeButton (TRANS ("change...")),
          upButton ({}, DrawableButton::ImageOnButtonBackground),
          downButton ({}, DrawableButton::ImageOnButtonBackground)

        listBox.setModel (this);
        addAndMakeVisible (listBox);
        listBox.setColour (ListBox::backgroundColourId, Colours::black.withAlpha (0.02f));
        listBox.setColour (ListBox::outlineColourId, Colours::black.withAlpha (0.1f));
        listBox.setOutlineThickness (1);

        addAndMakeVisible (addButton);
        addButton.onClick = [this] { addPath(); };
        addButton.setConnectedEdges (Button::ConnectedOnLeft | Button::ConnectedOnRight | Button::ConnectedOnBottom | Button::ConnectedOnTop);

        addAndMakeVisible (removeButton);
        removeButton.onClick = [this] { deleteSelected(); };
        removeButton.setConnectedEdges (Button::ConnectedOnLeft | Button::ConnectedOnRight | Button::ConnectedOnBottom | Button::ConnectedOnTop);

        addAndMakeVisible (changeButton);
        changeButton.onClick = [this] { editSelected(); };

        addAndMakeVisible (upButton);
        upButton.onClick = [this] { moveSelection (-1); };

        auto arrowColour = findColour (ListBox::textColourId);

        {
            Path arrowPath;
            arrowPath.addArrow ({ 50.0f, 100.0f, 50.0f, 0.0f }, 40.0f, 100.0f, 50.0f);
            DrawablePath arrowImage;
            arrowImage.setFill (arrowColour);
            arrowImage.setPath (arrowPath);

            upButton.setImages (&arrowImage);
        }

        addAndMakeVisible (downButton);
        downButton.onClick = [this] { moveSelection (1); };

        {
            Path arrowPath;
            arrowPath.addArrow ({ 50.0f, 0.0f, 50.0f, 100.0f }, 40.0f, 100.0f, 50.0f);
            DrawablePath arrowImage;
            arrowImage.setFill (arrowColour);
            arrowImage.setPath (arrowPath);

            downButton.setImages (&arrowImage);
        }

        updateButtons();
        */
    }
    
    pub fn update_buttons(&mut self)  {
        
        todo!();
        /*
            const bool anythingSelected = listBox.getNumSelectedRows() > 0;

        removeButton.setEnabled (anythingSelected);
        changeButton.setEnabled (anythingSelected);
        upButton.setEnabled (anythingSelected);
        downButton.setEnabled (anythingSelected);
        */
    }
    
    pub fn changed(&mut self)  {
        
        todo!();
        /*
            listBox.updateContent();
        listBox.repaint();
        updateButtons();
        */
    }
    
    /**
      | Changes the current path.
      |
      */
    pub fn set_path(&mut self, new_path: &FileSearchPath)  {
        
        todo!();
        /*
            if (newPath.toString() != path.toString())
        {
            path = newPath;
            changed();
        }
        */
    }
    
    /**
      | Sets a file or directory to be the default
      | starting point for the browser to show.
      | 
      | This is only used if the current file
      | hasn't been set.
      |
      */
    pub fn set_default_browse_target(&mut self, new_default_directory: &File)  {
        
        todo!();
        /*
            defaultBrowseTarget = newDefaultDirectory;
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return path.getNumPaths();
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
            if (rowIsSelected)
            g.fillAll (findColour (TextEditor::highlightColourId));

        g.setColour (findColour (ListBox::textColourId));
        Font f ((float) height * 0.7f);
        f.setHorizontalScale (0.9f);
        g.setFont (f);

        g.drawText (path[rowNumber].getFullPathName(),
                    4, 0, width - 6, height,
                    Justification::centredLeft, true);
        */
    }
    
    pub fn delete_key_pressed(&mut self, row: i32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (row, path.getNumPaths()))
        {
            path.remove (row);
            changed();
        }
        */
    }
    
    pub fn return_key_pressed(&mut self, row: i32)  {
        
        todo!();
        /*
            chooser = std::make_unique<FileChooser> (TRANS("Change folder..."), path[row], "*");
        auto chooserFlags = FileBrowserComponent::openMode | FileBrowserComponent::canSelectDirectories;

        chooser->launchAsync (chooserFlags, [this, row] (const FileChooser& fc)
        {
            if (fc.getResult() == File{})
                return;

            path.remove (row);
            path.add (fc.getResult(), row);
            changed();
        });
        */
    }
    
    pub fn list_box_item_double_clicked(&mut self, 
        row: i32,
        _1:  &MouseEvent)  {
        
        todo!();
        /*
            returnKeyPressed (row);
        */
    }
    
    pub fn selected_rows_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
            updateButtons();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            const int buttonH = 22;
        const int buttonY = getHeight() - buttonH - 4;
        listBox.setBounds (2, 2, getWidth() - 4, buttonY - 5);

        addButton.setBounds (2, buttonY, buttonH, buttonH);
        removeButton.setBounds (addButton.getRight(), buttonY, buttonH, buttonH);

        changeButton.changeWidthToFitText (buttonH);
        downButton.setSize (buttonH * 2, buttonH);
        upButton.setSize (buttonH * 2, buttonH);

        downButton.setTopRightPosition (getWidth() - 2, buttonY);
        upButton.setTopRightPosition (downButton.getX() - 4, buttonY);
        changeButton.setTopRightPosition (upButton.getX() - 8, buttonY);
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, _0: &Vec<String>) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn files_dropped(&mut self, 
        filenames: &Vec<String>,
        _1:        i32,
        mousey:    i32)  {
        
        todo!();
        /*
            for (int i = filenames.size(); --i >= 0;)
        {
            const File f (filenames[i]);

            if (f.isDirectory())
            {
                auto row = listBox.getRowContainingPosition (0, mouseY - listBox.getY());
                path.add (f, row);
                changed();
            }
        }
        */
    }
    
    pub fn add_path(&mut self)  {
        
        todo!();
        /*
            auto start = defaultBrowseTarget;

        if (start == File())
            start = path[0];

        if (start == File())
            start = File::getCurrentWorkingDirectory();

        chooser = std::make_unique<FileChooser> (TRANS("Add a folder..."), start, "*");
        auto chooserFlags = FileBrowserComponent::openMode | FileBrowserComponent::canSelectDirectories;

        chooser->launchAsync (chooserFlags, [this] (const FileChooser& fc)
        {
            if (fc.getResult() == File{})
                return;

            path.add (fc.getResult(), listBox.getSelectedRow());
            changed();
        });
        */
    }
    
    pub fn delete_selected(&mut self)  {
        
        todo!();
        /*
            deleteKeyPressed (listBox.getSelectedRow());
        changed();
        */
    }
    
    pub fn edit_selected(&mut self)  {
        
        todo!();
        /*
            returnKeyPressed (listBox.getSelectedRow());
        changed();
        */
    }
    
    pub fn move_selection(&mut self, delta: i32)  {
        
        todo!();
        /*
            jassert (delta == -1 || delta == 1);
        auto currentRow = listBox.getSelectedRow();

        if (isPositiveAndBelow (currentRow, path.getNumPaths()))
        {
            auto newRow = jlimit (0, path.getNumPaths() - 1, currentRow + delta);

            if (currentRow != newRow)
            {
                auto f = path[currentRow];
                path.remove (currentRow);
                path.add (f, newRow);
                listBox.selectRow (newRow);
                changed();
            }
        }
        */
    }
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_DirectoryContentsDisplayComponent.h]
pub trait DirectoryContentsDisplayComponentInterface: 
GetNumSelectedFiles 
+ GetSelectedFile
+ DeselectAllFiles
+ ScrollToTop
+ SetSelectedFile {}

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the list.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum DirectoryContentsDisplayComponentColourIds
{
    /**
      | The colour to use to fill a highlighted
      | row of the list.
      |
      */
    highlightColourId          = 0x1000540, 

    /**
      | The colour for the text.
      |
      */
    textColourId               = 0x1000541, 

    /**
      | The colour with which to draw the text
      | in highlighted sections.
      |
      */
    highlightedTextColourId    = 0x1000542,  
}

/**
  | A base class for components that display
  | a list of the files in a directory.
  | 
  | @see DirectoryContentsList
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DirectoryContentsDisplayComponent<'a> {

    /**
      | The list that this component is displaying
      |
      */
    directory_contents_list: &'a mut DirectoryContentsList<'a>,

    listeners:               ListenerList<Rc<RefCell<dyn FileBrowserListener>>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_DirectoryContentsDisplayComponent.cpp]
impl<'a> DirectoryContentsDisplayComponent<'a> {
    
    /**
      | Creates a DirectoryContentsDisplayComponent
      | for a given list of files.
      |
      */
    pub fn new(l: &mut DirectoryContentsList) -> Self {
    
        todo!();
        /*
        : directory_contents_list(l),
        */
    }
    
    /**
      | Adds a listener to be told when files
      | are selected or clicked. @see removeListener
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn FileBrowserListener)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    /**
      | Removes a listener. @see addListener
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn FileBrowserListener)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    pub fn send_selection_change_message(&mut self)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (dynamic_cast<Component*> (this));
        listeners.callChecked (checker, [] (FileBrowserListener& l) { l.selectionChanged(); });
        */
    }
    
    pub fn send_mouse_click_message(&mut self, 
        file: &File,
        e:    &MouseEvent)  {
        
        todo!();
        /*
            if (directoryContentsList.getDirectory().exists())
        {
            Component::BailOutChecker checker (dynamic_cast<Component*> (this));
            listeners.callChecked (checker, [&] (FileBrowserListener& l) { l.fileClicked (file, e); });
        }
        */
    }
    
    pub fn send_double_click_message(&mut self, file: &File)  {
        
        todo!();
        /*
            if (directoryContentsList.getDirectory().exists())
        {
            Component::BailOutChecker checker (dynamic_cast<Component*> (this));
            listeners.callChecked (checker, [&] (FileBrowserListener& l) { l.fileDoubleClicked (file); });
        }
        */
    }
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/VideoDemo.h]

/**
  | so that we can easily have two video windows
  | each with a file browser, wrap this up
  | as a class..
  |
  */
#[cfg(any(target_os="macos",target_os="windows"))]
#[no_copy]
#[leak_detector]
pub struct MovieComponentWithFileBrowser<'a> {
    base:         Component<'a>,
    video_comp:   VideoComponent<'a>,
    is_drag_over: bool, // default = false
    file_chooser: FilenameComponent<'a>, // default = { "movie", {}, true, false, false, "*", {}, "(choose a video file to play)" }
}

#[cfg(any(target_os="macos",target_os="windows"))] 
impl<'a> DragAndDropTarget for MovieComponentWithFileBrowser<'a> { 

}

#[cfg(any(target_os="macos",target_os="windows"))] 
impl<'a> ItemDragEnter for MovieComponentWithFileBrowser<'a> { 

    fn item_drag_enter(&mut self, _0: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            isDragOver = true;
            repaint();
        */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))] 
impl<'a> ItemDragMove for MovieComponentWithFileBrowser<'a> { 

}

#[cfg(any(target_os="macos",target_os="windows"))] 
impl<'a> ItemDragExit for MovieComponentWithFileBrowser<'a> { 

    fn item_drag_exit(&mut self, _0: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            isDragOver = false;
            repaint();
        */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))] 
impl<'a> ShouldDrawDragImageWhenOver for MovieComponentWithFileBrowser<'a> { 

}

#[cfg(any(target_os="macos",target_os="windows"))] 
impl<'a> ItemDropped for MovieComponentWithFileBrowser<'a> { 

    fn item_dropped(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            setFile (dragSourceDetails.description.toString());
            isDragOver = false;
            repaint();
        */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))] 
impl<'a> IsInterestedInDragSource for MovieComponentWithFileBrowser<'a> { 

    fn is_interested_in_drag_source(&mut self, _0: &DragAndDropTargetSourceDetails) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))]
impl<'a> FilenameComponentListener for MovieComponentWithFileBrowser<'a> {

    fn filename_component_changed(&mut self, _0: *mut FilenameComponent)  {
        
        todo!();
        /*
            auto url = Url (fileChooser.getCurrentFile());

            // this is called when the user changes the filename in the file chooser box
            auto result = videoComp.load (url);
            videoLoadingFinished (url, result);
        */
    }
    
}

#[cfg(any(target_os="macos",target_os="windows"))]
impl<'a> Default for MovieComponentWithFileBrowser<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : video_comp(true),

            addAndMakeVisible (videoComp);

            addAndMakeVisible (fileChooser);
            fileChooser.addListener (this);
            fileChooser.setBrowseButtonText ("browse")
        */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))]
impl<'a> MovieComponentWithFileBrowser<'a> {

    pub fn set_file(&mut self, file: &File)  {
        
        todo!();
        /*
            fileChooser.setCurrentFile (file, true);
        */
    }
    
    pub fn paint_over_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (isDragOver)
            {
                g.setColour (Colours::red);
                g.drawRect (fileChooser.getBounds(), 2);
            }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            videoComp.setBounds (getLocalBounds().reduced (10));
        */
    }
    
    pub fn video_loading_finished(
        &mut self, 
        url:    &Url,
        result: Result<(),()>
    ) {
        
        todo!();
        /*
            ignoreUnused (url);

            if (result.wasOk())
            {
                // loaded the file ok, so let's start it playing..

                videoComp.play();
                resized(); // update to reflect the video's aspect ratio
            }
            else
            {
                AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                  "Couldn't load the file!",
                                                  result.getErrorMessage());
            }
        */
    }
}

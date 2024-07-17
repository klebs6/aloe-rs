crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/ImagesDemo.h]

#[no_copy]
#[leak_detector]
pub struct ImagesDemo<'a> {
    base:                   Component<'a>,
    images_wildcard_filter: WildcardFileFilter,    //{ "*.jpeg;*.jpg;*.png;*.gif", "*", "Image File Filter"};
    directory_thread:       TimeSliceThread,       //{ "Image File Scanner Thread" };
    image_list:             DirectoryContentsList<'a>, //{ &imagesWildcardFilter, directoryThread };
    file_tree:              FileTreeComponent<'a>,     //{ imageList };
    image_preview:          ImageComponent<'a>,
    stretchable_manager:    StretchableLayoutManager,
    resizer_bar:            StretchableLayoutResizerBar<'a>, //{ &stretchableManager, 1, false };
}

impl<'a> FileBrowserListener for ImagesDemo<'a> {

    fn selection_changed(&mut self)  {
        
        todo!();
        /*
            // we're only really interested in when the selection changes, regardless of if it was
            // clicked or not so we'll only override this method
            auto selectedFile = fileTree.getSelectedFile();

            if (selectedFile.existsAsFile())
                imagePreview.setImage (ImageCache::getFromFile (selectedFile));

            // the image cache is a handy way to load images from files or directly from memory and
            // will keep them hanging around for a few seconds in case they are requested elsewhere
        */
    }
    
    fn file_clicked(&mut self, 
        _0: &File,
        _1: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    fn file_double_clicked(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
    
    fn browser_root_changed(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Default for ImagesDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);
            imageList.setDirectory (File::getSpecialLocation (File::userPicturesDirectory), true, true);
            directoryThread.startThread (1);

            fileTree.setTitle ("Files");
            fileTree.addListener (this);
            fileTree.setColour (TreeView::backgroundColourId, Colours::grey);
            addAndMakeVisible (fileTree);

            addAndMakeVisible (resizerBar);

            addAndMakeVisible (imagePreview);

            // we have to set up our StretchableLayoutManager so it know the limits and preferred sizes of it's contents
            stretchableManager.setItemLayout (0,            // for the fileTree
                                              -0.1, -0.9,   // must be between 50 pixels and 90% of the available space
                                              -0.3);        // and its preferred size is 30% of the total available space

            stretchableManager.setItemLayout (1,            // for the resize bar
                                              5, 5, 5);     // hard limit to 5 pixels

            stretchableManager.setItemLayout (2,            // for the imagePreview
                                              -0.1, -0.9,   // size must be between 50 pixels and 90% of the available space
                                              -0.7);        // and its preferred size is 70% of the total available space

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for ImagesDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            fileTree.removeListener (this);
         */
    }
}

impl<'a> ImagesDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::white);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (4);

            // make a list of two of our child components that we want to reposition
            Component* comps[] = { &fileTree, &resizerBar, &imagePreview };

            // this will position the 3 components, one above the other, to fit
            // vertically into the rectangle provided.
            stretchableManager.layOutComponents (comps, 3,
                                                 r.getX(), r.getY(), r.getWidth(), r.getHeight(),
                                                 true, true);
        */
    }
    
}

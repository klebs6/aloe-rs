crate::ix!();

#[no_copy]
#[leak_detector]
pub struct StatusItemContainer<'a,StatusItem:NSStatusItem,ImageType:NSImage> {
    base:        Timer,
    owner:       &'a mut SystemTrayIconComponent<'a,StatusItem,ImageType>,
    status_item: NSUniquePtr<Box<StatusItem>>,
    status_icon: NSUniquePtr<Box<ImageType>>,
}

impl<'a,StatusItem:NSStatusItem,ImageType:NSImage>  StatusItemContainer<'a,StatusItem,ImageType> {

    pub fn new(
        icon_comp: &mut SystemTrayIconComponent<'a,StatusItem,ImageType>,
        im:        &ImageType) -> Self {
    
        todo!();
        /*
        : owner(iconComp),
        : status_icon(imageToNSImage (im)),

        
        */
    }
    
    pub fn set_icon_size(&mut self)  {
        
        todo!();
        /*
            [statusIcon.get() setSize: NSMakeSize (20.0f, 20.0f)];
        */
    }
    
    pub fn update_icon(&mut self, new_image: &ImageType)  {
        
        todo!();
        /*
            statusIcon.reset (imageToNSImage (newImage));
            setIconSize();
            configureIcon();
        */
    }
    
    pub fn show_menu(&mut self, menu: &PopupMenu)  {
        
        todo!();
        /*
            if (NSMenu* m = createNSMenu (menu, "MenuBarItem", -2, -3, true))
            {
                setHighlighted (true);
                stopTimer();

                // There's currently no good alternative to this.
                [statusItem.get() popUpStatusItemMenu: m];

                startTimer (1);
            }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();
            setHighlighted (false);
        */
    }
}

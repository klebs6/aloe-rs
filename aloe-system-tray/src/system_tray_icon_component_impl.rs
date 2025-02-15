crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SystemTrayIconComponentImpl<'a,StatusItem:NSStatusItem,ImageType:NSImage> {
    status_item_holder: Box<StatusItemContainer<'a,StatusItem,ImageType>>,
}

impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> SystemTrayIconComponentImpl<'a,StatusItem,ImageType> {

    pub fn new(
        icon_comp: &mut SystemTrayIconComponent<'a,StatusItem,ImageType>,
        im:        &ImageType) -> Self {
    
        todo!();
        /*


            if (std::floor (NSFoundationVersionNumber) > NSFoundationVersionNumber10_10)
                statusItemHolder = std::make_unique<ButtonBasedStatusItem> (iconComp, im);
            else
                statusItemHolder = std::make_unique<ViewBasedStatusItem> (iconComp, im);
        */
    }
}

crate::ix!();

/**
  | Each type of item a toolbar can contain
  | must be given a unique ID. These are
  | the ones we'll use in this demo.
  */
pub enum DemoToolbarItemIds
{
    doc_new         = 1,
    doc_open        = 2,
    doc_save        = 3,
    doc_saveAs      = 4,
    edit_copy       = 5,
    edit_cut        = 6,
    edit_paste      = 7,
    aloeLogoButton  = 8,
    customComboBox  = 9
}

#[derive(Default)]
pub struct DemoToolbarItemFactory<'a> {
    icon_names:          StringArray,
    icons_from_zip_file: Vec<Box<Drawable<'a>>>,
}

impl<'a> ToolbarItemFactory for DemoToolbarItemFactory<'a> {

}

impl<'a> GetAllToolbarItemIds for DemoToolbarItemFactory<'a> {

    fn get_all_toolbar_item_ids(&mut self, _: &mut Vec<i32>) { todo!() }
}

impl<'a> GetDefaultItemSet for DemoToolbarItemFactory<'a> {

    fn get_default_item_set(&mut self, ids: &mut Vec<i32>)  {
        
        todo!();
        /*
            // This returns an ordered list of the set of items that make up a
                // toolbar's default set. Not all items need to be on this list, and
                // items can appear multiple times (e.g. the separators used here).
                ids.add (doc_new);
                ids.add (doc_open);
                ids.add (doc_save);
                ids.add (doc_saveAs);
                ids.add (spacerId);
                ids.add (separatorBarId);
                ids.add (edit_copy);
                ids.add (edit_cut);
                ids.add (edit_paste);
                ids.add (separatorBarId);
                ids.add (flexibleSpacerId);
                ids.add (customComboBox);
                ids.add (flexibleSpacerId);
                ids.add (separatorBarId);
                ids.add (aloeLogoButton);
        */
    }
}

impl<'a> ToolbarItemComponent for DemoToolbarItemFactory<'a> {

    fn create_item(&mut self, item_id: i32) -> *mut dyn ToolbarItemComponent {
        
        todo!();
        /*
            switch (itemId)
                {
                    case doc_new:           return createButtonFromZipFileSVG (itemId, "new",     "document-new.svg");
                    case doc_open:          return createButtonFromZipFileSVG (itemId, "open",    "document-open.svg");
                    case doc_save:          return createButtonFromZipFileSVG (itemId, "save",    "document-save.svg");
                    case doc_saveAs:        return createButtonFromZipFileSVG (itemId, "save as", "document-save-as.svg");
                    case edit_copy:         return createButtonFromZipFileSVG (itemId, "copy",    "edit-copy.svg");
                    case edit_cut:          return createButtonFromZipFileSVG (itemId, "cut",     "edit-cut.svg");
                    case edit_paste:        return createButtonFromZipFileSVG (itemId, "paste",   "edit-paste.svg");

                    case aloeLogoButton:
                    {
                        auto drawable = std::make_unique<DrawableImage>();
                        drawable->setImage (getImageFromAssets ("aloe_icon.png"));
                        return new ToolbarButton (itemId, "aloe!", std::move (drawable), {});
                    }

                    case customComboBox:    return new CustomToolbarComboBox (itemId);
                    default:                break;
                }

                return nullptr;
        */
    }
}

impl<'a> DemoToolbarItemFactory<'a> {

    pub fn get_all_toolbar_item_ids(&mut self, ids: &mut Vec<i32>)  {
        
        todo!();
        /*
            // This returns the complete list of all item IDs that are allowed to
                // go in our toolbar. Any items you might want to add must be listed here. The
                // order in which they are listed will be used by the toolbar customisation panel.

                ids.add (doc_new);
                ids.add (doc_open);
                ids.add (doc_save);
                ids.add (doc_saveAs);
                ids.add (edit_copy);
                ids.add (edit_cut);
                ids.add (edit_paste);
                ids.add (aloeLogoButton);
                ids.add (customComboBox);

                // If you're going to use separators, then they must also be added explicitly
                // to the list.
                ids.add (separatorBarId);
                ids.add (spacerId);
                ids.add (flexibleSpacerId);
        */
    }

    /**
      | This is a little utility to create a button
      | with one of the SVG images in our embedded
      | ZIP file "icons.zip"
      |
      */
    pub fn create_button_from_zip_filesvg(&mut self, 
        item_id:  i32,
        text:     &String,
        filename: &String) -> *mut ToolbarButton {
        
        todo!();
        /*
            if (iconsFromZipFile.size() == 0)
                {
                    // If we've not already done so, load all the images from the zip file..
                    ZipFile icons (createAssetInputStream ("icons.zip").release(), true);

                    for (int i = 0; i < icons.getNumEntries(); ++i)
                    {
                        std::unique_ptr<InputStream> svgFileStream (icons.createStreamForEntry (i));

                        if (svgFileStream.get() != nullptr)
                        {
                            iconNames.add (icons.getEntry (i)->filename);
                            iconsFromZipFile.add (Drawable::createFromImageDataStream (*svgFileStream));
                        }
                    }
                }

                auto* image = iconsFromZipFile[iconNames.indexOf (filename)];
                return new ToolbarButton (itemId, text, image->createCopy(), {});
        */
    }
}

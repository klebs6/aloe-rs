crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/XMLandJSONDemo.h]

#[no_copy]
#[leak_detector]
pub struct XmlTreeItem<'a> {
    base: TreeViewItem<'a>,
    xml:  &'a mut XmlElement,
}

impl<'a> XmlTreeItem<'a> {

    pub fn new(x: &mut XmlElement) -> Self {
    
        todo!();
        /*
        : xml(x),

        
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            if (xml.getTagName().isEmpty())
                return "unknown";

            return xml.getTagName();
        */
    }
    
    pub fn might_contain_sub_items(&mut self) -> bool {
        
        todo!();
        /*
            return xml.getFirstChildElement() != nullptr;
        */
    }
    
    pub fn paint_item(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            // if this item is selected, fill it with a background colour..
            if (isSelected())
                g.fillAll (Colours::blue.withAlpha (0.3f));

            // use a "colour" attribute in the xml tag for this node to set the text colour..
            g.setColour (Colour::fromString (xml.getStringAttribute ("colour", "ff000000")));
            g.setFont ((float) height * 0.7f);

            // draw the xml element's tag name..
            g.drawText (xml.getTagName(),
                        4, 0, width - 4, height,
                        Justification::centredLeft, true);
        */
    }
    
    pub fn item_openness_changed(&mut self, is_now_open: bool)  {
        
        todo!();
        /*
            if (isNowOpen)
            {
                // if we've not already done so, we'll now add the tree's sub-items. You could
                // also choose to delete the existing ones and refresh them if that's more suitable
                // in your app.
                if (getNumSubItems() == 0)
                {
                    // create and add sub-items to this node of the tree, corresponding to
                    // each sub-element in the XML..

                    for (auto* child : xml.getChildIterator())
                        if (child != nullptr)
                            addSubItem (new XmlTreeItem (*child));
                }
            }
            else
            {
                // in this case, we'll leave any sub-items in the tree when the node gets closed,
                // though you could choose to delete them if that's more appropriate for
                // your application.
            }
        */
    }
}

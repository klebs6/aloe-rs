crate::ix!();

#[no_copy]
#[leak_detector]
pub struct JsonTreeItem<'a> {
    base:       TreeViewItem<'a>,
    identifier: Identifier,
    json:       Var,
}

impl<'a> JsonTreeItem<'a> {

    pub fn new(
        i:     Identifier,
        value: Var

    ) -> Self {
    
        todo!();
        /*
        : identifier(i),
        : json(value),

        
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            return identifier.toString() + "_id";
        */
    }
    
    pub fn might_contain_sub_items(&mut self) -> bool {
        
        todo!();
        /*
            if (auto* obj = json.getDynamicObject())
                return obj->getProperties().size() > 0;

            return json.isArray();
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

            g.setColour (Colours::black);
            g.setFont ((float) height * 0.7f);

            // draw the element's tag name..
            g.drawText (getText(),
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
                    // the type of object this var represents

                    if (json.isArray())
                    {
                        for (int i = 0; i < json.size(); ++i)
                        {
                            auto& child = json[i];
                            jassert (! child.isVoid());
                            addSubItem (new JsonTreeItem ({}, child));
                        }
                    }
                    else if (auto* obj = json.getDynamicObject())
                    {
                        auto& props = obj->getProperties();

                        for (int i = 0; i < props.size(); ++i)
                        {
                            auto id = props.getName (i);

                            auto child = props[id];
                            jassert (! child.isVoid());

                            addSubItem (new JsonTreeItem (id, child));
                        }
                    }
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

    /**
      | Returns the text to display in the tree.
      | 
      | This is a little more complex for JSON
      | than XML as nodes can be strings, objects
      | or arrays.
      |
      */
    pub fn get_text(&self) -> String {
        
        todo!();
        /*
            String text;

            if (identifier.isValid())
                text << identifier.toString();

            if (! json.isVoid())
            {
                if (text.isNotEmpty() && (! json.isArray()))
                    text << ": ";

                if (json.isObject() && (! identifier.isValid()))
                    text << "[Array]";
                else if (! json.isArray())
                    text << json.toString();
            }

            return text;
        */
    }
}

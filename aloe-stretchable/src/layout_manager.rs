crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_StretchableLayoutManager.h]

/**
  | For laying out a set of components, where
  | the components have preferred sizes
  | and size limits, but where they are allowed
  | to stretch to fill the available space.
  | 
  | For example, if you have a component
  | containing several other components,
  | and each one should be given a share of
  | the total size, you could use one of these
  | to resize the child components when
  | the parent component is resized. Then
  | you could add a StretchableLayoutResizerBar
  | to easily let the user rescale them.
  | 
  | A StretchableLayoutManager operates
  | only in one dimension, so if you have
  | a set of components stacked vertically
  | on top of each other, you'd use one to
  | manage their heights. To build up complex
  | arrangements of components, e.g. for
  | applications with multiple nested
  | panels, you would use more than one StretchableLayoutManager.
  | 
  | E.g. by using two (one vertical, one
  | horizontal), you could create a resizable
  | spreadsheet-style table.
  | 
  | -----------
  | @code
  | 
  | class MyComp  : public Component
  | {
  |     StretchableLayoutManager myLayout;
  | 
  |     MyComp()
  |     {
  |         myLayout.setItemLayout (0,          // for item 0
  |                                 50, 100,    // must be between 50 and 100 pixels in size
  |                                 -0.6);      // and its preferred size is 60% of the total available space
  | 
  |         myLayout.setItemLayout (1,          // for item 1
  |                                 -0.2, -0.6, // size must be between 20% and 60% of the available space
  |                                 50);        // and its preferred size is 50 pixels
  |     }
  | 
  |     void resized()
  |     {
  |         // make a list of two of our child components that we want to reposition
  |         Component* comps[] = { myComp1, myComp2 };
  | 
  |         // this will position the 2 components, one above the other, to fit
  |         // vertically into the rectangle provided.
  |         myLayout.layOutComponents (comps, 2,
  |                                    0, 0, getWidth(), getHeight(),
  |                                    true);
  |     }
  | };
  | 
  | @see StretchableLayoutResizerBar
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct StretchableLayoutManager {
    items:      Vec<Box<StretchableLayoutManagerItemLayoutProperties>>,
    total_size: i32, // default = 0
}

impl Default for StretchableLayoutManager {
    
    /**
      | Creates an empty layout.
      | 
      | You'll need to add some item properties
      | to the layout before it can be used to
      | resize things - see setItemLayout().
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_StretchableLayoutManager.cpp]
impl StretchableLayoutManager {

    /**
      | Clears all the properties that have
      | been set with setItemLayout() and resets
      | this object to its initial state.
      |
      */
    pub fn clear_all_items(&mut self)  {
        
        todo!();
        /*
            items.clear();
        totalSize = 0;
        */
    }
    
    /**
      | For a numbered item, this sets its size
      | limits and preferred size.
      | 
      | -----------
      | @param itemIndex
      | 
      | the index of the item to change.
      | ----------
      | @param minimumSize
      | 
      | the minimum size that this item is allowed
      | to be - a positive number indicates an
      | absolute size in pixels. A negative
      | number indicates a proportion of the
      | available space (e.g -0.5 is 50%)
      | ----------
      | @param maximumSize
      | 
      | the maximum size that this item is allowed
      | to be - a positive number indicates an
      | absolute size in pixels. A negative
      | number indicates a proportion of the
      | available space
      | ----------
      | @param preferredSize
      | 
      | the size that this item would like to
      | be, if there's enough room. A positive
      | number indicates an absolute size in
      | pixels. A negative number indicates
      | a proportion of the available space
      | @see getItemLayout
      |
      */
    pub fn set_item_layout(&mut self, 
        item_index:     i32,
        minimum_size:   f64,
        maximum_size:   f64,
        preferred_size: f64)  {
        
        todo!();
        /*
            auto* layout = getInfoFor (itemIndex);

        if (layout == nullptr)
        {
            layout = new StretchableLayoutManagerItemLayoutProperties();
            layout->itemIndex = itemIndex;

            int i;
            for (i = 0; i < items.size(); ++i)
                if (items.getUnchecked (i)->itemIndex > itemIndex)
                    break;

            items.insert (i, layout);
        }

        layout->minSize = minimumSize;
        layout->maxSize = maximumSize;
        layout->preferredSize = preferredSize;
        layout->currentSize = 0;
        */
    }
    
    /**
      | For a numbered item, this returns its
      | size limits and preferred size.
      | 
      | -----------
      | @param itemIndex
      | 
      | the index of the item.
      | ----------
      | @param minimumSize
      | 
      | the minimum size that this item is allowed
      | to be - a positive number indicates an
      | absolute size in pixels. A negative
      | number indicates a proportion of the
      | available space (e.g -0.5 is 50%)
      | ----------
      | @param maximumSize
      | 
      | the maximum size that this item is allowed
      | to be - a positive number indicates an
      | absolute size in pixels. A negative
      | number indicates a proportion of the
      | available space
      | ----------
      | @param preferredSize
      | 
      | the size that this item would like to
      | be, if there's enough room. A positive
      | number indicates an absolute size in
      | pixels. A negative number indicates
      | a proportion of the available space
      | 
      | -----------
      | @return
      | 
      | false if the item's properties hadn't
      | been set @see setItemLayout
      |
      */
    pub fn get_item_layout(
        &self, 
        item_index:     i32,
        minimum_size:   &mut f64,
        maximum_size:   &mut f64,
        preferred_size: &mut f64

    ) -> bool {
        
        todo!();
        /*
            if (auto* layout = getInfoFor (itemIndex))
        {
            minimumSize = layout->minSize;
            maximumSize = layout->maxSize;
            preferredSize = layout->preferredSize;
            return true;
        }

        return false;
        */
    }
    
    pub fn set_total_size(&mut self, new_total_size: i32)  {
        
        todo!();
        /*
            totalSize = newTotalSize;

        fitComponentsIntoSpace (0, items.size(), totalSize, 0);
        */
    }
    
    /**
      | Returns the current position of one
      | of the items.
      | 
      | This is only a valid call after layOutComponents()
      | has been called, as it returns the last
      | position that this item was placed at.
      | If the layout was vertical, the value
      | returned will be the y position of the
      | top of the item, relative to the top of
      | the rectangle in which the items were
      | placed (so for example, item 0 will always
      | have position of 0, even in the rectangle
      | passed in to layOutComponents() wasn't
      | at y = 0). If the layout was done horizontally,
      | the position returned is the item's
      | left-hand position, again relative
      | to the x position of the rectangle used.
      | 
      | @see getItemCurrentSize, setItemPosition
      |
      */
    pub fn get_item_current_position(&self, item_index: i32) -> i32 {
        
        todo!();
        /*
            int pos = 0;

        for (int i = 0; i < itemIndex; ++i)
            if (auto* layout = getInfoFor (i))
                pos += layout->currentSize;

        return pos;
        */
    }
    
    /**
      | Returns the current size of one of the
      | items.
      | 
      | This is only meaningful after layOutComponents()
      | has been called, as it returns the last
      | size that this item was given. If the
      | layout was done vertically, it'll return
      | the item's height in pixels; if it was
      | horizontal, it'll return its width.
      | 
      | @see getItemCurrentRelativeSize
      |
      */
    pub fn get_item_current_absolute_size(&self, item_index: i32) -> i32 {
        
        todo!();
        /*
            if (auto* layout = getInfoFor (itemIndex))
            return layout->currentSize;

        return 0;
        */
    }
    
    /**
      | Returns the current size of one of the
      | items.
      | 
      | This is only meaningful after layOutComponents()
      | has been called, as it returns the last
      | size that this item was given. If the
      | layout was done vertically, it'll return
      | a negative value representing the item's
      | height relative to the last size used
      | for laying the components out; if the
      | layout was done horizontally it'll
      | be the proportion of its width.
      | 
      | @see getItemCurrentAbsoluteSize
      |
      */
    pub fn get_item_current_relative_size(&self, item_index: i32) -> f64 {
        
        todo!();
        /*
            if (auto* layout = getInfoFor (itemIndex))
            return -layout->currentSize / (double) totalSize;

        return 0;
        */
    }
    
    /**
      | Moves one of the items, shifting along
      | any other items as necessary in order
      | to get it to the desired position.
      | 
      | Calling this method will also update
      | the preferred sizes of the items it shuffles
      | along, so that they reflect their new
      | positions.
      | 
      | (This is the method that a StretchableLayoutResizerBar
      | uses to shift the items about when it's
      | dragged).
      | 
      | -----------
      | @param itemIndex
      | 
      | the item to move
      | ----------
      | @param newPosition
      | 
      | the absolute position that you'd like
      | this item to move to. The item might not
      | be able to always reach exactly this
      | position, because other items may have
      | minimum sizes that constrain how far
      | it can go
      |
      */
    pub fn set_item_position(
        &mut self, 
        item_index:   i32,
        new_position: i32

    ) {
        
        todo!();
        /*
            for (int i = items.size(); --i >= 0;)
        {
            auto* layout = items.getUnchecked(i);

            if (layout->itemIndex == itemIndex)
            {
                auto realTotalSize = jmax (totalSize, getMinimumSizeOfItems (0, items.size()));
                auto minSizeAfterThisComp = getMinimumSizeOfItems (i, items.size());
                auto maxSizeAfterThisComp = getMaximumSizeOfItems (i + 1, items.size());

                newPosition = jmax (newPosition, totalSize - maxSizeAfterThisComp - layout->currentSize);
                newPosition = jmin (newPosition, realTotalSize - minSizeAfterThisComp);

                auto endPos = fitComponentsIntoSpace (0, i, newPosition, 0);

                endPos += layout->currentSize;

                fitComponentsIntoSpace (i + 1, items.size(), totalSize - endPos, endPos);
                updatePrefSizesToMatchCurrentPositions();
                break;
            }
        }
        */
    }
    
    /**
      | Takes a set of components that correspond
      | to the layout's items, and positions
      | them to fill a space.
      | 
      | This will try to give each item its preferred
      | size, whether that's a relative size
      | or an absolute one.
      | 
      | -----------
      | @param components
      | 
      | an array of components that correspond
      | to each of the numbered items that the
      | StretchableLayoutManager object
      | has been told about with setItemLayout()
      | ----------
      | @param numComponents
      | 
      | the number of components in the array
      | that is passed-in. This should be the
      | same as the number of items this object
      | has been told about.
      | ----------
      | @param x
      | 
      | the left of the rectangle in which the
      | components should be laid out
      | ----------
      | @param y
      | 
      | the top of the rectangle in which the
      | components should be laid out
      | ----------
      | @param width
      | 
      | the width of the rectangle in which the
      | components should be laid out
      | ----------
      | @param height
      | 
      | the height of the rectangle in which
      | the components should be laid out
      | ----------
      | @param vertically
      | 
      | if true, the components will be positioned
      | in a vertical stack, so that they fill
      | the height of the rectangle. If false,
      | they will be placed side-by-side in
      | a horizontal line, filling the available
      | width
      | ----------
      | @param resizeOtherDimension
      | 
      | if true, this means that the components
      | will have their other dimension resized
      | to fit the space - i.e. if the 'vertically'
      | parameter is true, their x-positions
      | and widths are adjusted to fit the x and
      | width parameters; if 'vertically'
      | is false, their y-positions and heights
      | are adjusted to fit the y and height parameters.
      |
      */
    pub fn lay_out_components(&mut self, 
        components:             *mut *mut Component,
        num_components:         i32,
        x:                      i32,
        y:                      i32,
        w:                      i32,
        h:                      i32,
        vertically:             bool,
        resize_other_dimension: bool)  {
        
        todo!();
        /*
            setTotalSize (vertically ? h : w);
        int pos = vertically ? y : x;

        for (int i = 0; i < numComponents; ++i)
        {
            if (auto* layout = getInfoFor (i))
            {
                if (auto* c = components[i])
                {
                    if (i == numComponents - 1)
                    {
                        // if it's the last item, crop it to exactly fit the available space..
                        if (resizeOtherDimension)
                        {
                            if (vertically)
                                c->setBounds (x, pos, w, jmax (layout->currentSize, h - pos));
                            else
                                c->setBounds (pos, y, jmax (layout->currentSize, w - pos), h);
                        }
                        else
                        {
                            if (vertically)
                                c->setBounds (c->getX(), pos, c->getWidth(), jmax (layout->currentSize, h - pos));
                            else
                                c->setBounds (pos, c->getY(), jmax (layout->currentSize, w - pos), c->getHeight());
                        }
                    }
                    else
                    {
                        if (resizeOtherDimension)
                        {
                            if (vertically)
                                c->setBounds (x, pos, w, layout->currentSize);
                            else
                                c->setBounds (pos, y, layout->currentSize, h);
                        }
                        else
                        {
                            if (vertically)
                                c->setBounds (c->getX(), pos, c->getWidth(), layout->currentSize);
                            else
                                c->setBounds (pos, c->getY(), layout->currentSize, c->getHeight());
                        }
                    }
                }

                pos += layout->currentSize;
            }
        }
        */
    }
    
    pub fn get_info_for(&self, item_index: i32) -> *mut StretchableLayoutManagerItemLayoutProperties {
        
        todo!();
        /*
            for (auto* i : items)
            if (i->itemIndex == itemIndex)
                return i;

        return nullptr;
        */
    }
    
    pub fn fit_components_into_space(&mut self, 
        start_index:     i32,
        end_index:       i32,
        available_space: i32,
        start_pos:       i32) -> i32 {
        
        todo!();
        /*
            // calculate the total sizes
        double totalIdealSize = 0.0;
        int totalMinimums = 0;

        for (int i = startIndex; i < endIndex; ++i)
        {
            auto* layout = items.getUnchecked (i);

            layout->currentSize = sizeToRealSize (layout->minSize, totalSize);

            totalMinimums += layout->currentSize;
            totalIdealSize += sizeToRealSize (layout->preferredSize, totalSize);
       }

        if (totalIdealSize <= 0)
            totalIdealSize = 1.0;

        // now calc the best sizes..
        int extraSpace = availableSpace - totalMinimums;

        while (extraSpace > 0)
        {
            int numWantingMoreSpace = 0;
            int numHavingTakenExtraSpace = 0;

            // first figure out how many comps want a slice of the extra space..
            for (int i = startIndex; i < endIndex; ++i)
            {
                auto* layout = items.getUnchecked (i);

                auto sizeWanted = sizeToRealSize (layout->preferredSize, totalSize);

                auto bestSize = jlimit (layout->currentSize,
                                        jmax (layout->currentSize,
                                              sizeToRealSize (layout->maxSize, totalSize)),
                                        roundToInt (sizeWanted * availableSpace / totalIdealSize));

                if (bestSize > layout->currentSize)
                    ++numWantingMoreSpace;
            }

            // ..share out the extra space..
            for (int i = startIndex; i < endIndex; ++i)
            {
                auto* layout = items.getUnchecked (i);

                auto sizeWanted = sizeToRealSize (layout->preferredSize, totalSize);

                auto bestSize = jlimit (layout->currentSize,
                                        jmax (layout->currentSize, sizeToRealSize (layout->maxSize, totalSize)),
                                        roundToInt (sizeWanted * availableSpace / totalIdealSize));

                auto extraWanted = bestSize - layout->currentSize;

                if (extraWanted > 0)
                {
                    auto extraAllowed = jmin (extraWanted,
                                              extraSpace / jmax (1, numWantingMoreSpace));

                    if (extraAllowed > 0)
                    {
                        ++numHavingTakenExtraSpace;
                        --numWantingMoreSpace;

                        layout->currentSize += extraAllowed;
                        extraSpace -= extraAllowed;
                    }
                }
            }

            if (numHavingTakenExtraSpace <= 0)
                break;
        }

        // ..and calculate the end position
        for (int i = startIndex; i < endIndex; ++i)
        {
            auto* layout = items.getUnchecked(i);
            startPos += layout->currentSize;
        }

        return startPos;
        */
    }
    
    pub fn get_minimum_size_of_items(&self, 
        start_index: i32,
        end_index:   i32) -> i32 {
        
        todo!();
        /*
            int totalMinimums = 0;

        for (int i = startIndex; i < endIndex; ++i)
            totalMinimums += sizeToRealSize (items.getUnchecked (i)->minSize, totalSize);

        return totalMinimums;
        */
    }
    
    pub fn get_maximum_size_of_items(&self, 
        start_index: i32,
        end_index:   i32) -> i32 {
        
        todo!();
        /*
            int totalMaximums = 0;

        for (int i = startIndex; i < endIndex; ++i)
            totalMaximums += sizeToRealSize (items.getUnchecked (i)->maxSize, totalSize);

        return totalMaximums;
        */
    }
    
    pub fn update_pref_sizes_to_match_current_positions(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < items.size(); ++i)
        {
            auto* layout = items.getUnchecked (i);

            layout->preferredSize
                = (layout->preferredSize < 0) ? getItemCurrentRelativeSize (i)
                                              : getItemCurrentAbsoluteSize (i);
        }
        */
    }
    
    pub fn size_to_real_size(&mut self, 
        size:        f64,
        total_space: i32) -> i32 {
        
        todo!();
        /*
            if (size < 0)
            size *= -totalSpace;

        return roundToInt (size);
        */
    }
}

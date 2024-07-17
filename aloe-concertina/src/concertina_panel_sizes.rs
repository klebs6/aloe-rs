crate::ix!();

pub struct ConcertinaPanelSizes<'a> {
    sizes: Vec<ConcertinaPanel<'a>>,
}

impl<'a> ConcertinaPanelSizes<'a> {
    
    pub fn get_mut(&mut self, index: i32) -> &mut ConcertinaPanel {
        
        todo!();
        /*
            return sizes.getReference (index);
        */
    }
    
    pub fn get(&self, index: i32) -> &ConcertinaPanel {
        
        todo!();
        /*
            return sizes.getReference (index);
        */
    }
    
    pub fn with_moved_panel(&self, 
        index:           i32,
        target_position: i32,
        total_space:     i32) -> ConcertinaPanelSizes {
        
        todo!();
        /*
            auto num = sizes.size();
                totalSpace = jmax (totalSpace, getMinimumSize (0, num));
                targetPosition = jmax (targetPosition, totalSpace - getMaximumSize (index, num));

                ConcertinaPanelSizes newSizes (*this);
                newSizes.stretchRange (0, index, targetPosition - newSizes.getTotalSize (0, index), stretchLast);
                newSizes.stretchRange (index, num, totalSpace - newSizes.getTotalSize (0, index) - newSizes.getTotalSize (index, num), stretchFirst);
                return newSizes;
        */
    }
    
    pub fn fitted_into(&self, total_space: i32) -> ConcertinaPanelSizes {
        
        todo!();
        /*
            auto newSizes (*this);
                auto num = newSizes.sizes.size();
                totalSpace = jmax (totalSpace, getMinimumSize (0, num));
                newSizes.stretchRange (0, num, totalSpace - newSizes.getTotalSize (0, num), stretchAll);
                return newSizes;
        */
    }
    
    pub fn with_resized_panel(&self, 
        index:        i32,
        panel_height: i32,
        total_space:  i32) -> ConcertinaPanelSizes {
        
        todo!();
        /*
            ConcertinaPanelSizes newSizes (*this);

                if (totalSpace <= 0)
                {
                    newSizes.get(index).size = panelHeight;
                }
                else
                {
                    auto num = sizes.size();
                    auto minSize = getMinimumSize (0, num);
                    totalSpace = jmax (totalSpace, minSize);

                    newSizes.get(index).setSize (panelHeight);
                    newSizes.stretchRange (0, index,   totalSpace - newSizes.getTotalSize (0, num), stretchLast);
                    newSizes.stretchRange (index, num, totalSpace - newSizes.getTotalSize (0, num), stretchLast);
                    newSizes = newSizes.fittedInto (totalSpace);
                }

                return newSizes;
        */
    }
    
    pub fn grow_range_first(&mut self, 
        start:      i32,
        end:        i32,
        space_diff: i32)  {
        
        todo!();
        /*
            for (int attempts = 4; --attempts >= 0 && spaceDiff > 0;)
                    for (int i = start; i < end && spaceDiff > 0; ++i)
                        spaceDiff -= get (i).expand (spaceDiff);
        */
    }
    
    pub fn grow_range_last(&mut self, 
        start:      i32,
        end:        i32,
        space_diff: i32)  {
        
        todo!();
        /*
            for (int attempts = 4; --attempts >= 0 && spaceDiff > 0;)
                    for (int i = end; --i >= start && spaceDiff > 0;)
                        spaceDiff -= get (i).expand (spaceDiff);
        */
    }
    
    pub fn grow_range_all(&mut self, 
        start:      i32,
        end:        i32,
        space_diff: i32)  {
        
        todo!();
        /*
            Vec<Panel*> expandableItems;

                for (int i = start; i < end; ++i)
                    if (get(i).canExpand() && ! get(i).isMinimised())
                        expandableItems.add (& get(i));

                for (int attempts = 4; --attempts >= 0 && spaceDiff > 0;)
                    for (int i = expandableItems.size(); --i >= 0 && spaceDiff > 0;)
                        spaceDiff -= expandableItems.getUnchecked(i)->expand (spaceDiff / (i + 1));

                growRangeLast (start, end, spaceDiff);
        */
    }
    
    pub fn shrink_range_first(&mut self, 
        start:      i32,
        end:        i32,
        space_diff: i32)  {
        
        todo!();
        /*
            for (int i = start; i < end && spaceDiff > 0; ++i)
                    spaceDiff -= get(i).reduce (spaceDiff);
        */
    }
    
    pub fn shrink_range_last(&mut self, 
        start:      i32,
        end:        i32,
        space_diff: i32)  {
        
        todo!();
        /*
            for (int i = end; --i >= start && spaceDiff > 0;)
                    spaceDiff -= get(i).reduce (spaceDiff);
        */
    }
    
    pub fn stretch_range(&mut self, 
        start:         i32,
        end:           i32,
        amount_to_add: i32,
        expand_mode:   ConcertinaPanelSizesExpandMode)  {
        
        todo!();
        /*
            if (end > start)
                {
                    if (amountToAdd > 0)
                    {
                        if (expandMode == stretchAll)        growRangeAll   (start, end, amountToAdd);
                        else if (expandMode == stretchFirst) growRangeFirst (start, end, amountToAdd);
                        else if (expandMode == stretchLast)  growRangeLast  (start, end, amountToAdd);
                    }
                    else
                    {
                        if (expandMode == stretchFirst)  shrinkRangeFirst (start, end, -amountToAdd);
                        else                             shrinkRangeLast  (start, end, -amountToAdd);
                    }
                }
        */
    }
    
    pub fn get_total_size(&self, 
        start: i32,
        end:   i32) -> i32 {
        
        todo!();
        /*
            int tot = 0;
                while (start < end)  tot += get (start++).size;
                return tot;
        */
    }
    
    pub fn get_minimum_size(&self, 
        start: i32,
        end:   i32) -> i32 {
        
        todo!();
        /*
            int tot = 0;
                while (start < end)  tot += get (start++).minSize;
                return tot;
        */
    }
    
    pub fn get_maximum_size(&self, 
        start: i32,
        end:   i32) -> i32 {
        
        todo!();
        /*
            int tot = 0;

                while (start < end)
                {
                    auto mx = get (start++).maxSize;

                    if (mx > 0x100000)
                        return mx;

                    tot += mx;
                }

                return tot;
        */
    }
}

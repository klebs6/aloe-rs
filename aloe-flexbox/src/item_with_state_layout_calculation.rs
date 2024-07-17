crate::ix!();

///-----------------
pub struct FlexBoxLayoutCalculationItemWithState<'a> {
    item:                 *mut FlexItem<'a>,
    locked_width:         FlexBoxLayoutCalculationCoord, // default = 0
    locked_height:        FlexBoxLayoutCalculationCoord, // default = 0
    locked_margin_left:   FlexBoxLayoutCalculationCoord, // default = 0
    locked_margin_right:  FlexBoxLayoutCalculationCoord, // default = 0
    locked_margin_top:    FlexBoxLayoutCalculationCoord, // default = 0
    locked_margin_bottom: FlexBoxLayoutCalculationCoord, // default = 0
    preferred_width:      FlexBoxLayoutCalculationCoord, // default = 0
    preferred_height:     FlexBoxLayoutCalculationCoord, // default = 0
    locked:               bool, // default = false
}

impl<'a> FlexBoxLayoutCalculationItemWithState<'a> {

    pub fn new(source: &mut FlexItem) -> Self {
    
        todo!();
        /*
        : item(&source),

        
        */
    }
    
    pub fn reset_item_locked_size(&mut self)  {
        
        todo!();
        /*
            lockedWidth        = preferredWidth;
                lockedHeight       = preferredHeight;
                lockedMarginLeft   = getValueOrZeroIfAuto (item->margin.left);
                lockedMarginRight  = getValueOrZeroIfAuto (item->margin.right);
                lockedMarginTop    = getValueOrZeroIfAuto (item->margin.top);
                lockedMarginBottom = getValueOrZeroIfAuto (item->margin.bottom);
        */
    }
    
    pub fn set_width_checked(&mut self, new_width: FlexBoxLayoutCalculationCoord)  {
        
        todo!();
        /*
            if (isAssigned (item->maxWidth))  newWidth = jmin (newWidth, static_cast<FlexBoxLayoutCalculationCoord> (item->maxWidth));
                if (isAssigned (item->minWidth))  newWidth = jmax (newWidth, static_cast<FlexBoxLayoutCalculationCoord> (item->minWidth));

                lockedWidth = newWidth;
        */
    }
    
    pub fn set_height_checked(&mut self, new_height: FlexBoxLayoutCalculationCoord)  {
        
        todo!();
        /*
            if (isAssigned (item->maxHeight))  newHeight = jmin (newHeight, (FlexBoxLayoutCalculationCoord) item->maxHeight);
                if (isAssigned (item->minHeight))  newHeight = jmax (newHeight, (FlexBoxLayoutCalculationCoord) item->minHeight);

                lockedHeight = newHeight;
        */
    }
}

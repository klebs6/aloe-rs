crate::ix!();

pub struct FlexBoxLayoutCalculation<'a> {
    owner:                  &'a mut FlexBox<'a>,
    parent_width:           FlexBoxLayoutCalculationCoord,
    parent_height:          FlexBoxLayoutCalculationCoord,
    num_items:              i32,
    is_row_direction:       bool,
    container_line_length:  FlexBoxLayoutCalculationCoord,
    number_of_rows:         i32, // default = 1
    container_cross_length: FlexBoxLayoutCalculationCoord, // default = 0
    line_items:             HeapBlock<*mut FlexBoxLayoutCalculationItemWithState<'a>>,
    line_info:              HeapBlock<FlexBoxLayoutCalculationRowInfo>,
    item_states:            Vec<FlexBoxLayoutCalculationItemWithState<'a>>,
}

pub type FlexBoxLayoutCalculationCoord = f64;

impl<'a> FlexBoxLayoutCalculation<'a> {

    pub fn new(
        fb: &mut FlexBox,
        w:  FlexBoxLayoutCalculationCoord,
        h:  FlexBoxLayoutCalculationCoord) -> Self {
    
        todo!();
        /*


            : owner (fb), parentWidth (w), parentHeight (h), numItems (owner.items.size()),
             isRowDirection (fb.flexDirection == FlexBox::FlexBoxDirection::row
                          || fb.flexDirection == FlexBox::FlexBoxDirection::rowReverse),
             containerLineLength (isRowDirection ? parentWidth : parentHeight)

            lineItems.calloc (numItems * numItems);
            lineInfo.calloc (numItems);
        */
    }
    
    pub fn get_item(&self, x: i32, y: i32) -> &mut FlexBoxLayoutCalculationItemWithState {
        
        todo!();
        /*
            return *lineItems[y * numItems + x];
        */
    }
    
    pub fn is_auto(value: FlexBoxLayoutCalculationCoord) -> bool {
        
        todo!();
        /*
            return value == FlexItem::autoValue;
        */
    }
    
    pub fn is_assigned(value: FlexBoxLayoutCalculationCoord) -> bool {
        
        todo!();
        /*
            return value != FlexItem::notAssigned;
        */
    }
    
    pub fn get_value_or_zero_if_auto(value: FlexBoxLayoutCalculationCoord) -> FlexBoxLayoutCalculationCoord {
        
        todo!();
        /*
            return isAuto (value) ? FlexBoxLayoutCalculationCoord() : value;
        */
    }
    
    pub fn create_states(&mut self)  {
        
        todo!();
        /*
            itemStates.ensureStorageAllocated (numItems);

            for (auto& item : owner.items)
                itemStates.add (item);

            std::stable_sort (itemStates.begin(), itemStates.end(),
                              [] (const FlexBoxLayoutCalculationItemWithState& i1, const FlexBoxLayoutCalculationItemWithState& i2)  { return i1.item->order < i2.item->order; });

            for (auto& item : itemStates)
            {
                item.preferredWidth  = getPreferredWidth  (item);
                item.preferredHeight = getPreferredHeight (item);
            }
        */
    }
    
    pub fn initialise_items(&mut self)  {
        
        todo!();
        /*
            if (owner.flexWrap == FlexBox::FlexBoxWrap::noWrap)  // for single-line, all items go in line 1
            {
                lineInfo[0].numItems = numItems;
                int i = 0;

                for (auto& item : itemStates)
                {
                    item.resetItemLockedSize();
                    lineItems[i++] = &item;
                }
            }
            else // if multi-line, group the flexbox items into multiple lines
            {
                auto currentLength = containerLineLength;
                int column = 0, row = 0;
                bool firstRow = true;

                for (auto& item : itemStates)
                {
                    item.resetItemLockedSize();

                    const auto flexitemLength = getItemLength (item);

                    if (flexitemLength > currentLength)
                    {
                        if (! firstRow)
                            row++;

                        if (row >= numItems)
                            break;

                        column = 0;
                        currentLength = containerLineLength;
                        numberOfRows = jmax (numberOfRows, row + 1);
                    }

                    currentLength -= flexitemLength;
                    lineItems[row * numItems + column] = &item;
                    ++column;
                    lineInfo[row].numItems = jmax (lineInfo[row].numItems, column);
                    firstRow = false;
                }
            }
        */
    }
    
    pub fn resolve_flexible_lengths(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                resetRowItems (row);

                for (int maxLoops = numItems; --maxLoops >= 0;)
                {
                    resetUnlockedRowItems (row);

                    if (layoutRowItems (row))
                        break;
                }
            }
        */
    }
    
    pub fn resolve_auto_margins_on_main_axis(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                FlexBoxLayoutCalculationCoord allFlexGrow = 0;
                const auto numColumns = lineInfo[row].numItems;
                const auto remainingLength = containerLineLength - lineInfo[row].totalLength;

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    if (isRowDirection)
                    {
                        if (isAuto (item.item->margin.left))    ++allFlexGrow;
                        if (isAuto (item.item->margin.right))   ++allFlexGrow;
                    }
                    else
                    {
                        if (isAuto (item.item->margin.top))     ++allFlexGrow;
                        if (isAuto (item.item->margin.bottom))  ++allFlexGrow;
                    }
                }

                auto changeUnitWidth = remainingLength / allFlexGrow;

                if (changeUnitWidth > 0)
                {
                    for (int column = 0; column < numColumns; ++column)
                    {
                        auto& item = getItem (column, row);

                        if (isRowDirection)
                        {
                            if (isAuto (item.item->margin.left))    item.lockedMarginLeft  = changeUnitWidth;
                            if (isAuto (item.item->margin.right))   item.lockedMarginRight = changeUnitWidth;
                        }
                        else
                        {
                            if (isAuto (item.item->margin.top))     item.lockedMarginTop    = changeUnitWidth;
                            if (isAuto (item.item->margin.bottom))  item.lockedMarginBottom = changeUnitWidth;
                        }
                    }
                }
            }
        */
    }
    
    pub fn calculate_cross_sizes_by_line(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                FlexBoxLayoutCalculationCoord maxSize = 0;
                const auto numColumns = lineInfo[row].numItems;

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    maxSize = jmax (maxSize, isRowDirection ? item.lockedHeight + item.lockedMarginTop  + item.lockedMarginBottom
                                                            : item.lockedWidth  + item.lockedMarginLeft + item.lockedMarginRight);
                }

                lineInfo[row].crossSize = maxSize;
            }
        */
    }
    
    pub fn calculate_cross_size_of_all_items(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                const auto numColumns = lineInfo[row].numItems;

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    if (isAssigned (item.item->maxHeight) && item.lockedHeight > item.item->maxHeight)
                        item.lockedHeight = item.item->maxHeight;

                    if (isAssigned (item.item->maxWidth) && item.lockedWidth > item.item->maxWidth)
                        item.lockedWidth = item.item->maxWidth;
                }
            }
        */
    }
    
    pub fn align_lines_per_align_content(&mut self)  {
        
        todo!();
        /*
            containerCrossLength = isRowDirection ? parentHeight : parentWidth;

            if (owner.alignContent == FlexBox::FlexBoxAlignContent::flexStart)
            {
                for (int row = 0; row < numberOfRows; ++row)
                    for (int row2 = row; row2 < numberOfRows; ++row2)
                        lineInfo[row].lineY = row == 0 ? 0 : lineInfo[row - 1].lineY + lineInfo[row - 1].crossSize;
            }
            else if (owner.alignContent == FlexBox::FlexBoxAlignContent::flexEnd)
            {
                for (int row = 0; row < numberOfRows; ++row)
                {
                    FlexBoxLayoutCalculationCoord crossHeights = 0;

                    for (int row2 = row; row2 < numberOfRows; ++row2)
                        crossHeights += lineInfo[row2].crossSize;

                    lineInfo[row].lineY = containerCrossLength - crossHeights;
                }
            }
            else
            {
                FlexBoxLayoutCalculationCoord totalHeight = 0;

                for (int row = 0; row < numberOfRows; ++row)
                    totalHeight += lineInfo[row].crossSize;

                if (owner.alignContent == FlexBox::FlexBoxAlignContent::stretch)
                {
                    const auto difference = jmax (FlexBoxLayoutCalculationCoord(), (containerCrossLength - totalHeight) / numberOfRows);

                    for (int row = 0; row < numberOfRows; ++row)
                    {
                        lineInfo[row].crossSize += difference;
                        lineInfo[row].lineY = row == 0 ? 0 : lineInfo[row - 1].lineY + lineInfo[row - 1].crossSize;
                    }
                }
                else if (owner.alignContent == FlexBox::FlexBoxAlignContent::center)
                {
                    const auto additionalength = (containerCrossLength - totalHeight) / 2;

                    for (int row = 0; row < numberOfRows; ++row)
                        lineInfo[row].lineY = row == 0 ? additionalength : lineInfo[row - 1].lineY + lineInfo[row - 1].crossSize;
                }
                else if (owner.alignContent == FlexBox::FlexBoxAlignContent::spaceBetween)
                {
                    const auto additionalength = numberOfRows <= 1 ? FlexBoxLayoutCalculationCoord() : jmax (FlexBoxLayoutCalculationCoord(), (containerCrossLength - totalHeight)
                                                                                                / static_cast<FlexBoxLayoutCalculationCoord> (numberOfRows - 1));
                    lineInfo[0].lineY = 0;

                    for (int row = 1; row < numberOfRows; ++row)
                        lineInfo[row].lineY += additionalength + lineInfo[row - 1].lineY + lineInfo[row - 1].crossSize;
                }
                else if (owner.alignContent == FlexBox::FlexBoxAlignContent::spaceAround)
                {
                    const auto additionalength = numberOfRows <= 1 ? FlexBoxLayoutCalculationCoord() : jmax (FlexBoxLayoutCalculationCoord(), (containerCrossLength - totalHeight)
                                                                                                / static_cast<FlexBoxLayoutCalculationCoord> (2 + (2 * (numberOfRows - 1))));

                    lineInfo[0].lineY = additionalength;

                    for (int row = 1; row < numberOfRows; ++row)
                        lineInfo[row].lineY += (2 * additionalength) + lineInfo[row - 1].lineY + lineInfo[row - 1].crossSize;
                }
            }
        */
    }
    
    pub fn resolve_auto_margins_on_cross_axis(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                const auto numColumns = lineInfo[row].numItems;
                const auto crossSizeForLine = lineInfo[row].crossSize;

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    if (isRowDirection)
                    {
                        if (isAuto (item.item->margin.top) && isAuto (item.item->margin.bottom))
                            item.lockedMarginTop = (crossSizeForLine - item.lockedHeight) / 2;
                        else if (isAuto (item.item->margin.top))
                            item.lockedMarginTop = crossSizeForLine - item.lockedHeight - item.item->margin.bottom;
                    }
                    else if (isAuto (item.item->margin.left) && isAuto (item.item->margin.right))
                    {
                        item.lockedMarginLeft = jmax (FlexBoxLayoutCalculationCoord(), (crossSizeForLine - item.lockedWidth) / 2);
                    }
                    else if (isAuto (item.item->margin.top))
                    {
                        item.lockedMarginLeft = jmax (FlexBoxLayoutCalculationCoord(), crossSizeForLine - item.lockedHeight - item.item->margin.bottom);
                    }
                }
            }
        */
    }
    
    pub fn align_items_in_cross_axis_in_lines_per_align_items(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                const auto numColumns = lineInfo[row].numItems;
                const auto lineSize = lineInfo[row].crossSize;

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    if (item.item->alignSelf == FlexItem::AlignSelf::autoAlign)
                    {
                        if (owner.alignItems == FlexBox::FlexBoxAlignItems::stretch)
                        {
                            item.lockedMarginTop = item.item->margin.top;

                            if (isRowDirection)
                                item.setHeightChecked (lineSize - item.item->margin.top - item.item->margin.bottom);
                            else
                                item.setWidthChecked (lineSize - item.item->margin.left - item.item->margin.right);
                        }
                        else if (owner.alignItems == FlexBox::FlexBoxAlignItems::flexStart)
                        {
                            item.lockedMarginTop = item.item->margin.top;
                        }
                        else if (owner.alignItems == FlexBox::FlexBoxAlignItems::flexEnd)
                        {
                            if (isRowDirection)
                                item.lockedMarginTop = lineSize - item.lockedHeight - item.item->margin.bottom;
                            else
                                item.lockedMarginLeft = lineSize - item.lockedWidth - item.item->margin.right;
                        }
                        else if (owner.alignItems == FlexBox::FlexBoxAlignItems::center)
                        {
                            if (isRowDirection)
                                item.lockedMarginTop = (lineSize - item.lockedHeight - item.item->margin.top - item.item->margin.bottom) / 2;
                            else
                                item.lockedMarginLeft = (lineSize - item.lockedWidth - item.item->margin.left - item.item->margin.right) / 2;
                        }
                    }
                }
            }
        */
    }
    
    pub fn align_lines_per_align_self(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                const auto numColumns = lineInfo[row].numItems;
                const auto lineSize = lineInfo[row].crossSize;

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    if (! isAuto (item.item->margin.top))
                    {
                        if (item.item->alignSelf == FlexItem::AlignSelf::flexStart)
                        {
                            if (isRowDirection)
                                item.lockedMarginTop = item.item->margin.top;
                            else
                                item.lockedMarginLeft = item.item->margin.left;
                        }
                        else if (item.item->alignSelf == FlexItem::AlignSelf::flexEnd)
                        {
                            if (isRowDirection)
                                item.lockedMarginTop = lineSize - item.lockedHeight - item.item->margin.bottom;
                            else
                                item.lockedMarginLeft = lineSize - item.lockedWidth - item.item->margin.right;
                        }
                        else if (item.item->alignSelf == FlexItem::AlignSelf::center)
                        {
                            if (isRowDirection)
                                item.lockedMarginTop = item.item->margin.top + (lineSize - item.lockedHeight - item.item->margin.top - item.item->margin.bottom) / 2;
                            else
                                item.lockedMarginLeft = item.item->margin.left + (lineSize - item.lockedWidth - item.item->margin.left - item.item->margin.right) / 2;
                        }
                        else if (item.item->alignSelf == FlexItem::AlignSelf::stretch)
                        {
                            item.lockedMarginTop  = item.item->margin.top;
                            item.lockedMarginLeft = item.item->margin.left;

                            if (isRowDirection)
                                item.setHeightChecked (isAssigned (item.item->height) ? getPreferredHeight (item)
                                                                                      : lineSize - item.item->margin.top - item.item->margin.bottom);
                            else
                                item.setWidthChecked (isAssigned (item.item->width) ? getPreferredWidth (item)
                                                                                    : lineSize - item.item->margin.left - item.item->margin.right);
                        }
                    }
                }
            }
        */
    }
    
    pub fn align_items_by_justify_content(&mut self)  {
        
        todo!();
        /*
            FlexBoxLayoutCalculationCoord additionalMarginRight = 0, additionalMarginLeft = 0;

            recalculateTotalItemLengthPerLineArray();

            for (int row = 0; row < numberOfRows; ++row)
            {
                const auto numColumns = lineInfo[row].numItems;
                FlexBoxLayoutCalculationCoord x = 0;

                if (owner.justifyContent == FlexBox::FlexBoxJustifyContent::flexEnd)
                {
                    x = containerLineLength - lineInfo[row].totalLength;
                }
                else if (owner.justifyContent == FlexBox::FlexBoxJustifyContent::center)
                {
                    x = (containerLineLength - lineInfo[row].totalLength) / 2;
                }
                else if (owner.justifyContent == FlexBox::FlexBoxJustifyContent::spaceBetween)
                {
                    additionalMarginRight
                        = jmax (FlexBoxLayoutCalculationCoord(), (containerLineLength - lineInfo[row].totalLength) / jmax (1, numColumns - 1));
                }
                else if (owner.justifyContent == FlexBox::FlexBoxJustifyContent::spaceAround)
                {
                    additionalMarginLeft = additionalMarginRight
                        = jmax (FlexBoxLayoutCalculationCoord(), (containerLineLength - lineInfo[row].totalLength) / jmax (1, 2 * numColumns));
                }

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    if (isRowDirection)
                    {
                        item.lockedMarginLeft  += additionalMarginLeft;
                        item.lockedMarginRight += additionalMarginRight;
                        item.item->currentBounds.setPosition ((float) (x + item.lockedMarginLeft), (float) item.lockedMarginTop);
                        x += item.lockedWidth + item.lockedMarginLeft + item.lockedMarginRight;
                    }
                    else
                    {
                        item.lockedMarginTop    += additionalMarginLeft;
                        item.lockedMarginBottom += additionalMarginRight;
                        item.item->currentBounds.setPosition ((float) item.lockedMarginLeft, (float) (x + item.lockedMarginTop));
                        x += item.lockedHeight + item.lockedMarginTop + item.lockedMarginBottom;
                    }
                }
            }
        */
    }
    
    pub fn layout_all_items(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                const auto lineY = lineInfo[row].lineY;
                const auto numColumns = lineInfo[row].numItems;

                for (int column = 0; column < numColumns; ++column)
                {
                    auto& item = getItem (column, row);

                    if (isRowDirection)
                        item.item->currentBounds.setY ((float) (lineY + item.lockedMarginTop));
                    else
                        item.item->currentBounds.setX ((float) (lineY + item.lockedMarginLeft));

                    item.item->currentBounds.setSize ((float) item.lockedWidth,
                                                      (float) item.lockedHeight);
                }
            }

            reverseLocations();
            reverseWrap();
        */
    }
    
    pub fn reset_row_items(&mut self, row: i32)  {
        
        todo!();
        /*
            const auto numColumns = lineInfo[row].numItems;

            for (int column = 0; column < numColumns; ++column)
                resetItem (getItem (column, row));
        */
    }
    
    pub fn reset_unlocked_row_items(&mut self, row: i32)  {
        
        todo!();
        /*
            const auto numColumns = lineInfo[row].numItems;

            for (int column = 0; column < numColumns; ++column)
            {
                auto& item = getItem (column, row);

                if (! item.locked)
                    resetItem (item);
            }
        */
    }
    
    pub fn reset_item(&mut self, item: &mut FlexBoxLayoutCalculationItemWithState)  {
        
        todo!();
        /*
            item.locked = false;
            item.lockedWidth  = getPreferredWidth (item);
            item.lockedHeight = getPreferredHeight (item);
        */
    }
    
    pub fn layout_row_items(&mut self, row: i32) -> bool {
        
        todo!();
        /*
            const auto numColumns = lineInfo[row].numItems;
            auto flexContainerLength = containerLineLength;
            FlexBoxLayoutCalculationCoord totalItemsLength = 0, totalFlexGrow = 0, totalFlexShrink = 0;

            for (int column = 0; column < numColumns; ++column)
            {
                const auto& item = getItem (column, row);

                if (item.locked)
                {
                    flexContainerLength -= getItemLength (item);
                }
                else
                {
                    totalItemsLength += getItemLength (item);
                    totalFlexGrow   += item.item->flexGrow;
                    totalFlexShrink += item.item->flexShrink;
                }
            }

            FlexBoxLayoutCalculationCoord changeUnit = 0;
            const auto difference = flexContainerLength - totalItemsLength;
            const bool positiveFlexibility = difference > 0;

            if (positiveFlexibility)
            {
                if (totalFlexGrow != 0.0)
                    changeUnit = difference / totalFlexGrow;
            }
            else
            {
                if (totalFlexShrink != 0.0)
                    changeUnit = difference / totalFlexShrink;
            }

            bool ok = true;

            for (int column = 0; column < numColumns; ++column)
            {
                auto& item = getItem (column, row);

                if (! item.locked)
                    if (! addToItemLength (item, (positiveFlexibility ? item.item->flexGrow
                                                                      : item.item->flexShrink) * changeUnit, row))
                        ok = false;
            }

            return ok;
        */
    }
    
    pub fn recalculate_total_item_length_per_line_array(&mut self)  {
        
        todo!();
        /*
            for (int row = 0; row < numberOfRows; ++row)
            {
                lineInfo[row].totalLength = 0;
                const auto numColumns = lineInfo[row].numItems;

                for (int column = 0; column < numColumns; ++column)
                {
                    const auto& item = getItem (column, row);

                    lineInfo[row].totalLength += isRowDirection ? item.lockedWidth + item.lockedMarginLeft + item.lockedMarginRight
                                                                : item.lockedHeight + item.lockedMarginTop + item.lockedMarginBottom;
                }
            }
        */
    }
    
    pub fn reverse_locations(&mut self)  {
        
        todo!();
        /*
            if (owner.flexDirection == FlexBox::FlexBoxDirection::rowReverse)
            {
                for (auto& item : owner.items)
                    item.currentBounds.setX ((float) (containerLineLength - item.currentBounds.getRight()));
            }
            else if (owner.flexDirection == FlexBox::FlexBoxDirection::columnReverse)
            {
                for (auto& item : owner.items)
                    item.currentBounds.setY ((float) (containerLineLength - item.currentBounds.getBottom()));
            }
        */
    }
    
    pub fn reverse_wrap(&mut self)  {
        
        todo!();
        /*
            if (owner.flexWrap == FlexBox::FlexBoxWrap::wrapReverse)
            {
                if (isRowDirection)
                {
                    for (auto& item : owner.items)
                        item.currentBounds.setY ((float) (containerCrossLength - item.currentBounds.getBottom()));
                }
                else
                {
                    for (auto& item : owner.items)
                        item.currentBounds.setX ((float) (containerCrossLength - item.currentBounds.getRight()));
                }
            }
        */
    }
    
    pub fn get_item_length(&self, item: &FlexBoxLayoutCalculationItemWithState) -> FlexBoxLayoutCalculationCoord {
        
        todo!();
        /*
            return isRowDirection ? item.lockedWidth  + item.lockedMarginLeft + item.lockedMarginRight
                                  : item.lockedHeight + item.lockedMarginTop  + item.lockedMarginBottom;
        */
    }
    
    pub fn get_item_cross_size(&self, item: &FlexBoxLayoutCalculationItemWithState) -> FlexBoxLayoutCalculationCoord {
        
        todo!();
        /*
            return isRowDirection ? item.lockedHeight + item.lockedMarginTop  + item.lockedMarginBottom
                                  : item.lockedWidth  + item.lockedMarginLeft + item.lockedMarginRight;
        */
    }
    
    pub fn add_to_item_length(&self, 
        item:   &mut FlexBoxLayoutCalculationItemWithState,
        length: FlexBoxLayoutCalculationCoord,
        row:    i32) -> bool {
        
        todo!();
        /*
            bool ok = false;

            if (isRowDirection)
            {
                const auto prefWidth = getPreferredWidth (item);

                if (isAssigned (item.item->maxWidth) && item.item->maxWidth < prefWidth + length)
                {
                    item.lockedWidth = item.item->maxWidth;
                    item.locked = true;
                }
                else if (isAssigned (prefWidth) && item.item->minWidth > prefWidth + length)
                {
                    item.lockedWidth = item.item->minWidth;
                    item.locked = true;
                }
                else
                {
                    ok = true;
                    item.lockedWidth = prefWidth + length;
                }

                lineInfo[row].totalLength += item.lockedWidth + item.lockedMarginLeft + item.lockedMarginRight;
            }
            else
            {
                const auto prefHeight = getPreferredHeight (item);

                if (isAssigned (item.item->maxHeight) && item.item->maxHeight < prefHeight + length)
                {
                    item.lockedHeight = item.item->maxHeight;
                    item.locked = true;
                }
                else if (isAssigned (prefHeight) && item.item->minHeight > prefHeight + length)
                {
                    item.lockedHeight = item.item->minHeight;
                    item.locked = true;
                }
                else
                {
                    ok = true;
                    item.lockedHeight = prefHeight + length;
                }

                lineInfo[row].totalLength += item.lockedHeight + item.lockedMarginTop + item.lockedMarginBottom;
            }

            return ok;
        */
    }
    
    pub fn get_preferred_width(&self, item_with_state: &FlexBoxLayoutCalculationItemWithState) -> FlexBoxLayoutCalculationCoord {
        
        todo!();
        /*
            const auto& item = *itemWithState.item;
            auto preferredWidth = (item.flexBasis > 0 && isRowDirection)
                                     ? item.flexBasis
                                     : (isAssigned (item.width) ? item.width : item.minWidth);

            if (isAssigned (item.minWidth) && preferredWidth < item.minWidth)  return item.minWidth;
            if (isAssigned (item.maxWidth) && preferredWidth > item.maxWidth)  return item.maxWidth;

            return preferredWidth;
        */
    }
    
    pub fn get_preferred_height(&self, item_with_state: &FlexBoxLayoutCalculationItemWithState) -> FlexBoxLayoutCalculationCoord {
        
        todo!();
        /*
            const auto& item = *itemWithState.item;
            auto preferredHeight = (item.flexBasis > 0 && ! isRowDirection)
                                     ? item.flexBasis
                                     : (isAssigned (item.height) ? item.height : item.minHeight);

            if (isAssigned (item.minHeight) && preferredHeight < item.minHeight)  return item.minHeight;
            if (isAssigned (item.maxHeight) && preferredHeight > item.maxHeight)  return item.maxHeight;

            return preferredHeight;
        */
    }
}

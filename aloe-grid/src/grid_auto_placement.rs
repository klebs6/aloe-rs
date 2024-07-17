crate::ix!();

///---------------
pub struct GridAutoPlacementOccupancyPlaneCell { 
    column: i32,
    row:    i32,
}

#[derive(Eq,PartialEq)]
pub struct GridAutoPlacementOccupancyPlaneSortableCell {
    column:       i32,
    row:          i32,
    column_first: bool,
}

impl Ord for GridAutoPlacementOccupancyPlaneSortableCell {
    
    #[inline] fn cmp(&self, other: &GridAutoPlacementOccupancyPlaneSortableCell) -> std::cmp::Ordering {
        todo!();
        /*
            if (columnFirst)
                {
                    if (row == other.row)
                        return column < other.column;

                    return row < other.row;
                }

                if (row == other.row)
                    return column < other.column;

                return row < other.row;
        */
    }
}

impl PartialOrd<GridAutoPlacementOccupancyPlaneSortableCell> 
for GridAutoPlacementOccupancyPlaneSortableCell {

    #[inline] fn partial_cmp(&self, other: &GridAutoPlacementOccupancyPlaneSortableCell) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

///---------------
pub struct GridAutoPlacementOccupancyPlane {
    highest_cross_dimension: i32,
    column_first:            bool,
    occupied_cells:          HashSet<GridAutoPlacementOccupancyPlaneSortableCell>,
}

impl GridAutoPlacementOccupancyPlane {

    pub fn new(
        highest_column_to_use: i32,
        highest_row_to_use:    i32,
        is_column_first:       bool) -> Self {
    
        todo!();
        /*
            : highestCrossDimension (isColumnFirst ? highestRowToUse : highestColumnToUse),
                  columnFirst (isColumnFirst)
        */
    }
    
    pub fn set_cell(
        &mut self, 
        cell:        GridAutoPlacementOccupancyPlaneCell,
        column_span: i32,
        row_span:    i32) -> GridPlacementHelpersLineArea {
        
        todo!();
        /*
            for (int i = 0; i < columnSpan; i++)
                    for (int j = 0; j < rowSpan; j++)
                        setCell (cell.column + i, cell.row + j);

                return { { cell.column, cell.column + columnSpan }, { cell.row, cell.row + rowSpan } };
        */
    }
    
    pub fn set_cell_with_range(&mut self, 
        start: GridAutoPlacementOccupancyPlaneCell,
        end:   GridAutoPlacementOccupancyPlaneCell) -> GridPlacementHelpersLineArea {
        
        todo!();
        /*
            return setCell (start, std::abs (end.column - start.column),
                                       std::abs (end.row - start.row));
        */
    }
    
    pub fn next_available(
        &mut self, 
        reference_cell: GridAutoPlacementOccupancyPlaneCell,
        column_span:    i32,
        row_span:       i32) -> GridAutoPlacementOccupancyPlaneCell {
        
        todo!();
        /*
            while (isOccupied (referenceCell, columnSpan, rowSpan) || isOutOfBounds (referenceCell, columnSpan, rowSpan))
                    referenceCell = advance (referenceCell);

                return referenceCell;
        */
    }
    
    pub fn next_available_on_row(
        &mut self, 
        reference_cell: GridAutoPlacementOccupancyPlaneCell,
        column_span:    i32,
        row_span:       i32,
        row_number:     i32) -> GridAutoPlacementOccupancyPlaneCell {
        
        todo!();
        /*
            if (columnFirst && (rowNumber + rowSpan) > highestCrossDimension)
                    highestCrossDimension = rowNumber + rowSpan;

                while (isOccupied (referenceCell, columnSpan, rowSpan)
                       || (referenceCell.row != rowNumber))
                    referenceCell = advance (referenceCell);

                return referenceCell;
        */
    }
    
    pub fn next_available_on_column(
        &mut self, 
        reference_cell: GridAutoPlacementOccupancyPlaneCell,
        column_span:    i32,
        row_span:       i32,
        column_number:  i32) -> GridAutoPlacementOccupancyPlaneCell {
        
        todo!();
        /*
            if (! columnFirst && (columnNumber + columnSpan) > highestCrossDimension)
                    highestCrossDimension = columnNumber + columnSpan;

                while (isOccupied (referenceCell, columnSpan, rowSpan)
                       || (referenceCell.column != columnNumber))
                    referenceCell = advance (referenceCell);

                return referenceCell;
        */
    }
    
    pub fn set_cell_with_column_and_row(
        &mut self, 
        column: i32,
        row:    i32)  {
        
        todo!();
        /*
            occupiedCells.insert ({ column, row, columnFirst });
        */
    }
    
    pub fn is_occupied(&self, cell: GridAutoPlacementOccupancyPlaneCell) -> bool {
        
        todo!();
        /*
            return occupiedCells.count ({ cell.column, cell.row, columnFirst }) > 0;
        */
    }
    
    pub fn is_occupied_col_span_row_span(
        &self, 
        cell:        GridAutoPlacementOccupancyPlaneCell,
        column_span: i32,
        row_span:    i32) -> bool {
        
        todo!();
        /*
            for (int i = 0; i < columnSpan; i++)
                    for (int j = 0; j < rowSpan; j++)
                        if (isOccupied ({ cell.column + i, cell.row + j }))
                            return true;

                return false;
        */
    }
    
    pub fn is_out_of_bounds(
        &self, 
        cell:        GridAutoPlacementOccupancyPlaneCell,
        column_span: i32,
        row_span:    i32) -> bool {
        
        todo!();
        /*
            const auto crossSpan = columnFirst ? rowSpan : columnSpan;

                return (getCrossDimension (cell) + crossSpan) > getHighestCrossDimension();
        */
    }
    
    pub fn get_highest_cross_dimension(&self) -> i32 {
        
        todo!();
        /*
            GridAutoPlacementOccupancyPlaneCell cell { 1, 1 };

                if (occupiedCells.size() > 0)
                    cell = { occupiedCells.crbegin()->column, occupiedCells.crbegin()->row };

                return std::max (getCrossDimension (cell), highestCrossDimension);
        */
    }
    
    pub fn advance(&self, cell: GridAutoPlacementOccupancyPlaneCell) -> GridAutoPlacementOccupancyPlaneCell {
        
        todo!();
        /*
            if ((getCrossDimension (cell) + 1) >= getHighestCrossDimension())
                    return fromDimensions (getMainDimension (cell) + 1, 1);

                return fromDimensions (getMainDimension (cell), getCrossDimension (cell) + 1);
        */
    }
    
    pub fn get_main_dimension(&self, cell: GridAutoPlacementOccupancyPlaneCell) -> i32 {
        
        todo!();
        /*
            return columnFirst ? cell.column : cell.row;
        */
    }
    
    pub fn get_cross_dimension(&self, cell: GridAutoPlacementOccupancyPlaneCell) -> i32 {
        
        todo!();
        /*
            return columnFirst ? cell.row : cell.column;
        */
    }
    
    pub fn from_dimensions(&self, 
        main_dimension:  i32,
        cross_dimension: i32) -> GridAutoPlacementOccupancyPlaneCell {
        
        todo!();
        /*
            if (columnFirst)
                    return { mainDimension, crossDimension };

                return { crossDimension, mainDimension };
        */
    }
}

pub struct GridAutoPlacement<'a> {
    phantom: PhantomData<&'a ()>,
}

pub trait HasItemPlacementArray {
    type ItemPlacementArray;
}

impl<'a> HasItemPlacementArray for GridAutoPlacement<'a> {
    type ItemPlacementArray = Vec<(*mut GridItem<'a>,GridPlacementHelpersLineArea)>;
}

impl<'a> GridAutoPlacement<'a> {

    pub fn is_fixed(prop: GridItemStartAndEndProperty) -> bool {
        
        todo!();
        /*
            return prop.start.hasName() || prop.start.hasAbsolute() || prop.end.hasName() || prop.end.hasAbsolute();
        */
    }
    
    pub fn has_fully_fixed_placement(item: &GridItem) -> bool {
        
        todo!();
        /*
            if (item.area.isNotEmpty())
                return true;

            if (isFixed (item.column) && isFixed (item.row))
                return true;

            return false;
        */
    }
    
    pub fn has_partial_fixed_placement(item: &GridItem) -> bool {
        
        todo!();
        /*
            if (item.area.isNotEmpty())
                return false;

            if (isFixed (item.column) ^ isFixed (item.row))
                return true;

            return false;
        */
    }
    
    pub fn has_auto_placement(item: &GridItem) -> bool {
        
        todo!();
        /*
            return ! hasFullyFixedPlacement (item) && ! hasPartialFixedPlacement (item);
        */
    }
    
    pub fn has_dense_auto_flow(auto_flow: GridAutoFlow) -> bool {
        
        todo!();
        /*
            return autoFlow == GridAutoFlow::columnDense
                || autoFlow == GridAutoFlow::rowDense;
        */
    }
    
    pub fn is_column_auto_flow(auto_flow: GridAutoFlow) -> bool {
        
        todo!();
        /*
            return autoFlow == GridAutoFlow::column
                || autoFlow == GridAutoFlow::columnDense;
        */
    }
    
    pub fn get_span_from_auto(prop: GridItemStartAndEndProperty) -> i32 {
        
        todo!();
        /*
            if (prop.end.hasSpan())
                return prop.end.getNumber();

            if (prop.start.hasSpan())
                return prop.start.getNumber();

            return 1;
        */
    }
    
    pub fn deduce_all_items(&self, grid: &mut Grid) -> <Self as HasItemPlacementArray>::ItemPlacementArray {
        
        todo!();
        /*
            const auto namedAreas = GridPlacementHelpers::deduceNamedAreas (grid.templateAreas);

            AutoPlacementOccupancyPlane plane (jmax (grid.templateColumns.size() + 1, 2),
                                  jmax (grid.templateRows.size() + 1, 2),
                                  isColumnAutoFlow (grid.autoFlow));

            <Self as HasItemPlacementArray>::ItemPlacementArray itemPlacementArray;
            Vec<GridItem*> sortedItems;

            for (auto& item : grid.items)
                sortedItems.add (&item);

            std::stable_sort (sortedItems.begin(), sortedItems.end(),
                              [] (const GridItem* i1, const GridItem* i2)  { return i1->order < i2->order; });

            // place fixed items first
            for (auto* item : sortedItems)
            {
                if (hasFullyFixedPlacement (*item))
                {
                    const auto a = GridPlacementHelpers::deduceLineArea (*item, grid, namedAreas);
                    plane.setCell ({ a.column.start, a.row.start }, { a.column.end, a.row.end });
                    itemPlacementArray.add ({ item, a });
                }
            }

            AutoPlacementOccupancyPlane::GridAutoPlacementOccupancyPlaneCell lastInsertionCell = { 1, 1 };

            for (auto* item : sortedItems)
            {
                if (hasPartialFixedPlacement (*item))
                {
                    if (isFixed (item->column))
                    {
                        const auto p = GridPlacementHelpers::deduceLineRange (item->column, grid.templateColumns);
                        const auto columnSpan = std::abs (p.start - p.end);
                        const auto rowSpan = getSpanFromAuto (item->row);

                        const auto insertionCell = hasDenseAutoFlow (grid.autoFlow) ? AutoPlacementOccupancyPlane::GridAutoPlacementOccupancyPlaneCell { p.start, 1 }
                                                                                    : lastInsertionCell;
                        const auto nextAvailableCell = plane.nextAvailableOnColumn (insertionCell, columnSpan, rowSpan, p.start);
                        const auto lineArea = plane.setCell (nextAvailableCell, columnSpan, rowSpan);
                        lastInsertionCell = nextAvailableCell;

                        itemPlacementArray.add ({ item, lineArea });
                    }
                    else if (isFixed (item->row))
                    {
                        const auto p = GridPlacementHelpers::deduceLineRange (item->row, grid.templateRows);
                        const auto columnSpan = getSpanFromAuto (item->column);
                        const auto rowSpan = std::abs (p.start - p.end);

                        const auto insertionCell = hasDenseAutoFlow (grid.autoFlow) ? AutoPlacementOccupancyPlane::GridAutoPlacementOccupancyPlaneCell { 1, p.start }
                                                                                    : lastInsertionCell;

                        const auto nextAvailableCell = plane.nextAvailableOnRow (insertionCell, columnSpan, rowSpan, p.start);
                        const auto lineArea = plane.setCell (nextAvailableCell, columnSpan, rowSpan);

                        lastInsertionCell = nextAvailableCell;

                        itemPlacementArray.add ({ item, lineArea });
                    }
                }
            }

            lastInsertionCell = { 1, 1 };

            for (auto* item : sortedItems)
            {
                if (hasAutoPlacement (*item))
                {
                    const auto columnSpan = getSpanFromAuto (item->column);
                    const auto rowSpan = getSpanFromAuto (item->row);

                    const auto nextAvailableCell = plane.nextAvailable (lastInsertionCell, columnSpan, rowSpan);
                    const auto lineArea = plane.setCell (nextAvailableCell, columnSpan, rowSpan);

                    if (! hasDenseAutoFlow (grid.autoFlow))
                        lastInsertionCell = nextAvailableCell;

                    itemPlacementArray.add ({ item,  lineArea });
                }
            }

            return itemPlacementArray;
        */
    }
    
    pub fn get_highest_end_lines_numbers(items: &<Self as HasItemPlacementArray>::ItemPlacementArray) -> (i32,i32) {
        
        todo!();
        /*
            int columnEndLine = 1;
            int rowEndLine = 1;

            for (auto& item : items)
            {
                const auto p = item.second;
                columnEndLine = std::max (p.column.end, columnEndLine);
                rowEndLine = std::max (p.row.end, rowEndLine);
            }

            return { columnEndLine, rowEndLine };
        */
    }
    
    pub fn create_implicit_tracks(
        grid:  &Grid,
        items: &<Self as HasItemPlacementArray>::ItemPlacementArray) -> (Vec<GridTrackInfo>,Vec<GridTrackInfo>) {
        
        todo!();
        /*
            const auto columnAndRowLineEnds = getHighestEndLinesNumbers (items);

            Vec<GridTrackInfo> implicitColumnTracks, implicitRowTracks;

            for (int i = grid.templateColumns.size() + 1; i < columnAndRowLineEnds.first; i++)
                implicitColumnTracks.add (grid.autoColumns);

            for (int i = grid.templateRows.size() + 1; i < columnAndRowLineEnds.second; i++)
                implicitRowTracks.add (grid.autoRows);

            return { implicitColumnTracks, implicitRowTracks };
        */
    }
    
    pub fn apply_size_for_auto_tracks(
        columns:              &mut Vec<GridTrackInfo>,
        rows:                 &mut Vec<GridTrackInfo>,
        item_placement_array: &<Self as HasItemPlacementArray>::ItemPlacementArray)  {
        
        todo!();
        /*
            auto isSpan = [] (GridPlacementHelpers::GridPlacementHelpersLineRange r) -> bool { return std::abs (r.end - r.start) > 1; };

            auto getHighestItemOnRow = [isSpan] (int rowNumber, const <Self as HasItemPlacementArray>::ItemPlacementArray& itemPlacementArrayToUse) -> float
            {
                float highestRowSize = 0.0f;

                for (const auto& i : itemPlacementArrayToUse)
                    if (! isSpan (i.second.row) && i.second.row.start == rowNumber)
                        highestRowSize = std::max (highestRowSize, i.first->height + i.first->margin.top + i.first->margin.bottom);

                return highestRowSize;
            };

            auto getHighestItemOnColumn = [isSpan] (int rowNumber, const <Self as HasItemPlacementArray>::ItemPlacementArray& itemPlacementArrayToUse) -> float
            {
                float highestColumnSize = 0.0f;
                for (const auto& i : itemPlacementArrayToUse)
                    if (! isSpan (i.second.column) && i.second.column.start == rowNumber)
                        highestColumnSize = std::max (highestColumnSize, i.first->width + i.first->margin.left + i.first->margin.right);

                return highestColumnSize;
            };

            for (int i = 0; i < rows.size(); i++)
                if (rows.getReference (i).isAuto())
                    rows.getReference (i).size = getHighestItemOnRow (i + 1, itemPlacementArray);

            for (int i = 0; i < columns.size(); i++)
                if (columns.getReference (i).isAuto())
                    columns.getReference (i).size = getHighestItemOnColumn (i + 1, itemPlacementArray);
        */
    }
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_Grid.h]

/**
  | Container that handles geometry for
  | grid layouts (fixed columns and rows)
  | using a set of declarative rules.
  | 
  | Implemented from the `CSS Grid Layout`
  | specification as described at: https://css-tricks.com/snippets/css/complete-guide-grid/
  | 
  | @see GridItem
  | 
  | @tags{GUI}
  |
  */
pub struct Grid<'a> {

    /**
      | Specifies the alignment of content
      | inside the items along the rows.
      |
      */
    justify_items:    GridJustifyItems, // default = GridJustifyItems::stretch

    /**
      | Specifies the alignment of content
      | inside the items along the columns.
      |
      */
    align_items:      GridAlignItems, // default = GridAlignItems::stretch

    /**
      | Specifies the alignment of items along
      | the rows.
      |
      */
    justify_content:  GridJustifyContent, // default = GridJustifyContent::stretch

    /**
      | Specifies the alignment of items along
      | the columns.
      |
      */
    align_content:    GridAlignContent, // default = GridAlignContent::stretch

    /**
      | Specifies how the auto-placement algorithm
      | places items.
      |
      */
    auto_flow:        GridAutoFlow, // default = GridAutoFlow::row

    /**
      | The set of column tracks to lay out.
      |
      */
    template_columns: Vec<GridTrackInfo>,

    /**
      | The set of row tracks to lay out.
      |
      */
    template_rows:    Vec<GridTrackInfo>,

    /**
      | Template areas
      |
      */
    template_areas:   Vec<String>,

    /**
      | The row track for auto dimension.
      |
      */
    auto_rows:        GridTrackInfo,

    /**
      | The column track for auto dimension.
      |
      */
    auto_columns:     GridTrackInfo,

    /**
      | The gap in pixels between columns.
      |
      */
    column_gap:       GridPx, // default = { 0  }

    /**
      | The gap in pixels between rows.
      |
      */
    row_gap:          GridPx, // default = { 0  }

    /**
      | The set of items to lay-out.
      |
      */
    items:            Vec<GridItem<'a>>,
}

impl<'a> Default for Grid<'a> {
    
    /**
      | Creates an empty Grid container with
      | default parameters.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

lazy_static!{
    /*
    constexpr Grid::GridPx operator"" _px (long double px)          { return Grid::GridPx { px }; }
    constexpr Grid::GridPx operator"" _px (unsigned long long px)   { return Grid::GridPx { px }; }
    constexpr Grid::GridFr operator"" _fr (unsigned long long fr)   { return Grid::GridFr { fr }; }
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_Grid.cpp]
impl<'a> Grid<'a> {

    /**
      | Sets the gap between rows and columns
      | in pixels.
      |
      */
    pub fn set_gap(&mut self, size_in_pixels: GridPx)  {
        
        todo!();
        /*
            rowGap = columnGap = sizeInPixels;
        */
    }

    /**
      | Returns the number of columns.
      |
      */
    pub fn get_number_of_columns(&self) -> i32 {
        
        todo!();
        /*
            return templateColumns.size();
        */
    }

    /**
      | Returns the number of rows.
      |
      */
    pub fn get_number_of_rows(&self) -> i32 {
        
        todo!();
        /*
            return templateRows.size();
        */
    }
    
    /**
      | Lays-out the grid's items within the
      | given rectangle.
      |
      */
    pub fn perform_layout(&mut self, target_area: Rectangle<i32>)  {
        
        todo!();
        /*
            const auto itemsAndAreas = Grid::GridAutoPlacement().deduceAllItems (*this);

        const auto implicitTracks = Grid::GridAutoPlacement::createImplicitTracks (*this, itemsAndAreas);
        auto columnTracks = templateColumns;
        auto rowTracks = templateRows;
        columnTracks.addArray (implicitTracks.first);
        rowTracks.addArray (implicitTracks.second);

        Grid::GridAutoPlacement::applySizeForAutoTracks (columnTracks, rowTracks, itemsAndAreas);

        Grid::GridSizeCalculation calculation;
        calculation.computeSizes (targetArea.toFloat().getWidth(),
                                  targetArea.toFloat().getHeight(),
                                  columnGap,
                                  rowGap,
                                  columnTracks,
                                  rowTracks);

        for (auto& itemAndArea : itemsAndAreas)
        {
            const auto a = itemAndArea.second;
            const auto areaBounds = Grid::GridPlacementHelpers::getAreaBounds (a.column.start, a.column.end,
                                                                           a.row.start, a.row.end,
                                                                           columnTracks,
                                                                           rowTracks,
                                                                           calculation,
                                                                           alignContent,
                                                                           justifyContent,
                                                                           columnGap,
                                                                           rowGap);

            auto* item = itemAndArea.first;
            item->currentBounds = Grid::GridBoxAlignment::alignItem (*item, *this, areaBounds)
                                    + targetArea.toFloat().getPosition();

            if (auto* c = item->associatedComponent)
                c->setBounds (item->currentBounds.toNearestIntEdges());
        }
        */
    }
}

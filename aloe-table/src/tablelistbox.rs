crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TableListBox.h]

/**
  | A table of cells, using a TableHeaderComponent
  | as its header.
  | 
  | This component makes it easy to create
  | a table by providing a TableListBoxModel
  | as the data source.
  | 
  | @see TableListBoxModel, TableHeaderComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TableListBox<'a> {
    base:                        ListBox<'a>,
    base2:                       ListBoxModel,
    header:                      *mut TableHeaderComponent<'a>, // default = nullptr
    model:                       *mut dyn TableListBoxModel,
    column_id_now_being_dragged: i32, // default = 0
    auto_size_options_shown:     bool, // default = true
}

impl<'a> TableHeaderComponentListener for TableListBox<'a> {

    fn table_columns_changed(&mut self, _0: *mut TableHeaderComponent)  {
        
        todo!();
        /*
            setMinimumContentWidth (header->getTotalWidth());
        repaint();
        updateColumnComponents();
        */
    }
    
    fn table_columns_resized(&mut self, _0: *mut TableHeaderComponent)  {
        
        todo!();
        /*
            setMinimumContentWidth (header->getTotalWidth());
        repaint();
        updateColumnComponents();
        */
    }
    
    fn table_sort_order_changed(&mut self, _0: *mut TableHeaderComponent)  {
        
        todo!();
        /*
            if (model != nullptr)
            model->sortOrderChanged (header->getSortColumnId(),
                                     header->isSortedForwards());
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TableListBox.cpp]
impl<'a> TableListBox<'a> {

    /**
      | Returns the model currently in use.
      |
      */
    pub fn get_model(&self) -> *mut dyn TableListBoxModel {
        
        todo!();
        /*
            return model;
        */
    }
    
    /**
      | Returns the header component being
      | used in this table.
      |
      */
    pub fn get_header(&self) -> &mut TableHeaderComponent {
        
        todo!();
        /*
            return *header;
        */
    }

    /**
      | True if the auto-size options should
      | be shown on the menu. @see setAutoSizeMenuOptionShown
      |
      */
    pub fn is_auto_size_menu_option_shown(&self) -> bool {
        
        todo!();
        /*
            return autoSizeOptionsShown;
        */
    }

    /**
      | Creates a TableListBox.
      | 
      | The model pointer passed-in can be null,
      | in which case you can set it later with
      | setModel(). The TableListBox does
      | not take ownership of the model - it's
      | the caller's responsibility to manage
      | its lifetime and make sure it doesn't
      | get deleted while still being used.
      |
      */
    pub fn new(
        name: Option<&String>,
        m:    *mut dyn TableListBoxModel) -> Self {

        let name = name.unwrap_or(&String::new());

        todo!();
        /*
        : list_box(name, nullptr),
        : model(m),

            ListBox::model = this;

        setHeader (std::make_unique<TableListBoxHeader> (*this));
        */
    }
    
    /**
      | Changes the TableListBoxModel that
      | is being used for this table.
      | 
      | The TableListBox does not take ownership
      | of the model - it's the caller's responsibility
      | to manage its lifetime and make sure
      | it doesn't get deleted while still being
      | used.
      |
      */
    pub fn set_model(&mut self, new_model: *mut dyn TableListBoxModel)  {
        
        todo!();
        /*
            if (model != newModel)
        {
            model = newModel;
            updateContent();
        }
        */
    }
    
    /**
      | Sets the header component to use for
      | the table.
      | 
      | The table will take ownership of the
      | component that you pass in, and will
      | delete it when it's no longer needed.
      | 
      | The pointer passed in may not be null.
      |
      */
    pub fn set_header(&mut self, new_header: Box<TableHeaderComponent>)  {
        
        todo!();
        /*
            if (newHeader == nullptr)
        {
            jassertfalse; // you need to supply a real header for a table!
            return;
        }

        Rectangle<int> newBounds (100, 28);

        if (header != nullptr)
            newBounds = header->getBounds();

        header = newHeader.get();
        header->setBounds (newBounds);

        setHeaderComponent (std::move (newHeader));

        header->addListener (this);
        */
    }
    
    /**
      | Returns the height of the table header.
      | @see setHeaderHeight
      |
      */
    pub fn get_header_height(&self) -> i32 {
        
        todo!();
        /*
            return header->getHeight();
        */
    }
    
    /**
      | Changes the height of the table header
      | component. @see getHeaderHeight
      |
      */
    pub fn set_header_height(&mut self, new_height: i32)  {
        
        todo!();
        /*
            header->setSize (header->getWidth(), newHeight);
        resized();
        */
    }
    
    /**
      | Resizes a column to fit its contents.
      | 
      | This uses TableListBoxModel::getColumnAutoSizeWidth()
      | to find the best width, and applies that
      | to the column.
      | 
      | @see autoSizeAllColumns, TableHeaderComponent::setColumnWidth
      |
      */
    pub fn auto_size_column(&mut self, column_id: i32)  {
        
        todo!();
        /*
            auto width = model != nullptr ? model->getColumnAutoSizeWidth (columnId) : 0;

        if (width > 0)
            header->setColumnWidth (columnId, width);
        */
    }
    
    /**
      | Calls autoSizeColumn() for all columns
      | in the table.
      |
      */
    pub fn auto_size_all_columns(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < header->getNumColumns (true); ++i)
            autoSizeColumn (header->getColumnIdOfIndex (i, true));
        */
    }
    
    /**
      | Enables or disables the auto size options
      | on the popup menu.
      | 
      | By default, these are enabled.
      |
      */
    pub fn set_auto_size_menu_option_shown(&mut self, should_be_shown: bool)  {
        
        todo!();
        /*
            autoSizeOptionsShown = shouldBeShown;
        */
    }
    
    /**
      | Returns the position of one of the cells
      | in the table.
      | 
      | If relativeToComponentTopLeft is
      | true, the coordinates are relative
      | to the table component's top-left.
      | The row number isn't checked to see if
      | it's in-range, but the column ID must
      | exist or this will return an empty rectangle.
      | 
      | If relativeToComponentTopLeft is
      | false, the coordinates are relative
      | to the top-left of the table's top-left
      | cell.
      |
      */
    pub fn get_cell_position(&self, 
        column_id:                      i32,
        row_number:                     i32,
        relative_to_component_top_left: bool) -> Rectangle<i32> {
        
        todo!();
        /*
            auto headerCell = header->getColumnPosition (header->getIndexOfColumnId (columnId, true));

        if (relativeToComponentTopLeft)
            headerCell.translate (header->getX(), 0);

        return getRowPosition (rowNumber, relativeToComponentTopLeft)
                .withX (headerCell.getX())
                .withWidth (headerCell.getWidth());
        */
    }
    
    /**
      | Returns the component that currently
      | represents a given cell.
      | 
      | If the component for this cell is off-screen
      | or if the position is out-of-range,
      | this may return nullptr. @see getCellPosition
      |
      */
    pub fn get_cell_component(&self, 
        column_id:  i32,
        row_number: i32) -> *mut Component {
        
        todo!();
        /*
            if (auto* rowComp = dynamic_cast<TableListBoxRowComp*> (getComponentForRowNumber (rowNumber)))
            return rowComp->findChildComponentForColumn (columnId);

        return nullptr;
        */
    }
    
    /**
      | Scrolls horizontally if necessary
      | to make sure that a particular column
      | is visible.
      | 
      | @see ListBox::scrollToEnsureRowIsOnscreen
      |
      */
    pub fn scroll_to_ensure_column_is_onscreen(&mut self, column_id: i32)  {
        
        todo!();
        /*
            auto& scrollbar = getHorizontalScrollBar();
        auto pos = header->getColumnPosition (header->getIndexOfColumnId (columnId, true));

        auto x = scrollbar.getCurrentRangeStart();
        auto w = scrollbar.getCurrentRangeSize();

        if (pos.getX() < x)
            x = pos.getX();
        else if (pos.getRight() > x + w)
            x += jmax (0.0, pos.getRight() - (x + w));

        scrollbar.setCurrentRangeStart (x);
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return model != nullptr ? model->getNumRows() : 0;
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        _0: i32,
        _1: &mut Graphics,
        _2: i32,
        _3: i32,
        _4: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn refresh_component_for_row(&mut self, 
        row_number:                   i32,
        row_selected:                 bool,
        existing_component_to_update: *mut Component) -> *mut Component {
        
        todo!();
        /*
            if (existingComponentToUpdate == nullptr)
            existingComponentToUpdate = new TableListBoxRowComp (*this);

        static_cast<TableListBoxRowComp*> (existingComponentToUpdate)->update (rowNumber, rowSelected);

        return existingComponentToUpdate;
        */
    }
    
    pub fn selected_rows_changed(&mut self, row: i32)  {
        
        todo!();
        /*
            if (model != nullptr)
            model->selectedRowsChanged (row);
        */
    }
    
    pub fn delete_key_pressed(&mut self, row: i32)  {
        
        todo!();
        /*
            if (model != nullptr)
            model->deleteKeyPressed (row);
        */
    }
    
    pub fn return_key_pressed(&mut self, row: i32)  {
        
        todo!();
        /*
            if (model != nullptr)
            model->returnKeyPressed (row);
        */
    }
    
    pub fn background_clicked(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (model != nullptr)
            model->backgroundClicked (e);
        */
    }
    
    pub fn list_was_scrolled(&mut self)  {
        
        todo!();
        /*
            if (model != nullptr)
            model->listWasScrolled();
        */
    }
    
    pub fn table_column_dragging_changed(&mut self, 
        _0:                          *mut TableHeaderComponent,
        column_id_now_being_dragged: i32)  {
        
        todo!();
        /*
            columnIdNowBeingDragged = columnIdNowBeingDragged_;
        repaint();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            ListBox::resized();

        header->resizeAllColumnsToFit (getVisibleContentWidth());
        setMinimumContentWidth (header->getTotalWidth());
        */
    }
    
    pub fn update_column_components(&self)  {
        
        todo!();
        /*
            auto firstRow = getRowContainingPosition (0, 0);

        for (int i = firstRow + getNumRowsOnScreen() + 2; --i >= firstRow;)
            if (auto* rowComp = dynamic_cast<TableListBoxRowComp*> (getComponentForRowNumber (i)))
                rowComp->resized();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class TableInterface  : public AccessibilityTableInterface
        {
        
            explicit TableInterface (TableListBox& tableListBoxToWrap)
                : tableListBox (tableListBoxToWrap)
            {
            }

            int getNumRows() const override
            {
                if (auto* tableModel = tableListBox.getModel())
                    return tableModel->getNumRows();

                return 0;
            }

            int getNumColumns() const override
            {
                return tableListBox.getHeader().getNumColumns (false);
            }

            const AccessibilityHandler* getCellHandler (int row, int column) const override
            {
                if (isPositiveAndBelow (row, getNumRows()))
                {
                    if (isPositiveAndBelow (column, getNumColumns()))
                        if (auto* cellComponent = tableListBox.getCellComponent (tableListBox.getHeader().getColumnIdOfIndex (column, false), row))
                            return cellComponent->getAccessibilityHandler();

                    if (auto* rowComp = tableListBox.getComponentForRowNumber (row))
                        return rowComp->getAccessibilityHandler();
                }

                return nullptr;
            }

        
            TableListBox& tableListBox;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (TableInterface)
        };

        return std::make_unique<AccessibilityHandler> (*this,
                                                       AccessibilityRole::list,
                                                       AccessibilityActions{},
                                                       AccessibilityHandler::Interfaces { std::make_unique<TableInterface> (*this) });
        */
    }
}

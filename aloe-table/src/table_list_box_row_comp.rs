crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TableListBoxRowComp<'a> {
    base:                   Component<'a>,
    owner:                  &'a mut TableListBox<'a>,
    column_components:      Vec<Box<Component<'a>>>,
    row:                    i32, // default = -1
    is_selected:            bool, // default = false
    is_dragging:            bool, // default = false
    select_row_on_mouse_up: bool, // default = false
}

impl<'a> TooltipClient for TableListBoxRowComp<'a> {

    fn get_tooltip(&mut self) -> String {
        
        todo!();
        /*
            auto columnId = owner.getHeader().getColumnIdAtX (getMouseXYRelative().getX());

            if (columnId != 0)
                if (auto* m = owner.getModel())
                    return m->getCellTooltip (row, columnId);

            return {};
        */
    }
}

impl<'a> TableListBoxRowComp<'a> {

    pub fn new(tlb: &mut TableListBox) -> Self {
    
        todo!();
        /*
        : owner(tlb),

            setFocusContainerType (FocusContainerType::focusContainer);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (auto* tableModel = owner.getModel())
            {
                tableModel->paintRowBackground (g, row, getWidth(), getHeight(), isSelected);

                auto& headerComp = owner.getHeader();
                auto numColumns = headerComp.getNumColumns (true);
                auto clipBounds = g.getClipBounds();

                for (int i = 0; i < numColumns; ++i)
                {
                    if (columnComponents[i] == nullptr)
                    {
                        auto columnRect = headerComp.getColumnPosition (i).withHeight (getHeight());

                        if (columnRect.getX() >= clipBounds.getRight())
                            break;

                        if (columnRect.getRight() > clipBounds.getX())
                        {
                            Graphics::ScopedSaveState ss (g);

                            if (g.reduceClipRegion (columnRect))
                            {
                                g.setOrigin (columnRect.getX(), 0);
                                tableModel->paintCell (g, row, headerComp.getColumnIdOfIndex (i, true),
                                                       columnRect.getWidth(), columnRect.getHeight(), isSelected);
                            }
                        }
                    }
                }
            }
        */
    }
    
    pub fn update(&mut self, 
        new_row:         i32,
        is_now_selected: bool)  {
        
        todo!();
        /*
            jassert (newRow >= 0);

            if (newRow != row || isNowSelected != isSelected)
            {
                row = newRow;
                isSelected = isNowSelected;
                repaint();
            }

            auto* tableModel = owner.getModel();

            if (tableModel != nullptr && row < owner.getNumRows())
            {
                const Identifier columnProperty ("_tableColumnId");
                auto numColumns = owner.getHeader().getNumColumns (true);

                for (int i = 0; i < numColumns; ++i)
                {
                    auto columnId = owner.getHeader().getColumnIdOfIndex (i, true);
                    auto* comp = columnComponents[i];

                    if (comp != nullptr && columnId != static_cast<int> (comp->getProperties() [columnProperty]))
                    {
                        columnComponents.set (i, nullptr);
                        comp = nullptr;
                    }

                    comp = tableModel->refreshComponentForCell (row, columnId, isSelected, comp);
                    columnComponents.set (i, comp, false);

                    if (comp != nullptr)
                    {
                        comp->getProperties().set (columnProperty, columnId);

                        addAndMakeVisible (comp);
                        resizeCustomComp (i);
                    }
                }

                columnComponents.removeRange (numColumns, columnComponents.size());
            }
            else
            {
                columnComponents.clear();
            }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            for (int i = columnComponents.size(); --i >= 0;)
                resizeCustomComp (i);
        */
    }
    
    pub fn resize_custom_comp(&mut self, index: i32)  {
        
        todo!();
        /*
            if (auto* c = columnComponents.getUnchecked (index))
                c->setBounds (owner.getHeader().getColumnPosition (index)
                                .withY (0).withHeight (getHeight()));
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            isDragging = false;
            selectRowOnMouseUp = false;

            if (isEnabled())
            {
                if (! isSelected)
                {
                    owner.selectRowsBasedOnModifierKeys (row, e.mods, false);

                    auto columnId = owner.getHeader().getColumnIdAtX (e.x);

                    if (columnId != 0)
                        if (auto* m = owner.getModel())
                            m->cellClicked (row, columnId, e);
                }
                else
                {
                    selectRowOnMouseUp = true;
                }
            }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (isEnabled()
                 && owner.getModel() != nullptr
                 && e.mouseWasDraggedSinceMouseDown()
                 && ! isDragging)
            {
                SparseSet<int> rowsToDrag;

                if (owner.selectOnMouseDown || owner.isRowSelected (row))
                    rowsToDrag = owner.getSelectedRows();
                else
                    rowsToDrag.addRange (Range<int>::withStartAndLength (row, 1));

                if (rowsToDrag.size() > 0)
                {
                    auto dragDescription = owner.getModel()->getDragSourceDescription (rowsToDrag);

                    if (! (dragDescription.isVoid() || (dragDescription.isString() && dragDescription.toString().isEmpty())))
                    {
                        isDragging = true;
                        owner.startDragAndDrop (e, rowsToDrag, dragDescription, true);
                    }
                }
            }
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (selectRowOnMouseUp && e.mouseWasClicked() && isEnabled())
            {
                owner.selectRowsBasedOnModifierKeys (row, e.mods, true);

                auto columnId = owner.getHeader().getColumnIdAtX (e.x);

                if (columnId != 0)
                    if (TableListBoxModel* m = owner.getModel())
                        m->cellClicked (row, columnId, e);
            }
        */
    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto columnId = owner.getHeader().getColumnIdAtX (e.x);

            if (columnId != 0)
                if (auto* m = owner.getModel())
                    m->cellDoubleClicked (row, columnId, e);
        */
    }
    
    pub fn find_child_component_for_column(&self, column_id: i32) -> *mut Component<'a> {
        
        todo!();
        /*
            return columnComponents [owner.getHeader().getIndexOfColumnId (columnId, true)];
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<RowAccessibilityHandler> (*this);
        */
    }
}

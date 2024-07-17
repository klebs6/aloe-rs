crate::ix!();

pub trait VisibleRangeDataModelListener {

    fn total_range_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
        
        */
    }

    fn visible_range_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
        
        */
    }
}

pub struct VisibleRangeDataModel<'a> {
    value_tree:    ValueTree,
    total_range:   CachedValue<'a, Range<f64>>,
    visible_range: CachedValue<'a, Range<f64>>,
    listener_list: ListenerList<Box<dyn VisibleRangeDataModelListener>>,
}

impl<'a> ValueTreeListener for VisibleRangeDataModel<'a> {

}

impl<'a> Default for VisibleRangeDataModel<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : VisibleRangeDataModel (ValueTree (IDs::VISIBLE_RANGE)
        */
    }
}

impl<'a> From<&ValueTree> for VisibleRangeDataModel<'a> {

    fn from(vt: &ValueTree) -> Self {
    
        todo!();
        /*


            : valueTree (vt),
              totalRange   (valueTree, IDs::totalRange,   nullptr),
              visibleRange (valueTree, IDs::visibleRange, nullptr)

            jassert (valueTree.hasType (IDs::VISIBLE_RANGE));
            valueTree.addListener (this);
        */
    }
}
    
impl<'a> From<&VisibleRangeDataModel<'a>> for VisibleRangeDataModel<'a> {

    fn from(other: &VisibleRangeDataModel) -> Self {
    
        todo!();
        /*
        : visible_range_data_model(other.valueTree),

        
        */
    }
}

impl<'a> VisibleRangeDataModel<'a> {
    
    pub fn assign_from(&mut self, other: &VisibleRangeDataModel) -> &mut VisibleRangeDataModel {
        
        todo!();
        /*
            auto copy (other);
            swap (copy);
            return *this;
        */
    }
    
    pub fn get_total_range(&self) -> Range<f64> {
        
        todo!();
        /*
            return totalRange;
        */
    }
    
    pub fn set_total_range(&mut self, 
        value:        Range<f64>,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            totalRange.setValue (value, undoManager);
            setVisibleRange (visibleRange, undoManager);
        */
    }
    
    pub fn get_visible_range(&self) -> Range<f64> {
        
        todo!();
        /*
            return visibleRange;
        */
    }
    
    pub fn set_visible_range(&mut self, 
        value:        Range<f64>,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            visibleRange.setValue (totalRange.get().constrainRange (value), undoManager);
        */
    }
    
    pub fn add_listener(&mut self, listener: &mut dyn VisibleRangeDataModelListener)  {
        
        todo!();
        /*
            listenerList.add (&listener);
        */
    }
    
    pub fn remove_listener(&mut self, listener: &mut dyn VisibleRangeDataModelListener)  {
        
        todo!();
        /*
            listenerList.remove (&listener);
        */
    }
    
    pub fn swap(&mut self, other: &mut VisibleRangeDataModel)  {
        
        todo!();
        /*
            using std::swap;
            swap (other.valueTree, valueTree);
        */
    }
    
    pub fn value_tree_property_changed(&mut self, 
        _0:       &mut ValueTree,
        property: &Identifier)  {
        
        todo!();
        /*
            if (property == IDs::totalRange)
            {
                totalRange.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.totalRangeChanged (totalRange); });
            }
            else if (property == IDs::visibleRange)
            {
                visibleRange.forceUpdateOfCachedValue();
                listenerList.call ([this] (Listener& l) { l.visibleRangeChanged (visibleRange); });
            }
        */
    }
    
    pub fn value_tree_child_added(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_child_removed(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree,
        _2: i32)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_child_order_changed(&mut self, 
        _0: &mut ValueTree,
        _1: i32,
        _2: i32)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn value_tree_parent_changed(&mut self, _0: &mut ValueTree)  {
        
        todo!();
        /*
            jassertfalse;
        */
    }
}

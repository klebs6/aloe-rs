crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_SelectedItemSet.h]

pub trait HasSelectableItemType {
    type SelectableItemType;
}

pub trait ItemSelected: HasSelectableItemType {

    /**
      | Can be overridden to do special handling
      | when an item is selected.
      | 
      | For example, if the item is an object,
      | you might want to call it and tell it that
      | it's being selected.
      |
      */
    fn item_selected(
        &mut self, 
        _0: <Self as HasSelectableItemType>::SelectableItemType
    )  { }
}

pub trait ItemDeselected: HasSelectableItemType {

    /**
      | Can be overridden to do special handling
      | when an item is deselected.
      | 
      | For example, if the item is an object,
      | you might want to call it and tell it that
      | it's being deselected.
      |
      */
    fn item_deselected(
        &mut self, 
        _0: <Self as HasSelectableItemType>::SelectableItemType
    ) { }
}

pub trait HasItemType { type ItemType; }

/**
  | Manages a list of selectable items.
  | 
  | Use one of these to keep a track of things
  | that the user has highlighted, like
  | icons or things in a list.
  | 
  | The class is templated so that you can
  | use it to hold either a set of pointers
  | to objects, or a set of ID numbers or handles,
  | for cases where each item may not always
  | have a corresponding object.
  | 
  | To be informed when items are selected/deselected,
  | register a ChangeListener with this
  | object.
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct SelectedItemSet<'a, SelectableItemType> {
    base:           ChangeBroadcaster<'a>,
    selected_items: Vec<SelectableItemType>,
    phantom:        PhantomData<SelectableItemType>,
}

impl<'a, T> HasSelectableItemType for SelectedItemSet<'a, T> {
    type SelectableItemType = T;
}

impl<'a, T> HasItemType for SelectedItemSet<'a, T> {
    type ItemType = T;
}

impl<'a, T> ParameterType for SelectedItemSet<'a, T> {
    type Type = T;
}

impl<'a, T> Default for SelectedItemSet<'a, T> {
    
    /**
      | Creates an empty set.
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

impl<'a,SelectableItemType> SelectedItemSet<'a, SelectableItemType> {

    /**
      | Creates a set based on an array of items.
      |
      */
    pub fn new_from_items(items: &Vec<SelectableItemType>) -> Self {
    
        todo!();

        /*
        : selected_items(items),

        
        */
    }

    /**
      | Creates a copy of another set.
      |
      */
    pub fn new(other: &SelectedItemSet<SelectableItemType>) -> Self {
    
        todo!();
        /*
        : change_broadcaster(),
        : selected_items(other.selectedItems),

        
        */
    }

    /**
      | Creates a copy of another set.
      |
      */
    pub fn assign_from(&mut self, other: &SelectedItemSet<SelectableItemType>) -> &mut SelectedItemSet<SelectableItemType> {
        
        todo!();
        /*
            if (selectedItems != other.selectedItems)
            {
                changed();

                for (int i = selectedItems.size(); --i >= 0;)
                    if (! other.isSelected (selectedItems.getReference (i)))
                        itemDeselected (selectedItems.removeAndReturn (i));

                for (auto& i : other.selectedItems)
                {
                    if (! isSelected (i))
                    {
                        selectedItems.add (i);
                        itemSelected (i);
                    }
                }
            }

            return *this;
        */
    }

    /**
      | Clears any other currently selected
      | items, and selects this item.
      | 
      | If this item is already the only thing
      | selected, no change notification will
      | be sent out.
      | 
      | @see addToSelection, addToSelectionBasedOnModifiers
      |
      */
    pub fn select_only(&mut self, item: <Self as ParameterType>::Type)  {
        
        todo!();
        /*
            if (isSelected (item))
            {
                for (int i = selectedItems.size(); --i >= 0;)
                {
                    if (selectedItems.getUnchecked(i) != item)
                    {
                        deselect (selectedItems.getUnchecked(i));
                        i = jmin (i, selectedItems.size());
                    }
                }
            }
            else
            {
                changed();
                deselectAll();

                selectedItems.add (item);
                itemSelected (item);
            }
        */
    }

    /**
      | Selects an item.
      | 
      | If the item is already selected, no change
      | notification will be sent out. @see
      | selectOnly, addToSelectionBasedOnModifiers
      |
      */
    pub fn add_to_selection(&mut self, item: <Self as ParameterType>::Type)  {
        
        todo!();
        /*
            if (! isSelected (item))
            {
                changed();

                selectedItems.add (item);
                itemSelected (item);
            }
        */
    }

    /**
      | Selects or deselects an item.
      | 
      | This will use the modifier keys to decide
      | whether to deselect other items first.
      | 
      | So if the shift key is held down, the item
      | will be added without deselecting anything
      | (same as calling addToSelection()
      | )
      | 
      | If no modifiers are down, the current
      | selection will be cleared first (same
      | as calling selectOnly() )
      | 
      | If the ctrl (or command on the Mac) key
      | is held down, the item will be toggled
      | - so it'll be added to the set unless it's
      | already there, in which case it'll be
      | deselected.
      | 
      | If the items that you're selecting can
      | also be dragged, you may need to use the
      | addToSelectionOnMouseDown() and
      | addToSelectionOnMouseUp() calls
      | to handle the subtleties of this kind
      | of usage.
      | 
      | @see selectOnly, addToSelection,
      | addToSelectionOnMouseDown, addToSelectionOnMouseUp
      |
      */
    pub fn add_to_selection_based_on_modifiers(&mut self, 
        item:      <Self as ParameterType>::Type,
        modifiers: ModifierKeys)  {
        
        todo!();
        /*
            if (modifiers.isShiftDown())
            {
                addToSelection (item);
            }
            else if (modifiers.isCommandDown())
            {
                if (isSelected (item))
                    deselect (item);
                else
                    addToSelection (item);
            }
            else
            {
                selectOnly (item);
            }
        */
    }

    /**
      | Selects or deselects items that can
      | also be dragged, based on a mouse-down
      | event.
      | 
      | If you call addToSelectionOnMouseDown()
      | at the start of your mouseDown event,
      | and then call addToSelectionOnMouseUp()
      | at the end of your mouseUp event, this
      | makes it easy to handle multiple-selection
      | of sets of objects that can also be dragged.
      | 
      | For example, if you have several items
      | already selected, and you click on one
      | of them (without dragging), then you'd
      | expect this to deselect the other, and
      | just select the item you clicked on.
      | But if you had clicked on this item and
      | dragged it, you'd have expected them
      | all to stay selected.
      | 
      | When you call this method, you'll need
      | to store the boolean result, because
      | the addToSelectionOnMouseUp() method
      | will need to be know this value.
      | 
      | @see addToSelectionOnMouseUp, addToSelectionBasedOnModifiers
      |
      */
    pub fn add_to_selection_on_mouse_down(&mut self, 
        item:      <Self as ParameterType>::Type,
        modifiers: ModifierKeys) -> bool {
        
        todo!();
        /*
            if (isSelected (item))
                return ! modifiers.isPopupMenu();

            addToSelectionBasedOnModifiers (item, modifiers);
            return false;
        */
    }

    /**
      | Selects or deselects items that can
      | also be dragged, based on a mouse-up
      | event.
      | 
      | Call this during a mouseUp callback,
      | when you have previously called the
      | addToSelectionOnMouseDown() method
      | during your mouseDown event.
      | 
      | See addToSelectionOnMouseDown()
      | for more info
      | 
      | -----------
      | @param item
      | 
      | the item to select (or deselect)
      | ----------
      | @param modifiers
      | 
      | the modifiers from the mouse-up event
      | ----------
      | @param wasItemDragged
      | 
      | true if your item was dragged during
      | the mouse click
      | ----------
      | @param resultOfMouseDownSelectMethod
      | 
      | this is the boolean return value that
      | came back from the addToSelectionOnMouseDown()
      | call that you should have made during
      | the matching mouseDown event
      |
      */
    pub fn add_to_selection_on_mouse_up(&mut self, 
        item:                               <Self as ParameterType>::Type,
        modifiers:                          ModifierKeys,
        was_item_dragged:                   bool,
        result_of_mouse_down_select_method: bool)  {
        
        todo!();
        /*
            if (resultOfMouseDownSelectMethod && ! wasItemDragged)
                addToSelectionBasedOnModifiers (item, modifiers);
        */
    }

    /**
      | Deselects an item.
      |
      */
    pub fn deselect(&mut self, item: <Self as ParameterType>::Type)  {
        
        todo!();
        /*
            const int i = selectedItems.indexOf (item);

            if (i >= 0)
            {
                changed();
                itemDeselected (selectedItems.removeAndReturn (i));
            }
        */
    }

    /**
      | Deselects all items.
      |
      */
    pub fn deselect_all(&mut self)  {
        
        todo!();
        /*
            if (selectedItems.size() > 0)
            {
                changed();

                for (int i = selectedItems.size(); --i >= 0;)
                {
                    itemDeselected (selectedItems.removeAndReturn (i));
                    i = jmin (i, selectedItems.size());
                }
            }
        */
    }

    /**
      | Returns the number of currently selected
      | items. @see getSelectedItem
      |
      */
    pub fn get_num_selected(&self) -> i32 {
        
        todo!();
        /*
            return selectedItems.size();
        */
    }

    /**
      | Returns one of the currently selected
      | items.
      | 
      | If the index is out-of-range, this returns
      | a default-constructed SelectableItemType.
      | @see getNumSelected
      |
      */
    pub fn get_selected_item(&self, index: i32) -> SelectableItemType {
        
        todo!();
        /*
            return selectedItems [index];
        */
    }

    /**
      | True if this item is currently selected.
      |
      */
    pub fn is_selected(&self, item: <Self as ParameterType>::Type) -> bool {
        
        todo!();
        /*
            return selectedItems.contains (item);
        */
    }

    /**
      | Provides access to the array of items.
      |
      */
    pub fn get_item_array(&self) -> &Vec<SelectableItemType> {
        
        todo!();
        /*
            return selectedItems;
        */
    }

    /**
      | Provides iterator access to the array
      | of items.
      |
      */
    pub fn begin_mut(&mut self) -> *mut SelectableItemType {
        
        todo!();
        /*
            return selectedItems.begin();
        */
    }
    
    pub fn begin(&self) -> *const SelectableItemType {
        
        todo!();
        /*
            return selectedItems.begin();
        */
    }

    /**
      | Provides iterator access to the array
      | of items.
      |
      */
    pub fn end_mut(&mut self) -> *mut SelectableItemType {
        
        todo!();
        /*
            return selectedItems.end();
        */
    }

    /**
      | Provides iterator access to the array
      | of items.
      |
      */
    pub fn end(&self) -> *const SelectableItemType {
        
        todo!();
        /*
            return selectedItems.end();
        */
    }

    /**
      | Used internally, but can be called to
      | force a change message to be sent to the
      | ChangeListeners.
      |
      */
    pub fn changed(&mut self)  {
        
        todo!();
        /*
            sendChangeMessage();
        */
    }

    /**
      | Used internally, but can be called to
      | force a change message to be sent to the
      | ChangeListeners.
      |
      */
    pub fn changed_with_synchronous(&mut self, synchronous: bool)  {
        
        todo!();
        /*
            if (synchronous)
                sendSynchronousChangeMessage();
            else
                sendChangeMessage();
        */
    }
}

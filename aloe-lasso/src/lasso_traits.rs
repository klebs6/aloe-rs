crate::ix!();

/**
  | A class used by the LassoComponent to
  | manage the things that it selects.
  | 
  | This allows the LassoComponent to find
  | out which items are within the lasso,
  | and to change the list of selected items.
  | 
  | @see LassoComponent, SelectedItemSet
  | 
  | @tags{GUI}
  |
  */
pub trait LassoSource<SelectableItemType>: 
HasSelectableItemType<SelectableItemType = SelectableItemType>
+ FindLassoItemsInArea
+ GetLassoSelection
{ }

pub trait FindLassoItemsInArea: HasSelectableItemType {

    /**
      | Returns the set of items that lie within
      | a given lassoable region.
      | 
      | Your implementation of this method
      | must find all the relevant items that
      | lie within the given rectangle. and
      | add them to the itemsFound array.
      | 
      | The coordinates are relative to the
      | top-left of the lasso component's parent
      | component. (i.e. they are the same as
      | the size and position of the lasso component
      | itself).
      |
      */
    fn find_lasso_items_in_area(
        &mut self, 
        items_found: &mut Vec<<Self as HasSelectableItemType>::SelectableItemType>,
        area:        &Rectangle<i32>
    );
}

pub trait GetLassoSelection: HasSelectableItemType {

    /**
      | Returns the SelectedItemSet that the
      | lasso should update.
      | 
      | This set will be continuously updated
      | by the LassoComponent as it gets dragged
      | around, so make sure that you've got
      | a ChangeListener attached to the set
      | so that your UI objects will know when
      | the selection changes and be able to
      | update themselves appropriately.
      |
      */
    fn get_lasso_selection(&mut self) 
        -> &mut SelectedItemSet<<Self as HasSelectableItemType>::SelectableItemType>;
}

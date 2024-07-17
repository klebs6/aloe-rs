crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_LassoComponent.h]

/**
  | A component that acts as a rectangular
  | selection region, which you drag with
  | the mouse to select groups of objects
  | (in conjunction with a SelectedItemSet).
  | 
  | To use one of these:
  | 
  | - In your mouseDown or mouseDrag event,
  | add the LassoComponent to your parent
  | component, and call its beginLasso()
  | method, giving it a suitable LassoSource
  | object that it can use to find out which
  | items are in the active area.
  | 
  | - Each time your parent component gets
  | a mouseDrag event, call dragLasso()
  | to update the lasso's position - it will
  | use its LassoSource to calculate and
  | update the current selection.
  | 
  | - After the drag has finished and you
  | get a mouseUp callback, you should call
  | endLasso() to clean up. This will make
  | the lasso component invisible, and
  | you can remove it from the parent component,
  | or delete it.
  | 
  | The class takes into account the modifier
  | keys that are being held down while the
  | lasso is being dragged, so if shift is
  | pressed, then any lassoed items will
  | be added to the original selection;
  | if ctrl or command is pressed, they will
  | be xor'ed with any previously selected
  | items.
  | 
  | @see LassoSource, SelectedItemSet
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LassoComponent<'a, SelectableItemType> {
    base:               Component<'a>,
    original_selection: Vec<SelectableItemType>,
    source:             *mut dyn LassoSource<SelectableItemType>, // default = nullptr
    drag_start_pos:     Point<i32>,
}

impl<'a, SelectableItemType> Default for LassoComponent<'a, SelectableItemType> {

    fn default() -> Self {
        todo!();
    }
}

impl<'a, SelectableItemType> LassoComponent<'a, SelectableItemType> {

    /**
      | Call this in your mouseDown event, to
      | initialise a drag.
      | 
      | Pass in a suitable LassoSource object
      | which the lasso will use to find the items
      | and change the selection.
      | 
      | After using this method to initialise
      | the lasso, repeatedly call dragLasso()
      | in your component's mouseDrag callback.
      | 
      | @see dragLasso, endLasso, LassoSource
      |
      */
    pub fn begin_lasso(&mut self, 
        e:            &MouseEvent,
        lasso_source: *mut dyn LassoSource<SelectableItemType>)  {
        
        todo!();
        /*
            jassert (source == nullptr);  // this suggests that you didn't call endLasso() after the last drag...
            jassert (lassoSource != nullptr); // the source can't be null!
            jassert (getParentComponent() != nullptr);  // you need to add this to a parent component for it to work!

            source = lassoSource;

            if (lassoSource != nullptr)
                originalSelection = lassoSource->getLassoSelection().getItemArray();

            setSize (0, 0);
            dragStartPos = e.getMouseDownPosition();
        */
    }

    /**
      | Call this in your mouseDrag event, to
      | update the lasso's position.
      | 
      | This must be repeatedly calling when
      | the mouse is dragged, after you've first
      | initialised the lasso with beginLasso().
      | 
      | This method takes into account the modifier
      | keys that are being held down, so if shift
      | is pressed, then the lassoed items will
      | be added to any that were previously
      | selected; if ctrl or command is pressed,
      | then they will be xor'ed with previously
      | selected items.
      | 
      | @see beginLasso, endLasso
      |
      */
    pub fn drag_lasso(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (source != nullptr)
            {
                setBounds (Rectangle<int> (dragStartPos, e.getPosition()));
                setVisible (true);

                Vec<SelectableItemType> itemsInLasso;
                source->findLassoItemsInArea (itemsInLasso, getBounds());

                if (e.mods.isShiftDown())
                {
                    itemsInLasso.removeValuesIn (originalSelection); //  to avoid duplicates
                    itemsInLasso.addArray (originalSelection);
                }
                else if (e.mods.isCommandDown() || e.mods.isAltDown())
                {
                    auto originalMinusNew = originalSelection;
                    originalMinusNew.removeValuesIn (itemsInLasso);

                    itemsInLasso.removeValuesIn (originalSelection);
                    itemsInLasso.addArray (originalMinusNew);
                }

                source->getLassoSelection() = SelectedItemSet<SelectableItemType> (itemsInLasso);
            }
        */
    }

    /**
      | Call this in your mouseUp event, after
      | the lasso has been dragged. @see beginLasso,
      | dragLasso
      |
      */
    pub fn end_lasso(&mut self)  {
        
        todo!();
        /*
            source = nullptr;
            originalSelection.clear();
            setVisible (false);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawLasso (g, *this);

            // this suggests that you've left a lasso comp lying around after the
            // mouse drag has finished.. Be careful to call endLasso() when you get a
            // mouse-up event.
            jassert (isMouseButtonDownAnywhere());
        */
    }
    
    pub fn hit_test(&mut self, _0: i32, _1: i32) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

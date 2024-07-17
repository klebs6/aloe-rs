crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_FlexBox.h]

/**
  | Represents a FlexBox container, which
  | contains and manages the layout of a
  | set of FlexItem objects.
  | 
  | To use this class, set its parameters
  | appropriately (you can search online
  | for more help on exactly how the FlexBox
  | protocol works!), then add your sub-items
  | to the items array, and call performLayout()
  | in the resized() function of your
  | 
  | Component.
  | 
  | @see FlexItem
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct FlexBox<'a> {

    /**
      | Specifies how flex items are placed in the flex
      | container, and defines the direction of the main
      | axis.
      */
    flex_direction:  FlexBoxDirection, // default = FlexBoxDirection::row

    /**
      | Specifies whether items are forced into a single
      | line or can be wrapped onto multiple lines. If
      | wrapping is allowed, this property also controls
      | the direction in which lines are stacked.
      */
    flex_wrap:       FlexBoxWrap, // default = FlexBoxWrap::noWrap

    /**
      | Specifies how a flex container's lines are placed
      | within the flex container when there is extra space
      | on the cross-axis. This property has no effect
      | on single line layouts.
      */
    align_content:   FlexBoxAlignContent, // default = FlexBoxAlignContent::stretch

    /**
      | Specifies the alignment of flex items
      | along the cross-axis of each line.
      |
      */
    align_items:     FlexBoxAlignItems, // default = FlexBoxAlignItems::stretch

    /**
      | Defines how the container distributes space between
      | and around items along the main-axis. The alignment
      | is done after the lengths and auto margins are
      | applied, so that if there is at least one flexible
      | element, with flex-grow different from 0, it will
      | have no effect as there won't be any available
      | space.
      */
    justify_content: FlexBoxJustifyContent, // default = FlexBoxJustifyContent::flexStart

    /**
      | The set of items to lay-out.
      |
      */
    items:           Vec<FlexItem<'a>>,
}

impl<'a> Default for FlexBox<'a> {
    
    /**
      | Creates an empty FlexBox container
      | with default parameters.
      |
      */
    fn default() -> Self {

        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_FlexBox.cpp]
impl<'a> FlexBox<'a> {

    /**
      | Creates an empty FlexBox container
      | with the given content-justification
      | mode.
      |
      */
    pub fn new_from_justify_content_mode(jc: FlexBoxJustifyContent) -> Self {
    
        todo!();
        /*
        : justify_content(jc),

        
        */
    }
    
    /**
      | Creates an empty FlexBox container
      | with these parameters.
      |
      */
    pub fn new(
        d:  FlexBoxDirection,
        w:  FlexBoxWrap,
        ac: FlexBoxAlignContent,
        ai: FlexBoxAlignItems,
        jc: FlexBoxJustifyContent) -> Self {
    
        todo!();
        /*
        : flex_direction(d),
        : flex_wrap(w),
        : align_content(ac),
        : align_items(ai),
        : justify_content(jc),

        
        */
    }
    
    /**
      | Lays-out the box's items within the
      | given rectangle.
      |
      */
    pub fn perform_layout_with_target_area(&mut self, target_area: Rectangle<f32>)  {
        
        todo!();
        /*
            if (! items.isEmpty())
        {
            FlexBoxLayoutCalculation layout (*this, targetArea.getWidth(), targetArea.getHeight());

            layout.createStates();
            layout.initialiseItems();
            layout.resolveFlexibleLengths();
            layout.resolveAutoMarginsOnMainAxis();
            layout.calculateCrossSizesByLine();
            layout.calculateCrossSizeOfAllItems();
            layout.alignLinesPerAlignContent();
            layout.resolveAutoMarginsOnCrossAxis();
            layout.alignItemsInCrossAxisInLinesPerAlignItems();
            layout.alignLinesPerAlignSelf();
            layout.alignItemsByJustifyContent();
            layout.layoutAllItems();

            for (auto& item : items)
            {
                item.currentBounds += targetArea.getPosition();

                if (auto* comp = item.associatedComponent)
                    comp->setBounds (Rectangle<int>::leftTopRightBottom ((int) item.currentBounds.getX(),
                                                                         (int) item.currentBounds.getY(),
                                                                         (int) item.currentBounds.getRight(),
                                                                         (int) item.currentBounds.getBottom()));

                if (auto* box = item.associatedFlexBox)
                    box->performLayout (item.currentBounds);
            }
        }
        */
    }
    
    /**
      | Lays-out the box's items within the
      | given rectangle.
      |
      */
    pub fn perform_layout(&mut self, target_area: Rectangle<i32>)  {
        
        todo!();
        /*
            performLayout (targetArea.toFloat());
        */
    }
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/misc/aloe_BubbleComponent.h]

pub trait BubbleComponentInterface {

    /** 
      | Subclasses should override this to return
      | the size of the content they want to draw
      | inside the bubble.
      */
    fn get_content_size(
        &mut self, 
        width:  &mut i32,
        height: &mut i32);

    /** 
      | Subclasses should override this to draw
      | their bubble's contents.
      |
      | The graphics object's clip region and
      | the dimensions passed in here are set up
      | to paint just the rectangle inside the
      | bubble.
      */
    fn paint_content(
        &mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32);
}

/** 
  | A list of permitted placements for the
  | bubble, relative to the coordinates at which
  | it should be pointing.
  |
  | @see setAllowedPlacement
  */
pub enum BubblePlacement
{
    above   = 1,
    below   = 2,
    left    = 4,
    right   = 8
}

/**
  | A component for showing a message or other
  | graphics inside a speech-bubble-shaped
  | outline, pointing at a location on the screen.
  |
  | This is a base class that just draws and
  | positions the bubble shape, but leaves the
  | drawing of any content up to a subclass. See
  | BubbleMessageComponent for a subclass that
  | draws a text message.
  |
  | To use it, create your subclass, then either
  | add it to a parent component or put it on the
  | desktop with addToDesktop (0), use
  | setPosition() to resize and position it, then
  | make it visible.
  |
  | @see BubbleMessageComponent
  |
  | @tags{GUI}
  */
#[no_copy]
#[leak_detector]
pub struct BubbleComponent<'a> {
    base:                 Component<'a>,
    content:              Rectangle<i32>,
    arrow_tip:            Point<i32>,
    allowable_placements: i32,
    shadow:               DropShadowEffect,
}

impl<'a> Default for BubbleComponent<'a> {
    
    /** 
      | Creates a BubbleComponent.
      |
      | Your subclass will need to implement the
      | getContentSize() and paintContent()
      | methods to draw the bubble's contents.
      */
    fn default() -> Self {

        todo!();
        /*


            : allowablePlacements (above | below | left | right)
        setInterceptsMouseClicks (false, false);

        shadow.setShadowProperties (DropShadow (Colours::black.withAlpha (0.35f), 5, Point<int>()));
        setComponentEffect (&shadow);
        */
    }
    
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/misc/aloe_BubbleComponent.cpp]
impl<'a> BubbleComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawBubble (g, *this, arrowTip.toFloat(), content.toFloat());

        g.reduceClipRegion (content);
        g.setOrigin (content.getPosition());

        paintContent (g, content.getWidth(), content.getHeight());
        */
    }
    
    /** 
      | Tells the bubble which positions it's
      | allowed to put itself in, relative to the
      | point at which it's pointing.
      |
      | By default when setPosition() is called,
      | the bubble will place itself either above,
      | below, left, or right of the target
      | area. You can pass in a bitwise-'or' of
      | the values in BubblePlacement to restrict
      | this choice.
      |
      | E.g. if you only want your bubble to
      | appear above or below the target area, use
      | setAllowedPlacement (above | below);
      |
      | @see BubblePlacement
      */
    pub fn set_allowed_placement(&mut self, new_placement: i32)  {
        
        todo!();
        /*
            allowablePlacements = newPlacement;
        */
    }
    
    /** 
      | Moves and resizes the bubble to point at
      | a given component.
      |
      | This will resize the bubble to fit its
      | content, then find a position for it so
      | that it's next to, but doesn't overlap the
      | given component.
      |
      | It'll put itself either above, below, or
      | to the side of the component depending on
      | where there's the most space, honouring
      | any restrictions that were set with
      | setAllowedPlacement().
      */
    pub fn set_position_to_point_at_given_component(
        &mut self, 
        component_to_point_to: *mut Component<'a>,
        distance_from_target:  Option<i32>,
        arrow_length:          Option<i32>

    ) {

        let distance_from_target: i32 = distance_from_target.unwrap_or(15);
        let arrow_length: i32 = arrow_length.unwrap_or(10);
        
        todo!();
        /*
            jassert (componentToPointTo != nullptr);

        Rectangle<int> target;

        if (Component* p = getParentComponent())
            target = p->getLocalArea (componentToPointTo, componentToPointTo->getLocalBounds());
        else
            target = componentToPointTo->getScreenBounds().transformedBy (getTransform().inverted());

        setPosition (target, distanceFromTarget, arrowLength);
        */
    }
    
    /** 
      | Moves and resizes the bubble to point at
      | a given point.
      |
      | This will resize the bubble to fit its
      | content, then position it so that the tip of
      | the bubble points to the given
      | coordinate. The coordinates are relative to
      | either the bubble component's parent
      | component if it has one, or they are screen
      | coordinates if not.
      |
      | It'll put itself either above, below, or to
      | the side of this point, depending on where
      | there's the most space, honouring any
      | restrictions that were set with
      | setAllowedPlacement().
      */
    pub fn set_position_to_point_at_given_point(
        &mut self, 
        arrow_tip_pos: Point<i32>,
        arrow_length:  Option<i32>) 
    {
        let arrow_length: i32 = arrow_length.unwrap_or(10);

        todo!();
        /*
           setPosition (Rectangle<int> (arrowTipPos.x, arrowTipPos.y, 1, 1), arrowLength, arrowLength);
           */
    }
    
    /** 
      | Moves and resizes the bubble to point at a given
      | rectangle.
      |
      | This will resize the bubble to fit its content,
      | then find a position for it so that it's next
      | to, but doesn't overlap the given rectangle. The
      | rectangle's coordinates are relative to either
      | the bubble component's parent component if it
      | has one, or they are screen coordinates if not.
      |
      | It'll put itself either above, below, or to the
      | side of the component depending on where there's
      | the most space, honouring any restrictions that
      | were set with setAllowedPlacement().
      |
      | distanceFromTarget is the amount of space to
      | leave between the bubble and the target
      | rectangle, and arrowLength is the length of the
      | arrow that it will draw.
      */
    pub fn set_position_to_point_at_given_rectangle(
        &mut self, 
        rectangle_to_point_to: Rectangle<i32>,
        distance_from_target:  Option<i32>,
        arrow_length:          Option<i32>) 
    {
        let distance_from_target = distance_from_target.unwrap_or(15);
        let arrow_length         = arrow_length.unwrap_or(10);
        
        todo!();
        /*
            {
            int contentW = 150, contentH = 30;
            getContentSize (contentW, contentH);
            content.setBounds (distanceFromTarget, distanceFromTarget, contentW, contentH);
        }

        const int totalW = content.getWidth()  + distanceFromTarget * 2;
        const int totalH = content.getHeight() + distanceFromTarget * 2;

        auto availableSpace = (getParentComponent() != nullptr ? getParentComponent()->getLocalBounds()
                                                               : getParentMonitorArea().transformedBy (getTransform().inverted()));

        int spaceAbove = ((allowablePlacements & above) != 0) ? jmax (0, rectangleToPointTo.getY()  - availableSpace.getY()) : -1;
        int spaceBelow = ((allowablePlacements & below) != 0) ? jmax (0, availableSpace.getBottom() - rectangleToPointTo.getBottom()) : -1;
        int spaceLeft  = ((allowablePlacements & left)  != 0) ? jmax (0, rectangleToPointTo.getX()  - availableSpace.getX()) : -1;
        int spaceRight = ((allowablePlacements & right) != 0) ? jmax (0, availableSpace.getRight()  - rectangleToPointTo.getRight()) : -1;

        // look at whether the component is elongated, and if so, try to position next to its longer dimension.
        if (rectangleToPointTo.getWidth() > rectangleToPointTo.getHeight() * 2
             && (spaceAbove > totalH + 20 || spaceBelow > totalH + 20))
        {
            spaceLeft = spaceRight = 0;
        }
        else if (rectangleToPointTo.getWidth() < rectangleToPointTo.getHeight() / 2
                  && (spaceLeft > totalW + 20 || spaceRight > totalW + 20))
        {
            spaceAbove = spaceBelow = 0;
        }

        int targetX, targetY;

        if (jmax (spaceAbove, spaceBelow) >= jmax (spaceLeft, spaceRight))
        {
            targetX = rectangleToPointTo.getCentre().x;
            arrowTip.x = totalW / 2;

            if (spaceAbove >= spaceBelow)
            {
                // above
                targetY = rectangleToPointTo.getY();
                arrowTip.y = content.getBottom() + arrowLength;
            }
            else
            {
                // below
                targetY = rectangleToPointTo.getBottom();
                arrowTip.y = content.getY() - arrowLength;
            }
        }
        else
        {
            targetY = rectangleToPointTo.getCentre().y;
            arrowTip.y = totalH / 2;

            if (spaceLeft > spaceRight)
            {
                // on the left
                targetX = rectangleToPointTo.getX();
                arrowTip.x = content.getRight() + arrowLength;
            }
            else
            {
                // on the right
                targetX = rectangleToPointTo.getRight();
                arrowTip.x = content.getX() - arrowLength;
            }
        }

        setBounds (targetX - arrowTip.x, targetY - arrowTip.y, totalW, totalH);
        */
    }
}

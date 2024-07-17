crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/desktop/aloe_Displays.h]

/**
  | Manages details about connected display
  | devices.
  | 
  | @tags{GUI}
  |
  */
pub struct Displays {

    /**
      | An Vec containing the Display objects
      | for all of the connected displays.
      |
      */
    displays:      Vec<Display>,

    empty_display: Display,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/desktop/aloe_Displays.cpp]
impl Displays {
    
    pub fn find_displays(&mut self, master_scale: f32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(desktop: &mut Desktop) -> Self {
    
        todo!();
        /*
            init (desktop);
        */
    }
    
    pub fn init(&mut self, desktop: &mut Desktop)  {
        
        todo!();
        /*
            findDisplays (desktop.getGlobalScaleFactor());
        */
    }
    
    /**
      | Returns the Display object representing
      | the display containing a given Rectangle
      | (either in logical or physical pixels),
      | or nullptr if there are no connected
      | displays.
      | 
      | If the Rectangle lies outside all the
      | displays then the nearest one will be
      | returned.
      |
      */
    pub fn get_display_for_rect(
        &self, 
        rect:        Rectangle<i32>,
        is_physical: Option<bool>

    ) -> *const crate::Display {

        let is_physical: bool = is_physical.unwrap_or(false);
        
        todo!();
        /*
            int maxArea = -1;
        const Display* foundDisplay = nullptr;

        for (auto& display : displays)
        {
            auto displayArea = display.totalArea;

            if (isPhysical)
                displayArea = (displayArea.withZeroOrigin() * display.scale) + display.topLeftPhysical;

            displayArea = displayArea.getIntersection (rect);
            auto area = displayArea.getWidth() * displayArea.getHeight();

            if (area >= maxArea)
            {
                maxArea = area;
                foundDisplay = &display;
            }
        }

        return foundDisplay;
        */
    }
    
    /**
      | Returns the Display object representing
      | the display containing a given Point
      | (either in logical or physical pixels),
      | or nullptr if there are no connected
      | displays.
      | 
      | If the Point lies outside all the displays
      | then the nearest one will be returned.
      |
      */
    pub fn get_display_for_point(
        &self, 
        point:       Point<i32>,
        is_physical: Option<bool>

    ) -> *const crate::Display {

        let is_physical: bool = is_physical.unwrap_or(false);
        
        todo!();
        /*
            auto minDistance = std::numeric_limits<int>::max();
        const Display* foundDisplay = nullptr;

        for (auto& display : displays)
        {
            auto displayArea = display.totalArea;

            if (isPhysical)
                displayArea = (displayArea.withZeroOrigin() * display.scale) + display.topLeftPhysical;

            if (displayArea.contains (point))
                return &display;

            auto distance = displayArea.getCentre().getDistanceFrom (point);

            if (distance <= minDistance)
            {
                minDistance = distance;
                foundDisplay = &display;
            }
        }

        return foundDisplay;
        */
    }
    
    /**
      | Converts an integer Rectangle from
      | physical to logical pixels.
      | 
      | If useScaleFactorOfDisplay is not
      | null then its scale factor will be used
      | for the conversion regardless of the
      | display that the Rectangle to be converted
      | is on.
      |
      */
    pub fn physical_to_logical_i32(
        &self, 
        rect:                        Rectangle<i32>,
        use_scale_factor_of_display: *const Display

    ) -> Rectangle<i32> {

        todo!();
        /*
            return physicalToLogical (rect.toFloat(), useScaleFactorOfDisplay).toNearestInt();
        */
    }
    
    /**
      | Converts a floating-point Rectangle
      | from physical to logical pixels.
      | 
      | If useScaleFactorOfDisplay is not
      | null then its scale factor will be used
      | for the conversion regardless of the
      | display that the Rectangle to be converted
      | is on.
      |
      */
    pub fn physical_to_logical_f32(
        &self, 
        rect:                        Rectangle<f32>,
        use_scale_factor_of_display: *const Display

    ) -> Rectangle<f32> {
        
        todo!();
        /*
            const auto* display = useScaleFactorOfDisplay != nullptr ? useScaleFactorOfDisplay
                                                                 : getDisplayForRect (rect.toNearestInt(), true);

        if (display == nullptr)
            return rect;

        auto globalScale = Desktop::getInstance().getGlobalScaleFactor();

        return ((rect - display->topLeftPhysical.toFloat()) / (display->scale / globalScale))
                + (display->totalArea.getTopLeft().toFloat() * globalScale);
        */
    }
    
    /**
      | Converts an integer Rectangle from
      | logical to physical pixels.
      | 
      | If useScaleFactorOfDisplay is not
      | null then its scale factor will be used
      | for the conversion regardless of the
      | display that the Rectangle to be converted
      | is on.
      |
      */
    pub fn logical_to_physical_i32(
        &self, 
        rect:                        Rectangle<i32>,
        use_scale_factor_of_display: *const Display

    ) -> Rectangle<i32> {

        todo!();
        /*
            return logicalToPhysical (rect.toFloat(), useScaleFactorOfDisplay).toNearestInt();
        */
    }
    
    /**
      | Converts a floating-point Rectangle
      | from logical to physical pixels.
      | 
      | If useScaleFactorOfDisplay is not
      | null then its scale factor will be used
      | for the conversion regardless of the
      | display that the Rectangle to be converted
      | is on.
      |
      */
    pub fn logical_to_physical_f32(
        &self, 
        rect:                        Rectangle<f32>,
        use_scale_factor_of_display: *const Display

    ) -> Rectangle<f32> {

        todo!();
        /*
            const auto* display = useScaleFactorOfDisplay != nullptr ? useScaleFactorOfDisplay
                                                                 : getDisplayForRect (rect.toNearestInt(), false);

        if (display == nullptr)
            return rect;

        auto globalScale = Desktop::getInstance().getGlobalScaleFactor();

        return ((rect.toFloat() - (display->totalArea.getTopLeft().toFloat() * globalScale)) * (display->scale / globalScale))
                 + display->topLeftPhysical.toFloat();
        */
    }
    
    /**
      | Converts a Point from physical to logical
      | pixels.
      | 
      | If useScaleFactorOfDisplay is not
      | null then its scale factor will be used
      | for the conversion regardless of the
      | display that the Point to be converted
      | is on.
      |
      */
    pub fn physical_to_logical<ValueType>(
        &self, 
        point:                       Point<ValueType>,
        use_scale_factor_of_display: *const Display

    ) -> Point<ValueType> {

        todo!();

        /*
            const auto* display = useScaleFactorOfDisplay != nullptr ? useScaleFactorOfDisplay
                                                                 : getDisplayForPoint (point.roundToInt(), true);

        if (display == nullptr)
            return point;

        auto globalScale = Desktop::getInstance().getGlobalScaleFactor();

        Point<ValueType> logicalTopLeft  (static_cast<ValueType> (display->totalArea.getX()),       static_cast<ValueType> (display->totalArea.getY()));
        Point<ValueType> physicalTopLeft (static_cast<ValueType> (display->topLeftPhysical.getX()), static_cast<ValueType> (display->topLeftPhysical.getY()));

        return ((point - physicalTopLeft) / (display->scale / globalScale)) + (logicalTopLeft * globalScale);
        */
    }
    
    /**
      | Converts a Point from logical to physical
      | pixels.
      | 
      | If useScaleFactorOfDisplay is not
      | null then its scale factor will be used
      | for the conversion regardless of the
      | display that the Point to be converted
      | is on.
      |
      */
    pub fn logical_to_physical<ValueType>(
        &self, 
        point:                       Point<ValueType>,
        use_scale_factor_of_display: *const Display

    ) -> Point<ValueType> {

        todo!();

        /*
            const auto* display = useScaleFactorOfDisplay != nullptr ? useScaleFactorOfDisplay
                                                                 : getDisplayForPoint (point.roundToInt(), false);

        if (display == nullptr)
            return point;

        auto globalScale = Desktop::getInstance().getGlobalScaleFactor();

        Point<ValueType> logicalTopLeft  (static_cast<ValueType> (display->totalArea.getX()),       static_cast<ValueType> (display->totalArea.getY()));
        Point<ValueType> physicalTopLeft (static_cast<ValueType> (display->topLeftPhysical.getX()), static_cast<ValueType> (display->topLeftPhysical.getY()));

        return ((point - (logicalTopLeft * globalScale)) * (display->scale / globalScale)) + physicalTopLeft;
        */
    }
    
    /**
      | Returns the Display object representing
      | the display acting as the user's main
      | screen, or nullptr if there are no connected
      | displays.
      |
      */
    pub fn get_primary_display(&self) -> *const crate::Display {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        for (auto& d : displays)
            if (d.isMain)
                return &d;

        return nullptr;
        */
    }
    
    /**
      | Returns a RectangleList made up of all
      | the displays in LOGICAL pixels.
      |
      */
    pub fn get_rectangle_list(&self, user_areas_only: bool) -> RectangleList<i32> {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED
        RectangleList<int> rl;

        for (auto& d : displays)
            rl.addWithoutMerging (userAreasOnly ? d.userArea : d.totalArea);

        return rl;
        */
    }
    
    /**
      | Returns the smallest bounding box which
      | contains all the displays in LOGICAL
      | pixels.
      |
      */
    pub fn get_total_bounds(&self, user_areas_only: bool) -> Rectangle<i32> {
        
        todo!();
        /*
            return getRectangleList (userAreasOnly).getBounds();
        */
    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            Vec<Display> oldDisplays;
        oldDisplays.swapWith (displays);

        init (Desktop::getInstance());

        if (oldDisplays != displays)
        {
            for (auto i = ComponentPeer::getNumPeers(); --i >= 0;)
                if (auto* peer = ComponentPeer::getPeer (i))
                    peer->handleScreenSizeChange();
        }
        */
    }
    
    /**
      | This is called when the displays Vec
      | has been filled out with the info for
      | all connected displays and the totalArea
      | and userArea Rectangles need to be converted
      | from physical to logical coordinates.
      |
      */
    pub fn update_to_logical(&mut self)  {

        todo!();
        /*
            if (displays.size() == 1)
        {
            auto& display = displays.getReference (0);

            display.totalArea = (display.totalArea.toDouble() / display.scale).toNearestInt();
            display.userArea  = (display.userArea.toDouble()  / display.scale).toNearestInt();

            return;
        }

        Vec<DisplayNode> displayNodes;

        for (auto& d : displays)
        {
            DisplayNode node;

            node.display = &d;

            if (d.totalArea.getTopLeft() == Point<int>())
                node.isRoot = true;

            displayNodes.add (node);
        }

        auto* root = [&displayNodes]() -> DisplayNode*
        {
            for (auto& node : displayNodes)
                if (node.isRoot)
                    return &node;

            auto minDistance = std::numeric_limits<int>::max();
            DisplayNode* retVal = nullptr;

            for (auto& node : displayNodes)
            {
                auto distance = node.display->totalArea.getTopLeft().getDistanceFrom ({});

                if (distance < minDistance)
                {
                    minDistance = distance;
                    retVal = &node;
                }
            }

            if (retVal != nullptr)
                retVal->isRoot = true;

            return retVal;
        }();

        // Must have a root node!
        jassert (root != nullptr);

        // Recursively traverse the display graph from the root and work out logical bounds
        processDisplay (root, displayNodes);

        for (auto& node : displayNodes)
        {
            // All of the nodes should have a parent
            jassert (node.parent != nullptr);

            auto relativeUserArea = (node.display->userArea.toDouble() - node.display->totalArea.toDouble().getTopLeft()) / node.display->scale;

            // Now set Display::totalArea and ::userArea using the logical area that we have calculated
            node.display->topLeftPhysical = node.display->totalArea.getTopLeft();
            node.display->totalArea       = node.logicalArea.toNearestInt();
            node.display->userArea        = (relativeUserArea + node.logicalArea.getTopLeft()).toNearestInt();
        }
        */
    }

    /* -------------- Deprecated methods  -------------- */

    pub fn get_display_containing(
        &self, 
        position: Point<i32>

    ) -> &crate::Display {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED
        const auto* best = &displays.getReference (0);
        auto bestDistance = std::numeric_limits<int>::max();

        for (auto& d : displays)
        {
            if (d.totalArea.contains (position))
            {
                best = &d;
                break;
            }

            auto distance = d.totalArea.getCentre().getDistanceFrom (position);

            if (distance < bestDistance)
            {
                bestDistance = distance;
                best = &d;
            }
        }

        return *best;
        */
    }
    
    pub fn find_display_for_rect(
        &self, 
        rect:        Rectangle<i32>,
        is_physical: bool

    ) -> &crate::Display {
        
        todo!();
        /*
            if (auto* display = getDisplayForRect (rect, isPhysical))
            return *display;

        return emptyDisplay;
        */
    }
    
    pub fn find_display_for_point(
        &self, 
        point:       Point<i32>,
        is_physical: bool

    ) -> &crate::Display {
        
        todo!();
        /*
            if (auto* display = getDisplayForPoint (point, isPhysical))
            return *display;

        return emptyDisplay;
        */
    }
    
    pub fn get_main_display(&self) -> &crate::Display {
        
        todo!();
        /*
            if (auto* display = getPrimaryDisplay())
            return *display;

        return emptyDisplay;
        */
    }
}
impl PartialEq<Display> for Display {
    
    #[inline] fn eq(&self, other: &Display) -> bool {
        todo!();
        /*
            return d1.isMain          == d2.isMain
            && d1.totalArea       == d2.totalArea
            && d1.userArea        == d2.userArea
            && d1.topLeftPhysical == d2.topLeftPhysical
            && d1.scale           == d2.scale
            && d1.dpi             == d2.dpi;
        */
    }
}

impl Eq for Display {}

/*
  | These methods are used for converting the
  | totalArea and userArea Rectangles in Display
  | from physical to logical pixels. We do this by
  | constructing a graph of connected displays
  | where the root node has position (0, 0); this
  | can be safely converted to logical pixels using
  | its scale factor and we can then traverse the
  | graph and work out the logical pixels for all
  | the other connected displays. We need to do
  | this as the logical bounds of a display depend
  | not only on its scale factor but also the scale
  | factor of the displays connected to it.
  */

/**
  | Represents a node in our graph of displays.
  |
  */
pub struct DisplayNode {

    /**
      | The Display object that this represents.
      |
      */
    display:      *mut Display,

    /**
      | True if this represents the 'root' display
      | with position (0, 0).
      |
      */
    is_root:      bool, // default = false

    /**
      | The parent node of this node in our display
      | graph. This will have a correct logicalArea.
      |
      */
    parent:       *mut DisplayNode, // default = nullptr

    /**
      | The logical area to be calculated. This will be
      | valid after processDisplay() has been called on
      | this node.
      */
    logical_area: Rectangle<f64>,
}

/**
  | Recursive - will calculate and set the
  | logicalArea member of current.
  |
  */
pub fn process_display(
        current_node: *mut DisplayNode,
        all_nodes:    &mut Vec<DisplayNode>)  {
    
    todo!();
    /*
        const auto physicalArea = currentNode->display->totalArea.toDouble();
        const auto scale = currentNode->display->scale;

        if (! currentNode->isRoot)
        {
            const auto logicalWidth  = physicalArea.getWidth() / scale;
            const auto logicalHeight = physicalArea.getHeight() / scale;

            const auto physicalParentArea = currentNode->parent->display->totalArea.toDouble();
            const auto logicalParentArea  = currentNode->parent->logicalArea; // logical area of parent has already been calculated
            const auto parentScale        = currentNode->parent->display->scale;

            Rectangle<double> logicalArea (0.0, 0.0, logicalWidth, logicalHeight);

            if      (physicalArea.getRight() == physicalParentArea.getX())     logicalArea.setPosition ({ logicalParentArea.getX() - logicalWidth, physicalArea.getY() / parentScale });  // on left
            else if (physicalArea.getX() == physicalParentArea.getRight())     logicalArea.setPosition ({ logicalParentArea.getRight(),  physicalArea.getY() / parentScale });            // on right
            else if (physicalArea.getBottom() == physicalParentArea.getY())    logicalArea.setPosition ({ physicalArea.getX() / parentScale, logicalParentArea.getY() - logicalHeight }); // on top
            else if (physicalArea.getY() == physicalParentArea.getBottom())    logicalArea.setPosition ({ physicalArea.getX() / parentScale, logicalParentArea.getBottom() });            // on bottom
            else                                                               jassertfalse;

            currentNode->logicalArea = logicalArea;
        }
        else
        {
            // If currentNode is the root (position (0, 0)) then we can just scale the physical area
            currentNode->logicalArea = physicalArea / scale;
            currentNode->parent = currentNode;
        }

        // Find child nodes
        Vec<DisplayNode*> children;
        for (auto& node : allNodes)
        {
            // Already calculated
            if (node.parent != nullptr)
                continue;

            const auto otherPhysicalArea = node.display->totalArea.toDouble();

            // If the displays are touching on any side
            if (otherPhysicalArea.getX() == physicalArea.getRight()  || otherPhysicalArea.getRight() == physicalArea.getX()
                || otherPhysicalArea.getY() == physicalArea.getBottom() || otherPhysicalArea.getBottom() == physicalArea.getY())
            {
                node.parent = currentNode;
                children.add (&node);
            }
        }

        // Recursively process all child nodes
        for (auto child : children)
            processDisplay (child, allNodes);
    */
}

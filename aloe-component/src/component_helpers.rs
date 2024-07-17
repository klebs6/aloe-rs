crate::ix!();

pub fn find_container<'a, FocusContainerFn>(
    child:              *const Component<'a>,
    is_focus_container: FocusContainerFn) -> *mut Component {

    todo!();
    /*
        if (auto* parent = child->getParentComponent())
        {
            if ((parent->*isFocusContainer)() || parent->getParentComponent() == nullptr)
                return parent;

            return findContainer (parent, isFocusContainer);
        }

        return nullptr;
    */
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub fn run_modal_loop_callback(user_data: *mut c_void)  {
    
    todo!();
    /*
        return (void*) (pointer_sized_int) static_cast<Component*> (userData)->runModalLoop();
    */
}

pub fn get_colour_propertyid(colourid: i32) -> Identifier {
    
    todo!();
    /*
        char buffer[32];
        auto* end = buffer + numElementsInArray (buffer) - 1;
        auto* t = end;
        *t = 0;

        for (auto v = (uint32) colourID;;)
        {
            *--t = "0123456789abcdef" [v & 15];
            v >>= 4;

            if (v == 0)
                break;
        }

        for (int i = (int) sizeof (colourPropertyPrefix) - 1; --i >= 0;)
            *--t = colourPropertyPrefix[i];

        return t;
    */
}

pub fn hit_test<'a>(
    comp:        &mut Component<'a>,
    local_point: Point<i32>) -> bool {
    
    todo!();
    /*
        return isPositiveAndBelow (localPoint.x, comp.getWidth())
            && isPositiveAndBelow (localPoint.y, comp.getHeight())
            && comp.hitTest (localPoint.x, localPoint.y);
    */
}

/**
  | converts an unscaled position within
  | a peer to the local position within that
  | peer's component
  |
  */
pub fn raw_peer_position_to_local<'a,PointOrRect>(
    comp: &Component<'a>,
    pos:  PointOrRect) -> PointOrRect {

    todo!();
    /*
        if (comp.isTransformed())
            pos = pos.transformedBy (comp.getTransform().inverted());

        return ScalingHelpers::unscaledScreenPosToScaled (comp, pos);
    */
}

/**
  | converts a position within a peer's
  | component to the unscaled position
  | within the peer
  |
  */
pub fn local_position_to_raw_peer_pos<'a,PointOrRect>(
    comp: &Component<'a>,
    pos:  PointOrRect) -> PointOrRect {

    todo!();
    /*
        if (comp.isTransformed())
            pos = pos.transformedBy (comp.getTransform());

        return ScalingHelpers::scaledScreenPosToUnscaled (comp, pos);
    */
}

pub fn convert_from_parent_space<'a,PointOrRect>(
    comp:                  &Component<'a>,
    point_in_parent_space: PointOrRect) -> PointOrRect {

    todo!();
    /*
        const auto transformed = comp.affineTransform != nullptr ? pointInParentSpace.transformedBy (comp.affineTransform->inverted())
                                                                 : pointInParentSpace;

        if (comp.isOnDesktop())
        {
            if (auto* peer = comp.getPeer())
                return ScalingHelpers::unscaledScreenPosToScaled (comp, peer->globalToLocal (ScalingHelpers::scaledScreenPosToUnscaled (transformed)));

            jassertfalse;
            return transformed;
        }

        if (comp.getParentComponent() == nullptr)
            return ScalingHelpers::subtractPosition (ScalingHelpers::unscaledScreenPosToScaled (comp, ScalingHelpers::scaledScreenPosToUnscaled (transformed)), comp);

        return ScalingHelpers::subtractPosition (transformed, comp);
    */
}

pub fn convert_to_parent_space<'a,PointOrRect>(
    comp:                 &Component<'a>,
    point_in_local_space: PointOrRect) -> PointOrRect {

    todo!();
    /*
        const auto preTransform = [&]
        {
            if (comp.isOnDesktop())
            {
                if (auto* peer = comp.getPeer())
                    return ScalingHelpers::unscaledScreenPosToScaled (peer->localToGlobal (ScalingHelpers::scaledScreenPosToUnscaled (comp, pointInLocalSpace)));

                jassertfalse;
                return pointInLocalSpace;
            }

            if (comp.getParentComponent() == nullptr)
                return ScalingHelpers::unscaledScreenPosToScaled (ScalingHelpers::scaledScreenPosToUnscaled (comp, ScalingHelpers::addPosition (pointInLocalSpace, comp)));

            return ScalingHelpers::addPosition (pointInLocalSpace, comp);
        }();

        return comp.affineTransform != nullptr ? preTransform.transformedBy (*comp.affineTransform)
                                               : preTransform;
    */
}

pub fn convert_from_distant_parent_space<'a,PointOrRect>(
    parent:          *const Component<'a>,
    target:          &Component<'a>,
    coord_in_parent: PointOrRect) -> PointOrRect {

    todo!();
    /*
        auto* directParent = target.getParentComponent();
        jassert (directParent != nullptr);

        if (directParent == parent)
            return convertFromParentSpace (target, coordInParent);

        ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011)
        return convertFromParentSpace (target, convertFromDistantParentSpace (parent, *directParent, coordInParent));
        ALOE_END_IGNORE_WARNINGS_MSVC
    */
}

pub fn convert_coordinate<'a,PointOrRect>(
    target: *const Component<'a>,
    source: *const Component<'a>,
    p:      PointOrRect) -> PointOrRect {

    todo!();
    /*
        while (source != nullptr)
        {
            if (source == target)
                return p;

            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011)

            if (source->isParentOf (target))
                return convertFromDistantParentSpace (source, *target, p);

            ALOE_END_IGNORE_WARNINGS_MSVC

            p = convertToParentSpace (*source, p);
            source = source->getParentComponent();
        }

        jassert (source == nullptr);
        if (target == nullptr)
            return p;

        auto* topLevelComp = target->getTopLevelComponent();

        p = convertFromParentSpace (*topLevelComp, p);

        if (topLevelComp == target)
            return p;

        return convertFromDistantParentSpace (topLevelComp, *target, p);
    */
}

pub fn clip_obscured_regions<'a>(
    comp:      &Component<'a>,
    g:         &mut Graphics,
    clip_rect: Rectangle<i32>,
    delta:     Point<i32>) -> bool {
    
    todo!();
    /*
        bool wasClipped = false;

        for (int i = comp.childComponentList.size(); --i >= 0;)
        {
            auto& child = *comp.childComponentList.getUnchecked(i);

            if (child.isVisible() && ! child.isTransformed())
            {
                auto newClip = clipRect.getIntersection (child.boundsRelativeToParent);

                if (! newClip.isEmpty())
                {
                    if (child.isOpaque() && child.componentTransparency == 0)
                    {
                        g.excludeClipRegion (newClip + delta);
                        wasClipped = true;
                    }
                    else
                    {
                        auto childPos = child.getPosition();

                        if (clipObscuredRegions (child, g, newClip - childPos, childPos + delta))
                            wasClipped = true;
                    }
                }
            }
        }

        return wasClipped;
    */
}

pub fn get_parent_or_main_monitor_bounds<'a>(comp: &Component<'a>) -> Rectangle<i32> {
    
    todo!();
    /*
        if (auto* p = comp.getParentComponent())
            return p->getLocalBounds();

        return Desktop::getInstance().getDisplays().getPrimaryDisplay()->userArea;
    */
}

pub fn release_all_cached_image_resources<'a>(c: &mut Component<'a>)  {
    
    todo!();
    /*
        if (auto* cached = c.getCachedComponentImage())
            cached->releaseResources();

        for (auto* child : c.childComponentList)
            releaseAllCachedImageResources (*child);
    */
}

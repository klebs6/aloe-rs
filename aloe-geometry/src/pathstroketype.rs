crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_PathStrokeType.h]

/**
  | Describes a type of stroke used to render
  | a solid outline along a path.
  | 
  | A PathStrokeType object can be used
  | directly to create the shape of an outline
  | around a path, and is used by Graphics::strokePath
  | to specify the type of stroke to draw.
  | 
  | @see Path, Graphics::strokePath
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct PathStrokeType {
    thickness:   f32,
    joint_style: path_stroke_type::JointStyle,
    end_style:   path_stroke_type::EndCapStyle,
}

pub mod path_stroke_type {

    use super::*;
    
    /**
      | The type of shape to use for the corners
      | between two adjacent line segments.
      |
      */
    pub enum JointStyle
    {
        /**
          | Indicates that corners should be drawn
          | with sharp joints. Note that for angles
          | that curve back on themselves, drawing
          | a mitre could require extending the
          | point too far away from the path, so a
          | mitre limit is imposed and any corners
          | that exceed it are drawn as bevelled
          | instead.
          |
          */
        mitered,    

        /**
          | Indicates that corners should be drawn
          | as rounded-off.
          |
          */
        curved,     


        /**
          | Indicates that corners should be drawn
          | with a line flattening their outside
          | edge.
          |
          */
        beveled,     
    }

    /**
      | The type shape to use for the ends of lines.
      |
      */
    pub enum EndCapStyle
    {
        /**
          | Ends of lines are flat and don't extend
          | beyond the end point.
          |
          */
        butt,       

        /**
          | Ends of lines are flat, but stick out
          | beyond the end point for half the thickness
          | of the stroke.
          |
          */
        square,     

        /**
          | Ends of lines are rounded-off with a
          | circular shape.
          |
          */
        rounded,     
    }

    pub fn line_intersection(
            x1:                                f32,
            y1:                                f32,
            x2:                                f32,
            y2:                                f32,
            x3:                                f32,
            y3:                                f32,
            x4:                                f32,
            y4:                                f32,
            intersectionx:                     &mut f32,
            intersectiony:                     &mut f32,
            distance_beyond_line_1end_squared: &mut f32) -> bool {
        
        todo!();
        /*
            if (x2 != x3 || y2 != y3)
                {
                    auto dx1 = x2 - x1;
                    auto dy1 = y2 - y1;
                    auto dx2 = x4 - x3;
                    auto dy2 = y4 - y3;
                    auto divisor = dx1 * dy2 - dx2 * dy1;

                    if (divisor == 0.0f)
                    {
                        if (! ((dx1 == 0.0f && dy1 == 0.0f) || (dx2 == 0.0f && dy2 == 0.0f)))
                        {
                            if (dy1 == 0.0f && dy2 != 0.0f)
                            {
                                auto along = (y1 - y3) / dy2;
                                intersectionX = x3 + along * dx2;
                                intersectionY = y1;

                                distanceBeyondLine1EndSquared = intersectionX - x2;
                                distanceBeyondLine1EndSquared *= distanceBeyondLine1EndSquared;
                                if ((x2 > x1) == (intersectionX < x2))
                                    distanceBeyondLine1EndSquared = -distanceBeyondLine1EndSquared;

                                return along >= 0 && along <= 1.0f;
                            }

                            if (dy2 == 0.0f && dy1 != 0.0f)
                            {
                                auto along = (y3 - y1) / dy1;
                                intersectionX = x1 + along * dx1;
                                intersectionY = y3;

                                distanceBeyondLine1EndSquared = (along - 1.0f) * dx1;
                                distanceBeyondLine1EndSquared *= distanceBeyondLine1EndSquared;
                                if (along < 1.0f)
                                    distanceBeyondLine1EndSquared = -distanceBeyondLine1EndSquared;

                                return along >= 0 && along <= 1.0f;
                            }

                            if (dx1 == 0.0f && dx2 != 0.0f)
                            {
                                auto along = (x1 - x3) / dx2;
                                intersectionX = x1;
                                intersectionY = y3 + along * dy2;

                                distanceBeyondLine1EndSquared = intersectionY - y2;
                                distanceBeyondLine1EndSquared *= distanceBeyondLine1EndSquared;

                                if ((y2 > y1) == (intersectionY < y2))
                                    distanceBeyondLine1EndSquared = -distanceBeyondLine1EndSquared;

                                return along >= 0 && along <= 1.0f;
                            }

                            if (dx2 == 0.0f && dx1 != 0.0f)
                            {
                                auto along = (x3 - x1) / dx1;
                                intersectionX = x3;
                                intersectionY = y1 + along * dy1;

                                distanceBeyondLine1EndSquared = (along - 1.0f) * dy1;
                                distanceBeyondLine1EndSquared *= distanceBeyondLine1EndSquared;
                                if (along < 1.0f)
                                    distanceBeyondLine1EndSquared = -distanceBeyondLine1EndSquared;

                                return along >= 0 && along <= 1.0f;
                            }
                        }

                        intersectionX = 0.5f * (x2 + x3);
                        intersectionY = 0.5f * (y2 + y3);

                        distanceBeyondLine1EndSquared = 0.0f;
                        return false;
                    }

                    auto along1 = ((y1 - y3) * dx2 - (x1 - x3) * dy2) / divisor;

                    intersectionX = x1 + along1 * dx1;
                    intersectionY = y1 + along1 * dy1;

                    if (along1 >= 0 && along1 <= 1.0f)
                    {
                        auto along2 = ((y1 - y3) * dx1 - (x1 - x3) * dy1) / divisor;

                        if (along2 >= 0 && along2 <= 1.0f)
                        {
                            distanceBeyondLine1EndSquared = 0.0f;
                            return true;
                        }
                    }

                    distanceBeyondLine1EndSquared = along1 - 1.0f;
                    distanceBeyondLine1EndSquared *= distanceBeyondLine1EndSquared;
                    distanceBeyondLine1EndSquared *= (dx1 * dx1 + dy1 * dy1);

                    if (along1 < 1.0f)
                        distanceBeyondLine1EndSquared = -distanceBeyondLine1EndSquared;

                    return false;
                }

                intersectionX = x2;
                intersectionY = y2;

                distanceBeyondLine1EndSquared = 0.0f;
                return true;
        */
    }

    pub fn add_edge_and_joint(
            dest_path:                   &mut Path,
            style:                       path_stroke_type::JointStyle,
            max_miter_extension_squared: f32,
            width:                       f32,
            x1:                          f32,
            y1:                          f32,
            x2:                          f32,
            y2:                          f32,
            x3:                          f32,
            y3:                          f32,
            x4:                          f32,
            y4:                          f32,
            midx:                        f32,
            midy:                        f32)  {
        
        todo!();
        /*
            if (style == PathStrokeType::beveled
                    || (x3 == x4 && y3 == y4)
                    || (x1 == x2 && y1 == y2))
                {
                    destPath.lineTo (x2, y2);
                    destPath.lineTo (x3, y3);
                }
                else
                {
                    float jx, jy, distanceBeyondLine1EndSquared;

                    // if they intersect, use this point..
                    if (lineIntersection (x1, y1, x2, y2,
                                          x3, y3, x4, y4,
                                          jx, jy, distanceBeyondLine1EndSquared))
                    {
                        destPath.lineTo (jx, jy);
                    }
                    else
                    {
                        if (style == PathStrokeType::mitered)
                        {
                            if (distanceBeyondLine1EndSquared < maxMiterExtensionSquared
                                && distanceBeyondLine1EndSquared > 0.0f)
                            {
                                destPath.lineTo (jx, jy);
                            }
                            else
                            {
                                // the end sticks out too far, so just use a blunt joint
                                destPath.lineTo (x2, y2);
                                destPath.lineTo (x3, y3);
                            }
                        }
                        else
                        {
                            // curved joints
                            float angle1 = std::atan2 (x2 - midX, y2 - midY);
                            float angle2 = std::atan2 (x3 - midX, y3 - midY);
                            const float angleIncrement = 0.1f;

                            destPath.lineTo (x2, y2);

                            if (std::abs (angle1 - angle2) > angleIncrement)
                            {
                                if (angle2 > angle1 + MathConstants<float>::pi
                                     || (angle2 < angle1 && angle2 >= angle1 - MathConstants<float>::pi))
                                {
                                    if (angle2 > angle1)
                                        angle2 -= MathConstants<float>::twoPi;

                                    jassert (angle1 <= angle2 + MathConstants<float>::pi);

                                    angle1 -= angleIncrement;
                                    while (angle1 > angle2)
                                    {
                                        destPath.lineTo (midX + width * std::sin (angle1),
                                                         midY + width * std::cos (angle1));

                                        angle1 -= angleIncrement;
                                    }
                                }
                                else
                                {
                                    if (angle1 > angle2)
                                        angle1 -= MathConstants<float>::twoPi;

                                    jassert (angle1 >= angle2 - MathConstants<float>::pi);

                                    angle1 += angleIncrement;
                                    while (angle1 < angle2)
                                    {
                                        destPath.lineTo (midX + width * std::sin (angle1),
                                                         midY + width * std::cos (angle1));

                                        angle1 += angleIncrement;
                                    }
                                }
                            }

                            destPath.lineTo (x3, y3);
                        }
                    }
                }
        */
    }

    pub fn add_line_end(
            dest_path: &mut Path,
            style:     path_stroke_type::EndCapStyle,
            x1:        f32,
            y1:        f32,
            x2:        f32,
            y2:        f32,
            width:     f32)  {
        
        todo!();
        /*
            if (style == PathStrokeType::butt)
                {
                    destPath.lineTo (x2, y2);
                }
                else
                {
                    float offx1, offy1, offx2, offy2;

                    auto dx = x2 - x1;
                    auto dy = y2 - y1;
                    auto len = aloe_hypot (dx, dy);

                    if (len == 0.0f)
                    {
                        offx1 = offx2 = x1;
                        offy1 = offy2 = y1;
                    }
                    else
                    {
                        auto offset = width / len;
                        dx *= offset;
                        dy *= offset;

                        offx1 = x1 + dy;
                        offy1 = y1 - dx;
                        offx2 = x2 + dy;
                        offy2 = y2 - dx;
                    }

                    if (style == PathStrokeType::square)
                    {
                        // square ends
                        destPath.lineTo (offx1, offy1);
                        destPath.lineTo (offx2, offy2);
                        destPath.lineTo (x2, y2);
                    }
                    else
                    {
                        // rounded ends
                        auto midx = (offx1 + offx2) * 0.5f;
                        auto midy = (offy1 + offy2) * 0.5f;

                        destPath.cubicTo (x1 + (offx1 - x1) * 0.55f, y1 + (offy1 - y1) * 0.55f,
                                          offx1 + (midx - offx1) * 0.45f, offy1 + (midy - offy1) * 0.45f,
                                          midx, midy);

                        destPath.cubicTo (midx + (offx2 - midx) * 0.55f, midy + (offy2 - midy) * 0.55f,
                                          offx2 + (x2 - offx2) * 0.45f, offy2 + (y2 - offy2) * 0.45f,
                                          x2, y2);
                    }
                }
        */
    }

    pub struct Arrowhead
    {
        start_width:  f32,
        start_length: f32,
        end_width:    f32,
        end_length:   f32,
    }

    pub fn add_arrowhead(
            dest_path:       &mut Path,
            x1:              f32,
            y1:              f32,
            x2:              f32,
            y2:              f32,
            tipx:            f32,
            tipy:            f32,
            width:           f32,
            arrowhead_width: f32)  {
        
        todo!();
        /*
            Line<float> line (x1, y1, x2, y2);
                destPath.lineTo (line.getPointAlongLine (-(arrowheadWidth / 2.0f - width), 0));
                destPath.lineTo (tipX, tipY);
                destPath.lineTo (line.getPointAlongLine (arrowheadWidth - (arrowheadWidth / 2.0f - width), 0));
                destPath.lineTo (x2, y2);
        */
    }

    pub struct LineSection
    {
        /* ----------------- original line  ----------------- */
        x1:  f32,
        y1:  f32,
        x2:  f32,
        y2:  f32,


        /* ------------- the left-hand stroke  ------------- */
        lx1: f32,
        ly1: f32,
        lx2: f32,
        ly2: f32,

        /* ------------- the right-hand stroke  ------------- */
        rx1: f32,
        ry1: f32,
        rx2: f32,
        ry2: f32,
    }

    pub fn shorten_sub_path(
            sub_path:        &mut Vec<LineSection>,
            amount_at_start: f32,
            amount_at_end:   f32)  {
        
        todo!();
        /*
            while (amountAtEnd > 0 && subPath.size() > 0)
                {
                    auto& l = subPath.getReference (subPath.size() - 1);
                    auto dx = l.rx2 - l.rx1;
                    auto dy = l.ry2 - l.ry1;
                    auto len = aloe_hypot (dx, dy);

                    if (len <= amountAtEnd && subPath.size() > 1)
                    {
                        LineSection& prev = subPath.getReference (subPath.size() - 2);
                        prev.x2 = l.x2;
                        prev.y2 = l.y2;
                        subPath.removeLast();
                        amountAtEnd -= len;
                    }
                    else
                    {
                        auto prop = jmin (0.9999f, amountAtEnd / len);
                        dx *= prop;
                        dy *= prop;
                        l.rx1 += dx;
                        l.ry1 += dy;
                        l.lx2 += dx;
                        l.ly2 += dy;
                        break;
                    }
                }

                while (amountAtStart > 0 && subPath.size() > 0)
                {
                    auto& l = subPath.getReference (0);
                    auto dx = l.rx2 - l.rx1;
                    auto dy = l.ry2 - l.ry1;
                    auto len = aloe_hypot (dx, dy);

                    if (len <= amountAtStart && subPath.size() > 1)
                    {
                        LineSection& next = subPath.getReference (1);
                        next.x1 = l.x1;
                        next.y1 = l.y1;
                        subPath.remove (0);
                        amountAtStart -= len;
                    }
                    else
                    {
                        auto prop = jmin (0.9999f, amountAtStart / len);
                        dx *= prop;
                        dy *= prop;
                        l.rx2 -= dx;
                        l.ry2 -= dy;
                        l.lx1 -= dx;
                        l.ly1 -= dy;
                        break;
                    }
                }
        */
    }

    pub fn add_sub_path(
            dest_path:                   &mut Path,
            sub_path:                    &mut Vec<LineSection>,
            is_closed:                   bool,
            width:                       f32,
            max_miter_extension_squared: f32,
            joint_style:                 path_stroke_type::JointStyle,
            end_style:                   path_stroke_type::EndCapStyle,
            arrowhead:                   *const Arrowhead)  {
        
        todo!();
        /*
            jassert (subPath.size() > 0);

                if (arrowhead != nullptr)
                    shortenSubPath (subPath, arrowhead->startLength, arrowhead->endLength);

                auto& firstLine = subPath.getReference (0);

                auto lastX1 = firstLine.lx1;
                auto lastY1 = firstLine.ly1;
                auto lastX2 = firstLine.lx2;
                auto lastY2 = firstLine.ly2;

                if (isClosed)
                {
                    destPath.startNewSubPath (lastX1, lastY1);
                }
                else
                {
                    destPath.startNewSubPath (firstLine.rx2, firstLine.ry2);

                    if (arrowhead != nullptr && arrowhead->startWidth > 0.0f)
                        addArrowhead (destPath, firstLine.rx2, firstLine.ry2, lastX1, lastY1, firstLine.x1, firstLine.y1,
                                      width, arrowhead->startWidth);
                    else
                        addLineEnd (destPath, endStyle, firstLine.rx2, firstLine.ry2, lastX1, lastY1, width);
                }

                for (int i = 1; i < subPath.size(); ++i)
                {
                    const LineSection& l = subPath.getReference (i);

                    addEdgeAndJoint (destPath, jointStyle,
                                     maxMiterExtensionSquared, width,
                                     lastX1, lastY1, lastX2, lastY2,
                                     l.lx1, l.ly1, l.lx2, l.ly2,
                                     l.x1, l.y1);

                    lastX1 = l.lx1;
                    lastY1 = l.ly1;
                    lastX2 = l.lx2;
                    lastY2 = l.ly2;
                }

                auto& lastLine = subPath.getReference (subPath.size() - 1);

                if (isClosed)
                {
                    auto& l = subPath.getReference (0);

                    addEdgeAndJoint (destPath, jointStyle,
                                     maxMiterExtensionSquared, width,
                                     lastX1, lastY1, lastX2, lastY2,
                                     l.lx1, l.ly1, l.lx2, l.ly2,
                                     l.x1, l.y1);

                    destPath.closeSubPath();
                    destPath.startNewSubPath (lastLine.rx1, lastLine.ry1);
                }
                else
                {
                    destPath.lineTo (lastX2, lastY2);

                    if (arrowhead != nullptr && arrowhead->endWidth > 0.0f)
                        addArrowhead (destPath, lastX2, lastY2, lastLine.rx1, lastLine.ry1, lastLine.x2, lastLine.y2,
                                      width, arrowhead->endWidth);
                    else
                        addLineEnd (destPath, endStyle, lastX2, lastY2, lastLine.rx1, lastLine.ry1, width);
                }

                lastX1 = lastLine.rx1;
                lastY1 = lastLine.ry1;
                lastX2 = lastLine.rx2;
                lastY2 = lastLine.ry2;

                for (int i = subPath.size() - 1; --i >= 0;)
                {
                    auto& l = subPath.getReference (i);

                    addEdgeAndJoint (destPath, jointStyle,
                                     maxMiterExtensionSquared, width,
                                     lastX1, lastY1, lastX2, lastY2,
                                     l.rx1, l.ry1, l.rx2, l.ry2,
                                     l.x2, l.y2);

                    lastX1 = l.rx1;
                    lastY1 = l.ry1;
                    lastX2 = l.rx2;
                    lastY2 = l.ry2;
                }

                if (isClosed)
                {
                    addEdgeAndJoint (destPath, jointStyle,
                                     maxMiterExtensionSquared, width,
                                     lastX1, lastY1, lastX2, lastY2,
                                     lastLine.rx1, lastLine.ry1, lastLine.rx2, lastLine.ry2,
                                     lastLine.x2, lastLine.y2);
                }
                else
                {
                    // do the last line
                    destPath.lineTo (lastX2, lastY2);
                }

                destPath.closeSubPath();
        */
    }

    pub fn create_stroke(
            thickness:      f32,
            joint_style:    path_stroke_type::JointStyle,
            end_style:      path_stroke_type::EndCapStyle,
            dest_path:      &mut Path,
            source:         &Path,
            transform:      &AffineTransform,
            extra_accuracy: f32,
            arrowhead:      *const Arrowhead)  {
        
        todo!();
        /*
            jassert (extraAccuracy > 0);

                if (thickness <= 0)
                {
                    destPath.clear();
                    return;
                }

                const Path* sourcePath = &source;
                Path temp;

                if (sourcePath == &destPath)
                {
                    destPath.swapWithPath (temp);
                    sourcePath = &temp;
                }
                else
                {
                    destPath.clear();
                }

                destPath.setUsingNonZeroWinding (true);

                const float maxMiterExtensionSquared = 9.0f * thickness * thickness;
                const float width = 0.5f * thickness;

                // Iterate the path, creating a list of the
                // left/right-hand lines along either side of it...
                PathFlatteningIterator it (*sourcePath, transform, Path::defaultToleranceForMeasurement / extraAccuracy);

                Vec<LineSection> subPath;
                subPath.ensureStorageAllocated (512);
                LineSection l;
                l.x1 = 0;
                l.y1 = 0;

                const float minSegmentLength = 0.0001f;

                while (it.next())
                {
                    if (it.subPathIndex == 0)
                    {
                        if (subPath.size() > 0)
                        {
                            addSubPath (destPath, subPath, false, width, maxMiterExtensionSquared, jointStyle, endStyle, arrowhead);
                            subPath.clearQuick();
                        }

                        l.x1 = it.x1;
                        l.y1 = it.y1;
                    }

                    l.x2 = it.x2;
                    l.y2 = it.y2;

                    float dx = l.x2 - l.x1;
                    float dy = l.y2 - l.y1;

                    auto hypotSquared = dx * dx + dy * dy;

                    if (it.closesSubPath || hypotSquared > minSegmentLength || it.isLastInSubpath())
                    {
                        auto len = std::sqrt (hypotSquared);

                        if (len == 0.0f)
                        {
                            l.rx1 = l.rx2 = l.lx1 = l.lx2 = l.x1;
                            l.ry1 = l.ry2 = l.ly1 = l.ly2 = l.y1;
                        }
                        else
                        {
                            auto offset = width / len;
                            dx *= offset;
                            dy *= offset;

                            l.rx2 = l.x1 - dy;
                            l.ry2 = l.y1 + dx;
                            l.lx1 = l.x1 + dy;
                            l.ly1 = l.y1 - dx;

                            l.lx2 = l.x2 + dy;
                            l.ly2 = l.y2 - dx;
                            l.rx1 = l.x2 - dy;
                            l.ry1 = l.y2 + dx;
                        }

                        subPath.add (l);

                        if (it.closesSubPath)
                        {
                            addSubPath (destPath, subPath, true, width, maxMiterExtensionSquared, jointStyle, endStyle, arrowhead);
                            subPath.clearQuick();
                        }
                        else
                        {
                            l.x1 = it.x2;
                            l.y1 = it.y2;
                        }
                    }
                }

                if (subPath.size() > 0)
                    addSubPath (destPath, subPath, false, width, maxMiterExtensionSquared, jointStyle, endStyle, arrowhead);
        */
    }
}

impl PartialEq<PathStrokeType> for PathStrokeType {
    
    /**
      | Compares the stroke thickness, joint
      | and end styles of two stroke types.
      |
      */
    #[inline] fn eq(&self, other: &PathStrokeType) -> bool {
        todo!();
        /*
            return thickness == other.thickness
            && jointStyle == other.jointStyle
            && endStyle == other.endStyle;
        */
    }
}

impl Eq for PathStrokeType {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_PathStrokeType.cpp]
impl PathStrokeType {
    
    /**
      | Returns the stroke thickness.
      |
      */
    pub fn get_stroke_thickness(&self) -> f32 {
        
        todo!();
        /*
            return thickness;
        */
    }

    /**
      | Sets the stroke thickness.
      |
      */
    pub fn set_stroke_thickness(&mut self, new_thickness: f32)  {
        
        todo!();
        /*
            thickness = newThickness;
        */
    }

    /**
      | Returns the joint style.
      |
      */
    pub fn get_joint_style(&self) -> path_stroke_type::JointStyle {
        
        todo!();
        /*
            return jointStyle;
        */
    }

    /**
      | Sets the joint style.
      |
      */
    pub fn set_joint_style(&mut self, new_style: path_stroke_type::JointStyle)  {
        
        todo!();
        /*
            jointStyle = newStyle;
        */
    }

    /**
      | Returns the end-cap style.
      |
      */
    pub fn get_end_style(&self) -> path_stroke_type::EndCapStyle {
        
        todo!();
        /*
            return endStyle;
        */
    }

    /**
      | Sets the end-cap style.
      |
      */
    pub fn set_end_style(&mut self, new_style: path_stroke_type::EndCapStyle)  {
        
        todo!();
        /*
            endStyle = newStyle;
        */
    }
    
    /**
      | Creates a stroke type with a given line-width,
      | and default joint/end styles.
      |
      */
    pub fn new_with_stroke_thickness(stroke_thickness: f32) -> Self {
    
        todo!();
        /*
        : thickness(strokeThickness),
        : joint_style(mitered),
        : end_style(butt),

        
        */
    }
    
    /**
      | Creates a stroke type.
      | 
      | -----------
      | @param strokeThickness
      | 
      | the width of the line to use
      | ----------
      | @param jointStyle
      | 
      | the type of joints to use for corners
      | ----------
      | @param endStyle
      | 
      | the type of end-caps to use for the ends
      | of open paths.
      |
      */
    pub fn new_with_stroke_thickness_joint_style_and_end_cap_style(
        stroke_thickness: f32,
        joint:            path_stroke_type::JointStyle,
        end_style:        Option<path_stroke_type::EndCapStyle>) -> Self {
    
        let end_style: path_stroke_type::EndCapStyle = end_style.unwrap_or(path_stroke_type::EndCapStyle::butt);

        todo!();
        /*
        : thickness(strokeThickness),
        : joint_style(joint),
        : end_style(end_style),

        
        */
    }
    
    /**
      | Creates a copy of another stroke type.
      |
      */
    pub fn new_from_other(other: &PathStrokeType) -> Self {
    
        todo!();
        /*
        : thickness(other.thickness),
        : joint_style(other.jointStyle),
        : end_style(other.endStyle),
        */
    }
    
    /**
      | Copies another stroke onto this one.
      |
      */
    pub fn assign_from(&mut self, other: &PathStrokeType) -> &mut PathStrokeType {
        
        todo!();
        /*
            thickness = other.thickness;
        jointStyle = other.jointStyle;
        endStyle = other.endStyle;
        return *this;
        */
    }
    
    /**
      | Applies this stroke type to a path and
      | returns the resultant stroke as another
      | Path.
      | 
      | -----------
      | @note
      | 
      | it's ok for the source and destination
      | Paths to be the same object, so you can
      | easily turn a path into a stroked version
      | of itself.
      | 
      | -----------
      | @param destPath
      | 
      | the resultant stroked outline shape
      | will be copied into this path.
      | ----------
      | @param sourcePath
      | 
      | the path to use as the source
      | ----------
      | @param transform
      | 
      | an optional transform to apply to the
      | points from the source path as they are
      | being used
      | ----------
      | @param extraAccuracy
      | 
      | if this is greater than 1.0, it will subdivide
      | the path to a higher resolution, which
      | improves the quality if you'll later
      | want to enlarge the stroked path. So
      | for example, if you're planning on drawing
      | the stroke at 3x the size that you're
      | creating it, you should set this to 3.
      | 
      | @see createDashedStroke
      |
      */
    pub fn create_stroked_path(
        &self, 
        dest_path:      &mut Path,
        source_path:    &Path,
        transform:      Option<&AffineTransform>,
        extra_accuracy: Option<f32>)  {

        let transform      = transform.unwrap_or(&AffineTransform::default());
        let extra_accuracy = extra_accuracy.unwrap_or(1.0);
        
        todo!();
        /*
            PathStrokeHelpers::createStroke (thickness, jointStyle, endStyle, destPath, sourcePath,
                                         transform, extraAccuracy, nullptr);
        */
    }

    /**
      | Applies this stroke type to a path, creating
      | a dashed line.
      | 
      | This is similar to createStrokedPath,
      | but uses the array passed in to break
      | the stroke up into a series of dashes.
      | 
      | -----------
      | @note
      | 
      | it's ok for the source and destination
      | Paths to be the same object, so you can
      | easily turn a path into a stroked version
      | of itself.
      | 
      | -----------
      | @param destPath
      | 
      | the resultant stroked outline shape
      | will be copied into this path.
      | ----------
      | @param sourcePath
      | 
      | the path to use as the source
      | ----------
      | @param dashLengths
      | 
      | An array of alternating on/off lengths.
      | E.g. { 2, 3, 4, 5 } will create a line of
      | length 2, then skip a length of 3, then
      | add a line of length 4, skip 5, and keep
      | repeating this pattern.
      | ----------
      | @param numDashLengths
      | 
      | The number of lengths in the dashLengths
      | array. This should really be an even
      | number, otherwise the pattern will
      | get out of step as it repeats.
      | ----------
      | @param transform
      | 
      | an optional transform to apply to the
      | points from the source path as they are
      | being used
      | ----------
      | @param extraAccuracy
      | 
      | if this is greater than 1.0, it will subdivide
      | the path to a higher resolution, which
      | improves the quality if you'll later
      | want to enlarge the stroked path. So
      | for example, if you're planning on drawing
      | the stroke at 3x the size that you're
      | creating it, you should set this to 3.
      |
      */
    pub fn create_dashed_stroke(
        &self, 
        dest_path:        &mut Path,
        source_path:      &Path,
        dash_lengths:     *const f32,
        num_dash_lengths: i32,
        transform:        Option<&AffineTransform>,
        extra_accuracy:   Option<f32>

    )  {

        let transform = transform.unwrap_or(&AffineTransform::default());

        let extra_accuracy: f32 = extra_accuracy.unwrap_or(1.0);
        
        todo!();
        /*
            jassert (extraAccuracy > 0);

        if (thickness <= 0)
            return;

        Path newDestPath;
        PathFlatteningIterator it (sourcePath, transform, Path::defaultToleranceForMeasurement / extraAccuracy);

        bool first = true;
        int dashNum = 0;
        float pos = 0.0f, lineLen = 0.0f, lineEndPos = 0.0f;
        float dx = 0.0f, dy = 0.0f;

        for (;;)
        {
            const bool isSolid = ((dashNum & 1) == 0);
            const float dashLen = dashLengths [dashNum++ % numDashLengths];

            jassert (dashLen >= 0); // must be a positive increment!
            if (dashLen <= 0)
                continue;

            pos += dashLen;

            while (pos > lineEndPos)
            {
                if (! it.next())
                {
                    if (isSolid && ! first)
                        newDestPath.lineTo (it.x2, it.y2);

                    createStrokedPath (destPath, newDestPath, AffineTransform(), extraAccuracy);
                    return;
                }

                if (isSolid && ! first)
                    newDestPath.lineTo (it.x1, it.y1);
                else
                    newDestPath.startNewSubPath (it.x1, it.y1);

                dx = it.x2 - it.x1;
                dy = it.y2 - it.y1;
                lineLen = aloe_hypot (dx, dy);
                lineEndPos += lineLen;
                first = it.closesSubPath;
            }

            const float alpha = (pos - (lineEndPos - lineLen)) / lineLen;

            if (isSolid)
                newDestPath.lineTo (it.x1 + dx * alpha,
                                    it.y1 + dy * alpha);
            else
                newDestPath.startNewSubPath (it.x1 + dx * alpha,
                                             it.y1 + dy * alpha);
        }
        */
    }
        
    /**
      | Applies this stroke type to a path and
      | returns the resultant stroke as another
      | Path.
      | 
      | -----------
      | @note
      | 
      | it's ok for the source and destination
      | Paths to be the same object, so you can
      | easily turn a path into a stroked version
      | of itself.
      | 
      | -----------
      | @param destPath
      | 
      | the resultant stroked outline shape
      | will be copied into this path.
      | ----------
      | @param sourcePath
      | 
      | the path to use as the source
      | ----------
      | @param arrowheadStartWidth
      | 
      | the width of the arrowhead at the start
      | of the path
      | ----------
      | @param arrowheadStartLength
      | 
      | the length of the arrowhead at the start
      | of the path
      | ----------
      | @param arrowheadEndWidth
      | 
      | the width of the arrowhead at the end
      | of the path
      | ----------
      | @param arrowheadEndLength
      | 
      | the length of the arrowhead at the end
      | of the path
      | ----------
      | @param transform
      | 
      | an optional transform to apply to the
      | points from the source path as they are
      | being used
      | ----------
      | @param extraAccuracy
      | 
      | if this is greater than 1.0, it will subdivide
      | the path to a higher resolution, which
      | improves the quality if you'll later
      | want to enlarge the stroked path. So
      | for example, if you're planning on drawing
      | the stroke at 3x the size that you're
      | creating it, you should set this to 3.
      | @see createDashedStroke
      |
      */
    pub fn create_stroke_with_arrowheads(
        &self, 
        dest_path:              &mut Path,
        source_path:            &Path,
        arrowhead_start_width:  f32,
        arrowhead_start_length: f32,
        arrowhead_end_width:    f32,
        arrowhead_end_length:   f32,
        transform:              Option<&AffineTransform>,
        extra_accuracy:         Option<f32>)  {

        let transform      = transform.unwrap_or(&AffineTransform::default());
        let extra_accuracy = extra_accuracy.unwrap_or(1.0);
        
        todo!();
        /*
            PathStrokeHelpers::Arrowhead head;
        head.startWidth = arrowheadStartWidth;
        head.startLength = arrowheadStartLength;
        head.endWidth = arrowheadEndWidth;
        head.endLength = arrowheadEndLength;

        PathStrokeHelpers::createStroke (thickness, jointStyle, endStyle,
                                         destPath, sourcePath, transform, extraAccuracy, &head);
        */
    }
}

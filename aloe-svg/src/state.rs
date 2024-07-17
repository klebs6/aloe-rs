crate::ix!();

pub struct SVGState {
    original_file:  File,
    top_level_xml:  SvgStateXmlPath,
    width:          f32, // default = 512
    height:         f32, // default = 512
    view_boxw:      f32, // default = 0
    view_boxh:      f32, // default = 0
    transform:      AffineTransform,
    css_style_text: String,
}

impl SVGState {
    
    pub fn new(
        top_level: *const XmlElement,
        svg_file:  &File) -> Self {
    
        todo!();
        /*
        : original_file(svgFile),
        : top_level_xml(topLevel, nullptr),

        
        */
    }
    
    pub fn parse_svg_element(&mut self, xml: &SvgStateXmlPath) -> *mut Drawable {
        
        todo!();
        /*
            auto drawable = new DrawableComposite();
            setCommonAttributes (*drawable, xml);

            SVGState newState (*this);

            if (xml->hasAttribute ("transform"))
                newState.addTransform (xml);

            newState.width  = getCoordLength (xml->getStringAttribute ("width",  String (newState.width)),  viewBoxW);
            newState.height = getCoordLength (xml->getStringAttribute ("height", String (newState.height)), viewBoxH);

            if (newState.width  <= 0) newState.width  = 100;
            if (newState.height <= 0) newState.height = 100;

            Point<float> viewboxXY;

            if (xml->hasAttribute ("viewBox"))
            {
                auto viewBoxAtt = xml->getStringAttribute ("viewBox");
                auto viewParams = viewBoxAtt.getCharPointer();
                Point<float> vwh;

                if (parseCoords (viewParams, viewboxXY, true)
                     && parseCoords (viewParams, vwh, true)
                     && vwh.x > 0
                     && vwh.y > 0)
                {
                    newState.viewBoxW = vwh.x;
                    newState.viewBoxH = vwh.y;

                    auto placementFlags = parsePlacementFlags (xml->getStringAttribute ("preserveAspectRatio").trim());

                    if (placementFlags != 0)
                        newState.transform = RectanglePlacement (placementFlags)
                                                .getTransformToFit (Rectangle<float> (viewboxXY.x, viewboxXY.y, vwh.x, vwh.y),
                                                                    Rectangle<float> (newState.width, newState.height))
                                                .followedBy (newState.transform);
                }
            }
            else
            {
                if (viewBoxW == 0.0f)  newState.viewBoxW = newState.width;
                if (viewBoxH == 0.0f)  newState.viewBoxH = newState.height;
            }

            newState.parseSubElements (xml, *drawable);

            drawable->setContentArea ({ viewboxXY.x, viewboxXY.y, newState.viewBoxW, newState.viewBoxH });
            drawable->resetBoundingBoxToContentArea();

            return drawable;
        */
    }
    
    pub fn parse_path_string(&self, 
        path:        &mut Path,
        path_string: &String)  {
        
        todo!();
        /*
            auto d = pathString.getCharPointer().findEndOfWhitespace();

            Point<float> subpathStart, last, last2, p1, p2, p3;
            aloe_wchar currentCommand = 0, previousCommand = 0;
            bool isRelative = true;
            bool carryOn = true;

            while (! d.isEmpty())
            {
                if (CharPointer_ASCII ("MmLlHhVvCcSsQqTtAaZz").indexOf (*d) >= 0)
                {
                    currentCommand = d.getAndAdvance();
                    isRelative = currentCommand >= 'a';
                }

                switch (currentCommand)
                {
                case 'M':
                case 'm':
                case 'L':
                case 'l':
                    if (parseCoordsOrSkip (d, p1, false))
                    {
                        if (isRelative)
                            p1 += last;

                        if (currentCommand == 'M' || currentCommand == 'm')
                        {
                            subpathStart = p1;
                            path.startNewSubPath (p1);
                            currentCommand = 'l';
                        }
                        else
                            path.lineTo (p1);

                        last2 = last = p1;
                    }
                    break;

                case 'H':
                case 'h':
                    if (parseCoord (d, p1.x, false, true))
                    {
                        if (isRelative)
                            p1.x += last.x;

                        path.lineTo (p1.x, last.y);

                        last2.x = last.x;
                        last.x = p1.x;
                    }
                    else
                    {
                        ++d;
                    }
                    break;

                case 'V':
                case 'v':
                    if (parseCoord (d, p1.y, false, false))
                    {
                        if (isRelative)
                            p1.y += last.y;

                        path.lineTo (last.x, p1.y);

                        last2.y = last.y;
                        last.y = p1.y;
                    }
                    else
                    {
                        ++d;
                    }
                    break;

                case 'C':
                case 'c':
                    if (parseCoordsOrSkip (d, p1, false)
                         && parseCoordsOrSkip (d, p2, false)
                         && parseCoordsOrSkip (d, p3, false))
                    {
                        if (isRelative)
                        {
                            p1 += last;
                            p2 += last;
                            p3 += last;
                        }

                        path.cubicTo (p1, p2, p3);

                        last2 = p2;
                        last = p3;
                    }
                    break;

                case 'S':
                case 's':
                    if (parseCoordsOrSkip (d, p1, false)
                         && parseCoordsOrSkip (d, p3, false))
                    {
                        if (isRelative)
                        {
                            p1 += last;
                            p3 += last;
                        }

                        p2 = last;

                        if (CharPointer_ASCII ("CcSs").indexOf (previousCommand) >= 0)
                            p2 += (last - last2);

                        path.cubicTo (p2, p1, p3);

                        last2 = p1;
                        last = p3;
                    }
                    break;

                case 'Q':
                case 'q':
                    if (parseCoordsOrSkip (d, p1, false)
                         && parseCoordsOrSkip (d, p2, false))
                    {
                        if (isRelative)
                        {
                            p1 += last;
                            p2 += last;
                        }

                        path.quadraticTo (p1, p2);

                        last2 = p1;
                        last = p2;
                    }
                    break;

                case 'T':
                case 't':
                    if (parseCoordsOrSkip (d, p1, false))
                    {
                        if (isRelative)
                            p1 += last;

                        p2 = last;

                        if (CharPointer_ASCII ("QqTt").indexOf (previousCommand) >= 0)
                            p2 += (last - last2);

                        path.quadraticTo (p2, p1);

                        last2 = p2;
                        last = p1;
                    }
                    break;

                case 'A':
                case 'a':
                    if (parseCoordsOrSkip (d, p1, false))
                    {
                        String num;
                        bool flagValue = false;

                        if (parseNextNumber (d, num, false))
                        {
                            auto angle = degreesToRadians (parseSafeFloat (num));

                            if (parseNextFlag (d, flagValue))
                            {
                                auto largeArc = flagValue;

                                if (parseNextFlag (d, flagValue))
                                {
                                    auto sweep = flagValue;

                                    if (parseCoordsOrSkip (d, p2, false))
                                    {
                                        if (isRelative)
                                            p2 += last;

                                        if (last != p2)
                                        {
                                            double centreX, centreY, startAngle, deltaAngle;
                                            double rx = p1.x, ry = p1.y;

                                            endpointToCentreParameters (last.x, last.y, p2.x, p2.y,
                                                                        angle, largeArc, sweep,
                                                                        rx, ry, centreX, centreY,
                                                                        startAngle, deltaAngle);

                                            path.addCentredArc ((float) centreX, (float) centreY,
                                                                (float) rx, (float) ry,
                                                                angle, (float) startAngle, (float) (startAngle + deltaAngle),
                                                                false);

                                            path.lineTo (p2);
                                        }

                                        last2 = last;
                                        last = p2;
                                    }
                                }
                            }
                        }
                    }

                    break;

                case 'Z':
                case 'z':
                    path.closeSubPath();
                    last = last2 = subpathStart;
                    d.incrementToEndOfWhitespace();
                    currentCommand = 'M';
                    break;

                default:
                    carryOn = false;
                    break;
                }

                if (! carryOn)
                    break;

                previousCommand = currentCommand;
            }

            // paths that finish back at their start position often seem to be
            // left without a 'z', so need to be closed explicitly..
            if (path.getCurrentPosition() == subpathStart)
                path.closeSubPath();
        */
    }
    
    pub fn is_none(s: &String) -> bool {
        
        todo!();
        /*
            return s.equalsIgnoreCase ("none");
        */
    }
    
    pub fn set_common_attributes(
        d:   &mut Drawable,
        xml: &SvgStateXmlPath)  {
        
        todo!();
        /*
            auto compID = xml->getStringAttribute ("id");
            d.setName (compID);
            d.setComponentID (compID);

            if (isNone (xml->getStringAttribute ("display")))
                d.setVisible (false);
        */
    }
    
    pub fn parse_sub_elements(
        &mut self, 
        xml:               &SvgStateXmlPath,
        parent_drawable:   &mut DrawableComposite,
        should_parse_clip: Option<bool>

    )  {

        let should_parse_clip: bool = should_parse_clip.unwrap_or(true);

        todo!();
        /*
            for (auto* e : xml->getChildIterator())
            {
                const SvgStateXmlPath child (xml.getChild (e));

                if (auto* drawable = parseSubElement (child))
                {
                    parentDrawable.addChildComponent (drawable);

                    if (! isNone (getStyleAttribute (child, "display")))
                        drawable->setVisible (true);

                    if (shouldParseClip)
                        parseClipPath (child, *drawable);
                }
            }
        */
    }
    
    pub fn parse_sub_element(&mut self, xml: &SvgStateXmlPath) -> *mut Drawable {
        
        todo!();
        /*
            {
                Path path;
                if (parsePathElement (xml, path))
                    return parseShape (xml, path);
            }

            auto tag = xml->getTagNameWithoutNamespace();

            if (tag == "g")         return parseGroupElement (xml, true);
            if (tag == "svg")       return parseSVGElement (xml);
            if (tag == "text")      return parseText (xml, true);
            if (tag == "image")     return parseImage (xml, true);
            if (tag == "switch")    return parseSwitch (xml);
            if (tag == "a")         return parseLinkElement (xml);
            if (tag == "use")       return parseUseOther (xml);
            if (tag == "style")     parseCSSStyle (xml);
            if (tag == "defs")      parseDefs (xml);

            return nullptr;
        */
    }
    
    pub fn parse_path_element(&self, 
        xml:  &SvgStateXmlPath,
        path: &mut Path) -> bool {
        
        todo!();
        /*
            auto tag = xml->getTagNameWithoutNamespace();

            if (tag == "path")      { parsePath (xml, path);           return true; }
            if (tag == "rect")      { parseRect (xml, path);           return true; }
            if (tag == "circle")    { parseCircle (xml, path);         return true; }
            if (tag == "ellipse")   { parseEllipse (xml, path);        return true; }
            if (tag == "line")      { parseLine (xml, path);           return true; }
            if (tag == "polyline")  { parsePolygon (xml, true, path);  return true; }
            if (tag == "polygon")   { parsePolygon (xml, false, path); return true; }
            if (tag == "use")       { return parseUsePath (xml, path); }

            return false;
        */
    }
    
    pub fn parse_switch(&mut self, xml: &SvgStateXmlPath) -> *mut DrawableComposite {
        
        todo!();
        /*
            if (auto* group = xml->getChildByName ("g"))
                return parseGroupElement (xml.getChild (group), true);

            return nullptr;
        */
    }
    
    pub fn parse_group_element(&mut self, 
        xml:                    &SvgStateXmlPath,
        should_parse_transform: bool) -> *mut DrawableComposite {
        
        todo!();
        /*
            if (shouldParseTransform && xml->hasAttribute ("transform"))
            {
                SVGState newState (*this);
                newState.addTransform (xml);

                return newState.parseGroupElement (xml, false);
            }

            auto* drawable = new DrawableComposite();
            setCommonAttributes (*drawable, xml);
            parseSubElements (xml, *drawable);

            drawable->resetContentAreaAndBoundingBoxToFitChildren();
            return drawable;
        */
    }
    
    pub fn parse_link_element(&mut self, xml: &SvgStateXmlPath) -> *mut DrawableComposite {
        
        todo!();
        /*
            return parseGroupElement (xml, true); // TODO: support for making this clickable
        */
    }
    
    pub fn parse_path(&self, 
        xml:  &SvgStateXmlPath,
        path: &mut Path)  {
        
        todo!();
        /*
            parsePathString (path, xml->getStringAttribute ("d"));

            if (getStyleAttribute (xml, "fill-rule").trim().equalsIgnoreCase ("evenodd"))
                path.setUsingNonZeroWinding (false);
        */
    }
    
    pub fn parse_rect(&self, 
        xml:  &SvgStateXmlPath,
        rect: &mut Path)  {
        
        todo!();
        /*
            const bool hasRX = xml->hasAttribute ("rx");
            const bool hasRY = xml->hasAttribute ("ry");

            if (hasRX || hasRY)
            {
                float rx = getCoordLength (xml, "rx", viewBoxW);
                float ry = getCoordLength (xml, "ry", viewBoxH);

                if (! hasRX)
                    rx = ry;
                else if (! hasRY)
                    ry = rx;

                rect.addRoundedRectangle (getCoordLength (xml, "x", viewBoxW),
                                          getCoordLength (xml, "y", viewBoxH),
                                          getCoordLength (xml, "width", viewBoxW),
                                          getCoordLength (xml, "height", viewBoxH),
                                          rx, ry);
            }
            else
            {
                rect.addRectangle (getCoordLength (xml, "x", viewBoxW),
                                   getCoordLength (xml, "y", viewBoxH),
                                   getCoordLength (xml, "width", viewBoxW),
                                   getCoordLength (xml, "height", viewBoxH));
            }
        */
    }
    
    pub fn parse_circle(&self, 
        xml:    &SvgStateXmlPath,
        circle: &mut Path)  {
        
        todo!();
        /*
            auto cx = getCoordLength (xml, "cx", viewBoxW);
            auto cy = getCoordLength (xml, "cy", viewBoxH);
            auto radius = getCoordLength (xml, "r", viewBoxW);

            circle.addEllipse (cx - radius, cy - radius, radius * 2.0f, radius * 2.0f);
        */
    }
    
    pub fn parse_ellipse(&self, 
        xml:     &SvgStateXmlPath,
        ellipse: &mut Path)  {
        
        todo!();
        /*
            auto cx      = getCoordLength (xml, "cx", viewBoxW);
            auto cy      = getCoordLength (xml, "cy", viewBoxH);
            auto radiusX = getCoordLength (xml, "rx", viewBoxW);
            auto radiusY = getCoordLength (xml, "ry", viewBoxH);

            ellipse.addEllipse (cx - radiusX, cy - radiusY, radiusX * 2.0f, radiusY * 2.0f);
        */
    }
    
    pub fn parse_line(&self, 
        xml:  &SvgStateXmlPath,
        line: &mut Path)  {
        
        todo!();
        /*
            auto x1 = getCoordLength (xml, "x1", viewBoxW);
            auto y1 = getCoordLength (xml, "y1", viewBoxH);
            auto x2 = getCoordLength (xml, "x2", viewBoxW);
            auto y2 = getCoordLength (xml, "y2", viewBoxH);

            line.startNewSubPath (x1, y1);
            line.lineTo (x2, y2);
        */
    }
    
    pub fn parse_polygon(&self, 
        xml:         &SvgStateXmlPath,
        is_polyline: bool,
        path:        &mut Path)  {
        
        todo!();
        /*
            auto pointsAtt = xml->getStringAttribute ("points");
            auto points = pointsAtt.getCharPointer();
            Point<float> p;

            if (parseCoords (points, p, true))
            {
                Point<float> first (p), last;

                path.startNewSubPath (first);

                while (parseCoords (points, p, true))
                {
                    last = p;
                    path.lineTo (p);
                }

                if ((! isPolyline) || first == last)
                    path.closeSubPath();
            }
        */
    }
    
    pub fn get_linkedid(xml: &SvgStateXmlPath) -> String {
        
        todo!();
        /*
            auto link = xml->getStringAttribute ("xlink:href");

            if (link.startsWithChar ('#'))
                return link.substring (1);

            return {};
        */
    }
    
    pub fn parse_use_path(&self, 
        xml:  &SvgStateXmlPath,
        path: &mut Path) -> bool {
        
        todo!();
        /*
            auto linkedID = getLinkedID (xml);

            if (linkedID.isNotEmpty())
            {
                SvgStateUsePathOp op = { this, &path };
                return topLevelXml.applyOperationToChildWithID (linkedID, op);
            }

            return false;
        */
    }
    
    pub fn parse_use_other(&self, xml: &SvgStateXmlPath) -> *mut Drawable {
        
        todo!();
        /*
            if (auto* drawableText  = parseText (xml, false))    return drawableText;
            if (auto* drawableImage = parseImage (xml, false))   return drawableImage;

            return nullptr;
        */
    }
    
    pub fn parseurl(str_: &String) -> String {
        
        todo!();
        /*
            if (str.startsWithIgnoreCase ("url"))
                return str.fromFirstOccurrenceOf ("#", false, false)
                          .upToLastOccurrenceOf (")", false, false).trim();

            return {};
        */
    }
    
    pub fn parse_shape(
        &self, 
        xml:                    &SvgStateXmlPath,
        path:                   &mut Path,
        should_parse_transform: Option<bool>,
        additonal_transform:    *mut AffineTransform

    ) -> *mut Drawable {

        let should_parse_transform: bool = should_parse_transform.unwrap_or(true);

        todo!();
        /*
            if (shouldParseTransform && xml->hasAttribute ("transform"))
            {
                SVGState newState (*this);
                newState.addTransform (xml);

                return newState.parseShape (xml, path, false, additonalTransform);
            }

            auto dp = new DrawablePath();
            setCommonAttributes (*dp, xml);
            dp->setFill (Colours::transparentBlack);

            path.applyTransform (transform);

            if (additonalTransform != nullptr)
                path.applyTransform (*additonalTransform);

            dp->setPath (path);

            dp->setFill (getPathFillType (path, xml, "fill",
                                          getStyleAttribute (xml, "fill-opacity"),
                                          getStyleAttribute (xml, "opacity"),
                                          pathContainsClosedSubPath (path) ? Colours::black
                                                                           : Colours::transparentBlack));

            auto strokeType = getStyleAttribute (xml, "stroke");

            if (strokeType.isNotEmpty() && ! isNone (strokeType))
            {
                dp->setStrokeFill (getPathFillType (path, xml, "stroke",
                                                    getStyleAttribute (xml, "stroke-opacity"),
                                                    getStyleAttribute (xml, "opacity"),
                                                    Colours::transparentBlack));

                dp->setStrokeType (getStrokeFor (xml));
            }

            auto strokeDashArray = getStyleAttribute (xml, "stroke-dasharray");

            if (strokeDashArray.isNotEmpty())
                parseDashArray (strokeDashArray, *dp);

            return dp;
        */
    }
    
    pub fn path_contains_closed_sub_path(path: &Path) -> bool {
        
        todo!();
        /*
            for (Path::Iterator iter (path); iter.next();)
                if (iter.elementType == Path::Iterator::closePath)
                    return true;

            return false;
        */
    }
    
    pub fn parse_dash_array(
        &self, 
        dash_list: &String,
        dp:        &mut DrawablePath

    )  {
        
        todo!();
        /*
            if (dashList.equalsIgnoreCase ("null") || isNone (dashList))
                return;

            Vec<float> dashLengths;

            for (auto t = dashList.getCharPointer();;)
            {
                float value;
                if (! parseCoord (t, value, true, true))
                    break;

                dashLengths.add (value);

                t.incrementToEndOfWhitespace();

                if (*t == ',')
                    ++t;
            }

            if (dashLengths.size() > 0)
            {
                auto* dashes = dashLengths.getRawDataPointer();

                for (int i = 0; i < dashLengths.size(); ++i)
                {
                    if (dashes[i] <= 0)  // SVG uses zero-length dashes to mean a dotted line
                    {
                        if (dashLengths.size() == 1)
                            return;

                        const float nonZeroLength = 0.001f;
                        dashes[i] = nonZeroLength;

                        const int pairedIndex = i ^ 1;

                        if (isPositiveAndBelow (pairedIndex, dashLengths.size())
                              && dashes[pairedIndex] > nonZeroLength)
                            dashes[pairedIndex] -= nonZeroLength;
                    }
                }

                dp.setDashLengths (dashLengths);
            }
        */
    }
    
    pub fn parse_clip_path(
        &mut self, 
        xml: &SvgStateXmlPath,
        d:   &mut Drawable

    ) -> bool {
        
        todo!();
        /*
            const String clipPath (getStyleAttribute (xml, "clip-path"));

            if (clipPath.isNotEmpty())
            {
                auto urlID = parseURL (clipPath);

                if (urlID.isNotEmpty())
                {
                    SvgStateGetClipPathOp op = { this, &d };
                    return topLevelXml.applyOperationToChildWithID (urlID, op);
                }
            }

            return false;
        */
    }
    
    pub fn apply_clip_path(
        &mut self, 
        target:   &mut Drawable,
        xml_path: &SvgStateXmlPath

    ) -> bool {
        
        todo!();
        /*
            if (xmlPath->hasTagNameIgnoringNamespace ("clipPath"))
            {
                std::unique_ptr<DrawableComposite> drawableClipPath (new DrawableComposite());

                parseSubElements (xmlPath, *drawableClipPath, false);

                if (drawableClipPath->getNumChildComponents() > 0)
                {
                    setCommonAttributes (*drawableClipPath, xmlPath);
                    target.setClipPath (std::move (drawableClipPath));
                    return true;
                }
            }

            return false;
        */
    }
    
    pub fn add_gradient_stops_in(
        &self, 
        cg:       &mut ColourGradient,
        fill_xml: &SvgStateXmlPath

    ) -> bool {
        
        todo!();
        /*
            bool result = false;

            if (fillXml.xml != nullptr)
            {
                for (auto* e : fillXml->getChildWithTagNameIterator ("stop"))
                {
                    auto col = parseColour (fillXml.getChild (e), "stop-color", Colours::black);

                    auto opacity = getStyleAttribute (fillXml.getChild (e), "stop-opacity", "1");
                    col = col.withMultipliedAlpha (jlimit (0.0f, 1.0f, parseSafeFloat (opacity)));

                    auto offset = parseSafeFloat (e->getStringAttribute ("offset"));

                    if (e->getStringAttribute ("offset").containsChar ('%'))
                        offset *= 0.01f;

                    cg.addColour (jlimit (0.0f, 1.0f, offset), col);
                    result = true;
                }
            }

            return result;
        */
    }
    
    pub fn get_gradient_fill_type(
        &self, 
        fill_xml: &SvgStateXmlPath,
        path:     &Path,
        opacity:  f32

    ) -> FillType {
        
        todo!();
        /*
            ColourGradient gradient;

            {
                auto linkedID = getLinkedID (fillXml);

                if (linkedID.isNotEmpty())
                {
                    SvgStateSetGradientStopsOp op = { this, &gradient, };
                    topLevelXml.applyOperationToChildWithID (linkedID, op);
                }
            }

            addGradientStopsIn (gradient, fillXml);

            if (int numColours = gradient.getNumColours())
            {
                if (gradient.getColourPosition (0) > 0)
                    gradient.addColour (0.0, gradient.getColour (0));

                if (gradient.getColourPosition (numColours - 1) < 1.0)
                    gradient.addColour (1.0, gradient.getColour (numColours - 1));
            }
            else
            {
                gradient.addColour (0.0, Colours::black);
                gradient.addColour (1.0, Colours::black);
            }

            if (opacity < 1.0f)
                gradient.multiplyOpacity (opacity);

            jassert (gradient.getNumColours() > 0);

            gradient.isRadial = fillXml->hasTagNameIgnoringNamespace ("radialGradient");

            float gradientWidth = viewBoxW;
            float gradientHeight = viewBoxH;
            float dx = 0.0f;
            float dy = 0.0f;

            const bool userSpace = fillXml->getStringAttribute ("gradientUnits").equalsIgnoreCase ("userSpaceOnUse");

            if (! userSpace)
            {
                auto bounds = path.getBounds();
                dx = bounds.getX();
                dy = bounds.getY();
                gradientWidth = bounds.getWidth();
                gradientHeight = bounds.getHeight();
            }

            if (gradient.isRadial)
            {
                if (userSpace)
                    gradient.point1.setXY (dx + getCoordLength (fillXml->getStringAttribute ("cx", "50%"), gradientWidth),
                                           dy + getCoordLength (fillXml->getStringAttribute ("cy", "50%"), gradientHeight));
                else
                    gradient.point1.setXY (dx + gradientWidth  * getCoordLength (fillXml->getStringAttribute ("cx", "50%"), 1.0f),
                                           dy + gradientHeight * getCoordLength (fillXml->getStringAttribute ("cy", "50%"), 1.0f));

                auto radius = getCoordLength (fillXml->getStringAttribute ("r", "50%"), gradientWidth);
                gradient.point2 = gradient.point1 + Point<float> (radius, 0.0f);

                //xxx (the fx, fy focal point isn't handled properly here..)
            }
            else
            {
                if (userSpace)
                {
                    gradient.point1.setXY (dx + getCoordLength (fillXml->getStringAttribute ("x1", "0%"), gradientWidth),
                                           dy + getCoordLength (fillXml->getStringAttribute ("y1", "0%"), gradientHeight));

                    gradient.point2.setXY (dx + getCoordLength (fillXml->getStringAttribute ("x2", "100%"), gradientWidth),
                                           dy + getCoordLength (fillXml->getStringAttribute ("y2", "0%"), gradientHeight));
                }
                else
                {
                    gradient.point1.setXY (dx + gradientWidth  * getCoordLength (fillXml->getStringAttribute ("x1", "0%"), 1.0f),
                                           dy + gradientHeight * getCoordLength (fillXml->getStringAttribute ("y1", "0%"), 1.0f));

                    gradient.point2.setXY (dx + gradientWidth  * getCoordLength (fillXml->getStringAttribute ("x2", "100%"), 1.0f),
                                           dy + gradientHeight * getCoordLength (fillXml->getStringAttribute ("y2", "0%"), 1.0f));
                }

                if (gradient.point1 == gradient.point2)
                    return Colour (gradient.getColour (gradient.getNumColours() - 1));
            }

            FillType type (gradient);

            auto gradientTransform = parseTransform (fillXml->getStringAttribute ("gradientTransform"));

            if (gradient.isRadial)
            {
                type.transform = gradientTransform;
            }
            else
            {
                // Transform the perpendicular vector into the new coordinate space for the gradient.
                // This vector is now the slope of the linear gradient as it should appear in the new coord space
                auto perpendicular = Point<float> (gradient.point2.y - gradient.point1.y,
                                                   gradient.point1.x - gradient.point2.x)
                                        .transformedBy (gradientTransform.withAbsoluteTranslation (0, 0));

                auto newGradPoint1 = gradient.point1.transformedBy (gradientTransform);
                auto newGradPoint2 = gradient.point2.transformedBy (gradientTransform);

                // Project the transformed gradient vector onto the transformed slope of the linear
                // gradient as it should appear in the new coordinate space
                const float scale = perpendicular.getDotProduct (newGradPoint2 - newGradPoint1)
                                      / perpendicular.getDotProduct (perpendicular);

                type.gradient->point1 = newGradPoint1;
                type.gradient->point2 = newGradPoint2 - perpendicular * scale;
            }

            return type;
        */
    }
    
    pub fn get_path_fill_type(
        &self, 
        path:            &Path,
        xml:             &SvgStateXmlPath,
        fill_attribute:  &str,
        fill_opacity:    &String,
        overall_opacity: &String,
        default_colour:  Colour

    ) -> FillType {
        
        todo!();
        /*
            float opacity = 1.0f;

            if (overallOpacity.isNotEmpty())
                opacity = jlimit (0.0f, 1.0f, parseSafeFloat (overallOpacity));

            if (fillOpacity.isNotEmpty())
                opacity *= jlimit (0.0f, 1.0f, parseSafeFloat (fillOpacity));

            String fill (getStyleAttribute (xml, fillAttribute));
            String urlID = parseURL (fill);

            if (urlID.isNotEmpty())
            {
                SvgStateGetFillTypeOp op = { this, &path, opacity, FillType() };

                if (topLevelXml.applyOperationToChildWithID (urlID, op))
                    return op.fillType;
            }

            if (isNone (fill))
                return Colours::transparentBlack;

            return parseColour (xml, fillAttribute, defaultColour).withMultipliedAlpha (opacity);
        */
    }
    
    pub fn get_joint_style(join: &String) -> path_stroke_type::JointStyle {
        
        todo!();
        /*
            if (join.equalsIgnoreCase ("round"))  return PathStrokeType::curved;
            if (join.equalsIgnoreCase ("bevel"))  return PathStrokeType::beveled;

            return PathStrokeType::mitered;
        */
    }
    
    pub fn get_end_cap_style(cap: &String) -> path_stroke_type::EndCapStyle {
        
        todo!();
        /*
            if (cap.equalsIgnoreCase ("round"))   return PathStrokeType::rounded;
            if (cap.equalsIgnoreCase ("square"))  return PathStrokeType::square;

            return PathStrokeType::butt;
        */
    }
    
    pub fn get_stroke_width(&self, stroke_width: &String) -> f32 {
        
        todo!();
        /*
            auto transformScale = std::sqrt (std::abs (transform.getDeterminant()));
            return transformScale * getCoordLength (strokeWidth, viewBoxW);
        */
    }
    
    pub fn get_stroke_for(&self, xml: &SvgStateXmlPath) -> PathStrokeType {
        
        todo!();
        /*
            return PathStrokeType (getStrokeWidth (getStyleAttribute (xml, "stroke-width", "1")),
                                   getJointStyle  (getStyleAttribute (xml, "stroke-linejoin")),
                                   getEndCapStyle (getStyleAttribute (xml, "stroke-linecap")));
        */
    }
    
    pub fn use_text(&self, xml: &SvgStateXmlPath) -> *mut Drawable {
        
        todo!();
        /*
            auto translation = AffineTransform::translation (parseSafeFloat (xml->getStringAttribute ("x")),
                                                             parseSafeFloat (xml->getStringAttribute ("y")));

            SvgStateUseTextOp op = { this, &translation, nullptr };

            auto linkedID = getLinkedID (xml);

            if (linkedID.isNotEmpty())
                topLevelXml.applyOperationToChildWithID (linkedID, op);

            return op.target;
        */
    }
    
    pub fn parse_text(
        &self, 
        xml:                    &SvgStateXmlPath,
        should_parse_transform: bool,
        additonal_transform:    *mut AffineTransform

    ) -> *mut Drawable {

        todo!();
        /*
            if (shouldParseTransform && xml->hasAttribute ("transform"))
            {
                SVGState newState (*this);
                newState.addTransform (xml);

                return newState.parseText (xml, false, additonalTransform);
            }

            if (xml->hasTagName ("use"))
                return useText (xml);

            if (! xml->hasTagName ("text") && ! xml->hasTagNameIgnoringNamespace ("tspan"))
                return nullptr;

            Vec<float> xCoords, yCoords, dxCoords, dyCoords;

            getCoordList (xCoords,  getInheritedAttribute (xml, "x"),  true, true);
            getCoordList (yCoords,  getInheritedAttribute (xml, "y"),  true, false);
            getCoordList (dxCoords, getInheritedAttribute (xml, "dx"), true, true);
            getCoordList (dyCoords, getInheritedAttribute (xml, "dy"), true, false);

            auto font = getFont (xml);
            auto anchorStr = getStyleAttribute (xml, "text-anchor");

            auto dc = new DrawableComposite();
            setCommonAttributes (*dc, xml);

            for (auto* e : xml->getChildIterator())
            {
                if (e->isTextElement())
                {
                    auto text = e->getText().trim();

                    auto dt = new DrawableText();
                    dc->addAndMakeVisible (dt);

                    dt->setText (text);
                    dt->setFont (font, true);

                    if (additonalTransform != nullptr)
                        dt->setTransform (transform.followedBy (*additonalTransform));
                    else
                        dt->setTransform (transform);

                    dt->setColour (parseColour (xml, "fill", Colours::black)
                                     .withMultipliedAlpha (parseSafeFloat (getStyleAttribute (xml, "fill-opacity", "1"))));

                    Rectangle<float> bounds (xCoords[0], yCoords[0] - font.getAscent(),
                                             font.getStringWidthFloat (text), font.getHeight());

                    if (anchorStr == "middle")   bounds.setX (bounds.getX() - bounds.getWidth() / 2.0f);
                    else if (anchorStr == "end") bounds.setX (bounds.getX() - bounds.getWidth());

                    dt->setBoundingBox (bounds);
                }
                else if (e->hasTagNameIgnoringNamespace ("tspan"))
                {
                    dc->addAndMakeVisible (parseText (xml.getChild (e), true));
                }
            }

            return dc;
        */
    }
    
    pub fn get_font(&self, xml: &SvgStateXmlPath) -> Font {
        
        todo!();
        /*
            Font f;
            auto family = getStyleAttribute (xml, "font-family").unquoted();

            if (family.isNotEmpty())
                f.setTypefaceName (family);

            if (getStyleAttribute (xml, "font-style").containsIgnoreCase ("italic"))
                f.setItalic (true);

            if (getStyleAttribute (xml, "font-weight").containsIgnoreCase ("bold"))
                f.setBold (true);

            return f.withPointHeight (getCoordLength (getStyleAttribute (xml, "font-size", "15"), 1.0f));
        */
    }
    
    pub fn use_image(&self, xml: &SvgStateXmlPath) -> *mut Drawable {
        
        todo!();
        /*
            auto translation = AffineTransform::translation (parseSafeFloat (xml->getStringAttribute ("x")),
                                                             parseSafeFloat (xml->getStringAttribute ("y")));

            SvgStateUseImageOp op = { this, &translation, nullptr };

            auto linkedID = getLinkedID (xml);

            if (linkedID.isNotEmpty())
                topLevelXml.applyOperationToChildWithID (linkedID, op);

            return op.target;
        */
    }
    
    pub fn parse_image(
        &self, 
        xml:                    &SvgStateXmlPath,
        should_parse_transform: bool,
        additional_transform:   *mut AffineTransform

    ) -> *mut Drawable {

        todo!();
        /*
            if (shouldParseTransform && xml->hasAttribute ("transform"))
            {
                SVGState newState (*this);
                newState.addTransform (xml);

                return newState.parseImage (xml, false, additionalTransform);
            }

            if (xml->hasTagName ("use"))
                return useImage (xml);

            if (! xml->hasTagName ("image"))
                return nullptr;

            auto link = xml->getStringAttribute ("xlink:href");

            std::unique_ptr<InputStream> inputStream;
            MemoryOutputStream imageStream;

            if (link.startsWith ("data:"))
            {
                const auto indexOfComma = link.indexOf (",");
                auto format = link.substring (5, indexOfComma).trim();
                auto indexOfSemi = format.indexOf (";");

                if (format.substring (indexOfSemi + 1).trim().equalsIgnoreCase ("base64"))
                {
                    auto mime = format.substring (0, indexOfSemi).trim();

                    if (mime.equalsIgnoreCase ("image/png") || mime.equalsIgnoreCase ("image/jpeg"))
                    {
                        auto base64text = link.substring (indexOfComma + 1).removeCharacters ("\t\n\r ");

                        if (Base64::convertFromBase64 (imageStream, base64text))
                            inputStream.reset (new MemoryInputStream (imageStream.getData(), imageStream.getDataSize(), false));
                    }
                }
            }
            else
            {
                auto linkedFile = originalFile.getParentDirectory().getChildFile (link);

                if (linkedFile.existsAsFile())
                    inputStream = linkedFile.createInputStream();
            }

            if (inputStream != nullptr)
            {
                auto image = ImageFileFormat::loadFrom (*inputStream);

                if (image.isValid())
                {
                    auto* di = new DrawableImage();

                    setCommonAttributes (*di, xml);

                    Rectangle<float> imageBounds (parseSafeFloat (xml->getStringAttribute ("x")),
                                                  parseSafeFloat (xml->getStringAttribute ("y")),
                                                  parseSafeFloat (xml->getStringAttribute ("width",  String (image.getWidth()))),
                                                  parseSafeFloat (xml->getStringAttribute ("height", String (image.getHeight()))));

                    di->setImage (image.rescaled ((int) imageBounds.getWidth(),
                                                  (int) imageBounds.getHeight()));

                    di->setTransformToFit (imageBounds, RectanglePlacement (parsePlacementFlags (xml->getStringAttribute ("preserveAspectRatio").trim())));

                    if (additionalTransform != nullptr)
                        di->setTransform (di->getTransform().followedBy (transform).followedBy (*additionalTransform));
                    else
                        di->setTransform (di->getTransform().followedBy (transform));

                    return di;
                }
            }

            return nullptr;
        */
    }
    
    pub fn add_transform(&mut self, xml: &SvgStateXmlPath)  {
        
        todo!();
        /*
            transform = parseTransform (xml->getStringAttribute ("transform"))
                            .followedBy (transform);
        */
    }
    
    pub fn parse_coord(
        &self, 
        s:           &mut CharPointerType,
        value:       &mut f32,
        allow_units: bool,
        isx:         bool

    ) -> bool {
        
        todo!();
        /*
            String number;

            if (! parseNextNumber (s, number, allowUnits))
            {
                value = 0;
                return false;
            }

            value = getCoordLength (number, isX ? viewBoxW : viewBoxH);
            return true;
        */
    }
    
    pub fn parse_coords(
        &self, 
        s:           &mut CharPointerType,
        p:           &mut Point<f32>,
        allow_units: bool

    ) -> bool {
        
        todo!();
        /*
            return parseCoord (s, p.x, allowUnits, true)
                && parseCoord (s, p.y, allowUnits, false);
        */
    }
    
    pub fn parse_coords_or_skip(
        &self, 
        s:           &mut CharPointerType,
        p:           &mut Point<f32>,
        allow_units: bool

    ) -> bool {
        
        todo!();
        /*
            if (parseCoords (s, p, allowUnits))
                return true;

            if (! s.isEmpty()) ++s;
            return false;
        */
    }
    
    pub fn get_coord_length(
        &self, 
        s:                    &String,
        size_for_proportions: f32

    ) -> f32 {
        
        todo!();
        /*
            auto n = parseSafeFloat (s);
            auto len = s.length();

            if (len > 2)
            {
                auto dpi = 96.0f;

                auto n1 = s[len - 2];
                auto n2 = s[len - 1];

                if (n1 == 'i' && n2 == 'n')         n *= dpi;
                else if (n1 == 'm' && n2 == 'm')    n *= dpi / 25.4f;
                else if (n1 == 'c' && n2 == 'm')    n *= dpi / 2.54f;
                else if (n1 == 'p' && n2 == 'c')    n *= 15.0f;
                else if (n2 == '%')                 n *= 0.01f * sizeForProportions;
            }

            return n;
        */
    }
    
    pub fn get_coord_length_with_svg(
        &self, 
        xml:                  &SvgStateXmlPath,
        att_name:             *const u8,
        size_for_proportions: f32

    ) -> f32 {
        
        todo!();
        /*
            return getCoordLength (xml->getStringAttribute (attName), sizeForProportions);
        */
    }
    
    pub fn get_coord_list(
        &self, 
        coords:      &mut Vec<f32>,
        list:        &String,
        allow_units: bool,
        isx:         bool

    )  {
        
        todo!();
        /*
            auto text = list.getCharPointer();
            float value;

            while (parseCoord (text, value, allowUnits, isX))
                coords.add (value);
        */
    }
    
    pub fn parse_safe_float(s: &String) -> f32 {
        
        todo!();
        /*
            auto n = s.getFloatValue();
            return (std::isnan (n) || std::isinf (n)) ? 0.0f : n;
        */
    }
    
    pub fn parse_css_style(&mut self, xml: &SvgStateXmlPath)  {
        
        todo!();
        /*
            cssStyleText = xml->getAllSubText() + "\n" + cssStyleText;
        */
    }
    
    pub fn parse_defs(&mut self, xml: &SvgStateXmlPath)  {
        
        todo!();
        /*
            if (auto* style = xml->getChildByName ("style"))
                parseCSSStyle (xml.getChild (style));
        */
    }
    
    pub fn find_style_item(
        source: CharPointerType,
        name:   CharPointerType) -> CharPointerType {
        
        todo!();
        /*
            auto nameLength = (int) name.length();

            while (! source.isEmpty())
            {
                if (source.getAndAdvance() == '.'
                     && CharacterFunctions::compareIgnoreCaseUpTo (source, name, nameLength) == 0)
                {
                    auto endOfName = (source + nameLength).findEndOfWhitespace();

                    if (*endOfName == '{')
                        return endOfName;

                    if (*endOfName == ',')
                        return CharacterFunctions::find (endOfName, (aloe_wchar) '{');
                }
            }

            return source;
        */
    }
    
    pub fn get_style_attribute(
        &self, 
        xml:            &SvgStateXmlPath,
        attribute_name: &str,
        default_value:  Option<&str>

    ) -> String {

        let default_value = default_value.unwrap_or("");

        todo!();
        /*
            if (xml->hasAttribute (attributeName))
                return xml->getStringAttribute (attributeName, defaultValue);

            auto styleAtt = xml->getStringAttribute ("style");

            if (styleAtt.isNotEmpty())
            {
                auto value = getAttributeFromStyleList (styleAtt, attributeName, {});

                if (value.isNotEmpty())
                    return value;
            }
            else if (xml->hasAttribute ("class"))
            {
                for (auto i = cssStyleText.getCharPointer();;)
                {
                    auto openBrace = findStyleItem (i, xml->getStringAttribute ("class").getCharPointer());

                    if (openBrace.isEmpty())
                        break;

                    auto closeBrace = CharacterFunctions::find (openBrace, (aloe_wchar) '}');

                    if (closeBrace.isEmpty())
                        break;

                    auto value = getAttributeFromStyleList (String (openBrace + 1, closeBrace),
                                                            attributeName, defaultValue);
                    if (value.isNotEmpty())
                        return value;

                    i = closeBrace + 1;
                }
            }

            if (xml.parent != nullptr)
                return getStyleAttribute (*xml.parent, attributeName, defaultValue);

            return defaultValue;
        */
    }
    
    pub fn get_inherited_attribute(
        &self, 
        xml:            &SvgStateXmlPath,
        attribute_name: &str

    ) -> String {
        
        todo!();
        /*
            if (xml->hasAttribute (attributeName))
                return xml->getStringAttribute (attributeName);

            if (xml.parent != nullptr)
                return getInheritedAttribute (*xml.parent, attributeName);

            return {};
        */
    }
    
    pub fn parse_placement_flags(align: &String) -> i32 {
        
        todo!();
        /*
            if (align.isEmpty())
                return 0;

            if (isNone (align))
                return RectanglePlacement::stretchToFit;

            return (align.containsIgnoreCase ("slice") ? RectanglePlacement::fillDestination : 0)
                 | (align.containsIgnoreCase ("xMin")  ? RectanglePlacement::xLeft
                                                       : (align.containsIgnoreCase ("xMax") ? RectanglePlacement::xRight
                                                                                            : RectanglePlacement::xMid))
                 | (align.containsIgnoreCase ("yMin")  ? RectanglePlacement::yTop
                                                       : (align.containsIgnoreCase ("yMax") ? RectanglePlacement::yBottom
                                                                                            : RectanglePlacement::yMid));
        */
    }
    
    pub fn is_identifier_char(c: wchar_t) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isLetter (c) || c == '-';
        */
    }
    
    pub fn get_attribute_from_style_list(
        list:           &String,
        attribute_name: &str,
        default_value:  &String) -> String {
        
        todo!();
        /*
            int i = 0;

            for (;;)
            {
                i = list.indexOf (i, attributeName);

                if (i < 0)
                    break;

                if ((i == 0 || (i > 0 && ! isIdentifierChar (list [i - 1])))
                     && ! isIdentifierChar (list [i + attributeName.length()]))
                {
                    i = list.indexOfChar (i, ':');

                    if (i < 0)
                        break;

                    int end = list.indexOfChar (i, ';');

                    if (end < 0)
                        end = 0x7ffff;

                    return list.substring (i + 1, end).trim();
                }

                ++i;
            }

            return defaultValue;
        */
    }
    
    pub fn is_start_of_number(c: wchar_t) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isDigit (c) || c == '-' || c == '+';
        */
    }
    
    pub fn parse_next_number(
        text:        &mut CharPointerType,
        value:       &mut String,
        allow_units: bool) -> bool {
        
        todo!();
        /*
            auto s = text;

            while (s.isWhitespace() || *s == ',')
                ++s;

            auto start = s;

            if (isStartOfNumber (*s))
                ++s;

            while (s.isDigit())
                ++s;

            if (*s == '.')
            {
                ++s;

                while (s.isDigit())
                    ++s;
            }

            if ((*s == 'e' || *s == 'E') && isStartOfNumber (s[1]))
            {
                s += 2;

                while (s.isDigit())
                    ++s;
            }

            if (allowUnits)
                while (s.isLetter())
                    ++s;

            if (s == start)
            {
                text = s;
                return false;
            }

            value = String (start, s);

            while (s.isWhitespace() || *s == ',')
                ++s;

            text = s;
            return true;
        */
    }
    
    pub fn parse_next_flag(
        text:  &mut CharPointerType,
        value: &mut bool) -> bool {
        
        todo!();
        /*
            while (text.isWhitespace() || *text == ',')
                ++text;

            if (*text != '0' && *text != '1')
                return false;

            value = *(text++) != '0';

            while (text.isWhitespace() || *text == ',')
                 ++text;

            return true;
        */
    }
    
    pub fn parse_colour(
        &self, 
        xml:            &SvgStateXmlPath,
        attribute_name: &str,
        default_colour: Colour

    ) -> Colour {
        
        todo!();
        /*
            auto text = getStyleAttribute (xml, attributeName);

            if (text.startsWithChar ('#'))
            {
                uint32 hex[8] = { 0 };
                hex[6] = hex[7] = 15;

                int numChars = 0;
                auto s = text.getCharPointer();

                while (numChars < 8)
                {
                    auto hexValue = CharacterFunctions::getHexDigitValue (*++s);

                    if (hexValue >= 0)
                        hex[numChars++] = (uint32) hexValue;
                    else
                        break;
                }

                if (numChars <= 3)
                    return Colour ((uint8) (hex[0] * 0x11),
                                   (uint8) (hex[1] * 0x11),
                                   (uint8) (hex[2] * 0x11));

                return Colour ((uint8) ((hex[0] << 4) + hex[1]),
                               (uint8) ((hex[2] << 4) + hex[3]),
                               (uint8) ((hex[4] << 4) + hex[5]),
                               (uint8) ((hex[6] << 4) + hex[7]));
            }

            if (text.startsWith ("rgb") || text.startsWith ("hsl"))
            {
                auto tokens = [&text]
                {
                    auto openBracket = text.indexOfChar ('(');
                    auto closeBracket = text.indexOfChar (openBracket, ')');

                    StringArray arr;

                    if (openBracket >= 3 && closeBracket > openBracket)
                    {
                        arr.addTokens (text.substring (openBracket + 1, closeBracket), ",", "");
                        arr.trim();
                        arr.removeEmptyStrings();
                    }

                    return arr;
                }();

                auto alpha = [&tokens, &text]
                {
                    if ((text.startsWith ("rgba") || text.startsWith ("hsla")) && tokens.size() == 4)
                        return parseSafeFloat (tokens[3]);

                    return 1.0f;
                }();

                if (text.startsWith ("hsl"))
                    return Colour::fromHSL (parseSafeFloat (tokens[0]) / 360.0f,
                                            parseSafeFloat (tokens[1]) / 100.0f,
                                            parseSafeFloat (tokens[2]) / 100.0f,
                                            alpha);

                if (tokens[0].containsChar ('%'))
                    return Colour ((uint8) roundToInt (2.55f * parseSafeFloat (tokens[0])),
                                   (uint8) roundToInt (2.55f * parseSafeFloat (tokens[1])),
                                   (uint8) roundToInt (2.55f * parseSafeFloat (tokens[2])),
                                   alpha);

                return Colour ((uint8) tokens[0].getIntValue(),
                               (uint8) tokens[1].getIntValue(),
                               (uint8) tokens[2].getIntValue(),
                               alpha);
            }

            if (text == "inherit")
            {
                for (const SvgStateXmlPath* p = xml.parent; p != nullptr; p = p->parent)
                    if (getStyleAttribute (*p, attributeName).isNotEmpty())
                        return parseColour (*p, attributeName, defaultColour);
            }

            return Colours::findColourForName (text, defaultColour);
        */
    }
    
    pub fn parse_transform(t: String) -> AffineTransform {
        
        todo!();
        /*
            AffineTransform result;

            while (t.isNotEmpty())
            {
                StringArray tokens;
                tokens.addTokens (t.fromFirstOccurrenceOf ("(", false, false)
                                   .upToFirstOccurrenceOf (")", false, false),
                                  ", ", "");

                tokens.removeEmptyStrings (true);

                float numbers[6];

                for (int i = 0; i < numElementsInArray (numbers); ++i)
                    numbers[i] = parseSafeFloat (tokens[i]);

                AffineTransform trans;

                if (t.startsWithIgnoreCase ("matrix"))
                {
                    trans = AffineTransform (numbers[0], numbers[2], numbers[4],
                                             numbers[1], numbers[3], numbers[5]);
                }
                else if (t.startsWithIgnoreCase ("translate"))
                {
                    trans = AffineTransform::translation (numbers[0], numbers[1]);
                }
                else if (t.startsWithIgnoreCase ("scale"))
                {
                    trans = AffineTransform::scale (numbers[0], numbers[tokens.size() > 1 ? 1 : 0]);
                }
                else if (t.startsWithIgnoreCase ("rotate"))
                {
                    trans = AffineTransform::rotation (degreesToRadians (numbers[0]), numbers[1], numbers[2]);
                }
                else if (t.startsWithIgnoreCase ("skewX"))
                {
                    trans = AffineTransform::shear (std::tan (degreesToRadians (numbers[0])), 0.0f);
                }
                else if (t.startsWithIgnoreCase ("skewY"))
                {
                    trans = AffineTransform::shear (0.0f, std::tan (degreesToRadians (numbers[0])));
                }

                result = trans.followedBy (result);
                t = t.fromFirstOccurrenceOf (")", false, false).trimStart();
            }

            return result;
        */
    }
    
    pub fn endpoint_to_centre_parameters(
        x1:          f64,
        y1:          f64,
        x2:          f64,
        y2:          f64,
        angle:       f64,
        large_arc:   bool,
        sweep:       bool,
        rx:          &mut f64,
        ry:          &mut f64,
        centrex:     &mut f64,
        centrey:     &mut f64,
        start_angle: &mut f64,
        delta_angle: &mut f64)  {
        
        todo!();
        /*
            const double midX = (x1 - x2) * 0.5;
            const double midY = (y1 - y2) * 0.5;

            const double cosAngle = std::cos (angle);
            const double sinAngle = std::sin (angle);
            const double xp = cosAngle * midX + sinAngle * midY;
            const double yp = cosAngle * midY - sinAngle * midX;
            const double xp2 = xp * xp;
            const double yp2 = yp * yp;

            double rx2 = rx * rx;
            double ry2 = ry * ry;

            const double s = (xp2 / rx2) + (yp2 / ry2);
            double c;

            if (s <= 1.0)
            {
                c = std::sqrt (jmax (0.0, ((rx2 * ry2) - (rx2 * yp2) - (ry2 * xp2))
                                             / (( rx2 * yp2) + (ry2 * xp2))));

                if (largeArc == sweep)
                    c = -c;
            }
            else
            {
                const double s2 = std::sqrt (s);
                rx *= s2;
                ry *= s2;
                c = 0;
            }

            const double cpx = ((rx * yp) / ry) * c;
            const double cpy = ((-ry * xp) / rx) * c;

            centreX = ((x1 + x2) * 0.5) + (cosAngle * cpx) - (sinAngle * cpy);
            centreY = ((y1 + y2) * 0.5) + (sinAngle * cpx) + (cosAngle * cpy);

            const double ux = (xp - cpx) / rx;
            const double uy = (yp - cpy) / ry;
            const double vx = (-xp - cpx) / rx;
            const double vy = (-yp - cpy) / ry;

            const double length = aloe_hypot (ux, uy);

            startAngle = acos (jlimit (-1.0, 1.0, ux / length));

            if (uy < 0)
                startAngle = -startAngle;

            startAngle += MathConstants<double>::halfPi;

            deltaAngle = acos (jlimit (-1.0, 1.0, ((ux * vx) + (uy * vy))
                                                    / (length * aloe_hypot (vx, vy))));

            if ((ux * vy) - (uy * vx) < 0)
                deltaAngle = -deltaAngle;

            if (sweep)
            {
                if (deltaAngle < 0)
                    deltaAngle += MathConstants<double>::twoPi;
            }
            else
            {
                if (deltaAngle > 0)
                    deltaAngle -= MathConstants<double>::twoPi;
            }

            deltaAngle = fmod (deltaAngle, MathConstants<double>::twoPi);
        */
    }
}


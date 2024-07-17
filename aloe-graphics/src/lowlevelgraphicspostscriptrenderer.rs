crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/contexts/aloe_LowLevelGraphicsPostScriptRenderer.h]

/**
  | An implementation of LowLevelGraphicsContext
  | that turns the drawing operations into
  | a PostScript document.
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LowLevelGraphicsPostScriptRenderer<'a, W: Write> {
    out:          &'a mut W,
    total_width:  i32,
    total_height: i32,
    need_to_clip: bool,
    last_colour:  Colour,
    state_stack:  Vec<LowLevelGraphicsPostScriptRendererSavedState>,
}

impl<'a, W: Write> LowLevelGraphicsContext 
for LowLevelGraphicsPostScriptRenderer<'a, W> {}

/**
  | Describes a saved state
  |
  */
pub struct LowLevelGraphicsPostScriptRendererSavedState
{
    clip:      RectangleList<i32>,
    x_offset:  i32,
    y_offset:  i32,
    fill_type: FillType,
    font:      Font,
}

impl LowLevelGraphicsPostScriptRendererSavedState {
    
    pub fn new() -> Self {
    
        todo!();
        /*
        : x_offset(0),
        : y_offset(0),
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/contexts/aloe_LowLevelGraphicsPostScriptRenderer.cpp]
impl<'a, W: Write> LowLevelGraphicsPostScriptRenderer<'a, W> {

    pub fn new(
        resulting_post_script: &mut W,
        document_title:        &String,
        total_width:           i32,
        total_height:          i32) -> Self {
    
        todo!();
        /*
        : out(resultingPostScript),
        : total_width(totalWidth_),
        : total_height(totalHeight_),
        : need_to_clip(true),

            stateStack.add (new SavedState());
        stateStack.getLast()->clip = Rectangle<int> (totalWidth_, totalHeight_);

        const float scale = jmin ((520.0f / (float) totalWidth_), (750.0f / (float) totalHeight));

        out << "%!PS-Adobe-3.0 EPSF-3.0"
               "\n%%BoundingBox: 0 0 600 824"
               "\n%%Pages: 0"
               "\n%%Creator: Raw Material Software Limited - Aloe"
               "\n%%Title: " << documentTitle <<
               "\n%%CreationDate: none"
               "\n%%LanguageLevel: 2"
               "\n%%EndComments"
               "\n%%BeginProlog"
               "\n%%BeginResource: JRes"
               "\n/bd {bind def} bind def"
               "\n/c {setrgbcolor} bd"
               "\n/m {moveto} bd"
               "\n/l {lineto} bd"
               "\n/rl {rlineto} bd"
               "\n/ct {curveto} bd"
               "\n/cp {closepath} bd"
               "\n/pr {3 index 3 index moveto 1 index 0 rlineto 0 1 index rlineto pop neg 0 rlineto pop pop closepath} bd"
               "\n/doclip {initclip newpath} bd"
               "\n/endclip {clip newpath} bd"
               "\n%%EndResource"
               "\n%%EndProlog"
               "\n%%BeginSetup"
               "\n%%EndSetup"
               "\n%%Page: 1 1"
               "\n%%BeginPageSetup"
               "\n%%EndPageSetup\n\n"
            << "40 800 translate\n"
            << scale << ' ' << scale << " scale\n\n";
        */
    }
    
    pub fn write_clip(&mut self)  {
        
        todo!();
        /*
            if (needToClip)
        {
            needToClip = false;

            out << "doclip ";

            int itemsOnLine = 0;

            for (auto& i : stateStack.getLast()->clip)
            {
                if (++itemsOnLine == 6)
                {
                    itemsOnLine = 0;
                    out << '\n';
                }

                out << i.getX() << ' ' << -i.getY() << ' '
                    << i.getWidth() << ' ' << -i.getHeight() << " pr ";
            }

            out << "endclip\n";
        }
        */
    }
    
    pub fn write_colour(&mut self, colour: Colour)  {
        
        todo!();
        /*
            Colour c (Colours::white.overlaidWith (colour));

        if (lastColour != c)
        {
            lastColour = c;

            out << String (c.getFloatRed(), 3) << ' '
                << String (c.getFloatGreen(), 3) << ' '
                << String (c.getFloatBlue(), 3) << " c\n";
        }
        */
    }
    
    pub fn writexy(&self, x: f32, y: f32)  {
        
        todo!();
        /*
            out << String (x, 2) << ' '
            << String (-y, 2) << ' ';
        */
    }
    
    pub fn write_path(&self, path: &Path)  {
        
        todo!();
        /*
            out << "newpath ";

        float lastX = 0.0f;
        float lastY = 0.0f;
        int itemsOnLine = 0;

        Path::Iterator i (path);

        while (i.next())
        {
            if (++itemsOnLine == 4)
            {
                itemsOnLine = 0;
                out << '\n';
            }

            switch (i.elementType)
            {
            case Path::Iterator::startNewSubPath:
                writeXY (i.x1, i.y1);
                lastX = i.x1;
                lastY = i.y1;
                out << "m ";
                break;

            case Path::Iterator::lineTo:
                writeXY (i.x1, i.y1);
                lastX = i.x1;
                lastY = i.y1;
                out << "l ";
                break;

            case Path::Iterator::quadraticTo:
                {
                    const float cp1x = lastX + (i.x1 - lastX) * 2.0f / 3.0f;
                    const float cp1y = lastY + (i.y1 - lastY) * 2.0f / 3.0f;
                    const float cp2x = cp1x + (i.x2 - lastX) / 3.0f;
                    const float cp2y = cp1y + (i.y2 - lastY) / 3.0f;

                    writeXY (cp1x, cp1y);
                    writeXY (cp2x, cp2y);
                    writeXY (i.x2, i.y2);
                    out << "ct ";
                    lastX = i.x2;
                    lastY = i.y2;
                }
                break;

            case Path::Iterator::cubicTo:
                writeXY (i.x1, i.y1);
                writeXY (i.x2, i.y2);
                writeXY (i.x3, i.y3);
                out << "ct ";
                lastX = i.x3;
                lastY = i.y3;
                break;

            case Path::Iterator::closePath:
                out << "cp ";
                break;

            default:
                jassertfalse;
                break;
            }
        }

        out << '\n';
        */
    }
    
    pub fn write_transform(&self, trans: &AffineTransform)  {
        
        todo!();
        /*
            out << "[ "
            << trans.mat00 << ' '
            << trans.mat10 << ' '
            << trans.mat01 << ' '
            << trans.mat11 << ' '
            << trans.mat02 << ' '
            << trans.mat12 << " ] concat ";
        */
    }
    
    pub fn write_image(&self, 
        im:   &Image,
        sx:   i32,
        sy:   i32,
        maxw: i32,
        maxh: i32)  {
        
        todo!();
        /*
            out << "{<\n";

        const int w = jmin (maxW, im.getWidth());
        const int h = jmin (maxH, im.getHeight());

        int charsOnLine = 0;
        const ImageBitmapData srcData (im, 0, 0, w, h);
        Colour pixel;

        for (int y = h; --y >= 0;)
        {
            for (int x = 0; x < w; ++x)
            {
                const uint8* pixelData = srcData.getPixelPointer (x, y);

                if (x >= sx && y >= sy)
                {
                    if (im.isARGB())
                    {
                        PixelARGB p (*(const PixelARGB*) pixelData);
                        p.unpremultiply();
                        pixel = Colours::white.overlaidWith (Colour (p));
                    }
                    else if (im.isRGB())
                    {
                        pixel = Colour (*((const PixelRGB*) pixelData));
                    }
                    else
                    {
                        pixel = Colour ((uint8) 0, (uint8) 0, (uint8) 0, *pixelData);
                    }
                }
                else
                {
                    pixel = Colours::transparentWhite;
                }

                const uint8 pixelValues[3] = { pixel.getRed(), pixel.getGreen(), pixel.getBlue() };

                out << String::toHexString (pixelValues, 3, 0);
                charsOnLine += 3;

                if (charsOnLine > 100)
                {
                    out << '\n';
                    charsOnLine = 0;
                }
            }
        }

        out << "\n>}\n";
        */
    }
}

impl<'a, W: Write> IsVectorDevice for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn is_vector_device(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
    
impl<'a, W: Write> SetOrigin for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn set_origin(&mut self, o: Point<i32>)  {
        
        todo!();
        /*
            if (! o.isOrigin())
        {
            stateStack.getLast()->xOffset += o.x;
            stateStack.getLast()->yOffset += o.y;
            needToClip = true;
        }
        */
    }
}
    
impl<'a, W: Write> AddTransform for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn add_transform(&mut self, transform: &AffineTransform)  {
        
        todo!();
        /*
            //xxx
        jassertfalse;
        */
    }
}
    
impl<'a, W: Write> GetPhysicalPixelScaleFactor for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn get_physical_pixel_scale_factor(&mut self) -> f32 {
        
        todo!();
        /*
            return 1.0f;
        */
    }
}
    
impl<'a, W: Write> ClipToRectangle for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn clip_to_rectangle(&mut self, r: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            needToClip = true;
        return stateStack.getLast()->clip.clipTo (r.translated (stateStack.getLast()->xOffset, stateStack.getLast()->yOffset));
        */
    }
}
    
impl<'a, W: Write> ClipToRectangleList for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn clip_to_rectangle_list(&mut self, clip_region: &RectangleList<i32>) -> bool {
        
        todo!();
        /*
            needToClip = true;
        return stateStack.getLast()->clip.clipTo (clipRegion);
        */
    }
}
    
impl<'a, W: Write> ExcludeClipRectangle for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn exclude_clip_rectangle(&mut self, r: &Rectangle<i32>)  {
        
        todo!();
        /*
            needToClip = true;
        stateStack.getLast()->clip.subtract (r.translated (stateStack.getLast()->xOffset, stateStack.getLast()->yOffset));
        */
    }
}
    
impl<'a, W: Write> ClipToPath for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn clip_to_path(&mut self, 
        path:      &Path,
        transform: &AffineTransform)  {
        
        todo!();
        /*
            writeClip();

        Path p (path);
        p.applyTransform (transform.translated ((float) stateStack.getLast()->xOffset, (float) stateStack.getLast()->yOffset));
        writePath (p);
        out << "clip\n";
        */
    }
}
    
impl<'a, W: Write> ClipToImageAlpha for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn clip_to_image_alpha(&mut self, 
        source_image: &Image,
        transform:    &AffineTransform)  {
        
        todo!();
        /*
            needToClip = true;
        jassertfalse; // xxx
        */
    }
}
    
impl<'a, W: Write> ClipRegionIntersects for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn clip_region_intersects(&mut self, r: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            return stateStack.getLast()->clip.intersectsRectangle (r.translated (stateStack.getLast()->xOffset, stateStack.getLast()->yOffset));
        */
    }
}
    
impl<'a, W: Write> GetClipBounds for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn get_clip_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return stateStack.getLast()->clip.getBounds().translated (-stateStack.getLast()->xOffset,
                                                                  -stateStack.getLast()->yOffset);
        */
    }
}
    
impl<'a, W: Write> IsClipEmpty for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn is_clip_empty(&self) -> bool {
        
        todo!();
        /*
            return stateStack.getLast()->clip.isEmpty();
        */
    }
}
    
impl<'a, W: Write> SaveState for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn save_state(&mut self)  {
        
        todo!();
        /*
            stateStack.add (new SavedState (*stateStack.getLast()));
        */
    }
}
    
impl<'a, W: Write> RestoreState for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn restore_state(&mut self)  {
        
        todo!();
        /*
            jassert (stateStack.size() > 0);

        if (stateStack.size() > 0)
            stateStack.removeLast();
        */
    }
}
    
impl<'a, W: Write> BeginTransparencyLayer for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn begin_transparency_layer(&mut self, _0: f32)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl<'a, W: Write> EndTransparencyLayer for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn end_transparency_layer(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------
impl<'a, W: Write> SetFill for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn set_fill(&mut self, fill_type: &FillType)  {
        
        todo!();
        /*
            stateStack.getLast()->fillType = fillType;
        */
    }
}
    
impl<'a, W: Write> SetOpacity for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn set_opacity(&mut self, opacity: f32)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl<'a, W: Write> SetInterpolationQuality for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn set_interpolation_quality(&mut self, quality: GraphicsResamplingQuality)  {
        
        todo!();
        /*
        
        */
    }
}
 
impl<'a, W: Write> FillRectMaybeReplace for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn fill_rect_maybe_replace(
        &mut self, 
        r:                         &Rectangle<i32>,
        replace_existing_contents: bool

    ) {
        
        todo!();
        /*
            fillRect (r.toFloat());
        */
    }
}

impl<'a, W: Write> FillRect for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn fill_rect(&mut self, r: &Rectangle<f32>)  {
        
        todo!();
        /*
            if (stateStack.getLast()->fillType.isColour())
        {
            writeClip();
            writeColour (stateStack.getLast()->fillType.colour);

            auto r2 = r.translated ((float) stateStack.getLast()->xOffset,
                                    (float) stateStack.getLast()->yOffset);

            out << r2.getX() << ' ' << -r2.getBottom() << ' ' << r2.getWidth() << ' ' << r2.getHeight() << " rectfill\n";
        }
        else
        {
            Path p;
            p.addRectangle (r);
            fillPath (p, AffineTransform());
        }
        */
    }
}

impl<'a, W: Write> FillRectList for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn fill_rect_list(&mut self, list: &RectangleList<f32>)  {
        
        todo!();
        /*
            fillPath (list.toPath(), AffineTransform());
        */
    }
}

impl<'a, W: Write> FillPath for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn fill_path(&mut self, 
        path: &Path,
        t:    &AffineTransform)  {
        
        todo!();
        /*
            if (stateStack.getLast()->fillType.isColour())
        {
            writeClip();

            Path p (path);
            p.applyTransform (t.translated ((float) stateStack.getLast()->xOffset,
                                            (float) stateStack.getLast()->yOffset));
            writePath (p);

            writeColour (stateStack.getLast()->fillType.colour);

            out << "fill\n";
        }
        else if (stateStack.getLast()->fillType.isGradient())
        {
            // this doesn't work correctly yet - it could be improved to handle solid gradients, but
            // postscript can't do semi-transparent ones.
            notPossibleInPostscriptAssert;   // you can disable this warning by setting the WARN_ABOUT_NON_POSTSCRIPT_OPERATIONS flag at the top of this file

            writeClip();
            out << "gsave ";

            {
                Path p (path);
                p.applyTransform (t.translated ((float) stateStack.getLast()->xOffset, (float) stateStack.getLast()->yOffset));
                writePath (p);
                out << "clip\n";
            }

            auto bounds = stateStack.getLast()->clip.getBounds();

            // ideally this would draw lots of lines or ellipses to approximate the gradient, but for the
            // time-being, this just fills it with the average colour..
            writeColour (stateStack.getLast()->fillType.gradient->getColourAtPosition (0.5f));
            out << bounds.getX() << ' ' << -bounds.getBottom() << ' ' << bounds.getWidth() << ' ' << bounds.getHeight() << " rectfill\n";

            out << "grestore\n";
        }
        */
    }
}

impl<'a, W: Write> DrawTextLayout for LowLevelGraphicsPostScriptRenderer<'a, W> {}

impl<'a, W: Write> DrawImage for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn draw_image(&mut self, 
        source_image: &Image,
        transform:    &AffineTransform)  {
        
        todo!();
        /*
            const int w = sourceImage.getWidth();
        const int h = sourceImage.getHeight();

        writeClip();

        out << "gsave ";
        writeTransform (transform.translated ((float) stateStack.getLast()->xOffset, (float) stateStack.getLast()->yOffset)
                                 .scaled (1.0f, -1.0f));

        RectangleList<int> imageClip;
        sourceImage.createSolidAreaMask (imageClip, 0.5f);

        out << "newpath ";
        int itemsOnLine = 0;

        for (auto& i : imageClip)
        {
            if (++itemsOnLine == 6)
            {
                out << '\n';
                itemsOnLine = 0;
            }

            out << i.getX() << ' ' << i.getY() << ' ' << i.getWidth() << ' ' << i.getHeight() << " pr ";
        }

        out << " clip newpath\n";

        out << w << ' ' << h << " scale\n";
        out << w << ' ' << h << " 8 [" << w << " 0 0 -" << h << ' ' << (int) 0 << ' ' << h << " ]\n";

        writeImage (sourceImage, 0, 0, w, h);

        out << "false 3 colorimage grestore\n";
        needToClip = true;
        */
    }
}

impl<'a, W: Write> DrawLine for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn draw_line(&mut self, line: &Line<f32>)  {
        
        todo!();
        /*
            Path p;
        p.addLineSegment (line, 1.0f);
        fillPath (p, AffineTransform());
        */
    }
}

impl<'a, W: Write> SetFont for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn set_font(&mut self, new_font: &Font)  {
        
        todo!();
        /*
            stateStack.getLast()->font = newFont;
        */
    }
}

impl<'a, W: Write> GetFont for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn get_font(&mut self) -> &Font {
        
        todo!();
        /*
            return stateStack.getLast()->font;
        */
    }
}

impl<'a, W: Write> DrawGlyph for LowLevelGraphicsPostScriptRenderer<'a, W> {

    fn draw_glyph(&mut self, 
        glyph_number: i32,
        transform:    &AffineTransform)  {
        
        todo!();
        /*
            Path p;
        Font& font = stateStack.getLast()->font;
        font.getTypeface()->getOutlineForGlyph (glyphNumber, p);
        fillPath (p, AffineTransform::scale (font.getHeight() * font.getHorizontalScale(), font.getHeight()).followedBy (transform));
        */
    }
}

/**
  | this will throw an assertion if you try
  | to draw something that's not possible
  | in postscript
  |
  */
pub const WARN_ABOUT_NON_POSTSCRIPT_OPERATIONS: usize = 0;

#[cfg(all(ALOE_DEBUG,WARN_ABOUT_NON_POSTSCRIPT_OPERATIONS))]
macro_rules! notpossibleinpostscriptassert {
    () => {
        /*
                jassertfalse
        */
    }
}

#[cfg(not(all(ALOE_DEBUG,WARN_ABOUT_NON_POSTSCRIPT_OPERATIONS)))]
macro_rules! notpossibleinpostscriptassert {
    () => {
        /*
                jassertfalse
        */
    }
}

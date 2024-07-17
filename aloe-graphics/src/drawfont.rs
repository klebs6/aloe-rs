crate::ix!();

pub trait DrawWithinArea {

    fn draw_within_area(
        &self, 
        g:    &mut Graphics,
        area: &Rectangle<f32>);
}

pub trait Draw {

    fn draw(&self, g: &mut Graphics);
}

pub trait DrawWithTransform {

    fn draw_with_transform(&self, g: &mut Graphics, txform: AffineTransform);
}

pub trait DrawGlyphUnderline {

    fn draw_glyph_underline(
        &self, 
        g:         &Graphics,
        pg:        &PositionedGlyph,
        i:         i32,
        transform: AffineTransform);
}

impl DrawWithinArea for TextLayout {

    /**
      | Draws the layout within the specified
      | area.
      | 
      | The position of the text within the rectangle
      | is controlled by the justification
      | flags set in the original AttributedString
      | that was used to create this layout.
      |
      */
    fn draw_within_area(
        &self, 
        g:    &mut Graphics,
        area: &Rectangle<f32>)  {
        
        todo!();
        /*
            auto origin = justification.appliedToRectangle (Rectangle<float> (width, getHeight()), area).getPosition();

        auto& context   = g.getInternalContext();
        context.saveState();

        auto clip       = context.getClipBounds();
        auto clipTop    = (float) clip.getY()      - origin.y;
        auto clipBottom = (float) clip.getBottom() - origin.y;

        for (auto& line : *this)
        {
            auto lineRangeY = line.getLineBoundsY();

            if (lineRangeY.getEnd() < clipTop)
                continue;

            if (lineRangeY.getStart() > clipBottom)
                break;

            auto lineOrigin = origin + line.lineOrigin;

            for (auto* run : line.runs)
            {
                context.setFont (run->font);
                context.setFill (run->colour);

                for (auto& glyph : run->glyphs)
                    context.drawGlyph (glyph.glyphCode, AffineTransform::translation (lineOrigin.x + glyph.anchor.x,
                                                                                      lineOrigin.y + glyph.anchor.y));

                if (run->font.isUnderlined())
                {
                    auto runExtent = run->getRunBoundsX();
                    auto lineThickness = run->font.getDescent() * 0.3f;

                    context.fillRect ({ runExtent.getStart() + lineOrigin.x, lineOrigin.y + lineThickness * 2.0f,
                                        runExtent.getLength(), lineThickness });
                }
            }
        }

        context.restoreState();
        */
    }
}

impl DrawWithinArea for AttributedString {

    /**
      | Draws this string within the given area.
      | 
      | The layout of the string within the rectangle
      | is controlled by the justification
      | value passed to setJustification().
      |
      */
    fn draw_within_area(
        &self, 
        g:    &mut Graphics,
        area: &Rectangle<f32>)  {
        
        todo!();
        /*
            if (text.isNotEmpty() && g.clipRegionIntersects (area.getSmallestIntegerContainer()))
        {
            jassert (text.length() == getLength (attributes));

            if (! g.getInternalContext().drawTextLayout (*this, area))
            {
                TextLayout layout;
                layout.createLayout (*this, area.getWidth());
                layout.draw (g, area);
            }
        }
        */
    }
}

impl Draw for PositionedGlyph {

    /**
      | Draws the glyph into a graphics context.
      | (Note that this may change the context's
      | currently selected font).
      |
      */
    fn draw(&self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (! isWhitespace())
            drawGlyphWithFont (g, glyph, font, AffineTransform::translation (x, y));
        */
    }
}

impl DrawWithTransform for PositionedGlyph {

    /**
      | Draws the glyph into a graphics context,
      | with an extra transform applied to it.
      | (Note that this may change the context's
      | currently selected font).
      |
      */
    fn draw_with_transform(
        &self, 
        g:         &mut Graphics,
        transform: AffineTransform

    ) {

        todo!();
        /*
            if (! isWhitespace())
            drawGlyphWithFont (g, glyph, font, AffineTransform::translation (x, y).followedBy (transform));
        */
    }
}

impl Draw for GlyphArrangement {

    /**
      | Draws this glyph arrangement to a graphics
      | context.
      | 
      | This uses cached bitmaps so is much faster
      | than the draw (Graphics&, AffineTransform)
      | method, which renders the glyphs as
      | filled vectors.
      |
      */
    fn draw(&self, g: &mut Graphics)  {
        
        todo!();
        /*
            draw (g, {});
        */
    }
}

impl DrawWithTransform for GlyphArrangement {

    /**
      | Draws this glyph arrangement to a graphics
      | context.
      | 
      | This renders the paths as filled vectors,
      | so is far slower than the draw (Graphics&)
      | method for non-transformed arrangements.
      |
      */
    fn draw_with_transform(
        &self, 
        g:         &mut Graphics,
        transform: AffineTransform) 

    {
        todo!();
        /*
            auto& context = g.getInternalContext();
        auto lastFont = context.getFont();
        bool needToRestore = false;

        for (int i = 0; i < glyphs.size(); ++i)
        {
            auto& pg = glyphs.getReference (i);

            if (pg.font.isUnderlined())
                drawGlyphUnderline (g, pg, i, transform);

            if (! pg.isWhitespace())
            {
                if (lastFont != pg.font)
                {
                    lastFont = pg.font;

                    if (! needToRestore)
                    {
                        needToRestore = true;
                        context.saveState();
                    }

                    context.setFont (lastFont);
                }

                context.drawGlyph (pg.glyph, AffineTransform::translation (pg.x, pg.y)
                                                             .followedBy (transform));
            }
        }

        if (needToRestore)
            context.restoreState();
        */
    }
}

pub fn draw_glyph_with_font(
        g:     &mut Graphics,
        glyph: i32,
        font:  &Font,
        t:     AffineTransform)  {
    
    todo!();
    /*
        auto& context = g.getInternalContext();
        context.setFont (font);
        context.drawGlyph (glyph, t);
    */
}

impl DrawGlyphUnderline for GlyphArrangement {

    fn draw_glyph_underline(
        &self, 
        g:         &Graphics,
        pg:        &PositionedGlyph,
        i:         i32,
        transform: AffineTransform)  {
        
        todo!();
        /*
            auto lineThickness = (pg.font.getDescent()) * 0.3f;
        auto nextX = pg.x + pg.w;

        if (i < glyphs.size() - 1 && glyphs.getReference (i + 1).y == pg.y)
            nextX = glyphs.getReference (i + 1).x;

        Path p;
        p.addRectangle (pg.x, pg.y + lineThickness * 2.0f, nextX - pg.x, lineThickness);
        g.fillPath (p, transform);
        */
    }
}

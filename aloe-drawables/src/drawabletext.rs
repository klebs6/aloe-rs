crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableText.h]

/**
  | A drawable object which renders a line
  | of text.
  | 
  | @see Drawable
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct DrawableText<'a> {
    base:          Drawable<'a>,
    bounds:        Parallelogram<f32>,
    font_height:   f32,
    font_hscale:   f32,
    font:          Font,
    scaled_font:   Font,
    text:          String,
    colour:        Colour,
    justification: Justification,
}

impl<'a> Default for DrawableText<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        : colour (Colours::black),
        justification (Justification::centredLeft)

        setBoundingBox (Parallelogram<float> ({ 0.0f, 0.0f, 50.0f, 20.0f }));
        setFont (Font (15.0f), true);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableText.cpp]
impl<'a> DrawableText<'a> {
    
    /**
      | Returns the currently displayed text
      |
      */
    pub fn get_text(&self) -> &String {
        
        todo!();
        /*
            return text;
        */
    }

    /**
      | Returns the current text colour.
      |
      */
    pub fn get_colour(&self) -> Colour {
        
        todo!();
        /*
            return colour;
        */
    }

    /**
      | Returns the current font.
      |
      */
    pub fn get_font(&self) -> &Font {
        
        todo!();
        /*
            return font;
        */
    }

    /**
      | Returns the current justification.
      |
      */
    pub fn get_justification(&self) -> Justification {
        
        todo!();
        /*
            return justification;
        */
    }

    /**
      | Returns the parallelogram that defines
      | the text bounding box.
      |
      */
    pub fn get_bounding_box(&self) -> Parallelogram<f32> {
        
        todo!();
        /*
            return bounds;
        */
    }

    pub fn get_font_height(&self) -> f32 {
        
        todo!();
        /*
            return fontHeight;
        */
    }
    
    pub fn get_font_horizontal_scale(&self) -> f32 {
        
        todo!();
        /*
            return fontHScale;
        */
    }
    
    pub fn new(other: &DrawableText) -> Self {
    
        todo!();
        /*
        : drawable(other),
        : bounds(other.bounds),
        : font_height(other.fontHeight),
        : font_hscale(other.fontHScale),
        : font(other.font),
        : text(other.text),
        : colour(other.colour),
        : justification(other.justification),

            refreshBounds();
        */
    }
    
    pub fn create_copy(&self) -> Box<Drawable> {
        
        todo!();
        /*
            return std::make_unique<DrawableText> (*this);
        */
    }
    
    /**
      | Sets the text to display.
      |
      */
    pub fn set_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            if (text != newText)
        {
            text = newText;
            refreshBounds();
        }
        */
    }
    
    /**
      | Sets the colour of the text.
      |
      */
    pub fn set_colour(&mut self, new_colour: Colour)  {
        
        todo!();
        /*
            if (colour != newColour)
        {
            colour = newColour;
            repaint();
        }
        */
    }
    
    /**
      | Sets the font to use.
      | 
      | -----------
      | @note
      | 
      | the font height and horizontal scale
      | are set using setFontHeight() and setFontHorizontalScale().
      | If applySizeAndScale is true, then
      | these height and scale values will be
      | changed to match the dimensions of the
      | font supplied; if it is false, then the
      | new font object's height and scale are
      | ignored.
      |
      */
    pub fn set_font(&mut self, 
        new_font:             &Font,
        apply_size_and_scale: bool)  {
        
        todo!();
        /*
            if (font != newFont)
        {
            font = newFont;

            if (applySizeAndScale)
            {
                fontHeight = font.getHeight();
                fontHScale = font.getHorizontalScale();
            }

            refreshBounds();
        }
        */
    }
    
    /**
      | Changes the justification of the text
      | within the bounding box.
      |
      */
    pub fn set_justification(&mut self, new_justification: Justification)  {
        
        todo!();
        /*
            justification = newJustification;
        repaint();
        */
    }
    
    /**
      | Sets the bounding box that contains
      | the text.
      |
      */
    pub fn set_bounding_box(&mut self, new_bounds: Parallelogram<f32>)  {
        
        todo!();
        /*
            if (bounds != newBounds)
        {
            bounds = newBounds;
            refreshBounds();
        }
        */
    }
    
    pub fn set_font_height(&mut self, new_height: f32)  {
        
        todo!();
        /*
            if (fontHeight != newHeight)
        {
            fontHeight = newHeight;
            refreshBounds();
        }
        */
    }
    
    pub fn set_font_horizontal_scale(&mut self, new_scale: f32)  {
        
        todo!();
        /*
            if (fontHScale != newScale)
        {
            fontHScale = newScale;
            refreshBounds();
        }
        */
    }
    
    pub fn refresh_bounds(&mut self)  {
        
        todo!();
        /*
            auto w = bounds.getWidth();
        auto h = bounds.getHeight();

        auto height = jlimit (0.01f, jmax (0.01f, h), fontHeight);
        auto hscale = jlimit (0.01f, jmax (0.01f, w), fontHScale);

        scaledFont = font;
        scaledFont.setHeight (height);
        scaledFont.setHorizontalScale (hscale);

        setBoundsToEnclose (getDrawableBounds());
        repaint();
        */
    }
    
    pub fn get_text_area(&self, w: f32, h: f32) -> Rectangle<i32> {
        
        todo!();
        /*
            return Rectangle<float> (w, h).getSmallestIntegerContainer();
        */
    }
    
    pub fn get_text_transform(&self, w: f32, h: f32) -> AffineTransform {
        
        todo!();
        /*
            return AffineTransform::fromTargetPoints (Point<float>(),       bounds.topLeft,
                                                  Point<float> (w, 0),  bounds.topRight,
                                                  Point<float> (0, h),  bounds.bottomLeft);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            transformContextToCorrectOrigin (g);

        auto w = bounds.getWidth();
        auto h = bounds.getHeight();

        g.addTransform (getTextTransform (w, h));
        g.setFont (scaledFont);
        g.setColour (colour);

        g.drawFittedText (text, getTextArea (w, h), justification, 0x100000);
        */
    }
    
    pub fn get_drawable_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return bounds.getBoundingBox();
        */
    }
    
    pub fn get_outline_as_path(&self) -> PathBuf {
        
        todo!();
        /*
            auto w = bounds.getWidth();
        auto h = bounds.getHeight();
        auto area = getTextArea (w, h).toFloat();

        GlyphArrangement arr;
        arr.addFittedText (scaledFont, text,
                           area.getX(), area.getY(),
                           area.getWidth(), area.getHeight(),
                           justification,
                           0x100000);

        Path pathOfAllGlyphs;

        for (auto& glyph : arr)
        {
            Path gylphPath;
            glyph.createPath (gylphPath);
            pathOfAllGlyphs.addPath (gylphPath);
        }

        pathOfAllGlyphs.applyTransform (getTextTransform (w, h).followedBy (getTransform()));

        return pathOfAllGlyphs;
        */
    }
    
    pub fn replace_colour(
        &mut self, 
        original_colour:    Colour,
        replacement_colour: Colour

    ) -> bool {
        
        todo!();
        /*
            if (colour != originalColour)
            return false;

        setColour (replacementColour);
        return true;
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class DrawableTextAccessibilityHandler  : public AccessibilityHandler
        {
        
            DrawableTextAccessibilityHandler (DrawableText& drawableTextToWrap)
                : AccessibilityHandler (drawableTextToWrap, AccessibilityRole::staticText),
                  drawableText (drawableTextToWrap)
            {
            }

            String getTitle() const override  { return drawableText.getText(); }

        
            DrawableText& drawableText;
        };

        return std::make_unique<DrawableTextAccessibilityHandler> (*this);
        */
    }
}

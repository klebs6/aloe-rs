crate::ix!();

pub fn is_breakable_glyph(g: &PositionedGlyph) -> bool {
    
    todo!();
    /*
        return g.isWhitespace() || g.getCharacter() == '-';
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_GlyphArrangement.h]

/**
  | A glyph from a particular font, with
  | a particular size, style, typeface
  | and position.
  | 
  | You should rarely need to use this class
  | directly - for most purposes, the GlyphArrangement
  | class will do what you need for text layout.
  | 
  | @see GlyphArrangement, Font
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct PositionedGlyph {
    font:       Font,
    character:  wchar_t,
    glyph:      i32,
    x:          f32,
    y:          f32,
    w:          f32,
    whitespace: bool,
}

impl Default for PositionedGlyph {

    fn default() -> Self {
    
        todo!();
        /*
        : character(0),
        : glyph(0),
        : x(0),
        : y(0),
        : w(0),
        : whitespace(false),

        
        */
    }
}

impl PositionedGlyph {

    /**
      | Returns the character the glyph represents.
      |
      */
    pub fn get_character(&self) -> wchar_t {
        
        todo!();
        /*
            return character;
        */
    }

    /**
      | Checks whether the glyph is actually
      | empty.
      |
      */
    pub fn is_whitespace(&self) -> bool {
        
        todo!();
        /*
            return whitespace;
        */
    }

    /**
      | Returns the position of the glyph's
      | left-hand edge.
      |
      */
    pub fn get_left(&self) -> f32 {
        
        todo!();
        /*
            return x;
        */
    }

    /**
      | Returns the position of the glyph's
      | right-hand edge.
      |
      */
    pub fn get_right(&self) -> f32 {
        
        todo!();
        /*
            return x + w;
        */
    }

    /**
      | Returns the y position of the glyph's
      | baseline.
      |
      */
    pub fn get_baseliney(&self) -> f32 {
        
        todo!();
        /*
            return y;
        */
    }

    /**
      | Returns the y position of the top of the
      | glyph.
      |
      */
    pub fn get_top(&self) -> f32 {
        
        todo!();
        /*
            return y - font.getAscent();
        */
    }

    /**
      | Returns the y position of the bottom
      | of the glyph.
      |
      */
    pub fn get_bottom(&self) -> f32 {
        
        todo!();
        /*
            return y + font.getDescent();
        */
    }

    /**
      | Returns the bounds of the glyph.
      |
      */
    pub fn get_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return { x, getTop(), w, font.getHeight() };
        */
    }

    
    pub fn new(
        font:         &Font,
        character:    wchar_t,
        glyph_number: i32,
        anchorx:      f32,
        baseliney:    f32,
        width:        f32,
        whitespace:   bool) -> Self {
    
        todo!();
        /*
        : font(font_),
        : character(character_),
        : glyph(glyphNumber),
        : x(anchorX),
        : y(baselineY),
        : w(width),
        : whitespace(whitespace_),

        
        */
    }
    
    /**
      | Returns the path for this glyph.
      | 
      | -----------
      | @param path
      | 
      | the glyph's outline will be appended
      | to this path
      |
      */
    pub fn create_path(&self, path: &mut Path)  {
        
        todo!();
        /*
            if (! isWhitespace())
        {
            if (auto* t = font.getTypeface())
            {
                Path p;
                t->getOutlineForGlyph (glyph, p);

                path.addPath (p, AffineTransform::scale (font.getHeight() * font.getHorizontalScale(), font.getHeight())
                                                 .translated (x, y));
            }
        }
        */
    }
    
    /**
      | Checks to see if a point lies within this
      | glyph.
      |
      */
    pub fn hit_test(&self, px: f32, py: f32) -> bool {
        
        todo!();
        /*
            if (getBounds().contains (px, py) && ! isWhitespace())
        {
            if (auto* t = font.getTypeface())
            {
                Path p;
                t->getOutlineForGlyph (glyph, p);

                AffineTransform::translation (-x, -y)
                                .scaled (1.0f / (font.getHeight() * font.getHorizontalScale()), 1.0f / font.getHeight())
                                .transformPoint (px, py);

                return p.contains (px, py);
            }
        }

        return false;
        */
    }
    
    /**
      | Shifts the glyph's position by a relative
      | amount.
      |
      */
    pub fn move_by(&mut self, 
        deltax: f32,
        deltay: f32)  {
        
        todo!();
        /*
            x += deltaX;
        y += deltaY;
        */
    }
}

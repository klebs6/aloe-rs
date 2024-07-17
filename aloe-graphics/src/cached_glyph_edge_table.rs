crate::ix!();

/**
  | Caches a glyph as an edge-table.
  | 
  | @tags{Graphics}
  |
  */
#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct CachedGlyphEdgeTable<RendererType> {
    base:                       ReferenceCountedObject,
    font:                       Font,
    edge_table:                 Box<EdgeTable>,
    glyph:                      i32, // default = 0
    last_access_count:          i32, // default = 0
    snap_to_integer_coordinate: bool, // default = false
    phantom:                    PhantomData<RendererType>,
}

impl<RendererType> CachedGlyphEdgeTable<RendererType> {

    pub fn draw(&self, 
        state: &mut RendererType,
        pos:   Point<f32>)  {
        
        todo!();
        /*
            if (snapToIntegerCoordinate)
                pos.x = std::floor (pos.x + 0.5f);

            if (edgeTable != nullptr)
                state.fillEdgeTable (*edgeTable, pos.x, roundToInt (pos.y));
        */
    }
    
    pub fn generate(&mut self, 
        new_font:     &Font,
        glyph_number: i32)  {
        
        todo!();
        /*
            font = newFont;
            auto* typeface = newFont.getTypeface();
            snapToIntegerCoordinate = typeface->isHinted();
            glyph = glyphNumber;

            auto fontHeight = font.getHeight();
            edgeTable.reset (typeface->getEdgeTableForGlyph (glyphNumber,
                                                             AffineTransform::scale (fontHeight * font.getHorizontalScale(),
                                                                                     fontHeight), fontHeight));
        */
    }
}

crate::ix!();

///---------------
pub struct GridBoxAlignment {

}

impl GridBoxAlignment {

    pub fn align_item(
        item: &GridItem,
        grid: &Grid,
        area: Rectangle<f32>) -> Rectangle<f32> {
        
        todo!();
        /*
            // if item align is auto, inherit value from grid
                Grid::GridAlignItems alignType = Grid::GridAlignItems::start;
                Grid::GridJustifyItems justifyType = Grid::GridJustifyItems::start;

                if (item.alignSelf == GridItem::AlignSelf::autoValue)
                    alignType = grid.alignItems;
                else
                    alignType = static_cast<Grid::GridAlignItems> (item.alignSelf);

                if (item.justifySelf == GridItem::JustifySelf::autoValue)
                    justifyType = grid.justifyItems;
                else
                    justifyType = static_cast<Grid::GridJustifyItems> (item.justifySelf);

                // subtract margin from area
                area = BorderSize<float> (item.margin.top, item.margin.left, item.margin.bottom, item.margin.right)
                          .subtractedFrom (area);

                // align and justify
                auto r = area;

                if (item.width     != (float) GridItem::notAssigned)  r.setWidth  (item.width);
                if (item.height    != (float) GridItem::notAssigned)  r.setHeight (item.height);
                if (item.maxWidth  != (float) GridItem::notAssigned)  r.setWidth  (jmin (item.maxWidth,  r.getWidth()));
                if (item.minWidth  > 0.0f)                            r.setWidth  (jmax (item.minWidth,  r.getWidth()));
                if (item.maxHeight != (float) GridItem::notAssigned)  r.setHeight (jmin (item.maxHeight, r.getHeight()));
                if (item.minHeight > 0.0f)                            r.setHeight (jmax (item.minHeight, r.getHeight()));

                if (alignType == Grid::GridAlignItems::start && justifyType == Grid::GridJustifyItems::start)
                    return r;

                if (alignType   == Grid::GridAlignItems::end)       r.setY (r.getY() + (area.getHeight() - r.getHeight()));
                if (justifyType == Grid::GridJustifyItems::end)     r.setX (r.getX() + (area.getWidth()  - r.getWidth()));
                if (alignType   == Grid::GridAlignItems::center)    r.setCentre (r.getCentreX(),    area.getCentreY());
                if (justifyType == Grid::GridJustifyItems::center)  r.setCentre (area.getCentreX(), r.getCentreY());

                return r;
        */
    }
}

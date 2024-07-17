crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ToolbarSpacer {
    fixed_size: f32,
    draw_bar:   bool,
}

impl ToolbarItemComponent for ToolbarSpacer {

    fn create_item(&mut self, item_id: i32) -> *mut dyn ToolbarItemComponent {
        todo!();
    }
}

impl ToolbarSpacer {

    pub fn new(
        itemid:          i32,
        size_to_use:     f32,
        should_draw_bar: bool) -> Self {
    
        todo!();
        /*


            : ToolbarItemComponent (itemID, {}, false),
              fixedSize (sizeToUse),
              drawBar (shouldDrawBar)

            setWantsKeyboardFocus (false);
        */
    }
    
    pub fn get_toolbar_item_sizes(&mut self, 
        toolbar_thickness:   i32,
        is_toolbar_vertical: bool,
        preferred_size:      &mut i32,
        min_size:            &mut i32,
        max_size:            &mut i32) -> bool {
        
        todo!();
        /*
            if (fixedSize <= 0)
            {
                preferredSize = toolbarThickness * 2;
                minSize = 4;
                maxSize = 32768;
            }
            else
            {
                maxSize = roundToInt ((float) toolbarThickness * fixedSize);
                minSize = drawBar ? maxSize : jmin (4, maxSize);
                preferredSize = maxSize;

                if (getEditingMode() == editableOnPalette)
                    preferredSize = maxSize = toolbarThickness / (drawBar ? 3 : 2);
            }

            return true;
        */
    }
    
    pub fn get_resize_order(&self) -> i32 {
        
        todo!();
        /*
            return fixedSize <= 0 ? 0 : 1;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto w = getWidth();
            auto h = getHeight();

            if (drawBar)
            {
                g.setColour (findColour (Toolbar::separatorColourId, true));

                auto thickness = 0.2f;

                if (isToolbarVertical())
                    g.fillRect ((float) w * 0.1f, (float) h * (0.5f - thickness * 0.5f), (float) w * 0.8f, (float) h * thickness);
                else
                    g.fillRect ((float) w * (0.5f - thickness * 0.5f), (float) h * 0.1f, (float) w * thickness, (float) h * 0.8f);
            }

            if (getEditingMode() != normalMode && ! drawBar)
            {
                g.setColour (findColour (Toolbar::separatorColourId, true));

                auto indentX = jmin (2, (w - 3) / 2);
                auto indentY = jmin (2, (h - 3) / 2);
                g.drawRect (indentX, indentY, w - indentX * 2, h - indentY * 2, 1);

                if (fixedSize <= 0)
                {
                    float x1, y1, x2, y2, x3, y3, x4, y4, hw, hl;

                    if (isToolbarVertical())
                    {
                        x1 = (float) w * 0.5f;
                        y1 = (float) h * 0.4f;
                        x2 = x1;
                        y2 = (float) indentX * 2.0f;

                        x3 = x1;
                        y3 = (float) h * 0.6f;
                        x4 = x1;
                        y4 = (float) h - y2;

                        hw = (float) w * 0.15f;
                        hl = (float) w * 0.2f;
                    }
                    else
                    {
                        x1 = (float) w * 0.4f;
                        y1 = (float) h * 0.5f;
                        x2 = (float) indentX * 2.0f;
                        y2 = y1;

                        x3 = (float) w * 0.6f;
                        y3 = y1;
                        x4 = (float) w - x2;
                        y4 = y1;

                        hw = (float) h * 0.15f;
                        hl = (float) h * 0.2f;
                    }

                    Path p;
                    p.addArrow ({ x1, y1, x2, y2 }, 1.5f, hw, hl);
                    p.addArrow ({ x3, y3, x4, y4 }, 1.5f, hw, hl);
                    g.fillPath (p);
                }
            }
        */
    }
}

crate::ix!();

pub fn create_drawable_fromsvg<'a>(data: *const u8) -> Box<Drawable<'a>> {
    
    todo!();
    /*
        auto xml = parseXML (data);
        jassert (xml != nullptr);
        return Drawable::createFromSVG (*xml);
    */
}

pub fn create_path_from_data(
        height: f32,
        data:   *const u8,
        size:   usize) -> Path {
    
    todo!();
    /*
        Path p;
        p.loadPathFromData (data, size);
        p.scaleToFit (0, 0, height * 2.0f, height, true);
        return p;
    */
}

pub fn look_and_feelv2_create_base_colour(
        button_colour:                     Colour,
        has_keyboard_focus:                bool,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool) -> Colour {
    
    todo!();
    /*
        const float sat = hasKeyboardFocus ? 1.3f : 0.9f;
            const Colour baseColour (buttonColour.withMultipliedSaturation (sat));

            if (shouldDrawButtonAsDown)        return baseColour.contrasting (0.2f);
            if (shouldDrawButtonAsHighlighted) return baseColour.contrasting (0.1f);

            return baseColour;
    */
}

pub fn look_and_feelv2_layout_tooltip_text(
        text:   &String,
        colour: Colour) -> TextLayout {
    
    todo!();
    /*
        const float tooltipFontSize = 13.0f;
            const int maxToolTipWidth = 400;

            AttributedString s;
            s.setJustification (Justification::centred);
            s.append (text, Font (tooltipFontSize, Font::bold), colour);

            TextLayout tl;
            tl.createLayoutWithBalancedLineLengths (s, (float) maxToolTipWidth);
            return tl;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V2.h]

/**
  | This LookAndFeel subclass implements
  | the aloe style from around 2008-12.
  | 
  | @see LookAndFeel, LookAndFeel_V1,
  | LookAndFeel_V3
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LookAndFeel_V2<'a> {
    folder_image:   Box<Drawable<'a>>,
    document_image: Box<Drawable<'a>>,
}

impl<'a> LookAndFeel for LookAndFeel_V2<'a> {

}

impl<'a> HasLookAndFeelData for LookAndFeel_V2<'a> {

    type LookAndFeelData = LookAndFeelData;

    fn get_look_and_feel_data(&self) -> &Self::LookAndFeelData {
        todo!();
    }
}

impl<'a> SidePanelLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn get_side_panel_title_font(&mut self, _0: &mut SidePanel) -> Font {
        
        todo!();
        /*
            return Font (18.0f);
        */
    }
    
    fn get_side_panel_title_justification(&mut self, panel: &mut SidePanel) -> Justification {
        
        todo!();
        /*
            return panel.isPanelOnLeft() ? Justification::centredRight
                                     : Justification::centredLeft;
        */
    }
    
    fn get_side_panel_dismiss_button_shape(&mut self, panel: &mut SidePanel) -> Path {
        
        todo!();
        /*
            return getCrossShape ((float) panel.getTitleBarHeight());
        */
    }
}

impl<'a> extra_look_and_feel_traits::LassoComponentMethods for LookAndFeel_V2<'a> {

    fn draw_lasso(&mut self, 
        g:          &mut Graphics,
        lasso_comp: &mut Component)  {
        
        todo!();
        /*
            const int outlineThickness = 1;

        g.fillAll (lassoComp.findColour (0x1000440 /*lassoFillColourId*/));

        g.setColour (lassoComp.findColour (0x1000441 /*lassoOutlineColourId*/));
        g.drawRect (lassoComp.getLocalBounds(), outlineThickness);
        */
    }
}

impl<'a> extra_look_and_feel_traits::AudioDeviceSelectorComponentMethods for LookAndFeel_V2<'a> {

    fn draw_level_meter(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32,
        level:  f32)  {
        
        todo!();
        /*
            g.setColour (Colours::white.withAlpha (0.7f));
        g.fillRoundedRectangle (0.0f, 0.0f, (float) width, (float) height, 3.0f);
        g.setColour (Colours::black.withAlpha (0.2f));
        g.drawRoundedRectangle (1.0f, 1.0f, (float) width - 2.0f, (float) height - 2.0f, 3.0f, 1.0f);

        const int totalBlocks = 7;
        const int numBlocks = roundToInt (totalBlocks * level);
        auto w = ((float) width - 6.0f) / (float) totalBlocks;

        for (int i = 0; i < totalBlocks; ++i)
        {
            if (i >= numBlocks)
                g.setColour (Colours::lightblue.withAlpha (0.6f));
            else
                g.setColour (i < totalBlocks - 1 ? Colours::blue.withAlpha (0.5f)
                                                 : Colours::red);

            g.fillRoundedRectangle (3.0f + (float) i * w + w * 0.1f,
                                    3.0f,
                                    (float) w * 0.8f,
                                    (float) height - 6.0f,
                                    (float) w * 0.4f);
        }
        */
    }
}

impl<'a> extra_look_and_feel_traits::KeyMappingEditorComponentMethods for LookAndFeel_V2<'a> {

    fn draw_keymap_change_button(&mut self, 
        g:               &mut Graphics,
        width:           i32,
        height:          i32,
        button:          &mut Button,
        key_description: &String)  {
        
        todo!();
        /*
            auto textColour = button.findColour (0x100ad01 /*KeyMappingEditorComponent::textColourId*/, true);

        if (keyDescription.isNotEmpty())
        {
            if (button.isEnabled())
            {
                auto alpha = button.isDown() ? 0.3f : (button.isOver() ? 0.15f : 0.08f);
                g.fillAll (textColour.withAlpha (alpha));

                g.setOpacity (0.3f);
                drawBevel (g, 0, 0, width, height, 2);
            }

            g.setColour (textColour);
            g.setFont ((float) height * 0.6f);
            g.drawFittedText (keyDescription,
                              3, 0, width - 6, height,
                              Justification::centred, 1);
        }
        else
        {
            const float thickness = 7.0f;
            const float indent = 22.0f;

            Path p;
            p.addEllipse (0.0f, 0.0f, 100.0f, 100.0f);
            p.addRectangle (indent, 50.0f - thickness, 100.0f - indent * 2.0f, thickness * 2.0f);
            p.addRectangle (50.0f - thickness, indent, thickness * 2.0f, 50.0f - indent - thickness);
            p.addRectangle (50.0f - thickness, 50.0f + thickness, thickness * 2.0f, 50.0f - indent - thickness);
            p.setUsingNonZeroWinding (false);

            g.setColour (textColour.withAlpha (button.isDown() ? 0.7f : (button.isOver() ? 0.5f : 0.3f)));
            g.fillPath (p, p.getTransformToScaleToFit (2.0f, 2.0f, (float) width - 4.0f, (float) height - 4.0f, true));
        }

        if (button.hasKeyboardFocus (false))
        {
            g.setColour (textColour.withAlpha (0.4f));
            g.drawRect (0, 0, width, height);
        }
        */
    }
}

impl<'a> StretchableLayoutResizerBarLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_stretchable_layout_resizer_bar(&mut self, 
        g:                 &mut Graphics,
        w:                 i32,
        h:                 i32,
        is_vertical_bar:   bool,
        is_mouse_over:     bool,
        is_mouse_dragging: bool)  {
        
        todo!();
        /*
            auto alpha = 0.5f;

        if (isMouseOver || isMouseDragging)
        {
            g.fillAll (Colour (0x190000ff));
            alpha = 1.0f;
        }

        auto cx = (float) w * 0.5f;
        auto cy = (float) h * 0.5f;
        auto cr = (float) jmin (w, h) * 0.4f;

        g.setGradientFill (ColourGradient (Colours::white.withAlpha (alpha), cx + cr * 0.1f, cy + cr,
                                           Colours::black.withAlpha (alpha), cx, cy - cr * 4.0f,
                                           true));

        g.fillEllipse (cx - cr, cy - cr, cr * 2.0f, cr * 2.0f);
        */
    }
}

impl<'a> ProgressBarLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn is_progress_bar_opaque(&mut self, progress_bar: &mut ProgressBar) -> bool {
        
        todo!();
        /*
            return progressBar.findColour (ProgressBar::backgroundColourId).isOpaque();
        */
    }

    fn draw_progress_bar(
        &mut self, 
        g:            &mut Graphics,
        progress_bar: &mut ProgressBar,
        width:        i32,
        height:       i32,
        progress:     f64,
        text_to_show: &String

    ) {
        
        todo!();
        /*
            const Colour background (progressBar.findColour (ProgressBar::backgroundColourId));
        const Colour foreground (progressBar.findColour (ProgressBar::foregroundColourId));

        g.fillAll (background);

        if (progress >= 0.0f && progress < 1.0f)
        {
            drawGlassLozenge (g, 1.0f, 1.0f,
                              (float) jlimit (0.0, width - 2.0, progress * (width - 2.0)),
                              (float) (height - 2),
                              foreground,
                              0.5f, 0.0f,
                              true, true, true, true);
        }
        else
        {
            // spinning bar..
            g.setColour (foreground);

            const int stripeWidth = height * 2;
            const int position = (int) (Time::getMillisecondCounter() / 15) % stripeWidth;

            Path p;

            for (float x = (float) (- position); x < (float) (width + stripeWidth); x += (float) stripeWidth)
                p.addQuadrilateral (x, 0.0f,
                                    x + (float) stripeWidth * 0.5f, 0.0f,
                                    x, (float) height,
                                    x - (float) stripeWidth * 0.5f, (float) height);

            Image im (Image::ARGB, width, height, true);

            {
                Graphics g2 (im);
                drawGlassLozenge (g2, 1.0f, 1.0f,
                                  (float) (width - 2),
                                  (float) (height - 2),
                                  foreground,
                                  0.5f, 0.0f,
                                  true, true, true, true);
            }

            g.setTiledImageFill (im, 0, 0, 0.85f);
            g.fillPath (p);
        }

        if (textToShow.isNotEmpty())
        {
            g.setColour (Colour::contrasting (background, foreground));
            g.setFont ((float) height * 0.6f);

            g.drawText (textToShow, 0, 0, width, height, Justification::centred, false);
        }
        */
    }
}

impl<'a> ConcertinaPanelLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_concertina_panel_header(&mut self, 
        g:             &mut Graphics,
        area:          &Rectangle<i32>,
        is_mouse_over: bool,
        is_mouse_down: bool,
        _4:            &mut ConcertinaPanel,
        panel:         &mut Component)  {
        
        todo!();
        /*
            g.fillAll (Colours::grey.withAlpha (isMouseOver ? 0.9f : 0.7f));
        g.setColour (Colours::black.withAlpha (0.5f));
        g.drawRect (area);

        g.setColour (Colours::white);
        g.setFont (Font ((float) area.getHeight() * 0.7f).boldened());
        g.drawFittedText (panel.getName(), 4, 0, area.getWidth() - 6, area.getHeight(), Justification::centredLeft, 1);
        */
    }
}

impl<'a> ToolbarLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn paint_toolbar_background(&mut self, 
        g:       &mut Graphics,
        w:       i32,
        h:       i32,
        toolbar: &mut Toolbar)  {
        
        todo!();
        /*
            auto background = toolbar.findColour (Toolbar::backgroundColourId);

        g.setGradientFill (ColourGradient (background, 0.0f, 0.0f,
                                           background.darker (0.1f),
                                           toolbar.isVertical() ? (float) w - 1.0f : 0.0f,
                                           toolbar.isVertical() ? 0.0f : (float) h - 1.0f,
                                           false));
        g.fillAll();
        */
    }
    
    fn create_toolbar_missing_items_button(&mut self, toolbar: &mut Toolbar) -> *mut Button {
        
        todo!();
        /*
            return createTabBarExtrasButton();
        */
    }
    
    fn paint_toolbar_button_background(&mut self, 
        g:             &mut Graphics,
        width:         i32,
        height:        i32,
        is_mouse_over: bool,
        is_mouse_down: bool,
        component:     &mut dyn ToolbarItemComponent)  {
        
        todo!();
        /*
            if (isMouseDown)
            g.fillAll (component.findColour (Toolbar::buttonMouseDownBackgroundColourId, true));
        else if (isMouseOver)
            g.fillAll (component.findColour (Toolbar::buttonMouseOverBackgroundColourId, true));
        */
    }
    
    fn paint_toolbar_button_label(&mut self, 
        g:         &mut Graphics,
        x:         i32,
        y:         i32,
        width:     i32,
        height:    i32,
        text:      &String,
        component: &mut dyn ToolbarItemComponent)  {
        
        todo!();
        /*
            g.setColour (component.findColour (Toolbar::labelTextColourId, true)
                        .withAlpha (component.isEnabled() ? 1.0f : 0.25f));

        auto fontHeight = jmin (14.0f, (float) height * 0.85f);
        g.setFont (fontHeight);

        g.drawFittedText (text,
                          x, y, width, height,
                          Justification::centred,
                          jmax (1, height / (int) fontHeight));
        */
    }
}

impl<'a> CallOutBoxLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn get_call_out_box_border_size(&mut self, _0: &CallOutBox) -> i32 {
        
        todo!();
        /*
            return 20;
        */
    }
    
    fn get_call_out_box_corner_size(&mut self, _0: &CallOutBox) -> f32 {
        
        todo!();
        /*
            return 9.0f;
        */
    }

    fn draw_call_out_box_background(&mut self, 
        box_:         &mut CallOutBox,
        g:            &mut Graphics,
        path:         &Path,
        cached_image: &mut Image)  {
        
        todo!();
        /*
            if (cachedImage.isNull())
        {
            cachedImage = Image (Image::ARGB, box.getWidth(), box.getHeight(), true);
            Graphics g2 (cachedImage);

            DropShadow (Colours::black.withAlpha (0.7f), 8, Point<int> (0, 2)).drawForPath (g2, path);
        }

        g.setColour (Colours::black);
        g.drawImageAt (cachedImage, 0, 0);

        g.setColour (Colour::greyLevel (0.23f).withAlpha (0.9f));
        g.fillPath (path);

        g.setColour (Colours::white.withAlpha (0.8f));
        g.strokePath (path, PathStrokeType (2.0f));
        */
    }
}

impl<'a> TableHeaderComponentLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_table_header_background(
        &mut self, 
        g:      &mut Graphics,
        header: &mut TableHeaderComponent

    ) {
        
        todo!();
        /*
            g.fillAll (Colours::white);

        auto area = header.getLocalBounds();
        area.removeFromTop (area.getHeight() / 2);

        auto backgroundColour = header.findColour (TableHeaderComponent::backgroundColourId);

        g.setGradientFill (ColourGradient (backgroundColour,
                                           0.0f, (float) area.getY(),
                                           backgroundColour.withMultipliedSaturation (.5f),
                                           0.0f, (float) area.getBottom(),
                                           false));
        g.fillRect (area);

        g.setColour (header.findColour (TableHeaderComponent::outlineColourId));
        g.fillRect (area.removeFromBottom (1));

        for (int i = header.getNumColumns (true); --i >= 0;)
            g.fillRect (header.getColumnPosition (i).removeFromRight (1));
        */
    }
    
    fn draw_table_header_column(
        &mut self, 
        g:             &mut Graphics,
        header:        &mut TableHeaderComponent,
        column_name:   &String,
        column_id:     i32,
        width:         i32,
        height:        i32,
        is_mouse_over: bool,
        is_mouse_down: bool,
        column_flags:  i32

    ) {
        
        todo!();
        /*
            auto highlightColour = header.findColour (TableHeaderComponent::highlightColourId);

        if (isMouseDown)
            g.fillAll (highlightColour);
        else if (isMouseOver)
            g.fillAll (highlightColour.withMultipliedAlpha (0.625f));

        Rectangle<int> area (width, height);
        area.reduce (4, 0);

        if ((columnFlags & (TableHeaderComponent::sortedForwards | TableHeaderComponent::sortedBackwards)) != 0)
        {
            Path sortArrow;
            sortArrow.addTriangle (0.0f, 0.0f,
                                   0.5f, (columnFlags & TableHeaderComponent::sortedForwards) != 0 ? -0.8f : 0.8f,
                                   1.0f, 0.0f);

            g.setColour (Colour (0x99000000));
            g.fillPath (sortArrow, sortArrow.getTransformToScaleToFit (area.removeFromRight (height / 2).reduced (2).toFloat(), true));
        }

        g.setColour (header.findColour (TableHeaderComponent::textColourId));
        g.setFont (Font ((float) height * 0.5f, Font::bold));
        g.drawFittedText (columnName, area, Justification::centredLeft, 1);
        */
    }
}

impl<'a> GroupComponentLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_group_component_outline(
        &mut self, 
        g:        &mut Graphics,
        width:    i32,
        height:   i32,
        text:     &String,
        position: &Justification,
        group:    &mut GroupComponent

    ) {
        
        todo!();
        /*
            const float textH = 15.0f;
        const float indent = 3.0f;
        const float textEdgeGap = 4.0f;
        auto cs = 5.0f;

        Font f (textH);

        Path p;
        auto x = indent;
        auto y = f.getAscent() - 3.0f;
        auto w = jmax (0.0f, (float) width - x * 2.0f);
        auto h = jmax (0.0f, (float) height - y  - indent);
        cs = jmin (cs, w * 0.5f, h * 0.5f);
        auto cs2 = 2.0f * cs;

        auto textW = text.isEmpty() ? 0
                                    : jlimit (0.0f,
                                              jmax (0.0f, w - cs2 - textEdgeGap * 2),
                                              (float) f.getStringWidth (text) + textEdgeGap * 2.0f);
        auto textX = cs + textEdgeGap;

        if (position.testFlags (Justification::horizontallyCentred))
            textX = cs + (w - cs2 - textW) * 0.5f;
        else if (position.testFlags (Justification::right))
            textX = w - cs - textW - textEdgeGap;

        p.startNewSubPath (x + textX + textW, y);
        p.lineTo (x + w - cs, y);

        p.addArc (x + w - cs2, y, cs2, cs2, 0, MathConstants<float>::halfPi);
        p.lineTo (x + w, y + h - cs);

        p.addArc (x + w - cs2, y + h - cs2, cs2, cs2, MathConstants<float>::halfPi, MathConstants<float>::pi);
        p.lineTo (x + cs, y + h);

        p.addArc (x, y + h - cs2, cs2, cs2, MathConstants<float>::pi, MathConstants<float>::pi * 1.5f);
        p.lineTo (x, y + cs);

        p.addArc (x, y, cs2, cs2, MathConstants<float>::pi * 1.5f, MathConstants<float>::twoPi);
        p.lineTo (x + textX, y);

        auto alpha = group.isEnabled() ? 1.0f : 0.5f;

        g.setColour (group.findColour (GroupComponent::outlineColourId)
                        .withMultipliedAlpha (alpha));

        g.strokePath (p, PathStrokeType (2.0f));

        g.setColour (group.findColour (GroupComponent::textColourId)
                        .withMultipliedAlpha (alpha));
        g.setFont (f);
        g.drawText (text,
                    roundToInt (x + textX), 0,
                    roundToInt (textW),
                    roundToInt (textH),
                    Justification::centred, true);
        */
    }
}

impl<'a> FilenameComponentLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn create_filename_component_browse_button(&mut self, text: &String) -> *mut Button {
        
        todo!();
        /*
            return new TextButton (text, TRANS("click to browse for a different file"));
        */
    }
    
    fn layout_filename_component(&mut self, 
        filename_comp: &mut FilenameComponent,
        filename_box:  *mut ComboBox,
        browse_button: *mut Button)  {
        
        todo!();
        /*
            if (browseButton == nullptr || filenameBox == nullptr)
            return;

        browseButton->setSize (80, filenameComp.getHeight());

        if (auto* tb = dynamic_cast<TextButton*> (browseButton))
            tb->changeWidthToFitText();

        browseButton->setTopRightPosition (filenameComp.getWidth(), 0);

        filenameBox->setBounds (0, 0, browseButton->getX(), filenameComp.getHeight());
        */
    }
}

impl<'a> PropertyComponentLookAndFeelMethods for LookAndFeel_V2<'a> {
 
    fn draw_property_panel_section_header(&mut self, 
        g:       &mut Graphics,
        name:    &String,
        is_open: bool,
        width:   i32,
        height:  i32)  {
        
        todo!();
        /*
            auto buttonSize = (float) height * 0.75f;
        auto buttonIndent = ((float) height - buttonSize) * 0.5f;

        drawTreeviewPlusMinusBox (g, Rectangle<float> (buttonIndent, buttonIndent, buttonSize, buttonSize), Colours::white, isOpen, false);

        auto textX = (int) (buttonIndent * 2.0f + buttonSize + 2.0f);

        g.setColour (Colours::black);
        g.setFont (Font ((float) height * 0.7f, Font::bold));
        g.drawText (name, textX, 0, width - textX - 4, height, Justification::centredLeft, true);
        */
    }
    
    fn draw_property_component_background(&mut self, 
        g:         &mut Graphics,
        width:     i32,
        height:    i32,
        component: &mut PropertyComponent)  {
        
        todo!();
        /*
            g.setColour (component.findColour (PropertyComponent::backgroundColourId));
        g.fillRect (0, 0, width, height - 1);
        */
    }
    
    fn draw_property_component_label(&mut self, 
        g:         &mut Graphics,
        _1:        i32,
        height:    i32,
        component: &mut PropertyComponent)  {
        
        todo!();
        /*
            g.setColour (component.findColour (PropertyComponent::labelTextColourId)
                        .withMultipliedAlpha (component.isEnabled() ? 1.0f : 0.6f));

        g.setFont ((float) jmin (height, 24) * 0.65f);

        auto r = getPropertyComponentContentPosition (component);

        g.drawFittedText (component.getName(),
                          3, r.getY(), r.getX() - 5, r.getHeight(),
                          Justification::centredLeft, 2);
        */
    }
    
    fn get_property_component_content_position(&mut self, component: &mut PropertyComponent) -> Rectangle<i32> {
        
        todo!();
        /*
            const int textW = jmin (200, component.getWidth() / 3);
        return Rectangle<int> (textW, 1, component.getWidth() - textW - 1, component.getHeight() - 3);
        */
    }
    
    fn get_property_panel_section_header_height(&mut self, section_title: &String) -> i32 {
        
        todo!();
        /*
            return sectionTitle.isEmpty() ? 0 : 22;
        */
    }
}

impl<'a> TabbedButtonBarLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn get_tab_button_overlap(&mut self, tab_depth: i32) -> i32 {
        
        todo!();
        /*
            return 1 + tabDepth / 3;
        */
    }
    
    fn get_tab_button_space_around_image(&mut self) -> i32 {
        
        todo!();
        /*
            return 4;
        */
    }
    
    fn get_tab_button_best_width(&mut self, 
        button:    &mut TabBarButton,
        tab_depth: i32) -> i32 {
        
        todo!();
        /*
            int width = Font ((float) tabDepth * 0.6f).getStringWidth (button.getButtonText().trim())
                       + getTabButtonOverlap (tabDepth) * 2;

        if (auto* extraComponent = button.getExtraComponent())
            width += button.getTabbedButtonBar().isVertical() ? extraComponent->getHeight()
                                                              : extraComponent->getWidth();

        return jlimit (tabDepth * 2, tabDepth * 8, width);
        */
    }
    
    fn get_tab_button_extra_component_bounds(&mut self, 
        button:    &TabBarButton,
        text_area: &mut Rectangle<i32>,
        comp:      &mut Component) -> Rectangle<i32> {
        
        todo!();
        /*
            Rectangle<int> extraComp;

        auto orientation = button.getTabbedButtonBar().getOrientation();

        if (button.getExtraComponentPlacement() == TabBarButton::beforeText)
        {
            switch (orientation)
            {
                case TabbedButtonBar::TabsAtBottom:
                case TabbedButtonBar::TabsAtTop:     extraComp = textArea.removeFromLeft   (comp.getWidth()); break;
                case TabbedButtonBar::TabsAtLeft:    extraComp = textArea.removeFromBottom (comp.getHeight()); break;
                case TabbedButtonBar::TabsAtRight:   extraComp = textArea.removeFromTop    (comp.getHeight()); break;
                default:                             jassertfalse; break;
            }
        }
        else
        {
            switch (orientation)
            {
                case TabbedButtonBar::TabsAtBottom:
                case TabbedButtonBar::TabsAtTop:     extraComp = textArea.removeFromRight  (comp.getWidth()); break;
                case TabbedButtonBar::TabsAtLeft:    extraComp = textArea.removeFromTop    (comp.getHeight()); break;
                case TabbedButtonBar::TabsAtRight:   extraComp = textArea.removeFromBottom (comp.getHeight()); break;
                default:                             jassertfalse; break;
            }
        }

        return extraComp;
        */
    }
    
    fn create_tab_button_shape(&mut self, 
        button:        &mut TabBarButton,
        p:             &mut Path,
        is_mouse_over: bool,
        is_mouse_down: bool)  {
        
        todo!();
        /*
            auto activeArea = button.getActiveArea();
        auto w = (float) activeArea.getWidth();
        auto h = (float) activeArea.getHeight();

        auto length = w;
        auto depth = h;

        if (button.getTabbedButtonBar().isVertical())
            std::swap (length, depth);

        const float indent = (float) getTabButtonOverlap ((int) depth);
        const float overhang = 4.0f;

        switch (button.getTabbedButtonBar().getOrientation())
        {
            case TabbedButtonBar::TabsAtLeft:
                p.startNewSubPath (w, 0.0f);
                p.lineTo (0.0f, indent);
                p.lineTo (0.0f, h - indent);
                p.lineTo (w, h);
                p.lineTo (w + overhang, h + overhang);
                p.lineTo (w + overhang, -overhang);
                break;

            case TabbedButtonBar::TabsAtRight:
                p.startNewSubPath (0.0f, 0.0f);
                p.lineTo (w, indent);
                p.lineTo (w, h - indent);
                p.lineTo (0.0f, h);
                p.lineTo (-overhang, h + overhang);
                p.lineTo (-overhang, -overhang);
                break;

            case TabbedButtonBar::TabsAtBottom:
                p.startNewSubPath (0.0f, 0.0f);
                p.lineTo (indent, h);
                p.lineTo (w - indent, h);
                p.lineTo (w, 0.0f);
                p.lineTo (w + overhang, -overhang);
                p.lineTo (-overhang, -overhang);
                break;

            case TabbedButtonBar::TabsAtTop:
            default:
                p.startNewSubPath (0.0f, h);
                p.lineTo (indent, 0.0f);
                p.lineTo (w - indent, 0.0f);
                p.lineTo (w, h);
                p.lineTo (w + overhang, h + overhang);
                p.lineTo (-overhang, h + overhang);
                break;
        }

        p.closeSubPath();

        p = p.createPathWithRoundedCorners (3.0f);
        */
    }
    
    fn fill_tab_button_shape(&mut self, 
        button:        &mut TabBarButton,
        g:             &mut Graphics,
        path:          &Path,
        is_mouse_over: bool,
        is_mouse_down: bool)  {
        
        todo!();
        /*
            auto tabBackground = button.getTabBackgroundColour();
        const bool isFrontTab = button.isFrontTab();

        g.setColour (isFrontTab ? tabBackground
                                : tabBackground.withMultipliedAlpha (0.9f));

        g.fillPath (path);

        g.setColour (button.findColour (isFrontTab ? TabbedButtonBar::frontOutlineColourId
                                                   : TabbedButtonBar::tabOutlineColourId, false)
                        .withMultipliedAlpha (button.isEnabled() ? 1.0f : 0.5f));

        g.strokePath (path, PathStrokeType (isFrontTab ? 1.0f : 0.5f));
        */
    }
    
    fn get_tab_button_font(&mut self, 
        _0:     &mut TabBarButton,
        height: f32) -> Font {
        
        todo!();
        /*
            return { height * 0.6f };
        */
    }
    
    fn draw_tab_button_text(&mut self, 
        button:        &mut TabBarButton,
        g:             &mut Graphics,
        is_mouse_over: bool,
        is_mouse_down: bool)  {
        
        todo!();
        /*
            auto area = button.getTextArea().toFloat();

        auto length = area.getWidth();
        auto depth  = area.getHeight();

        if (button.getTabbedButtonBar().isVertical())
            std::swap (length, depth);

        Font font (getTabButtonFont (button, depth));
        font.setUnderline (button.hasKeyboardFocus (false));

        AffineTransform t;

        switch (button.getTabbedButtonBar().getOrientation())
        {
            case TabbedButtonBar::TabsAtLeft:   t = t.rotated (MathConstants<float>::pi * -0.5f).translated (area.getX(), area.getBottom()); break;
            case TabbedButtonBar::TabsAtRight:  t = t.rotated (MathConstants<float>::pi *  0.5f).translated (area.getRight(), area.getY()); break;
            case TabbedButtonBar::TabsAtTop:
            case TabbedButtonBar::TabsAtBottom: t = t.translated (area.getX(), area.getY()); break;
            default:                            jassertfalse; break;
        }

        Colour col;

        if (button.isFrontTab() && (button.isColourSpecified (TabbedButtonBar::frontTextColourId)
                                        || isColourSpecified (TabbedButtonBar::frontTextColourId)))
            col = findColour (TabbedButtonBar::frontTextColourId);
        else if (button.isColourSpecified (TabbedButtonBar::tabTextColourId)
                     || isColourSpecified (TabbedButtonBar::tabTextColourId))
            col = findColour (TabbedButtonBar::tabTextColourId);
        else
            col = button.getTabBackgroundColour().contrasting();

        auto alpha = button.isEnabled() ? ((isMouseOver || isMouseDown) ? 1.0f : 0.8f) : 0.3f;

        g.setColour (col.withMultipliedAlpha (alpha));
        g.setFont (font);
        g.addTransform (t);

        g.drawFittedText (button.getButtonText().trim(),
                          0, 0, (int) length, (int) depth,
                          Justification::centred,
                          jmax (1, ((int) depth) / 12));
        */
    }
    
    fn draw_tab_button(&mut self, 
        button:        &mut TabBarButton,
        g:             &mut Graphics,
        is_mouse_over: bool,
        is_mouse_down: bool)  {
        
        todo!();
        /*
            Path tabShape;
        createTabButtonShape (button, tabShape, isMouseOver, isMouseDown);

        auto activeArea = button.getActiveArea();
        tabShape.applyTransform (AffineTransform::translation ((float) activeArea.getX(),
                                                               (float) activeArea.getY()));

        DropShadow (Colours::black.withAlpha (0.5f), 2, Point<int> (0, 1)).drawForPath (g, tabShape);

        fillTabButtonShape (button, g, tabShape, isMouseOver, isMouseDown);
        drawTabButtonText (button, g, isMouseOver, isMouseDown);
        */
    }
    
    fn draw_tabbed_button_bar_background(&mut self, 
        _0: &mut TabbedButtonBar,
        _1: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    fn draw_tab_area_behind_front_button(&mut self, 
        bar: &mut TabbedButtonBar,
        g:   &mut Graphics,
        w:   i32,
        h:   i32)  {
        
        todo!();
        /*
            auto shadowSize = 0.2f;

        Rectangle<int> shadowRect, line;
        ColourGradient gradient (Colours::black.withAlpha (bar.isEnabled() ? 0.25f : 0.15f), 0, 0,
                                 Colours::transparentBlack, 0, 0, false);

        switch (bar.getOrientation())
        {
            case TabbedButtonBar::TabsAtLeft:
                gradient.point1.x = (float) w;
                gradient.point2.x = (float) w * (1.0f - shadowSize);
                shadowRect.setBounds ((int) gradient.point2.x, 0, w - (int) gradient.point2.x, h);
                line.setBounds (w - 1, 0, 1, h);
                break;

            case TabbedButtonBar::TabsAtRight:
                gradient.point2.x = (float) w * shadowSize;
                shadowRect.setBounds (0, 0, (int) gradient.point2.x, h);
                line.setBounds (0, 0, 1, h);
                break;

            case TabbedButtonBar::TabsAtTop:
                gradient.point1.y = (float) h;
                gradient.point2.y = (float) h * (1.0f - shadowSize);
                shadowRect.setBounds (0, (int) gradient.point2.y, w, h - (int) gradient.point2.y);
                line.setBounds (0, h - 1, w, 1);
                break;

            case TabbedButtonBar::TabsAtBottom:
                gradient.point2.y = (float) h * shadowSize;
                shadowRect.setBounds (0, 0, w, (int) gradient.point2.y);
                line.setBounds (0, 0, w, 1);
                break;

            default: break;
        }

        g.setGradientFill (gradient);
        g.fillRect (shadowRect.expanded (2, 2));

        g.setColour (Colour (0x80000000));
        g.fillRect (line);
        */
    }
    
    fn create_tab_bar_extras_button(&mut self) -> *mut Button {
        
        todo!();
        /*
            auto thickness = 7.0f;
        auto indent = 22.0f;

        Path p;
        p.addEllipse (-10.0f, -10.0f, 120.0f, 120.0f);

        DrawablePath ellipse;
        ellipse.setPath (p);
        ellipse.setFill (Colour (0x99ffffff));

        p.clear();
        p.addEllipse (0.0f, 0.0f, 100.0f, 100.0f);
        p.addRectangle (indent, 50.0f - thickness, 100.0f - indent * 2.0f, thickness * 2.0f);
        p.addRectangle (50.0f - thickness, indent, thickness * 2.0f, 50.0f - indent - thickness);
        p.addRectangle (50.0f - thickness, 50.0f + thickness, thickness * 2.0f, 50.0f - indent - thickness);
        p.setUsingNonZeroWinding (false);

        DrawablePath dp;
        dp.setPath (p);
        dp.setFill (Colour (0x59000000));

        DrawableComposite normalImage;
        normalImage.addAndMakeVisible (ellipse.createCopy().release());
        normalImage.addAndMakeVisible (dp.createCopy().release());

        dp.setFill (Colour (0xcc000000));

        DrawableComposite overImage;
        overImage.addAndMakeVisible (ellipse.createCopy().release());
        overImage.addAndMakeVisible (dp.createCopy().release());

        auto db = new DrawableButton ("tabs", DrawableButton::ImageFitted);
        db->setImages (&normalImage, &overImage, nullptr);
        return db;
        */
    }
}

impl<'a> TooltipWindowLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn get_tooltip_bounds(&mut self, 
        tip_text:    &String,
        screen_pos:  Point<i32>,
        parent_area: Rectangle<i32>) -> Rectangle<i32> {
        
        todo!();
        /*
            const TextLayout tl (LookAndFeelHelpers::layoutTooltipText (tipText, Colours::black));

        auto w = (int) (tl.getWidth() + 14.0f);
        auto h = (int) (tl.getHeight() + 6.0f);

        return Rectangle<int> (screenPos.x > parentArea.getCentreX() ? screenPos.x - (w + 12) : screenPos.x + 24,
                               screenPos.y > parentArea.getCentreY() ? screenPos.y - (h + 6)  : screenPos.y + 6,
                               w, h)
                 .constrainedWithin (parentArea);
        */
    }
    
    fn draw_tooltip(&mut self, 
        g:      &mut Graphics,
        text:   &String,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            g.fillAll (findColour (TooltipWindow::backgroundColourId));

       #if ! ALOE_MAC // The mac windows already have a non-optional 1 pix outline, so don't double it here..
        g.setColour (findColour (TooltipWindow::outlineColourId));
        g.drawRect (0, 0, width, height, 1);
       #endif

        LookAndFeelHelpers::layoutTooltipText (text, findColour (TooltipWindow::textColourId))
            .draw (g, Rectangle<float> ((float) width, (float) height));
        */
    }
}

impl<'a> DocumentWindowLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_document_window_title_bar(&mut self, 
        window:                  &mut DocumentWindow,
        g:                       &mut Graphics,
        w:                       i32,
        h:                       i32,
        title_spacex:            i32,
        title_spacew:            i32,
        icon:                    *const Image,
        draw_title_text_on_left: bool)  {
        
        todo!();
        /*
            if (w * h == 0)
            return;

        const bool isActive = window.isActiveWindow();

        g.setGradientFill (ColourGradient::vertical (window.getBackgroundColour(), 0,
                                                     window.getBackgroundColour().contrasting (isActive ? 0.15f : 0.05f), (float) h));
        g.fillAll();

        Font font ((float) h * 0.65f, Font::bold);
        g.setFont (font);

        int textW = font.getStringWidth (window.getName());
        int iconW = 0;
        int iconH = 0;

        if (icon != nullptr)
        {
            iconH = (int) font.getHeight();
            iconW = icon->getWidth() * iconH / icon->getHeight() + 4;
        }

        textW = jmin (titleSpaceW, textW + iconW);
        int textX = drawTitleTextOnLeft ? titleSpaceX
                                        : jmax (titleSpaceX, (w - textW) / 2);

        if (textX + textW > titleSpaceX + titleSpaceW)
            textX = titleSpaceX + titleSpaceW - textW;

        if (icon != nullptr)
        {
            g.setOpacity (isActive ? 1.0f : 0.6f);
            g.drawImageWithin (*icon, textX, (h - iconH) / 2, iconW, iconH,
                               RectanglePlacement::centred, false);
            textX += iconW;
            textW -= iconW;
        }

        if (window.isColourSpecified (DocumentWindow::textColourId) || isColourSpecified (DocumentWindow::textColourId))
            g.setColour (window.findColour (DocumentWindow::textColourId));
        else
            g.setColour (window.getBackgroundColour().contrasting (isActive ? 0.7f : 0.4f));

        g.drawText (window.getName(), textX, 0, textW, h, Justification::centredLeft, true);
        */
    }
    
    fn create_document_window_button(&mut self, button_type: i32) -> *mut Button {
        
        todo!();
        /*
            Path shape;
        const float crossThickness = 0.25f;

        if (buttonType == DocumentWindow::closeButton)
        {
            shape.addLineSegment (Line<float> (0.0f, 0.0f, 1.0f, 1.0f), crossThickness * 1.4f);
            shape.addLineSegment (Line<float> (1.0f, 0.0f, 0.0f, 1.0f), crossThickness * 1.4f);

            return new GlassWindowButton ("close", Colour (0xffdd1100), shape, shape);
        }

        if (buttonType == DocumentWindow::minimiseButton)
        {
            shape.addLineSegment (Line<float> (0.0f, 0.5f, 1.0f, 0.5f), crossThickness);

            return new GlassWindowButton ("minimise", Colour (0xffaa8811), shape, shape);
        }

        if (buttonType == DocumentWindow::maximiseButton)
        {
            shape.addLineSegment (Line<float> (0.5f, 0.0f, 0.5f, 1.0f), crossThickness);
            shape.addLineSegment (Line<float> (0.0f, 0.5f, 1.0f, 0.5f), crossThickness);

            Path fullscreenShape;
            fullscreenShape.startNewSubPath (45.0f, 100.0f);
            fullscreenShape.lineTo (0.0f, 100.0f);
            fullscreenShape.lineTo (0.0f, 0.0f);
            fullscreenShape.lineTo (100.0f, 0.0f);
            fullscreenShape.lineTo (100.0f, 45.0f);
            fullscreenShape.addRectangle (45.0f, 45.0f, 100.0f, 100.0f);
            PathStrokeType (30.0f).createStrokedPath (fullscreenShape, fullscreenShape);

            return new GlassWindowButton ("maximise", Colour (0xff119911), shape, fullscreenShape);
        }

        jassertfalse;
        return nullptr;
        */
    }
    
    fn position_document_window_buttons(&mut self, 
        _0:                                 &mut DocumentWindow,
        title_barx:                         i32,
        title_bary:                         i32,
        title_barw:                         i32,
        title_barh:                         i32,
        minimise_button:                    *mut Button,
        maximise_button:                    *mut Button,
        close_button:                       *mut Button,
        position_title_bar_buttons_on_left: bool)  {
        
        todo!();
        /*
            const int buttonW = titleBarH - titleBarH / 8;

        int x = positionTitleBarButtonsOnLeft ? titleBarX + 4
                                              : titleBarX + titleBarW - buttonW - buttonW / 4;

        if (closeButton != nullptr)
        {
            closeButton->setBounds (x, titleBarY, buttonW, titleBarH);
            x += positionTitleBarButtonsOnLeft ? buttonW : -(buttonW + buttonW / 4);
        }

        if (positionTitleBarButtonsOnLeft)
            std::swap (minimiseButton, maximiseButton);

        if (maximiseButton != nullptr)
        {
            maximiseButton->setBounds (x, titleBarY, buttonW, titleBarH);
            x += positionTitleBarButtonsOnLeft ? buttonW : -buttonW;
        }

        if (minimiseButton != nullptr)
            minimiseButton->setBounds (x, titleBarY, buttonW, titleBarH);
        */
    }
}

impl<'a> ResizableWindowLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_corner_resizer(&mut self, 
        g:                 &mut Graphics,
        w:                 i32,
        h:                 i32,
        is_mouse_over:     bool,
        is_mouse_dragging: bool)  {
        
        todo!();
        /*
            auto lineThickness = jmin ((float) w, (float) h) * 0.075f;

        for (float i = 0.0f; i < 1.0f; i += 0.3f)
        {
            g.setColour (Colours::lightgrey);

            g.drawLine ((float) w * i,
                        (float) h + 1.0f,
                        (float) w + 1.0f,
                        (float) h * i,
                        lineThickness);

            g.setColour (Colours::darkgrey);

            g.drawLine ((float) w * i + lineThickness,
                        (float) h + 1.0f,
                        (float) w + 1.0f,
                        (float) h * i + lineThickness,
                        lineThickness);
        }
        */
    }
    
    fn draw_resizable_frame(&mut self, 
        g:      &mut Graphics,
        w:      i32,
        h:      i32,
        border: &BorderSize<i32>)  {
        
        todo!();
        /*
            if (! border.isEmpty())
        {
            const Rectangle<int> fullSize (0, 0, w, h);
            auto centreArea = border.subtractedFrom (fullSize);

            Graphics::ScopedSaveState ss (g);

            g.excludeClipRegion (centreArea);

            g.setColour (Colour (0x50000000));
            g.drawRect (fullSize);

            g.setColour (Colour (0x19000000));
            g.drawRect (centreArea.expanded (1, 1));
        }
        */
    }
    
    fn fill_resizable_window_background(&mut self, 
        g:      &mut Graphics,
        w:      i32,
        h:      i32,
        border: &BorderSize<i32>,
        window: &mut ResizableWindow)  {
        
        todo!();
        /*
            g.fillAll (window.getBackgroundColour());
        */
    }
    
    fn draw_resizable_window_border(&mut self, 
        _0:     &mut Graphics,
        w:      i32,
        h:      i32,
        border: &BorderSize<i32>,
        _4:     &mut ResizableWindow)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> SliderLookAndFeelMethods for LookAndFeel_V2<'a> {

}

impl<'a> GetSliderLayout for LookAndFeel_V2<'a> {

    fn get_slider_layout(&mut self, slider: &mut Slider) 
    -> SliderLayout 
    {
        todo!();

        /*
            // 1. compute the actually visible textBox size from the slider textBox size and some additional constraints

        int minXSpace = 0;
        int minYSpace = 0;

        auto textBoxPos = slider.getTextBoxPosition();

        if (textBoxPos == Slider::TextBoxLeft || textBoxPos == Slider::TextBoxRight)
            minXSpace = 30;
        else
            minYSpace = 15;

        auto localBounds = slider.getLocalBounds();

        auto textBoxWidth  = jmax (0, jmin (slider.getTextBoxWidth(),  localBounds.getWidth() - minXSpace));
        auto textBoxHeight = jmax (0, jmin (slider.getTextBoxHeight(), localBounds.getHeight() - minYSpace));

        Slider::SliderLayout layout;

        // 2. set the textBox bounds

        if (textBoxPos != Slider::NoTextBox)
        {
            if (slider.isBar())
            {
                layout.textBoxBounds = localBounds;
            }
            else
            {
                layout.textBoxBounds.setWidth (textBoxWidth);
                layout.textBoxBounds.setHeight (textBoxHeight);

                if (textBoxPos == Slider::TextBoxLeft)           layout.textBoxBounds.setX (0);
                else if (textBoxPos == Slider::TextBoxRight)     layout.textBoxBounds.setX (localBounds.getWidth() - textBoxWidth);
                else /* above or below -> centre horizontally */ layout.textBoxBounds.setX ((localBounds.getWidth() - textBoxWidth) / 2);

                if (textBoxPos == Slider::TextBoxAbove)          layout.textBoxBounds.setY (0);
                else if (textBoxPos == Slider::TextBoxBelow)     layout.textBoxBounds.setY (localBounds.getHeight() - textBoxHeight);
                else /* left or right -> centre vertically */    layout.textBoxBounds.setY ((localBounds.getHeight() - textBoxHeight) / 2);
            }
        }

        // 3. set the slider bounds

        layout.sliderBounds = localBounds;

        if (slider.isBar())
        {
            layout.sliderBounds.reduce (1, 1);   // bar border
        }
        else
        {
            if (textBoxPos == Slider::TextBoxLeft)       layout.sliderBounds.removeFromLeft (textBoxWidth);
            else if (textBoxPos == Slider::TextBoxRight) layout.sliderBounds.removeFromRight (textBoxWidth);
            else if (textBoxPos == Slider::TextBoxAbove) layout.sliderBounds.removeFromTop (textBoxHeight);
            else if (textBoxPos == Slider::TextBoxBelow) layout.sliderBounds.removeFromBottom (textBoxHeight);

            const int thumbIndent = getSliderThumbRadius (slider);

            if (slider.isHorizontal())    layout.sliderBounds.reduce (thumbIndent, 0);
            else if (slider.isVertical()) layout.sliderBounds.reduce (0, thumbIndent);
        }

        return layout;
        */
    }
}

impl<'a> GetSliderEffect for LookAndFeel_V2<'a> {

    fn get_slider_effect(&mut self, _0: &mut Slider) -> *mut dyn ImageEffectFilter {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

impl<'a> CreateSliderTextBox for LookAndFeel_V2<'a> {

    fn create_slider_text_box(&mut self, slider: &mut Slider) -> *mut Label {
        
        todo!();
        /*
            auto l = new SliderLabelComp();

        l->setJustificationType (Justification::centred);
        l->setKeyboardType (TextInputTarget::decimalKeyboard);

        l->setColour (Label::textColourId, slider.findColour (Slider::textBoxTextColourId));
        l->setColour (Label::backgroundColourId,
                      (slider.getSliderStyle() == Slider::LinearBar || slider.getSliderStyle() == Slider::LinearBarVertical)
                                ? Colours::transparentBlack
                                : slider.findColour (Slider::textBoxBackgroundColourId));
        l->setColour (Label::outlineColourId, slider.findColour (Slider::textBoxOutlineColourId));
        l->setColour (TextEditor::textColourId, slider.findColour (Slider::textBoxTextColourId));
        l->setColour (TextEditor::backgroundColourId,
                      slider.findColour (Slider::textBoxBackgroundColourId)
                            .withAlpha ((slider.getSliderStyle() == Slider::LinearBar || slider.getSliderStyle() == Slider::LinearBarVertical)
                                            ? 0.7f : 1.0f));
        l->setColour (TextEditor::outlineColourId, slider.findColour (Slider::textBoxOutlineColourId));
        l->setColour (TextEditor::highlightColourId, slider.findColour (Slider::textBoxHighlightColourId));

        return l;
        */
    }
}

impl<'a> CreateSliderButton for LookAndFeel_V2<'a> {

    fn create_slider_button(
        &mut self, 
        _0:           &mut Slider,
        is_increment: bool

    ) -> *mut Button {
        
        todo!();
        /*
            return new TextButton (isIncrement ? "+" : "-", String());
        */
    }
}

impl<'a> DrawRotarySlider for LookAndFeel_V2<'a> {

    fn draw_rotary_slider(
        &mut self, 
        g:                  &mut Graphics,
        x:                  i32,
        y:                  i32,
        width:              i32,
        height:             i32,
        slider_pos:         f32,
        rotary_start_angle: f32,
        rotary_end_angle:   f32,
        slider:             &mut Slider

    ) {
        
        todo!();
        /*
            const float radius = jmin ((float) width * 0.5f, (float) height * 0.5f) - 2.0f;
        const float centreX = (float) x + (float) width * 0.5f;
        const float centreY = (float) y + (float) height * 0.5f;
        const float rx = centreX - radius;
        const float ry = centreY - radius;
        const float rw = radius * 2.0f;
        const float angle = rotaryStartAngle + sliderPos * (rotaryEndAngle - rotaryStartAngle);
        const bool isMouseOver = slider.isMouseOverOrDragging() && slider.isEnabled();

        if (radius > 12.0f)
        {
            if (slider.isEnabled())
                g.setColour (slider.findColour (Slider::rotarySliderFillColourId).withAlpha (isMouseOver ? 1.0f : 0.7f));
            else
                g.setColour (Colour (0x80808080));

            const float thickness = 0.7f;

            {
                Path filledArc;
                filledArc.addPieSegment (rx, ry, rw, rw, rotaryStartAngle, angle, thickness);
                g.fillPath (filledArc);
            }

            {
                const float innerRadius = radius * 0.2f;
                Path p;
                p.addTriangle (-innerRadius, 0.0f,
                               0.0f, -radius * thickness * 1.1f,
                               innerRadius, 0.0f);

                p.addEllipse (-innerRadius, -innerRadius, innerRadius * 2.0f, innerRadius * 2.0f);

                g.fillPath (p, AffineTransform::rotation (angle).translated (centreX, centreY));
            }

            if (slider.isEnabled())
                g.setColour (slider.findColour (Slider::rotarySliderOutlineColourId));
            else
                g.setColour (Colour (0x80808080));

            Path outlineArc;
            outlineArc.addPieSegment (rx, ry, rw, rw, rotaryStartAngle, rotaryEndAngle, thickness);
            outlineArc.closeSubPath();

            g.strokePath (outlineArc, PathStrokeType (slider.isEnabled() ? (isMouseOver ? 2.0f : 1.2f) : 0.3f));
        }
        else
        {
            if (slider.isEnabled())
                g.setColour (slider.findColour (Slider::rotarySliderFillColourId).withAlpha (isMouseOver ? 1.0f : 0.7f));
            else
                g.setColour (Colour (0x80808080));

            Path p;
            p.addEllipse (-0.4f * rw, -0.4f * rw, rw * 0.8f, rw * 0.8f);
            PathStrokeType (rw * 0.1f).createStrokedPath (p, p);

            p.addLineSegment (Line<float> (0.0f, 0.0f, 0.0f, -radius), rw * 0.2f);

            g.fillPath (p, AffineTransform::rotation (angle).translated (centreX, centreY));
        }
        */
    }
}

impl<'a> GetSliderThumbRadius for LookAndFeel_V2<'a> {

    fn get_slider_thumb_radius(&mut self, slider: &mut Slider) -> i32 {
        
        todo!();
        /*
            return jmin (7,
                     slider.getHeight() / 2,
                     slider.getWidth() / 2) + 2;
        */
    }
}

impl<'a> DrawLinearSliderThumb for LookAndFeel_V2<'a> {

    fn draw_linear_slider_thumb(
        &mut self, 
        g:              &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        style:          SliderStyle,
        slider:         &mut Slider

    ) {
        
        todo!();
        /*
            auto sliderRadius = (float) (getSliderThumbRadius (slider) - 2);

        auto knobColour = LookAndFeelHelpers::createBaseColour (slider.findColour (Slider::thumbColourId),
                                                                slider.hasKeyboardFocus (false) && slider.isEnabled(),
                                                                slider.isMouseOverOrDragging() && slider.isEnabled(),
                                                                slider.isMouseButtonDown() && slider.isEnabled());

        const float outlineThickness = slider.isEnabled() ? 0.8f : 0.3f;

        if (style == Slider::LinearHorizontal || style == Slider::LinearVertical)
        {
            float kx, ky;

            if (style == Slider::LinearVertical)
            {
                kx = (float) x + (float) width * 0.5f;
                ky = sliderPos;
            }
            else
            {
                kx = sliderPos;
                ky = (float) y + (float) height * 0.5f;
            }

            drawGlassSphere (g,
                             kx - sliderRadius,
                             ky - sliderRadius,
                             sliderRadius * 2.0f,
                             knobColour, outlineThickness);
        }
        else
        {
            if (style == Slider::ThreeValueVertical)
            {
                drawGlassSphere (g, (float) x + (float) width * 0.5f - sliderRadius,
                                 sliderPos - sliderRadius,
                                 sliderRadius * 2.0f,
                                 knobColour, outlineThickness);
            }
            else if (style == Slider::ThreeValueHorizontal)
            {
                drawGlassSphere (g,sliderPos - sliderRadius,
                                 (float) y + (float) height * 0.5f - sliderRadius,
                                 sliderRadius * 2.0f,
                                 knobColour, outlineThickness);
            }

            if (style == Slider::TwoValueVertical || style == Slider::ThreeValueVertical)
            {
                auto sr = jmin (sliderRadius, (float) width * 0.4f);

                drawGlassPointer (g, jmax (0.0f, (float) x + (float) width * 0.5f - sliderRadius * 2.0f),
                                  minSliderPos - sliderRadius,
                                  sliderRadius * 2.0f, knobColour, outlineThickness, 1);

                drawGlassPointer (g,
                                  jmin ((float) x + (float) width - sliderRadius * 2.0f,
                                        (float) x + (float) width * 0.5f),
                                  maxSliderPos - sr,
                                  sliderRadius * 2.0f,
                                  knobColour,
                                  outlineThickness,
                                  3);
            }
            else if (style == Slider::TwoValueHorizontal || style == Slider::ThreeValueHorizontal)
            {
                auto sr = jmin (sliderRadius, (float) height * 0.4f);

                drawGlassPointer (g, minSliderPos - sr,
                                  jmax (0.0f, (float) y + (float) height * 0.5f - sliderRadius * 2.0f),
                                  sliderRadius * 2.0f, knobColour, outlineThickness, 2);

                drawGlassPointer (g,
                                  maxSliderPos - sliderRadius,
                                  jmin ((float) y + (float) height - sliderRadius * 2.0f,
                                        (float) y + (float) height * 0.5f),
                                  sliderRadius * 2.0f,
                                  knobColour,
                                  outlineThickness,
                                  4);
            }
        }
        */
    }
}

impl<'a> DrawLinearSliderBackground for LookAndFeel_V2<'a> {

    fn draw_linear_slider_background(
        &mut self, 
        g:              &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        style:          SliderStyle,
        slider:         &mut Slider

    ) {
        
        todo!();
        /*
            auto sliderRadius = (float) (getSliderThumbRadius (slider) - 2);
        auto trackColour = slider.findColour (Slider::trackColourId);
        auto gradCol1 = trackColour.overlaidWith (Colours::black.withAlpha (slider.isEnabled() ? 0.25f : 0.13f));
        auto gradCol2 = trackColour.overlaidWith (Colour (0x14000000));

        Path indent;

        if (slider.isHorizontal())
        {
            const float iy = (float) y + (float) height * 0.5f - sliderRadius * 0.5f;
            const float ih = sliderRadius;

            g.setGradientFill (ColourGradient::vertical (gradCol1, iy, gradCol2, iy + ih));

            indent.addRoundedRectangle ((float) x - sliderRadius * 0.5f, iy,
                                        (float) width + sliderRadius, ih,
                                        5.0f);
        }
        else
        {
            const float ix = (float) x + (float) width * 0.5f - sliderRadius * 0.5f;
            const float iw = sliderRadius;

            g.setGradientFill (ColourGradient::horizontal (gradCol1, ix, gradCol2, ix + iw));

            indent.addRoundedRectangle (ix, (float) y - sliderRadius * 0.5f,
                                        iw, (float) height + sliderRadius,
                                        5.0f);
        }

        g.fillPath (indent);

        g.setColour (Colour (0x4c000000));
        g.strokePath (indent, PathStrokeType (0.5f));
        */
    }
}

impl<'a> DrawLinearSlider for LookAndFeel_V2<'a> {

    fn draw_linear_slider(
        &mut self, 
        g:              &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        style:          SliderStyle,
        slider:         &mut Slider

    ) {
        
        todo!();
        /*
            g.fillAll (slider.findColour (Slider::backgroundColourId));

        if (style == Slider::LinearBar || style == Slider::LinearBarVertical)
        {
            const bool isMouseOver = slider.isMouseOverOrDragging() && slider.isEnabled();

            auto baseColour = LookAndFeelHelpers::createBaseColour (slider.findColour (Slider::thumbColourId)
                                                                          .withMultipliedSaturation (slider.isEnabled() ? 1.0f : 0.5f),
                                                                    false, isMouseOver,
                                                                    isMouseOver || slider.isMouseButtonDown());

            drawShinyButtonShape (g,
                                  (float) x,
                                  style == Slider::LinearBarVertical ? sliderPos
                                                                     : (float) y,
                                  style == Slider::LinearBarVertical ? (float) width
                                                                     : (sliderPos - (float) x),
                                  style == Slider::LinearBarVertical ? ((float) height - sliderPos)
                                                                     : (float) height, 0.0f,
                                  baseColour,
                                  slider.isEnabled() ? 0.9f : 0.3f,
                                  true, true, true, true);
        }
        else
        {
            drawLinearSliderBackground (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
            drawLinearSliderThumb (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
        }
        */
    }
}

impl<'a> GetSliderPopupPlacement for LookAndFeel_V2<'a> {

    fn get_slider_popup_placement(&mut self, _0: &mut Slider) -> i32 {
        
        todo!();
        /*
            return BubbleComponent::above
                | BubbleComponent::below
                | BubbleComponent::left
                | BubbleComponent::right;
        */
    }
}

impl<'a> GetSliderPopupFont for LookAndFeel_V2<'a> {

    fn get_slider_popup_font(&mut self, _0: &mut Slider) -> Font {
        
        todo!();
        /*
            return Font (15.0f, Font::bold);
        */
    }
}

impl<'a> LabelLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_label(&mut self, 
        g:     &mut Graphics,
        label: &mut Label)  {
        
        todo!();
        /*
            g.fillAll (label.findColour (Label::backgroundColourId));

        if (! label.isBeingEdited())
        {
            auto alpha = label.isEnabled() ? 1.0f : 0.5f;
            const Font font (getLabelFont (label));

            g.setColour (label.findColour (Label::textColourId).withMultipliedAlpha (alpha));
            g.setFont (font);

            auto textArea = getLabelBorderSize (label).subtractedFrom (label.getLocalBounds());

            g.drawFittedText (label.getText(), textArea, label.getJustificationType(),
                              jmax (1, (int) ((float) textArea.getHeight() / font.getHeight())),
                              label.getMinimumHorizontalScale());

            g.setColour (label.findColour (Label::outlineColourId).withMultipliedAlpha (alpha));
        }
        else if (label.isEnabled())
        {
            g.setColour (label.findColour (Label::outlineColourId));
        }

        g.drawRect (label.getLocalBounds());
        */
    }

    fn get_label_font(&mut self, label: &mut Label) -> Font {
        
        todo!();
        /*
            return label.getFont();
        */
    }
    
    fn get_label_border_size(&mut self, label: &mut Label) -> BorderSize<i32> {
        
        todo!();
        /*
            return label.getBorderSize();
        */
    }
}

impl<'a> ComboBoxLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_combo_box(&mut self, 
        g:                    &mut Graphics,
        width:                i32,
        height:               i32,
        is_mouse_button_down: bool,
        buttonx:              i32,
        buttony:              i32,
        buttonw:              i32,
        buttonh:              i32,
        box_:                 &mut ComboBox)  {
        
        todo!();
        /*
            g.fillAll (box.findColour (ComboBox::backgroundColourId));

        if (box.isEnabled() && box.hasKeyboardFocus (false))
        {
            g.setColour (box.findColour (ComboBox::focusedOutlineColourId));
            g.drawRect (0, 0, width, height, 2);
        }
        else
        {
            g.setColour (box.findColour (ComboBox::outlineColourId));
            g.drawRect (0, 0, width, height);
        }

        auto outlineThickness = box.isEnabled() ? (isMouseButtonDown ? 1.2f : 0.5f) : 0.3f;

        auto baseColour = LookAndFeelHelpers::createBaseColour (box.findColour (ComboBox::buttonColourId),
                                                                box.hasKeyboardFocus (true),
                                                                false, isMouseButtonDown)
                             .withMultipliedAlpha (box.isEnabled() ? 1.0f : 0.5f);

        drawGlassLozenge (g,
                          (float) buttonX + outlineThickness, (float) buttonY + outlineThickness,
                          (float) buttonW - outlineThickness * 2.0f, (float) buttonH - outlineThickness * 2.0f,
                          baseColour, outlineThickness, -1.0f,
                          true, true, true, true);

        if (box.isEnabled())
        {
            const float arrowX = 0.3f;
            const float arrowH = 0.2f;

            const auto x = (float) buttonX;
            const auto y = (float) buttonY;
            const auto w = (float) buttonW;
            const auto h = (float) buttonH;

            Path p;
            p.addTriangle (x + w * 0.5f,            y + h * (0.45f - arrowH),
                           x + w * (1.0f - arrowX), y + h * 0.45f,
                           x + w * arrowX,          y + h * 0.45f);

            p.addTriangle (x + w * 0.5f,            y + h * (0.55f + arrowH),
                           x + w * (1.0f - arrowX), y + h * 0.55f,
                           x + w * arrowX,          y + h * 0.55f);

            g.setColour (box.findColour (ComboBox::arrowColourId));
            g.fillPath (p);
        }
        */
    }
    
    fn get_combo_box_font(&mut self, box_: &mut ComboBox) -> Font {
        
        todo!();
        /*
            return Font (jmin (15.0f, (float) box.getHeight() * 0.85f));
        */
    }
    
    fn create_combo_box_text_box(&mut self, _0: &mut ComboBox) -> *mut Label {
        
        todo!();
        /*
            return new Label (String(), String());
        */
    }
    
    fn position_combo_box_text(&mut self, 
        box_:  &mut ComboBox,
        label: &mut Label)  {
        
        todo!();
        /*
            label.setBounds (1, 1,
                         box.getWidth() + 3 - box.getHeight(),
                         box.getHeight() - 2);

        label.setFont (getComboBoxFont (box));
        */
    }
    
    fn get_options_for_combo_box_popup_menu(
        &mut self, 
        box_:  &mut ComboBox,
        label: &mut Label

    ) -> PopupMenuOptions {
        
        todo!();
        /*
            return PopupMenu::Options().withTargetComponent (&box)
                                   .withItemThatMustBeVisible (box.getSelectedId())
                                   .withInitiallySelectedItem (box.getSelectedId())
                                   .withMinimumWidth (box.getWidth())
                                   .withMaximumNumColumns (1)
                                   .withStandardItemHeight (label.getHeight());
        */
    }
    
    fn draw_combo_box_text_when_nothing_selected(&mut self, 
        g:     &mut Graphics,
        box_:  &mut ComboBox,
        label: &mut Label)  {
        
        todo!();
        /*
            g.setColour (findColour (ComboBox::textColourId).withMultipliedAlpha (0.5f));

        auto font = label.getLookAndFeel().getLabelFont (label);

        g.setFont (font);

        auto textArea = getLabelBorderSize (label).subtractedFrom (label.getLocalBounds());

        g.drawFittedText (box.getTextWhenNothingSelected(), textArea, label.getJustificationType(),
                          jmax (1, (int) ((float) textArea.getHeight() / font.getHeight())),
                          label.getMinimumHorizontalScale());
        */
    }
}

impl<'a> PopupMenuLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn get_menu_window_flags(&mut self) -> i32 {
        
        todo!();
        /*
            return ComponentPeer::windowHasDropShadow;
        */
    }
    
    fn draw_menu_bar_background(
        &mut self, 
        g:        &mut Graphics,
        width:    i32,
        height:   i32,
        _3:       bool,
        menu_bar: &mut MenuBarComponent

    ) {
        
        todo!();
        /*
            auto baseColour = LookAndFeelHelpers::createBaseColour (menuBar.findColour (PopupMenu::backgroundColourId),
                                                                false, false, false);

        if (menuBar.isEnabled())
            drawShinyButtonShape (g, -4.0f, 0.0f, (float) width + 8.0f, (float) height,
                                  0.0f, baseColour, 0.4f, true, true, true, true);
        else
            g.fillAll (baseColour);
        */
    }
    
    fn get_menu_bar_font(
        &mut self, 
        menu_bar:   &mut MenuBarComponent,
        item_index: i32,
        item_text:  &String

    ) -> Font {
        
        todo!();
        /*
            return Font ((float) menuBar.getHeight() * 0.7f);
        */
    }
    
    fn get_menu_bar_item_width(
        &mut self, 
        menu_bar:   &mut MenuBarComponent,
        item_index: i32,
        item_text:  &String

    ) -> i32 {
        
        todo!();
        /*
            return getMenuBarFont (menuBar, itemIndex, itemText)
                .getStringWidth (itemText) + menuBar.getHeight();
        */
    }
    
    fn draw_menu_bar_item(
        &mut self, 
        g:                  &mut Graphics,
        width:              i32,
        height:             i32,
        item_index:         i32,
        item_text:          &String,
        is_mouse_over_item: bool,
        is_menu_open:       bool,
        is_mouse_over_bar:  bool,
        menu_bar:           &mut MenuBarComponent

    ) {
        
        todo!();
        /*
            if (! menuBar.isEnabled())
        {
            g.setColour (menuBar.findColour (PopupMenu::textColourId)
                                .withMultipliedAlpha (0.5f));
        }
        else if (isMenuOpen || isMouseOverItem)
        {
            g.fillAll (menuBar.findColour (PopupMenu::highlightedBackgroundColourId));
            g.setColour (menuBar.findColour (PopupMenu::highlightedTextColourId));
        }
        else
        {
            g.setColour (menuBar.findColour (PopupMenu::textColourId));
        }

        g.setFont (getMenuBarFont (menuBar, itemIndex, itemText));
        g.drawFittedText (itemText, 0, 0, width, height, Justification::centred, 1);
        */
    }
    
    fn get_parent_component_for_menu_options(&mut self, options: &PopupMenuOptions) -> *mut Component {
        
        todo!();
        /*
            return options.getParentComponent();
        */
    }
    
    fn prepare_popup_menu_window(&mut self, _0: &mut Component)  {
        
        todo!();
        /*
        
        */
    }
    
    fn should_popup_menu_scale_with_target_component(&mut self, _0: &PopupMenuOptions) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    fn get_popup_menu_border_size(&mut self) -> i32 {
        
        todo!();
        /*
            return 2;
        */
    }
    
    fn get_popup_menu_border_size_with_options(&mut self, _0: &PopupMenuOptions) -> i32 {
        
        todo!();
        /*
            return getPopupMenuBorderSize();
        */
    }
    
    fn draw_popup_menu_column_separator_with_options(
        &mut self, 
        _0: &mut Graphics,
        _1: &Rectangle<i32>,
        _2: &PopupMenuOptions

    ) {
        
        todo!();
        /*
        
        */
    }
    
    fn get_popup_menu_column_separator_width_with_options(&mut self, _0: &PopupMenuOptions) -> i32 {
        0
    }
    
   
    fn get_default_menu_bar_height(&mut self) -> i32 {
        24
    }


    fn get_popup_menu_font(&mut self) -> Font {
        
        todo!();
        /*
            return Font (17.0f);
        */
    }
    
    fn get_ideal_popup_menu_item_size(&mut self, 
        text:                      &String,
        is_separator:              bool,
        standard_menu_item_height: i32,
        ideal_width:               &mut i32,
        ideal_height:              &mut i32)  {
        
        todo!();
        /*
            if (isSeparator)
        {
            idealWidth = 50;
            idealHeight = standardMenuItemHeight > 0 ? standardMenuItemHeight / 2 : 10;
        }
        else
        {
            Font font (getPopupMenuFont());

            if (standardMenuItemHeight > 0 && font.getHeight() > (float) standardMenuItemHeight / 1.3f)
                font.setHeight ((float) standardMenuItemHeight / 1.3f);

            idealHeight = standardMenuItemHeight > 0 ? standardMenuItemHeight : roundToInt (font.getHeight() * 1.3f);
            idealWidth = font.getStringWidth (text) + idealHeight * 2;
        }
        */
    }
    
    fn get_ideal_popup_menu_item_size_with_options(&mut self, 
        text:                      &String,
        is_separator:              bool,
        standard_menu_item_height: i32,
        ideal_width:               &mut i32,
        ideal_height:              &mut i32,
        _5:                        &PopupMenuOptions)  {
        
        todo!();
        /*
            getIdealPopupMenuItemSize (text,
                                   isSeparator,
                                   standardMenuItemHeight,
                                   idealWidth,
                                   idealHeight);
        */
    }

    fn draw_popup_menu_background(
        &mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32

    ) {
        
        todo!();
        /*
            auto background = findColour (PopupMenu::backgroundColourId);

        g.fillAll (background);
        g.setColour (background.overlaidWith (Colour (0x2badd8e6)));

        for (int i = 0; i < height; i += 3)
            g.fillRect (0, i, width, 1);

       #if ! ALOE_MAC
        g.setColour (findColour (PopupMenu::textColourId).withAlpha (0.6f));
        g.drawRect (0, 0, width, height);
       #endif
        */
    }
    
    fn draw_popup_menu_background_with_options(
        &mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32,
        _3:     &PopupMenuOptions

    ) {
        
        todo!();
        /*
            drawPopupMenuBackground (g, width, height);
        */
    }
    
    fn draw_popup_menu_up_down_arrow(
        &mut self, 
        g:                  &mut Graphics,
        width:              i32,
        height:             i32,
        is_scroll_up_arrow: bool

    ) {
        
        todo!();
        /*
            auto background = findColour (PopupMenu::backgroundColourId);

        g.setGradientFill (ColourGradient (background, 0.0f, (float) height * 0.5f,
                                           background.withAlpha (0.0f),
                                           0.0f, isScrollUpArrow ? ((float) height) : 0.0f,
                                           false));

        g.fillRect (1, 1, width - 2, height - 2);

        auto hw = (float) width * 0.5f;
        auto arrowW = (float) height * 0.3f;
        auto y1 = (float) height * (isScrollUpArrow ? 0.6f : 0.3f);
        auto y2 = (float) height * (isScrollUpArrow ? 0.3f : 0.6f);

        Path p;
        p.addTriangle (hw - arrowW, y1,
                       hw + arrowW, y1,
                       hw, y2);

        g.setColour (findColour (PopupMenu::textColourId).withAlpha (0.5f));
        g.fillPath (p);
        */
    }
    
    fn draw_popup_menu_up_down_arrow_with_options(
        &mut self, 
        g:                  &mut Graphics,
        width:              i32,
        height:             i32,
        is_scroll_up_arrow: bool,
        _4:                 &PopupMenuOptions

    ) {
        
        todo!();
        /*
            drawPopupMenuUpDownArrow (g, width, height, isScrollUpArrow);
        */
    }
    
    fn draw_popup_menu_item(
        &mut self, 
        g:                  &mut Graphics,
        area:               &Rectangle<i32>,
        is_separator:       bool,
        is_active:          bool,
        is_highlighted:     bool,
        is_ticked:          bool,
        has_sub_menu:       bool,
        text:               &String,
        shortcut_key_text:  &String,
        icon:               *const Drawable,
        text_colour_to_use: *const Colour

    ) {
        
        todo!();
        /*
            if (isSeparator)
        {
            auto r = area.reduced (5, 0);
            r.removeFromTop (r.getHeight() / 2 - 1);

            g.setColour (Colour (0x33000000));
            g.fillRect (r.removeFromTop (1));

            g.setColour (Colour (0x66ffffff));
            g.fillRect (r.removeFromTop (1));
        }
        else
        {
            auto textColour = findColour (PopupMenu::textColourId);

            if (textColourToUse != nullptr)
                textColour = *textColourToUse;

            auto r = area.reduced (1);

            if (isHighlighted)
            {
                g.setColour (findColour (PopupMenu::highlightedBackgroundColourId));
                g.fillRect (r);

                g.setColour (findColour (PopupMenu::highlightedTextColourId));
            }
            else
            {
                g.setColour (textColour);
            }

            if (! isActive)
                g.setOpacity (0.3f);

            Font font (getPopupMenuFont());

            auto maxFontHeight = (float) area.getHeight() / 1.3f;

            if (font.getHeight() > maxFontHeight)
                font.setHeight (maxFontHeight);

            g.setFont (font);

            auto iconArea = r.removeFromLeft ((r.getHeight() * 5) / 4).reduced (3).toFloat();

            if (icon != nullptr)
            {
                icon->drawWithin (g, iconArea, RectanglePlacement::centred | RectanglePlacement::onlyReduceInSize, 1.0f);
            }
            else if (isTicked)
            {
                auto tick = getTickShape (1.0f);
                g.fillPath (tick, tick.getTransformToScaleToFit (iconArea, true));
            }

            if (hasSubMenu)
            {
                auto arrowH = 0.6f * getPopupMenuFont().getAscent();

                auto x = (float) r.removeFromRight ((int) arrowH).getX();
                auto halfH = (float) r.getCentreY();

                Path p;
                p.addTriangle (x, halfH - arrowH * 0.5f,
                               x, halfH + arrowH * 0.5f,
                               x + arrowH * 0.6f, halfH);

                g.fillPath (p);
            }

            r.removeFromRight (3);
            g.drawFittedText (text, r, Justification::centredLeft, 1);

            if (shortcutKeyText.isNotEmpty())
            {
                Font f2 (font);
                f2.setHeight (f2.getHeight() * 0.75f);
                f2.setHorizontalScale (0.95f);
                g.setFont (f2);

                g.drawText (shortcutKeyText, r, Justification::centredRight, true);
            }
        }
        */
    }
    
    fn draw_popup_menu_item_with_options(
        &mut self, 
        g:              &mut Graphics,
        area:           &Rectangle<i32>,
        is_highlighted: bool,
        item:           &PopupMenuItem,
        _4:             &PopupMenuOptions

    ) {
        
        todo!();
        /*
            const auto colour = item.colour != Colour() ? &item.colour : nullptr;
        const auto hasSubMenu = item.subMenu != nullptr
                                && (item.itemID == 0 || item.subMenu->getNumItems() > 0);

        drawPopupMenuItem (g,
                           area,
                           item.isSeparator,
                           item.isEnabled,
                           isHighlighted,
                           item.isTicked,
                           hasSubMenu,
                           item.text,
                           item.shortcutKeyDescription,
                           item.image.get(),
                           colour);
        */
    }
    
    fn draw_popup_menu_section_header(
        &mut self, 
        g:            &mut Graphics,
        area:         &Rectangle<i32>,
        section_name: &String

    ) {
        
        todo!();
        /*
            g.setFont (getPopupMenuFont().boldened());
        g.setColour (findColour (PopupMenu::headerTextColourId));

        g.drawFittedText (sectionName,
                          area.getX() + 12, area.getY(), area.getWidth() - 16, (int) ((float) area.getHeight() * 0.8f),
                          Justification::bottomLeft, 1);
        */
    }
    
    fn draw_popup_menu_section_header_with_options(
        &mut self, 
        g:            &mut Graphics,
        area:         &Rectangle<i32>,
        section_name: &String,
        _3:           &PopupMenuOptions

    ) {
        
        todo!();
        /*
            drawPopupMenuSectionHeader (g, area, sectionName);
        */
    }
}

impl<'a> AlertWindowLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn create_alert_window(&mut self, 
        title:                &String,
        message:              &String,
        button1:              &String,
        button2:              &String,
        button3:              &String,
        icon_type:            MessageBoxIconType,
        num_buttons:          i32,
        associated_component: *mut Component) -> *mut AlertWindow {
        
        todo!();
        /*
            AlertWindow* aw = new AlertWindow (title, message, iconType, associatedComponent);

        if (numButtons == 1)
        {
            aw->addButton (button1, 0,
                           KeyPress (KeyPress::escapeKey),
                           KeyPress (KeyPress::returnKey));
        }
        else
        {
            const KeyPress button1ShortCut ((int) CharacterFunctions::toLowerCase (button1[0]), 0, 0);
            KeyPress button2ShortCut ((int) CharacterFunctions::toLowerCase (button2[0]), 0, 0);
            if (button1ShortCut == button2ShortCut)
                button2ShortCut = KeyPress();

            if (numButtons == 2)
            {
                aw->addButton (button1, 1, KeyPress (KeyPress::returnKey), button1ShortCut);
                aw->addButton (button2, 0, KeyPress (KeyPress::escapeKey), button2ShortCut);
            }
            else if (numButtons == 3)
            {
                aw->addButton (button1, 1, button1ShortCut);
                aw->addButton (button2, 2, button2ShortCut);
                aw->addButton (button3, 0, KeyPress (KeyPress::escapeKey));
            }
        }

        return aw;
        */
    }
    
    fn draw_alert_box(&mut self, 
        g:           &mut Graphics,
        alert:       &mut AlertWindow,
        text_area:   &Rectangle<i32>,
        text_layout: &mut TextLayout)  {
        
        todo!();
        /*
            g.fillAll (alert.findColour (AlertWindow::backgroundColourId));

        int iconSpaceUsed = 0;

        const int iconWidth = 80;
        int iconSize = jmin (iconWidth + 50, alert.getHeight() + 20);

        if (alert.containsAnyExtraComponents() || alert.getNumButtons() > 2)
            iconSize = jmin (iconSize, textArea.getHeight() + 50);

        const Rectangle<int> iconRect (iconSize / -10, iconSize / -10,
                                       iconSize, iconSize);

        if (alert.getAlertType() != MessageBoxIconType::NoIcon)
        {
            Path icon;
            uint32 colour;
            char character;

            if (alert.getAlertType() == MessageBoxIconType::WarningIcon)
            {
                colour = 0x55ff5555;
                character = '!';

                icon.addTriangle ((float) iconRect.getX() + (float) iconRect.getWidth() * 0.5f, (float) iconRect.getY(),
                                  (float) iconRect.getRight(), (float) iconRect.getBottom(),
                                  (float) iconRect.getX(), (float) iconRect.getBottom());

                icon = icon.createPathWithRoundedCorners (5.0f);
            }
            else
            {
                colour    = alert.getAlertType() == MessageBoxIconType::InfoIcon ? (uint32) 0x605555ff : (uint32) 0x40b69900;
                character = alert.getAlertType() == MessageBoxIconType::InfoIcon ? 'i' : '?';

                icon.addEllipse (iconRect.toFloat());
            }

            GlyphArrangement ga;
            ga.addFittedText (Font ((float) iconRect.getHeight() * 0.9f, Font::bold),
                              String::charToString ((aloe_wchar) (uint8) character),
                              (float) iconRect.getX(), (float) iconRect.getY(),
                              (float) iconRect.getWidth(), (float) iconRect.getHeight(),
                              Justification::centred, false);
            ga.createPath (icon);

            icon.setUsingNonZeroWinding (false);
            g.setColour (Colour (colour));
            g.fillPath (icon);

            iconSpaceUsed = iconWidth;
        }

        g.setColour (alert.findColour (AlertWindow::textColourId));

        textLayout.draw (g, Rectangle<int> (textArea.getX() + iconSpaceUsed,
                                            textArea.getY(),
                                            textArea.getWidth() - iconSpaceUsed,
                                            textArea.getHeight()).toFloat());

        g.setColour (alert.findColour (AlertWindow::outlineColourId));
        g.drawRect (0, 0, alert.getWidth(), alert.getHeight());
        */
    }
    
    fn get_alert_box_window_flags(&mut self) -> i32 {
        
        todo!();
        /*
            return ComponentPeer::windowAppearsOnTaskbar
                | ComponentPeer::windowHasDropShadow;
        */
    }
    
    fn get_widths_for_text_buttons(&mut self, 
        _0:      &mut AlertWindow,
        buttons: &[*mut TextButton]) -> Vec<i32> {
        
        todo!();
        /*
            const int n = buttons.size();
        Vec<int> buttonWidths;

        const int buttonHeight = getAlertWindowButtonHeight();

        for (int i = 0; i < n; ++i)
            buttonWidths.add (getTextButtonWidthToFitText (*buttons.getReference (i), buttonHeight));

        return buttonWidths;
        */
    }
    
    fn get_alert_window_button_height(&mut self) -> i32 {
        
        todo!();
        /*
            return 28;
        */
    }
    
    /**
      | Override this function to supply a custom
      | font for the alert window title.
      | 
      | This default implementation will use
      | a boldened and slightly larger version
      | of the alert window message font.
      | 
      | @see getAlertWindowMessageFont.
      |
      */
    fn get_alert_window_title_font(&mut self) -> Font {
        
        todo!();
        /*
            Font messageFont = getAlertWindowMessageFont();
        return messageFont.withHeight (messageFont.getHeight() * 1.1f).boldened();
        */
    }
    
    /**
      | Override this function to supply a custom
      | font for the alert window message.
      | 
      | This default implementation will use
      | the default font with height set to 15.0f.
      | 
      | @see getAlertWindowTitleFont
      |
      */
    fn get_alert_window_message_font(&mut self) -> Font {
        
        todo!();
        /*
            return Font (15.0f);
        */
    }
    
    fn get_alert_window_font(&mut self) -> Font {
        
        todo!();
        /*
            return Font (12.0f);
        */
    }
}

impl<'a> BubbleComponentLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_bubble(&mut self, 
        g:    &mut Graphics,
        comp: &mut BubbleComponent,
        tip:  &Point<f32>,
        body: &Rectangle<f32>)  {
        
        todo!();
        /*
            Path p;
        p.addBubble (body.reduced (0.5f), body.getUnion (Rectangle<float> (tip.x, tip.y, 1.0f, 1.0f)),
                     tip, 5.0f, jmin (15.0f, body.getWidth() * 0.2f, body.getHeight() * 0.2f));

        g.setColour (comp.findColour (BubbleComponent::backgroundColourId));
        g.fillPath (p);

        g.setColour (comp.findColour (BubbleComponent::outlineColourId));
        g.strokePath (p, PathStrokeType (1.0f));
        */
    }
}

impl<'a> TreeViewLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_treeview_plus_minus_box(&mut self, 
        g:                 &mut Graphics,
        area:              &Rectangle<f32>,
        background_colour: Colour,
        is_open:           bool,
        is_mouse_over:     bool)  {
        
        todo!();
        /*
            auto boxSize = roundToInt (jmin (16.0f, area.getWidth(), area.getHeight()) * 0.7f) | 1;

        auto x = ((int) area.getWidth()  - boxSize) / 2 + (int) area.getX();
        auto y = ((int) area.getHeight() - boxSize) / 2 + (int) area.getY();

        Rectangle<float> boxArea ((float) x, (float) y, (float) boxSize, (float) boxSize);

        g.setColour (Colour (0xe5ffffff));
        g.fillRect (boxArea);

        g.setColour (Colour (0x80000000));
        g.drawRect (boxArea);

        auto size = (float) boxSize * 0.5f + 1.0f;
        auto centre = (float) (boxSize / 2);

        g.fillRect ((float) x + ((float) boxSize - size) * 0.5f, (float) y + centre, size, 1.0f);

        if (! isOpen)
            g.fillRect ((float) x + centre, (float) y + ((float) boxSize - size) * 0.5f, 1.0f, size);
        */
    }
    
    fn are_lines_drawn_for_tree_view(&mut self, _0: &mut TreeView) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    fn get_tree_view_indent_size(&mut self, _0: &mut TreeView) -> i32 {
        
        todo!();
        /*
            return 24;
        */
    }
}

impl<'a> FileBrowserComponentLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn get_default_folder_image(&mut self) -> *const Drawable {
        
        todo!();
        /*
            if (folderImage == nullptr)
            folderImage = createDrawableFromSVG (R"svgdata(
    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="706" height="532">
      <defs>
        <linearGradient id="a">
          <stop stop-color="#adf" offset="0"/>
          <stop stop-color="#ecfaff" offset="1"/>
        </linearGradient>
        <linearGradient id="b" x1=".6" x2="0" y1=".9" xlink:href="#a"/>
        <linearGradient id="c" x1=".6" x2=".1" y1=".9" y2=".3" xlink:href="#a"/>
      </defs>
      <g class="currentLayer">
        <path d="M112.1 104c-8.2 2.2-13.2 11.6-11.3 21l68.3 342.7c1.9 9.4 10.1 15.2 18.4 13l384.3-104.1c8.2-2.2 13.2-11.6 11.3-21l-48-266a15.8 15.8 0 0 0-18.4-12.8l-224.2 38s-20.3-41.3-28.3-39.3z" display="block" fill="url(#b)" stroke="#446c98" stroke-width="7"/>
        <path d="M608.6 136.8L235.2 208a22.7 22.7 0 0 0-16 19l-40.8 241c1.7 8.4 9.6 14.5 17.8 12.3l380-104c8-2.2 10.7-10.2 12.3-18.4l38-210.1c.4-15.4-10.4-11.8-18-11.1z" display="block" fill="url(#c)" opacity=".8" stroke="#446c98" stroke-width="7"/>
      </g>
    </svg>
    )svgdata");

        return folderImage.get();
        */
    }
    
    fn get_default_document_file_image(&mut self) -> *const Drawable {
        
        todo!();
        /*
            if (documentImage == nullptr)
            documentImage = createDrawableFromSVG (R"svgdata(
    <svg version="1" viewBox="-10 -10 450 600" xmlns="http://www.w3.org/2000/svg">
      <path d="M17 0h290l120 132v426c0 10-8 19-17 19H17c-9 0-17-9-17-19V19C0 8 8 0 17 0z" fill="#e5e5e5" stroke="#888888" stroke-width="7"/>
      <path d="M427 132H324c-9 0-17-9-17-19V0l120 132z" fill="#ccc"/>
    </svg>
    )svgdata");

        return documentImage.get();
        */
    }

    fn create_file_chooser_header_text(&mut self, 
        title:        &String,
        instructions: &String) -> AttributedString {
        
        todo!();
        /*
            AttributedString s;
        s.setJustification (Justification::centred);

        auto colour = findColour (FileChooserDialogBox::titleTextColourId);
        s.append (title + "\n\n", Font (17.0f, Font::bold), colour);
        s.append (instructions, Font (14.0f), colour);

        return s;
        */
    }
    
    fn draw_file_browser_row(&mut self, 
        g:                     &mut Graphics,
        width:                 i32,
        height:                i32,
        _3:                    &File,
        filename:              &String,
        icon:                  *mut Image,
        file_size_description: &String,
        file_time_description: &String,
        is_directory:          bool,
        is_item_selected:      bool,
        item_index:            i32,
        dcc:                   &mut DirectoryContentsDisplayComponent)  {
        
        todo!();
        /*
            auto fileListComp = dynamic_cast<Component*> (&dcc);

        if (isItemSelected)
            g.fillAll (fileListComp != nullptr ? fileListComp->findColour (DirectoryContentsDisplayComponent::highlightColourId)
                                               : findColour (DirectoryContentsDisplayComponent::highlightColourId));

        const int x = 32;
        g.setColour (Colours::black);

        if (icon != nullptr && icon->isValid())
        {
            g.drawImageWithin (*icon, 2, 2, x - 4, height - 4,
                               RectanglePlacement::centred | RectanglePlacement::onlyReduceInSize,
                               false);
        }
        else
        {
            if (auto* d = isDirectory ? getDefaultFolderImage()
                                      : getDefaultDocumentFileImage())
                d->drawWithin (g, Rectangle<float> (2.0f, 2.0f, x - 4.0f, (float) height - 4.0f),
                               RectanglePlacement::centred | RectanglePlacement::onlyReduceInSize, 1.0f);
        }

        if (isItemSelected)
            g.setColour (fileListComp != nullptr ? fileListComp->findColour (DirectoryContentsDisplayComponent::highlightedTextColourId)
                                                 : findColour (DirectoryContentsDisplayComponent::highlightedTextColourId));
        else
            g.setColour (fileListComp != nullptr ? fileListComp->findColour (DirectoryContentsDisplayComponent::textColourId)
                                                 : findColour (DirectoryContentsDisplayComponent::textColourId));

        g.setFont ((float) height * 0.7f);

        if (width > 450 && ! isDirectory)
        {
            auto sizeX = roundToInt ((float) width * 0.7f);
            auto dateX = roundToInt ((float) width * 0.8f);

            g.drawFittedText (filename,
                              x, 0, sizeX - x, height,
                              Justification::centredLeft, 1);

            g.setFont ((float) height * 0.5f);
            g.setColour (Colours::darkgrey);

            if (! isDirectory)
            {
                g.drawFittedText (fileSizeDescription,
                                  sizeX, 0, dateX - sizeX - 8, height,
                                  Justification::centredRight, 1);

                g.drawFittedText (fileTimeDescription,
                                  dateX, 0, width - 8 - dateX, height,
                                  Justification::centredRight, 1);
            }
        }
        else
        {
            g.drawFittedText (filename,
                              x, 0, width - x, height,
                              Justification::centredLeft, 1);

        }
        */
    }
    
    fn create_file_browser_go_up_button(&mut self) -> *mut Button {
        
        todo!();
        /*
            auto goUpButton = new DrawableButton ("up", DrawableButton::ImageOnButtonBackground);

        Path arrowPath;
        arrowPath.addArrow ({ 50.0f, 100.0f, 50.0f, 0.0f }, 40.0f, 100.0f, 50.0f);

        DrawablePath arrowImage;
        arrowImage.setFill (Colours::black.withAlpha (0.4f));
        arrowImage.setPath (arrowPath);

        goUpButton->setImages (&arrowImage);

        return goUpButton;
        */
    }
    
    fn layout_file_browser_component(&mut self, 
        browser_comp:        &mut FileBrowserComponent,
        file_list_component: *mut DirectoryContentsDisplayComponent,
        preview_comp:        *mut FilePreviewComponent,
        current_path_box:    *mut ComboBox,
        filename_box:        *mut TextEditor,
        go_up_button:        *mut Button)  {
        
        todo!();
        /*
            const int x = 8;
        auto w = browserComp.getWidth() - x - x;

        if (previewComp != nullptr)
        {
            auto previewWidth = w / 3;
            previewComp->setBounds (x + w - previewWidth, 0, previewWidth, browserComp.getHeight());

            w -= previewWidth + 4;
        }

        int y = 4;

        const int controlsHeight = 22;
        const int upButtonWidth = 50;
        auto bottomSectionHeight = controlsHeight + 8;

        currentPathBox->setBounds (x, y, w - upButtonWidth - 6, controlsHeight);
        goUpButton->setBounds (x + w - upButtonWidth, y, upButtonWidth, controlsHeight);

        y += controlsHeight + 4;

        if (auto listAsComp = dynamic_cast<Component*> (fileListComponent))
        {
            listAsComp->setBounds (x, y, w, browserComp.getHeight() - y - bottomSectionHeight);
            y = listAsComp->getBottom() + 4;
        }

        filenameBox->setBounds (x + 50, y, w - 50, controlsHeight);
        */
    }
}

impl<'a> TextEditorLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn fill_text_editor_background(
        &mut self, 
        g:           &mut Graphics,
        width:       i32,
        height:      i32,
        text_editor: &mut TextEditor
    ) {
        
        todo!();
        /*
            g.fillAll (textEditor.findColour (TextEditor::backgroundColourId));
        */
    }
    
    fn draw_text_editor_outline(
        &mut self, 
        g:           &mut Graphics,
        width:       i32,
        height:      i32,
        text_editor: &mut TextEditor

    ) {
        
        todo!();
        /*
            if (textEditor.isEnabled())
        {
            if (textEditor.hasKeyboardFocus (true) && ! textEditor.isReadOnly())
            {
                const int border = 2;

                g.setColour (textEditor.findColour (TextEditor::focusedOutlineColourId));
                g.drawRect (0, 0, width, height, border);

                g.setOpacity (1.0f);
                auto shadowColour = textEditor.findColour (TextEditor::shadowColourId).withMultipliedAlpha (0.75f);
                drawBevel (g, 0, 0, width, height + 2, border + 2, shadowColour, shadowColour);
            }
            else
            {
                g.setColour (textEditor.findColour (TextEditor::outlineColourId));
                g.drawRect (0, 0, width, height);

                g.setOpacity (1.0f);
                auto shadowColour = textEditor.findColour (TextEditor::shadowColourId);
                drawBevel (g, 0, 0, width, height + 2, 3, shadowColour, shadowColour);
            }
        }
        */
    }
    
    fn create_caret_component(&mut self, key_focus_owner: *mut Component) -> *mut CaretComponent {
        
        todo!();
        /*
            return new CaretComponent (keyFocusOwner);
        */
    }
}

impl<'a> ImageButtonLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_image_button(&mut self, 
        g:              &mut Graphics,
        image:          *mut Image,
        imagex:         i32,
        imagey:         i32,
        imagew:         i32,
        imageh:         i32,
        overlay_colour: &Colour,
        image_opacity:  f32,
        button:         &mut ImageButton)  {
        
        todo!();
        /*
            if (! button.isEnabled())
            imageOpacity *= 0.3f;

        AffineTransform t = RectanglePlacement (RectanglePlacement::stretchToFit)
                                .getTransformToFit (image->getBounds().toFloat(),
                                                    Rectangle<int> (imageX, imageY, imageW, imageH).toFloat());

        if (! overlayColour.isOpaque())
        {
            g.setOpacity (imageOpacity);
            g.drawImageTransformed (*image, t, false);
        }

        if (! overlayColour.isTransparent())
        {
            g.setColour (overlayColour);
            g.drawImageTransformed (*image, t, true);
        }
        */
    }
    
}

impl<'a> ButtonLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn draw_button_background(&mut self, 
        g:                                 &mut Graphics,
        button:                            &mut Button,
        background_colour:                 &Colour,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            const int width = button.getWidth();
        const int height = button.getHeight();

        const float outlineThickness = button.isEnabled() ? ((shouldDrawButtonAsDown || shouldDrawButtonAsHighlighted) ? 1.2f : 0.7f) : 0.4f;
        const float halfThickness = outlineThickness * 0.5f;

        const float indentL = button.isConnectedOnLeft()   ? 0.1f : halfThickness;
        const float indentR = button.isConnectedOnRight()  ? 0.1f : halfThickness;
        const float indentT = button.isConnectedOnTop()    ? 0.1f : halfThickness;
        const float indentB = button.isConnectedOnBottom() ? 0.1f : halfThickness;

        const Colour baseColour (LookAndFeelHelpers::createBaseColour (backgroundColour,
                                                                       button.hasKeyboardFocus (true),
                                                                       shouldDrawButtonAsHighlighted,
                                                                       shouldDrawButtonAsDown)
                                   .withMultipliedAlpha (button.isEnabled() ? 1.0f : 0.5f));

        drawGlassLozenge (g,
                          indentL,
                          indentT,
                          (float) width - indentL - indentR,
                          (float) height - indentT - indentB,
                          baseColour, outlineThickness, -1.0f,
                          button.isConnectedOnLeft(),
                          button.isConnectedOnRight(),
                          button.isConnectedOnTop(),
                          button.isConnectedOnBottom());
        */
    }
    
    fn get_text_button_font(&mut self, 
        _0:            &mut TextButton,
        button_height: i32) -> Font {
        
        todo!();
        /*
            return Font (jmin (15.0f, (float) buttonHeight * 0.6f));
        */
    }
    
    fn get_text_button_width_to_fit_text(&mut self, 
        b:             &mut TextButton,
        button_height: i32) -> i32 {
        
        todo!();
        /*
            return getTextButtonFont (b, buttonHeight).getStringWidth (b.getButtonText()) + buttonHeight;
        */
    }
    
    fn draw_button_text(&mut self, 
        g:                                 &mut Graphics,
        button:                            &mut TextButton,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            Font font (getTextButtonFont (button, button.getHeight()));
        g.setFont (font);
        g.setColour (button.findColour (button.getToggleState() ? TextButton::textColourOnId
                                                                : TextButton::textColourOffId)
                           .withMultipliedAlpha (button.isEnabled() ? 1.0f : 0.5f));

        const int yIndent = jmin (4, button.proportionOfHeight (0.3f));
        const int cornerSize = jmin (button.getHeight(), button.getWidth()) / 2;

        const int fontHeight = roundToInt (font.getHeight() * 0.6f);
        const int leftIndent  = jmin (fontHeight, 2 + cornerSize / (button.isConnectedOnLeft() ? 4 : 2));
        const int rightIndent = jmin (fontHeight, 2 + cornerSize / (button.isConnectedOnRight() ? 4 : 2));
        const int textWidth = button.getWidth() - leftIndent - rightIndent;

        if (textWidth > 0)
            g.drawFittedText (button.getButtonText(),
                              leftIndent, yIndent, textWidth, button.getHeight() - yIndent * 2,
                              Justification::centred, 2);
        */
    }
    
    fn draw_tick_box(&mut self, 
        g:                                 &mut Graphics,
        component:                         &mut Component,
        x:                                 f32,
        y:                                 f32,
        w:                                 f32,
        h:                                 f32,
        ticked:                            bool,
        is_enabled:                        bool,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            const float boxSize = w * 0.7f;

        drawGlassSphere (g, x, y + (h - boxSize) * 0.5f, boxSize,
                         LookAndFeelHelpers::createBaseColour (component.findColour (TextButton::buttonColourId)
                                                                        .withMultipliedAlpha (isEnabled ? 1.0f : 0.5f),
                                                               true, shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown),
                         isEnabled ? ((shouldDrawButtonAsDown || shouldDrawButtonAsHighlighted) ? 1.1f : 0.5f) : 0.3f);

        if (ticked)
        {
            Path tick;
            tick.startNewSubPath (1.5f, 3.0f);
            tick.lineTo (3.0f, 6.0f);
            tick.lineTo (6.0f, 0.0f);

            g.setColour (component.findColour (isEnabled ? ToggleButton::tickColourId
                                                         : ToggleButton::tickDisabledColourId));

            const AffineTransform trans (AffineTransform::scale (w / 9.0f, h / 9.0f)
                                                         .translated (x, y));

            g.strokePath (tick, PathStrokeType (2.5f), trans);
        }
        */
    }
    
    fn draw_toggle_button(&mut self, 
        g:                                 &mut Graphics,
        button:                            &mut ToggleButton,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            if (button.hasKeyboardFocus (true))
        {
            g.setColour (button.findColour (TextEditor::focusedOutlineColourId));
            g.drawRect (0, 0, button.getWidth(), button.getHeight());
        }

        float fontSize = jmin (15.0f, (float) button.getHeight() * 0.75f);
        const float tickWidth = fontSize * 1.1f;

        drawTickBox (g, button, 4.0f, ((float) button.getHeight() - tickWidth) * 0.5f,
                     tickWidth, tickWidth,
                     button.getToggleState(),
                     button.isEnabled(),
                     shouldDrawButtonAsHighlighted,
                     shouldDrawButtonAsDown);

        g.setColour (button.findColour (ToggleButton::textColourId));
        g.setFont (fontSize);

        if (! button.isEnabled())
            g.setOpacity (0.5f);

        g.drawFittedText (button.getButtonText(),
                          button.getLocalBounds().withTrimmedLeft (roundToInt (tickWidth) + 5)
                                                 .withTrimmedRight (2),
                          Justification::centredLeft, 10);
        */
    }
    
    fn change_toggle_button_width_to_fit_text(&mut self, button: &mut ToggleButton)  {
        
        todo!();
        /*
            auto fontSize = jmin (15.0f, (float) button.getHeight() * 0.75f);
        auto tickWidth = fontSize * 1.1f;

        Font font (fontSize);

        button.setSize (font.getStringWidth (button.getButtonText()) + roundToInt (tickWidth) + 9,
                        button.getHeight());
        */
    }
    
    fn draw_drawable_button(&mut self, 
        g:                                 &mut Graphics,
        button:                            &mut DrawableButton,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            bool toggleState = button.getToggleState();

        g.fillAll (button.findColour (toggleState ? DrawableButton::backgroundOnColourId
                                                  : DrawableButton::backgroundColourId));

        const int textH = (button.getStyle() == DrawableButton::ImageAboveTextLabel)
                            ? jmin (16, button.proportionOfHeight (0.25f))
                            : 0;

        if (textH > 0)
        {
            g.setFont ((float) textH);

            g.setColour (button.findColour (toggleState ? DrawableButton::textColourOnId
                                                        : DrawableButton::textColourId)
                            .withMultipliedAlpha (button.isEnabled() ? 1.0f : 0.4f));

            g.drawFittedText (button.getButtonText(),
                              2, button.getHeight() - textH - 1,
                              button.getWidth() - 4, textH,
                              Justification::centred, 1);
        }
        */
    }

}

impl<'a> ScrollBarLookAndFeelMethods for LookAndFeel_V2<'a> {

    fn are_scrollbar_buttons_visible(&mut self) -> bool {
        true
    }

    fn draw_scrollbar_button(&mut self, 
        g:                                 &mut Graphics,
        scrollbar:                         &mut ScrollBar,
        width:                             i32,
        height:                            i32,
        button_direction:                  i32,
        is_scrollbar_vertical:             bool,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            Path p;

        const auto w = (float) width;
        const auto h = (float) height;

        if (buttonDirection == 0)
            p.addTriangle (w * 0.5f, h * 0.2f,
                           w * 0.1f, h * 0.7f,
                           w * 0.9f, h * 0.7f);
        else if (buttonDirection == 1)
            p.addTriangle (w * 0.8f, h * 0.5f,
                           w * 0.3f, h * 0.1f,
                           w * 0.3f, h * 0.9f);
        else if (buttonDirection == 2)
            p.addTriangle (w * 0.5f, h * 0.8f,
                           w * 0.1f, h * 0.3f,
                           w * 0.9f, h * 0.3f);
        else if (buttonDirection == 3)
            p.addTriangle (w * 0.2f, h * 0.5f,
                           w * 0.7f, h * 0.1f,
                           w * 0.7f, h * 0.9f);

        if (shouldDrawButtonAsDown)
            g.setColour (scrollbar.findColour (ScrollBar::thumbColourId).contrasting (0.2f));
        else
            g.setColour (scrollbar.findColour (ScrollBar::thumbColourId));

        g.fillPath (p);

        g.setColour (Colour (0x80000000));
        g.strokePath (p, PathStrokeType (0.5f));
        */
    }
    
    fn draw_scrollbar(&mut self, 
        g:                     &mut Graphics,
        scrollbar:             &mut ScrollBar,
        x:                     i32,
        y:                     i32,
        width:                 i32,
        height:                i32,
        is_scrollbar_vertical: bool,
        thumb_start_position:  i32,
        thumb_size:            i32,
        is_mouse_over:         bool,
        is_mouse_down:         bool)  {
        
        todo!();
        /*
            g.fillAll (scrollbar.findColour (ScrollBar::backgroundColourId));

        Path slotPath, thumbPath;

        const float slotIndent = jmin (width, height) > 15 ? 1.0f : 0.0f;
        const float slotIndentx2 = slotIndent * 2.0f;
        const float thumbIndent = slotIndent + 1.0f;
        const float thumbIndentx2 = thumbIndent * 2.0f;

        float gx1 = 0.0f, gy1 = 0.0f, gx2 = 0.0f, gy2 = 0.0f;

        if (isScrollbarVertical)
        {
            slotPath.addRoundedRectangle ((float) x + slotIndent,
                                          (float) y + slotIndent,
                                          (float) width - slotIndentx2,
                                          (float) height - slotIndentx2,
                                          ((float) width - slotIndentx2) * 0.5f);

            if (thumbSize > 0)
                thumbPath.addRoundedRectangle ((float) x + thumbIndent,
                                               (float) thumbStartPosition + thumbIndent,
                                               (float) width - thumbIndentx2,
                                               (float) thumbSize - thumbIndentx2,
                                               ((float) width - thumbIndentx2) * 0.5f);
            gx1 = (float) x;
            gx2 = (float) x + (float) width * 0.7f;
        }
        else
        {
            slotPath.addRoundedRectangle ((float) x + slotIndent,
                                          (float) y + slotIndent,
                                          (float) width - slotIndentx2,
                                          (float) height - slotIndentx2,
                                          ((float) height - slotIndentx2) * 0.5f);

            if (thumbSize > 0)
                thumbPath.addRoundedRectangle ((float) thumbStartPosition + thumbIndent,
                                               (float) y + thumbIndent,
                                               (float) thumbSize - thumbIndentx2,
                                               (float) height - thumbIndentx2,
                                               ((float) height - thumbIndentx2) * 0.5f);
            gy1 = (float) y;
            gy2 = (float) y + (float) height * 0.7f;
        }

        const Colour thumbColour (scrollbar.findColour (ScrollBar::thumbColourId));
        Colour trackColour1, trackColour2;

        if (scrollbar.isColourSpecified (ScrollBar::trackColourId)
             || isColourSpecified (ScrollBar::trackColourId))
        {
            trackColour1 = trackColour2 = scrollbar.findColour (ScrollBar::trackColourId);
        }
        else
        {
            trackColour1 = thumbColour.overlaidWith (Colour (0x44000000));
            trackColour2 = thumbColour.overlaidWith (Colour (0x19000000));
        }

        g.setGradientFill (ColourGradient (trackColour1, gx1, gy1,
                                           trackColour2, gx2, gy2, false));
        g.fillPath (slotPath);

        if (isScrollbarVertical)
        {
            gx1 = (float) x + (float) width * 0.6f;
            gx2 = (float) x + (float) width;
        }
        else
        {
            gy1 = (float) y + (float) height * 0.6f;
            gy2 = (float) y + (float) height;
        }

        g.setGradientFill (ColourGradient (Colours::transparentBlack,gx1, gy1,
                           Colour (0x19000000), gx2, gy2, false));
        g.fillPath (slotPath);

        g.setColour (thumbColour);
        g.fillPath (thumbPath);

        g.setGradientFill (ColourGradient (Colour (0x10000000), gx1, gy1,
                           Colours::transparentBlack, gx2, gy2, false));

        {
            Graphics::ScopedSaveState ss (g);

            if (isScrollbarVertical)
                g.reduceClipRegion (x + width / 2, y, width, height);
            else
                g.reduceClipRegion (x, y + height / 2, width, height);

            g.fillPath (thumbPath);
        }

        g.setColour (Colour (0x4c000000));
        g.strokePath (thumbPath, PathStrokeType (0.4f));
        */
    }
    
    fn get_scrollbar_effect(&mut self) -> *mut dyn ImageEffectFilter {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    fn get_minimum_scrollbar_thumb_size(&mut self, scrollbar: &mut ScrollBar) -> i32 {
        
        todo!();
        /*
            return jmin (scrollbar.getWidth(), scrollbar.getHeight()) * 2;
        */
    }
    
    fn get_default_scrollbar_width(&mut self) -> i32 {
        
        todo!();
        /*
            return 18;
        */
    }
    
    fn get_scrollbar_button_size(&mut self, scrollbar: &mut ScrollBar) -> i32 {
        
        todo!();
        /*
            return 2 + (scrollbar.isVertical() ? scrollbar.getWidth()
                                           : scrollbar.getHeight());
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V2.cpp]
impl<'a> Default for LookAndFeel_V2<'a> {
    
    fn default() -> Self {
    
        todo!();
        /*


            // initialise the standard set of colours..
        const uint32 textButtonColour      = 0xffbbbbff;
        const uint32 textHighlightColour   = 0x401111ee;
        const uint32 standardOutlineColour = 0xb2808080;

        static const uint32 standardColours[] =
        {
            TextButton::buttonColourId,                 textButtonColour,
            TextButton::buttonOnColourId,               0xff4444ff,
            TextButton::textColourOnId,                 0xff000000,
            TextButton::textColourOffId,                0xff000000,

            ToggleButton::textColourId,                 0xff000000,
            ToggleButton::tickColourId,                 0xff000000,
            ToggleButton::tickDisabledColourId,         0xff808080,

            TextEditor::backgroundColourId,             0xffffffff,
            TextEditor::textColourId,                   0xff000000,
            TextEditor::highlightColourId,              textHighlightColour,
            TextEditor::highlightedTextColourId,        0xff000000,
            TextEditor::outlineColourId,                0x00000000,
            TextEditor::focusedOutlineColourId,         textButtonColour,
            TextEditor::shadowColourId,                 0x38000000,

            CaretComponent::caretColourId,              0xff000000,

            Label::backgroundColourId,                  0x00000000,
            Label::textColourId,                        0xff000000,
            Label::outlineColourId,                     0x00000000,

            ScrollBar::backgroundColourId,              0x00000000,
            ScrollBar::thumbColourId,                   0xffffffff,

            TreeView::linesColourId,                    0x4c000000,
            TreeView::backgroundColourId,               0x00000000,
            TreeView::dragAndDropIndicatorColourId,     0x80ff0000,
            TreeView::selectedItemBackgroundColourId,   0x00000000,
            TreeView::oddItemsColourId,                 0x00000000,
            TreeView::evenItemsColourId,                0x00000000,

            PopupMenu::backgroundColourId,              0xffffffff,
            PopupMenu::textColourId,                    0xff000000,
            PopupMenu::headerTextColourId,              0xff000000,
            PopupMenu::highlightedTextColourId,         0xffffffff,
            PopupMenu::highlightedBackgroundColourId,   0x991111aa,

            ComboBox::buttonColourId,                   0xffbbbbff,
            ComboBox::outlineColourId,                  standardOutlineColour,
            ComboBox::textColourId,                     0xff000000,
            ComboBox::backgroundColourId,               0xffffffff,
            ComboBox::arrowColourId,                    0x99000000,
            ComboBox::focusedOutlineColourId,           0xffbbbbff,

            PropertyComponent::backgroundColourId,      0x66ffffff,
            PropertyComponent::labelTextColourId,       0xff000000,

            TextPropertyComponent::backgroundColourId,  0xffffffff,
            TextPropertyComponent::textColourId,        0xff000000,
            TextPropertyComponent::outlineColourId,     standardOutlineColour,

            BooleanPropertyComponent::backgroundColourId, 0xffffffff,
            BooleanPropertyComponent::outlineColourId,  standardOutlineColour,

            ListBox::backgroundColourId,                0xffffffff,
            ListBox::outlineColourId,                   standardOutlineColour,
            ListBox::textColourId,                      0xff000000,

            Slider::backgroundColourId,                 0x00000000,
            Slider::thumbColourId,                      textButtonColour,
            Slider::trackColourId,                      0x7fffffff,
            Slider::rotarySliderFillColourId,           0x7f0000ff,
            Slider::rotarySliderOutlineColourId,        0x66000000,
            Slider::textBoxTextColourId,                0xff000000,
            Slider::textBoxBackgroundColourId,          0xffffffff,
            Slider::textBoxHighlightColourId,           textHighlightColour,
            Slider::textBoxOutlineColourId,             standardOutlineColour,

            ResizableWindow::backgroundColourId,        0xff777777,
            //DocumentWindow::textColourId,               0xff000000, // (this is deliberately not set)

            AlertWindow::backgroundColourId,            0xffededed,
            AlertWindow::textColourId,                  0xff000000,
            AlertWindow::outlineColourId,               0xff666666,

            ProgressBar::backgroundColourId,            0xffeeeeee,
            ProgressBar::foregroundColourId,            0xffaaaaee,

            TooltipWindow::backgroundColourId,          0xffeeeebb,
            TooltipWindow::textColourId,                0xff000000,
            TooltipWindow::outlineColourId,             0x4c000000,

            TabbedComponent::backgroundColourId,        0x00000000,
            TabbedComponent::outlineColourId,           0xff777777,
            TabbedButtonBar::tabOutlineColourId,        0x80000000,
            TabbedButtonBar::frontOutlineColourId,      0x90000000,

            Toolbar::backgroundColourId,                0xfff6f8f9,
            Toolbar::separatorColourId,                 0x4c000000,
            Toolbar::buttonMouseOverBackgroundColourId, 0x4c0000ff,
            Toolbar::buttonMouseDownBackgroundColourId, 0x800000ff,
            Toolbar::labelTextColourId,                 0xff000000,
            Toolbar::editingModeOutlineColourId,        0xffff0000,

            DrawableButton::textColourId,               0xff000000,
            DrawableButton::textColourOnId,             0xff000000,
            DrawableButton::backgroundColourId,         0x00000000,
            DrawableButton::backgroundOnColourId,       0xaabbbbff,

            HyperlinkButton::textColourId,              0xcc1111ee,

            GroupComponent::outlineColourId,            0x66000000,
            GroupComponent::textColourId,               0xff000000,

            BubbleComponent::backgroundColourId,        0xeeeeeebb,
            BubbleComponent::outlineColourId,           0x77000000,

            TableHeaderComponent::textColourId,         0xff000000,
            TableHeaderComponent::backgroundColourId,   0xffe8ebf9,
            TableHeaderComponent::outlineColourId,      0x33000000,
            TableHeaderComponent::highlightColourId,    0x8899aadd,

            DirectoryContentsDisplayComponent::highlightColourId,              textHighlightColour,
            DirectoryContentsDisplayComponent::textColourId,                   0xff000000,
            DirectoryContentsDisplayComponent::highlightedTextColourId,        0xff000000,

            0x1000440, /*LassoComponent::lassoFillColourId*/        0x66dddddd,
            0x1000441, /*LassoComponent::lassoOutlineColourId*/     0x99111111,

            0x1005000, /*MidiKeyboardComponent::whiteNoteColourId*/               0xffffffff,
            0x1005001, /*MidiKeyboardComponent::blackNoteColourId*/               0xff000000,
            0x1005002, /*MidiKeyboardComponent::keySeparatorLineColourId*/        0x66000000,
            0x1005003, /*MidiKeyboardComponent::mouseOverKeyOverlayColourId*/     0x80ffff00,
            0x1005004, /*MidiKeyboardComponent::keyDownOverlayColourId*/          0xffb6b600,
            0x1005005, /*MidiKeyboardComponent::textLabelColourId*/               0xff000000,
            0x1005006, /*MidiKeyboardComponent::upDownButtonBackgroundColourId*/  0xffd3d3d3,
            0x1005007, /*MidiKeyboardComponent::upDownButtonArrowColourId*/       0xff000000,
            0x1005008, /*MidiKeyboardComponent::shadowColourId*/                  0x4c000000,

            0x1004500, /*CodeEditorComponent::backgroundColourId*/                0xffffffff,
            0x1004502, /*CodeEditorComponent::highlightColourId*/                 textHighlightColour,
            0x1004503, /*CodeEditorComponent::defaultTextColourId*/               0xff000000,
            0x1004504, /*CodeEditorComponent::lineNumberBackgroundId*/            0x44999999,
            0x1004505, /*CodeEditorComponent::lineNumberTextId*/                  0x44000000,

            0x1007000, /*ColourSelector::backgroundColourId*/                     0xffe5e5e5,
            0x1007001, /*ColourSelector::labelTextColourId*/                      0xff000000,

            0x100ad00, /*KeyMappingEditorComponent::backgroundColourId*/          0x00000000,
            0x100ad01, /*KeyMappingEditorComponent::textColourId*/                0xff000000,

            FileSearchPathListComponent::backgroundColourId,        0xffffffff,

            FileChooserDialogBox::titleTextColourId,                0xff000000,

            SidePanel::backgroundColour,                            0xffffffff,
            SidePanel::titleTextColour,                             0xff000000,
            SidePanel::shadowBaseColour,                            0xff000000,
            SidePanel::dismissButtonNormalColour,                   textButtonColour,
            SidePanel::dismissButtonOverColour,                     textButtonColour,
            SidePanel::dismissButtonDownColour,                     0xff4444ff,

            FileBrowserComponent::currentPathBoxBackgroundColourId,    0xffffffff,
            FileBrowserComponent::currentPathBoxTextColourId,          0xff000000,
            FileBrowserComponent::currentPathBoxArrowColourId,         0x99000000,
            FileBrowserComponent::filenameBoxBackgroundColourId,       0xffffffff,
            FileBrowserComponent::filenameBoxTextColourId,             0xff000000,
        };

        for (int i = 0; i < numElementsInArray (standardColours); i += 2)
            setColour ((int) standardColours [i], Colour ((uint32) standardColours [i + 1]));
        */
    }
}
    
impl<'a> LookAndFeel_V2<'a> {

    pub fn draw_spinning_wait_animation(&mut self, 
        g:      &mut Graphics,
        colour: &Colour,
        x:      i32,
        y:      i32,
        w:      i32,
        h:      i32)  {
        
        todo!();
        /*
            const float radius = (float) jmin (w, h) * 0.4f;
        const float thickness = radius * 0.15f;
        Path p;
        p.addRoundedRectangle (radius * 0.4f, thickness * -0.5f,
                               radius * 0.6f, thickness,
                               thickness * 0.5f);

        const float cx = (float) x + (float) w * 0.5f;
        const float cy = (float) y + (float) h * 0.5f;

        const uint32 animationIndex = (Time::getMillisecondCounter() / (1000 / 10)) % 12;

        for (uint32 i = 0; i < 12; ++i)
        {
            const uint32 n = (i + 12 - animationIndex) % 12;

            g.setColour (colour.withMultipliedAlpha ((float) (n + 1) / 12.0f));
            g.fillPath (p, AffineTransform::rotation ((float) i * (MathConstants<float>::pi / 6.0f))
                                           .translated (cx, cy));
        }
        */
    }
    
    pub fn create_drop_shadower_for_component(&mut self, _0: *mut Component) -> *mut DropShadower {
        
        todo!();
        /*
            return new DropShadower (DropShadow (Colours::black.withAlpha (0.4f), 10, Point<int> (0, 2)));
        */
    }
    
       
    pub fn get_tick_shape(&mut self, height: f32) -> Path {
        
        todo!();
        /*
            static const unsigned char data[] =
        {
            109,0,224,168,68,0,0,119,67,108,0,224,172,68,0,128,146,67,113,0,192,148,68,0,0,219,67,0,96,110,68,0,224,56,68,113,0,64,51,68,0,32,130,68,0,64,20,68,0,224,
            162,68,108,0,128,3,68,0,128,168,68,113,0,128,221,67,0,192,175,68,0,0,207,67,0,32,179,68,113,0,0,201,67,0,224,173,68,0,0,181,67,0,224,161,68,108,0,128,168,67,
            0,128,154,68,113,0,128,141,67,0,192,138,68,0,128,108,67,0,64,131,68,113,0,0,62,67,0,128,119,68,0,0,5,67,0,128,114,68,113,0,0,102,67,0,192,88,68,0,128,155,
            67,0,192,88,68,113,0,0,190,67,0,192,88,68,0,128,232,67,0,224,131,68,108,0,128,246,67,0,192,139,68,113,0,64,33,68,0,128,87,68,0,0,93,68,0,224,26,68,113,0,
            96,140,68,0,128,188,67,0,224,168,68,0,0,119,67,99,101
        };

        return createPathFromData (height, data, sizeof (data));
        */
    }
    
    pub fn get_cross_shape(&mut self, height: f32) -> Path {
        
        todo!();
        /*
            static const unsigned char data[] =
        {
            109,0,0,17,68,0,96,145,68,108,0,192,13,68,0,192,147,68,113,0,0,213,67,0,64,174,68,0,0,168,67,0,64,174,68,113,0,0,104,67,0,64,174,68,0,0,5,67,0,64,
            153,68,113,0,0,18,67,0,64,153,68,0,0,24,67,0,64,153,68,113,0,0,135,67,0,64,153,68,0,128,207,67,0,224,130,68,108,0,0,220,67,0,0,126,68,108,0,0,204,67,
            0,128,117,68,113,0,0,138,67,0,64,82,68,0,0,138,67,0,192,57,68,113,0,0,138,67,0,192,37,68,0,128,210,67,0,64,10,68,113,0,128,220,67,0,64,45,68,0,0,8,
            68,0,128,78,68,108,0,192,14,68,0,0,87,68,108,0,64,20,68,0,0,80,68,113,0,192,57,68,0,0,32,68,0,128,88,68,0,0,32,68,113,0,64,112,68,0,0,32,68,0,
            128,124,68,0,64,68,68,113,0,0,121,68,0,192,67,68,0,128,119,68,0,192,67,68,113,0,192,108,68,0,192,67,68,0,32,89,68,0,96,82,68,113,0,128,69,68,0,0,97,68,
            0,0,56,68,0,64,115,68,108,0,64,49,68,0,128,124,68,108,0,192,55,68,0,96,129,68,113,0,0,92,68,0,224,146,68,0,192,129,68,0,224,146,68,113,0,64,110,68,0,64,
            168,68,0,64,87,68,0,64,168,68,113,0,128,66,68,0,64,168,68,0,64,27,68,0,32,150,68,99,101
        };

        return createPathFromData (height, data, sizeof (data));
        */
    }
    
    /**
      | Draws a 3D raised (or indented) bevel
      | using two colours.
      | 
      | The bevel is drawn inside the given rectangle,
      | and greater bevel thicknesses extend
      | inwards.
      | 
      | The top-left colour is used for the top-
      | and left-hand edges of the bevel; the
      | bottom-right colour is used for the
      | bottom- and right-hand edges.
      | 
      | If useGradient is true, then the bevel
      | fades out to make it look more curved
      | and less angular. If sharpEdgeOnOutside
      | is true, the outside of the bevel is sharp,
      | and it fades towards the centre; if sharpEdgeOnOutside
      | is false, then the centre edges are sharp
      | and it fades towards the outside.
      |
      */
    pub fn draw_bevel(
        &mut self, 
        g:                     &mut Graphics,
        x:                     i32,
        y:                     i32,
        width:                 i32,
        height:                i32,
        bevel_thickness:       i32,
        top_left_colour:       Option<Colour>,
        bottom_right_colour:   Option<Colour>,
        use_gradient:          Option<bool>,
        sharp_edge_on_outside: Option<bool>

    ) {

        let top_left_colour       = top_left_colour.unwrap_or(colours::white);
        let bottom_right_colour   = bottom_right_colour.unwrap_or(colours::black);
        let use_gradient          = use_gradient.unwrap_or(true);
        let sharp_edge_on_outside = sharp_edge_on_outside.unwrap_or(true);
        
        todo!();
        /*
            if (g.clipRegionIntersects (Rectangle<int> (x, y, width, height)))
        {
            auto& context = g.getInternalContext();
            Graphics::ScopedSaveState ss (g);

            for (int i = bevelThickness; --i >= 0;)
            {
                const float op = useGradient ? (float) (sharpEdgeOnOutside ? bevelThickness - i : i) / (float) bevelThickness
                                             : 1.0f;

                context.setFill (topLeftColour.withMultipliedAlpha (op));
                context.fillRect (Rectangle<int> (x + i, y + i, width - i * 2, 1), false);
                context.setFill (topLeftColour.withMultipliedAlpha (op * 0.75f));
                context.fillRect (Rectangle<int> (x + i, y + i + 1, 1, height - i * 2 - 2), false);
                context.setFill (bottomRightColour.withMultipliedAlpha (op));
                context.fillRect (Rectangle<int> (x + i, y + height - i - 1, width - i * 2, 1), false);
                context.setFill (bottomRightColour.withMultipliedAlpha (op  * 0.75f));
                context.fillRect (Rectangle<int> (x + width - i - 1, y + i + 1, 1, height - i * 2 - 2), false);
            }
        }
        */
    }
    
    pub fn draw_shiny_button_shape(&mut self, 
        g:               &mut Graphics,
        x:               f32,
        y:               f32,
        w:               f32,
        h:               f32,
        max_corner_size: f32,
        base_colour:     &Colour,
        stroke_width:    f32,
        flat_on_left:    bool,
        flat_on_right:   bool,
        flat_on_top:     bool,
        flat_on_bottom:  bool)  {
        
        todo!();
        /*
            if (w <= strokeWidth * 1.1f || h <= strokeWidth * 1.1f)
            return;

        auto cs = jmin (maxCornerSize, w * 0.5f, h * 0.5f);

        Path outline;
        outline.addRoundedRectangle (x, y, w, h, cs, cs,
                                     ! (flatOnLeft  || flatOnTop),
                                     ! (flatOnRight || flatOnTop),
                                     ! (flatOnLeft  || flatOnBottom),
                                     ! (flatOnRight || flatOnBottom));

        ColourGradient cg (baseColour, 0.0f, y,
                           baseColour.overlaidWith (Colour (0x070000ff)), 0.0f, y + h,
                           false);

        cg.addColour (0.5,  baseColour.overlaidWith (Colour (0x33ffffff)));
        cg.addColour (0.51, baseColour.overlaidWith (Colour (0x110000ff)));

        g.setGradientFill (cg);
        g.fillPath (outline);

        g.setColour (Colour (0x80000000));
        g.strokePath (outline, PathStrokeType (strokeWidth));
        */
    }
    
    /**
      | Utility function to draw a shiny, glassy
      | circle (for round LED-type buttons).
      |
      */
    pub fn draw_glass_sphere(&mut self, 
        g:                 &mut Graphics,
        x:                 f32,
        y:                 f32,
        diameter:          f32,
        colour:            &Colour,
        outline_thickness: f32)  {
        
        todo!();
        /*
            if (diameter <= outlineThickness)
            return;

        Path p;
        p.addEllipse (x, y, diameter, diameter);

        {
            ColourGradient cg (Colours::white.overlaidWith (colour.withMultipliedAlpha (0.3f)), 0, y,
                               Colours::white.overlaidWith (colour.withMultipliedAlpha (0.3f)), 0, y + diameter, false);

            cg.addColour (0.4, Colours::white.overlaidWith (colour));

            g.setGradientFill (cg);
            g.fillPath (p);
        }

        g.setGradientFill (ColourGradient (Colours::white, 0, y + diameter * 0.06f,
                                           Colours::transparentWhite, 0, y + diameter * 0.3f, false));
        g.fillEllipse (x + diameter * 0.2f, y + diameter * 0.05f, diameter * 0.6f, diameter * 0.4f);

        ColourGradient cg (Colours::transparentBlack,
                           x + diameter * 0.5f, y + diameter * 0.5f,
                           Colours::black.withAlpha (0.5f * outlineThickness * colour.getFloatAlpha()),
                           x, y + diameter * 0.5f, true);

        cg.addColour (0.7, Colours::transparentBlack);
        cg.addColour (0.8, Colours::black.withAlpha (0.1f * outlineThickness));

        g.setGradientFill (cg);
        g.fillPath (p);

        g.setColour (Colours::black.withAlpha (0.5f * colour.getFloatAlpha()));
        g.drawEllipse (x, y, diameter, diameter, outlineThickness);
        */
    }
    
    pub fn draw_glass_pointer(&mut self, 
        g:                 &mut Graphics,
        x:                 f32,
        y:                 f32,
        diameter:          f32,
        colour:            &Colour,
        outline_thickness: f32,
        direction:         i32)  {
        
        todo!();
        /*
            if (diameter <= outlineThickness)
            return;

        Path p;
        p.startNewSubPath (x + diameter * 0.5f, y);
        p.lineTo (x + diameter, y + diameter * 0.6f);
        p.lineTo (x + diameter, y + diameter);
        p.lineTo (x, y + diameter);
        p.lineTo (x, y + diameter * 0.6f);
        p.closeSubPath();

        p.applyTransform (AffineTransform::rotation ((float) direction * MathConstants<float>::halfPi,
                                                     x + diameter * 0.5f,
                                                     y + diameter * 0.5f));

        {
            ColourGradient cg (Colours::white.overlaidWith (colour.withMultipliedAlpha (0.3f)), 0, y,
                               Colours::white.overlaidWith (colour.withMultipliedAlpha (0.3f)), 0, y + diameter, false);

            cg.addColour (0.4, Colours::white.overlaidWith (colour));

            g.setGradientFill (cg);
            g.fillPath (p);
        }

        ColourGradient cg (Colours::transparentBlack,
                           x + diameter * 0.5f, y + diameter * 0.5f,
                           Colours::black.withAlpha (0.5f * outlineThickness * colour.getFloatAlpha()),
                           x - diameter * 0.2f, y + diameter * 0.5f, true);

        cg.addColour (0.5, Colours::transparentBlack);
        cg.addColour (0.7, Colours::black.withAlpha (0.07f * outlineThickness));

        g.setGradientFill (cg);
        g.fillPath (p);

        g.setColour (Colours::black.withAlpha (0.5f * colour.getFloatAlpha()));
        g.strokePath (p, PathStrokeType (outlineThickness));
        */
    }
    
    /**
      | Utility function to draw a shiny, glassy
      | oblong (for text buttons).
      |
      */
    pub fn draw_glass_lozenge(&mut self, 
        g:                 &mut Graphics,
        x:                 f32,
        y:                 f32,
        width:             f32,
        height:            f32,
        colour:            &Colour,
        outline_thickness: f32,
        corner_size:       f32,
        flat_on_left:      bool,
        flat_on_right:     bool,
        flat_on_top:       bool,
        flat_on_bottom:    bool)  {
        
        todo!();
        /*
            if (width <= outlineThickness || height <= outlineThickness)
            return;

        auto intX = (int) x;
        auto intY = (int) y;
        auto intW = (int) width;
        auto intH = (int) height;

        auto cs = cornerSize < 0 ? jmin (width * 0.5f, height * 0.5f) : cornerSize;
        auto edgeBlurRadius = height * 0.75f + (height - cs * 2.0f);
        auto intEdge = (int) edgeBlurRadius;

        Path outline;
        outline.addRoundedRectangle (x, y, width, height, cs, cs,
                                     ! (flatOnLeft || flatOnTop),
                                     ! (flatOnRight || flatOnTop),
                                     ! (flatOnLeft || flatOnBottom),
                                     ! (flatOnRight || flatOnBottom));

        {
            ColourGradient cg (colour.darker (0.2f), 0, y,
                               colour.darker (0.2f), 0, y + height, false);

            cg.addColour (0.03, colour.withMultipliedAlpha (0.3f));
            cg.addColour (0.4, colour);
            cg.addColour (0.97, colour.withMultipliedAlpha (0.3f));

            g.setGradientFill (cg);
            g.fillPath (outline);
        }

        ColourGradient cg (Colours::transparentBlack, x + edgeBlurRadius, y + height * 0.5f,
                           colour.darker (0.2f), x, y + height * 0.5f, true);

        cg.addColour (jlimit (0.0, 1.0, 1.0 - (cs * 0.5f) / edgeBlurRadius), Colours::transparentBlack);
        cg.addColour (jlimit (0.0, 1.0, 1.0 - (cs * 0.25f) / edgeBlurRadius), colour.darker (0.2f).withMultipliedAlpha (0.3f));

        if (! (flatOnLeft || flatOnTop || flatOnBottom))
        {
            Graphics::ScopedSaveState ss (g);

            g.setGradientFill (cg);
            g.reduceClipRegion (intX, intY, intEdge, intH);
            g.fillPath (outline);
        }

        if (! (flatOnRight || flatOnTop || flatOnBottom))
        {
            cg.point1.setX (x + width - edgeBlurRadius);
            cg.point2.setX (x + width);

            Graphics::ScopedSaveState ss (g);

            g.setGradientFill (cg);
            g.reduceClipRegion (intX + intW - intEdge, intY, 2 + intEdge, intH);
            g.fillPath (outline);
        }

        {
            auto leftIndent  = (flatOnTop || flatOnLeft)  ? 0.0f : cs * 0.4f;
            auto rightIndent = (flatOnTop || flatOnRight) ? 0.0f : cs * 0.4f;

            Path highlight;
            highlight.addRoundedRectangle (x + leftIndent,
                                           y + cs * 0.1f,
                                           width - (leftIndent + rightIndent),
                                           height * 0.4f,
                                           cs * 0.4f,
                                           cs * 0.4f,
                                           ! (flatOnLeft || flatOnTop),
                                           ! (flatOnRight || flatOnTop),
                                           ! (flatOnLeft || flatOnBottom),
                                           ! (flatOnRight || flatOnBottom));

            g.setGradientFill (ColourGradient (colour.brighter (10.0f), 0, y + height * 0.06f,
                                               Colours::transparentWhite, 0, y + height * 0.4f, false));
            g.fillPath (highlight);
        }

        g.setColour (colour.darker().withMultipliedAlpha (1.5f));
        g.strokePath (outline, PathStrokeType (outlineThickness));
        */
    }
}

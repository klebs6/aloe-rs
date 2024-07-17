crate::ix!();

pub fn draw_button_shape(
        g:           &mut Graphics,
        outline:     &Path,
        base_colour: Colour,
        height:      f32)  {
    
    todo!();
    /*
        const float mainBrightness = baseColour.getBrightness();
        const float mainAlpha = baseColour.getFloatAlpha();

        g.setGradientFill (ColourGradient::vertical (baseColour.brighter (0.2f), 0.0f,
                                                     baseColour.darker (0.25f), height));
        g.fillPath (outline);

        g.setColour (Colours::white.withAlpha (0.4f * mainAlpha * mainBrightness * mainBrightness));
        g.strokePath (outline, PathStrokeType (1.0f), AffineTransform::translation (0.0f, 1.0f)
                                                            .scaled (1.0f, (height - 1.6f) / height));

        g.setColour (Colours::black.withAlpha (0.4f * mainAlpha));
        g.strokePath (outline, PathStrokeType (1.0f));
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V3.h]

/**
  | The latest Aloe look-and-feel style,
  | as introduced in 2013. @see LookAndFeel,
  | LookAndFeel_V1, LookAndFeel_V2
  | 
  | @tags{GUI}
  |
  */
pub struct LookAndFeel_V3<'a> {
    base:                           LookAndFeel_V2<'a>,
    background_texture:             Image,
    background_texture_base_colour: Colour,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V3.cpp]
impl<'a> Default for LookAndFeel_V3<'a> {
    
    fn default() -> Self {
    
        todo!();
        /*


            setColour (TreeView::selectedItemBackgroundColourId, Colour (0x301111ee));

        const Colour textButtonColour (0xffeeeeff);
        setColour (TextButton::buttonColourId, textButtonColour);
        setColour (TextButton::buttonOnColourId, Colour (0xff888888));
        setColour (ComboBox::buttonColourId, textButtonColour);
        setColour (ComboBox::focusedOutlineColourId, textButtonColour);
        setColour (TextEditor::outlineColourId, Colours::transparentBlack);
        setColour (TabbedButtonBar::tabOutlineColourId, Colour (0x66000000));
        setColour (TabbedComponent::outlineColourId, Colour (0x66000000));
        setColour (Slider::trackColourId, Colour (0xbbffffff));
        setColour (Slider::thumbColourId, Colour (0xffddddff));
        setColour (BubbleComponent::backgroundColourId, Colour (0xeeeeeedd));
        setColour (ScrollBar::thumbColourId, Colour::greyLevel (0.8f).contrasting().withAlpha (0.13f));
        setColour (TableHeaderComponent::backgroundColourId, Colours::white.withAlpha (0.6f));
        setColour (TableHeaderComponent::outlineColourId,    Colours::black.withAlpha (0.5f));
        */
    }
}

impl<'a> LookAndFeel_V3<'a> {

    pub fn are_scrollbar_buttons_visible(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn draw_stretchable_layout_resizer_bar(&mut self, 
        g:                 &mut Graphics,
        w:                 i32,
        h:                 i32,
        is_vertical_bar:   bool,
        is_mouse_over:     bool,
        is_mouse_dragging: bool)  {
        
        todo!();
        /*
            if (isMouseOver || isMouseDragging)
            g.fillAll (Colours::yellow.withAlpha (0.4f));
        */
    }
    
    pub fn draw_scrollbar(&mut self, 
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
            Path thumbPath;

        if (thumbSize > 0)
        {
            const float thumbIndent = (float) (isScrollbarVertical ? width : height) * 0.25f;
            const float thumbIndentx2 = thumbIndent * 2.0f;

            if (isScrollbarVertical)
                thumbPath.addRoundedRectangle ((float) x + thumbIndent, (float) thumbStartPosition + thumbIndent,
                                               (float) width - thumbIndentx2, (float) thumbSize - thumbIndentx2, ((float) width - thumbIndentx2) * 0.5f);
            else
                thumbPath.addRoundedRectangle ((float) thumbStartPosition + thumbIndent, (float) y + thumbIndent,
                                               (float) thumbSize - thumbIndentx2, (float) height - thumbIndentx2, ((float) height - thumbIndentx2) * 0.5f);
        }

        Colour thumbCol (scrollbar.findColour (ScrollBar::thumbColourId, true));

        if (isMouseOver || isMouseDown)
            thumbCol = thumbCol.withMultipliedAlpha (2.0f);

        g.setColour (thumbCol);
        g.fillPath (thumbPath);

        g.setColour (thumbCol.contrasting ((isMouseOver  || isMouseDown) ? 0.2f : 0.1f));
        g.strokePath (thumbPath, PathStrokeType (1.0f));
        */
    }
    
    pub fn draw_concertina_panel_header(&mut self, 
        g:             &mut Graphics,
        area:          &Rectangle<i32>,
        is_mouse_over: bool,
        is_mouse_down: bool,
        _4:            &mut ConcertinaPanel,
        panel:         &mut Component)  {
        
        todo!();
        /*
            const Colour bkg (Colours::grey);

        g.setGradientFill (ColourGradient::vertical (Colours::white.withAlpha (isMouseOver ? 0.4f : 0.2f), (float) area.getY(),
                                                     Colours::darkgrey.withAlpha (0.1f), (float) area.getBottom()));
        g.fillAll();

        g.setColour (bkg.contrasting().withAlpha (0.1f));
        g.fillRect (area.withHeight (1));
        g.fillRect (area.withTop (area.getBottom() - 1));

        g.setColour (bkg.contrasting());
        g.setFont (Font ((float) area.getHeight() * 0.6f).boldened());
        g.drawFittedText (panel.getName(), 4, 0, area.getWidth() - 6, area.getHeight(), Justification::centredLeft, 1);
        */
    }
    
    pub fn draw_button_background(&mut self, 
        g:                                 &mut Graphics,
        button:                            &mut Button,
        background_colour:                 &Colour,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            Colour baseColour (backgroundColour.withMultipliedSaturation (button.hasKeyboardFocus (true) ? 1.3f : 0.9f)
                                           .withMultipliedAlpha (button.isEnabled() ? 0.9f : 0.5f));

        if (shouldDrawButtonAsDown || shouldDrawButtonAsHighlighted)
            baseColour = baseColour.contrasting (shouldDrawButtonAsDown ? 0.2f : 0.1f);

        const bool flatOnLeft   = button.isConnectedOnLeft();
        const bool flatOnRight  = button.isConnectedOnRight();
        const bool flatOnTop    = button.isConnectedOnTop();
        const bool flatOnBottom = button.isConnectedOnBottom();

        const float width  = (float) button.getWidth()  - 1.0f;
        const float height = (float) button.getHeight() - 1.0f;

        if (width > 0 && height > 0)
        {
            const float cornerSize = 4.0f;

            Path outline;
            outline.addRoundedRectangle (0.5f, 0.5f, width, height, cornerSize, cornerSize,
                                         ! (flatOnLeft  || flatOnTop),
                                         ! (flatOnRight || flatOnTop),
                                         ! (flatOnLeft  || flatOnBottom),
                                         ! (flatOnRight || flatOnBottom));

            drawButtonShape (g, outline, baseColour, height);
        }
        */
    }
    
    pub fn draw_table_header_background(&mut self, 
        g:      &mut Graphics,
        header: &mut TableHeaderComponent)  {
        
        todo!();
        /*
            auto r = header.getLocalBounds();
        auto outlineColour = header.findColour (TableHeaderComponent::outlineColourId);

        g.setColour (outlineColour);
        g.fillRect (r.removeFromBottom (1));

        g.setColour (header.findColour (TableHeaderComponent::backgroundColourId));
        g.fillRect (r);

        g.setColour (outlineColour);

        for (int i = header.getNumColumns (true); --i >= 0;)
            g.fillRect (header.getColumnPosition (i).removeFromRight (1));
        */
    }
    
    pub fn get_tab_button_overlap(&mut self, tab_depth: i32) -> i32 {
        
        todo!();
        /*
            return -1;
        */
    }
    
    pub fn get_tab_button_space_around_image(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn create_tab_text_layout(&mut self, 
        button:      &TabBarButton,
        length:      f32,
        depth:       f32,
        colour:      Colour,
        text_layout: &mut TextLayout)  {
        
        todo!();
        /*
            Font font (depth * 0.5f);
        font.setUnderline (button.hasKeyboardFocus (false));

        AttributedString s;
        s.setJustification (Justification::centred);
        s.append (button.getButtonText().trim(), font, colour);

        textLayout.createLayout (s, length);
        */
    }
    
    pub fn draw_tab_button(&mut self, 
        button:        &mut TabBarButton,
        g:             &mut Graphics,
        is_mouse_over: bool,
        is_mouse_down: bool)  {
        
        todo!();
        /*
            const Rectangle<int> activeArea (button.getActiveArea());

        const TabbedButtonBar::Orientation o = button.getTabbedButtonBar().getOrientation();

        const Colour bkg (button.getTabBackgroundColour());

        if (button.getToggleState())
        {
            g.setColour (bkg);
        }
        else
        {
            Point<int> p1, p2;

            switch (o)
            {
                case TabbedButtonBar::TabsAtBottom:   p1 = activeArea.getBottomLeft(); p2 = activeArea.getTopLeft();    break;
                case TabbedButtonBar::TabsAtTop:      p1 = activeArea.getTopLeft();    p2 = activeArea.getBottomLeft(); break;
                case TabbedButtonBar::TabsAtRight:    p1 = activeArea.getTopRight();   p2 = activeArea.getTopLeft();    break;
                case TabbedButtonBar::TabsAtLeft:     p1 = activeArea.getTopLeft();    p2 = activeArea.getTopRight();   break;
                default:                              jassertfalse; break;
            }

            g.setGradientFill (ColourGradient (bkg.brighter (0.2f), p1.toFloat(),
                                               bkg.darker (0.1f),   p2.toFloat(), false));
        }

        g.fillRect (activeArea);

        g.setColour (button.findColour (TabbedButtonBar::tabOutlineColourId));

        Rectangle<int> r (activeArea);

        if (o != TabbedButtonBar::TabsAtBottom)   g.fillRect (r.removeFromTop (1));
        if (o != TabbedButtonBar::TabsAtTop)      g.fillRect (r.removeFromBottom (1));
        if (o != TabbedButtonBar::TabsAtRight)    g.fillRect (r.removeFromLeft (1));
        if (o != TabbedButtonBar::TabsAtLeft)     g.fillRect (r.removeFromRight (1));

        const float alpha = button.isEnabled() ? ((isMouseOver || isMouseDown) ? 1.0f : 0.8f) : 0.3f;

        Colour col (bkg.contrasting().withMultipliedAlpha (alpha));

        if (TabbedButtonBar* bar = button.findParentComponentOfClass<TabbedButtonBar>())
        {
            TabbedButtonBar::ColourIds colID = button.isFrontTab() ? TabbedButtonBar::frontTextColourId
                                                                   : TabbedButtonBar::tabTextColourId;

            if (bar->isColourSpecified (colID))
                col = bar->findColour (colID);
            else if (isColourSpecified (colID))
                col = findColour (colID);
        }

        const Rectangle<float> area (button.getTextArea().toFloat());

        float length = area.getWidth();
        float depth  = area.getHeight();

        if (button.getTabbedButtonBar().isVertical())
            std::swap (length, depth);

        TextLayout textLayout;
        createTabTextLayout (button, length, depth, col, textLayout);

        AffineTransform t;

        switch (o)
        {
            case TabbedButtonBar::TabsAtLeft:   t = t.rotated (MathConstants<float>::pi * -0.5f).translated (area.getX(), area.getBottom()); break;
            case TabbedButtonBar::TabsAtRight:  t = t.rotated (MathConstants<float>::pi *  0.5f).translated (area.getRight(), area.getY()); break;
            case TabbedButtonBar::TabsAtTop:
            case TabbedButtonBar::TabsAtBottom: t = t.translated (area.getX(), area.getY()); break;
            default:                            jassertfalse; break;
        }

        g.addTransform (t);
        textLayout.draw (g, Rectangle<float> (length, depth));
        */
    }
    
    pub fn draw_tab_area_behind_front_button(&mut self, 
        bar: &mut TabbedButtonBar,
        g:   &mut Graphics,
        w:   i32,
        h:   i32)  {
        
        todo!();
        /*
            const float shadowSize = 0.15f;

        Rectangle<int> shadowRect, line;
        ColourGradient gradient (Colours::black.withAlpha (bar.isEnabled() ? 0.08f : 0.04f), 0, 0,
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

        g.setColour (bar.findColour (TabbedButtonBar::tabOutlineColourId));
        g.fillRect (line);
        */
    }
    
    pub fn draw_text_editor_outline(&mut self, 
        g:           &mut Graphics,
        width:       i32,
        height:      i32,
        text_editor: &mut TextEditor)  {
        
        todo!();
        /*
            if (textEditor.isEnabled())
        {
            if (textEditor.hasKeyboardFocus (true) && ! textEditor.isReadOnly())
            {
                g.setColour (textEditor.findColour (TextEditor::focusedOutlineColourId));
                g.drawRect (0, 0, width, height, 2);
            }
            else
            {
                g.setColour (textEditor.findColour (TextEditor::outlineColourId));
                g.drawRect (0, 0, width, height);
            }
        }
        */
    }
    
    pub fn draw_treeview_plus_minus_box(&mut self, 
        g:                 &mut Graphics,
        area:              &Rectangle<f32>,
        background_colour: Colour,
        is_open:           bool,
        is_mouse_over:     bool)  {
        
        todo!();
        /*
            Path p;
        p.addTriangle (0.0f, 0.0f, 1.0f, isOpen ? 0.0f : 0.5f, isOpen ? 0.5f : 0.0f, 1.0f);

        g.setColour (backgroundColour.contrasting().withAlpha (isMouseOver ? 0.5f : 0.3f));
        g.fillPath (p, p.getTransformToScaleToFit (area.reduced (2, area.getHeight() / 4), true));
        */
    }
    
    pub fn are_lines_drawn_for_tree_view(&mut self, _0: &mut TreeView) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_tree_view_indent_size(&mut self, _0: &mut TreeView) -> i32 {
        
        todo!();
        /*
            return 20;
        */
    }
    
    pub fn draw_combo_box(&mut self, 
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

        g.setColour (box.findColour (ComboBox::arrowColourId).withMultipliedAlpha (box.isEnabled() ? 1.0f : 0.3f));
        g.fillPath (p);
        */
    }
    
    pub fn draw_linear_slider(
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
            const float fx = (float) x, fy = (float) y, fw = (float) width, fh = (float) height;

            Path p;

            if (style == Slider::LinearBarVertical)
                p.addRectangle (fx, sliderPos, fw, 1.0f + fh - sliderPos);
            else
                p.addRectangle (fx, fy, sliderPos - fx, fh);

            auto baseColour = slider.findColour (Slider::thumbColourId)
                                    .withMultipliedSaturation (slider.isEnabled() ? 1.0f : 0.5f)
                                    .withMultipliedAlpha (0.8f);

            g.setGradientFill (ColourGradient::vertical (baseColour.brighter (0.08f), 0.0f,
                                                         baseColour.darker (0.08f), (float) height));
            g.fillPath (p);

            g.setColour (baseColour.darker (0.2f));

            if (style == Slider::LinearBarVertical)
                g.fillRect (fx, sliderPos, fw, 1.0f);
            else
                g.fillRect (sliderPos, fy, 1.0f, fh);
        }
        else
        {
            drawLinearSliderBackground (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
            drawLinearSliderThumb (g, x, y, width, height, sliderPos, minSliderPos, maxSliderPos, style, slider);
        }
        */
    }
    
    pub fn draw_linear_slider_background(
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
            const float sliderRadius = (float) (getSliderThumbRadius (slider) - 2);

        const Colour trackColour (slider.findColour (Slider::trackColourId));
        const Colour gradCol1 (trackColour.overlaidWith (Colour (slider.isEnabled() ? 0x13000000 : 0x09000000)));
        const Colour gradCol2 (trackColour.overlaidWith (Colour (0x06000000)));
        Path indent;

        if (slider.isHorizontal())
        {
            auto iy = (float) y + (float) height * 0.5f - sliderRadius * 0.5f;

            g.setGradientFill (ColourGradient::vertical (gradCol1, iy, gradCol2, iy + sliderRadius));

            indent.addRoundedRectangle ((float) x - sliderRadius * 0.5f, iy, (float) width + sliderRadius, sliderRadius, 5.0f);
        }
        else
        {
            auto ix = (float) x + (float) width * 0.5f - sliderRadius * 0.5f;

            g.setGradientFill (ColourGradient::horizontal (gradCol1, ix, gradCol2, ix + sliderRadius));

            indent.addRoundedRectangle (ix, (float) y - sliderRadius * 0.5f, sliderRadius, (float) height + sliderRadius, 5.0f);
        }

        g.fillPath (indent);

        g.setColour (trackColour.contrasting (0.5f));
        g.strokePath (indent, PathStrokeType (0.5f));
        */
    }
    
    pub fn draw_popup_menu_background(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            g.fillAll (findColour (PopupMenu::backgroundColourId));
        ignoreUnused (width, height);

       #if ! ALOE_MAC
        g.setColour (findColour (PopupMenu::textColourId).withAlpha (0.6f));
        g.drawRect (0, 0, width, height);
       #endif
        */
    }
    
    pub fn draw_menu_bar_background(&mut self, 
        g:        &mut Graphics,
        width:    i32,
        height:   i32,
        _3:       bool,
        menu_bar: &mut MenuBarComponent)  {
        
        todo!();
        /*
            auto colour = menuBar.findColour (PopupMenu::backgroundColourId);

        Rectangle<int> r (width, height);

        g.setColour (colour.contrasting (0.15f));
        g.fillRect (r.removeFromTop (1));
        g.fillRect (r.removeFromBottom (1));

        g.setGradientFill (ColourGradient::vertical (colour, 0, colour.darker (0.08f), (float) height));
        g.fillRect (r);
        */
    }
    
    pub fn draw_keymap_change_button(&mut self, 
        g:               &mut Graphics,
        width:           i32,
        height:          i32,
        button:          &mut Button,
        key_description: &String)  {
        
        todo!();
        /*
            const Colour textColour (button.findColour (0x100ad01 /*KeyMappingEditorComponent::textColourId*/, true));

        if (keyDescription.isNotEmpty())
        {
            if (button.isEnabled())
            {
                g.setColour (textColour.withAlpha (button.isDown() ? 0.4f : (button.isOver() ? 0.2f : 0.1f)));
                g.fillRoundedRectangle (button.getLocalBounds().toFloat(), 4.0f);
                g.drawRoundedRectangle (button.getLocalBounds().toFloat(), 4.0f, 1.0f);
            }

            g.setColour (textColour);
            g.setFont ((float) height * 0.6f);
            g.drawFittedText (keyDescription, 4, 0, width - 8, height, Justification::centred, 1);
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

            g.setColour (textColour.darker(0.1f).withAlpha (button.isDown() ? 0.7f : (button.isOver() ? 0.5f : 0.3f)));
            g.fillPath (p, p.getTransformToScaleToFit (2.0f, 2.0f, (float) width - 4.0f, (float) height - 4.0f, true));
        }

        if (button.hasKeyboardFocus (false))
        {
            g.setColour (textColour.withAlpha (0.4f));
            g.drawRect (0, 0, width, height);
        }
        */
    }
    
    pub fn create_document_window_button(&mut self, button_type: i32) -> *mut Button {
        
        todo!();
        /*
            Path shape;
        const float crossThickness = 0.25f;

        if (buttonType == DocumentWindow::closeButton)
        {
            shape.addLineSegment (Line<float> (0.0f, 0.0f, 1.0f, 1.0f), crossThickness * 1.4f);
            shape.addLineSegment (Line<float> (1.0f, 0.0f, 0.0f, 1.0f), crossThickness * 1.4f);

            return new LookAndFeel_V3_DocumentWindowButton ("close", Colour (0xffdd1100), shape, shape);
        }

        if (buttonType == DocumentWindow::minimiseButton)
        {
            shape.addLineSegment (Line<float> (0.0f, 0.5f, 1.0f, 0.5f), crossThickness);

            return new LookAndFeel_V3_DocumentWindowButton ("minimise", Colour (0xffaa8811), shape, shape);
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

            return new LookAndFeel_V3_DocumentWindowButton ("maximise", Colour (0xff119911), shape, fullscreenShape);
        }

        jassertfalse;
        return nullptr;
        */
    }
    
    pub fn get_tick_shape(&mut self, height: f32) -> Path {
        
        todo!();
        /*
            static const unsigned char pathData[]
            = { 110,109,32,210,202,64,126,183,148,64,108,39,244,247,64,245,76,124,64,108,178,131,27,65,246,76,252,64,108,175,242,4,65,246,76,252,
                64,108,236,5,68,65,0,0,160,180,108,240,150,90,65,21,136,52,63,108,48,59,16,65,0,0,32,65,108,32,210,202,64,126,183,148,64, 99,101,0,0 };

        Path p;
        p.loadPathFromData (pathData, sizeof (pathData));
        p.scaleToFit (0, 0, height * 2.0f, height, true);
        return p;
        */
    }
    
    pub fn get_cross_shape(&mut self, height: f32) -> Path {
        
        todo!();
        /*
            static const unsigned char pathData[]
            = { 110,109,88,57,198,65,29,90,171,65,108,63,53,154,65,8,172,126,65,108,76,55,198,65,215,163,38,65,108,141,151,175,65,82,184,242,64,108,117,147,131,65,90,100,81,65,108,184,30,47,65,82,184,242,64,108,59,223,1,65,215,163,38,65,108,84,227,89,65,8,172,126,65,
                108,35,219,1,65,29,90,171,65,108,209,34,47,65,231,251,193,65,108,117,147,131,65,207,247,149,65,108,129,149,175,65,231,251,193,65,99,101,0,0 };

        Path p;
        p.loadPathFromData (pathData, sizeof (pathData));
        p.scaleToFit (0, 0, height * 2.0f, height, true);
        return p;
        */
    }
}

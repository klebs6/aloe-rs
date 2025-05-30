crate::ix!();

pub fn draw_triangle(
        g:       &mut Graphics,
        x1:      f32,
        y1:      f32,
        x2:      f32,
        y2:      f32,
        x3:      f32,
        y3:      f32,
        fill:    Colour,
        outline: Colour)  {
    
    todo!();
    /*
        Path p;
        p.addTriangle (x1, y1, x2, y2, x3, y3);
        g.setColour (fill);
        g.fillPath (p);

        g.setColour (outline);
        g.strokePath (p, PathStrokeType (0.3f));
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V1.h]

/**
  | The original Aloe look-and-feel, as
  | used back from 2002 to about 2007ish.
  | @see LookAndFeel, LookAndFeel_V2,
  | LookAndFeel_V3
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LookAndFeel_V1<'a> {
    base:             LookAndFeel_V2<'a>,
    scrollbar_shadow: DropShadowEffect,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V1.cpp]
impl<'a> Default for LookAndFeel_V1<'a> {
    
    fn default() -> Self {
    
        todo!();
        /*


            setColour (TextButton::buttonColourId,          Colour (0xffbbbbff));
        setColour (ListBox::outlineColourId,            findColour (ComboBox::outlineColourId));
        setColour (ScrollBar::thumbColourId,            Colour (0xffbbbbdd));
        setColour (ScrollBar::backgroundColourId,       Colours::transparentBlack);
        setColour (Slider::thumbColourId,               Colours::white);
        setColour (Slider::trackColourId,               Colour (0x7f000000));
        setColour (Slider::textBoxOutlineColourId,      Colours::grey);
        setColour (ProgressBar::backgroundColourId,     Colours::white.withAlpha (0.6f));
        setColour (ProgressBar::foregroundColourId,     Colours::green.withAlpha (0.7f));
        setColour (PopupMenu::backgroundColourId,             Colour (0xffeef5f8));
        setColour (PopupMenu::highlightedBackgroundColourId,  Colour (0xbfa4c2ce));
        setColour (PopupMenu::highlightedTextColourId,        Colours::black);
        setColour (TextEditor::focusedOutlineColourId,  findColour (TextButton::buttonColourId));

        scrollbarShadow.setShadowProperties (DropShadow (Colours::black.withAlpha (0.5f), 2, Point<int>()));
        */
    }
}
    
impl<'a> LookAndFeel_V1<'a> {

    pub fn draw_button_background(&mut self, 
        g:                                 &mut Graphics,
        button:                            &mut Button,
        background_colour:                 &Colour,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            const int width = button.getWidth();
        const int height = button.getHeight();

        const float indent = 2.0f;
        const int cornerSize = jmin (roundToInt ((float) width * 0.4f),
                                     roundToInt ((float) height * 0.4f));

        Path p;
        p.addRoundedRectangle (indent, indent,
                               (float) width - indent * 2.0f,
                               (float) height - indent * 2.0f,
                               (float) cornerSize);

        Colour bc (backgroundColour.withMultipliedSaturation (0.3f));

        if (shouldDrawButtonAsHighlighted)
        {
            if (shouldDrawButtonAsDown)
                bc = bc.brighter();
            else if (bc.getBrightness() > 0.5f)
                bc = bc.darker (0.1f);
            else
                bc = bc.brighter (0.1f);
        }

        g.setColour (bc);
        g.fillPath (p);

        g.setColour (bc.contrasting().withAlpha ((shouldDrawButtonAsHighlighted) ? 0.6f : 0.4f));
        g.strokePath (p, PathStrokeType ((shouldDrawButtonAsHighlighted) ? 2.0f : 1.4f));
        */
    }
    
    pub fn draw_tick_box(&mut self, 
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
            Path box;
        box.addRoundedRectangle (0.0f, 2.0f, 6.0f, 6.0f, 1.0f);

        g.setColour (isEnabled ? Colours::blue.withAlpha (shouldDrawButtonAsDown ? 0.3f : 0.1f)
                               : Colours::lightgrey.withAlpha (0.1f));

        AffineTransform trans (AffineTransform::scale (w / 9.0f, h / 9.0f).translated (x, y));

        g.fillPath (box, trans);

        g.setColour (Colours::black.withAlpha (0.6f));
        g.strokePath (box, PathStrokeType (0.9f), trans);

        if (ticked)
        {
            Path tick;
            tick.startNewSubPath (1.5f, 3.0f);
            tick.lineTo (3.0f, 6.0f);
            tick.lineTo (6.0f, 0.0f);

            g.setColour (isEnabled ? Colours::black : Colours::grey);
            g.strokePath (tick, PathStrokeType (2.5f), trans);
        }
        */
    }
    
    pub fn draw_toggle_button(&mut self, 
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

        const int tickWidth = jmin (20, button.getHeight() - 4);

        drawTickBox (g, button, 4.0f, (float) (button.getHeight() - tickWidth) * 0.5f,
                     (float) tickWidth, (float) tickWidth,
                     button.getToggleState(),
                     button.isEnabled(),
                     shouldDrawButtonAsHighlighted,
                     shouldDrawButtonAsDown);

        g.setColour (button.findColour (ToggleButton::textColourId));
        g.setFont (jmin (15.0f, (float) button.getHeight() * 0.6f));

        if (! button.isEnabled())
            g.setOpacity (0.5f);

        const int textX = tickWidth + 5;

        g.drawFittedText (button.getButtonText(),
                          textX, 4,
                          button.getWidth() - textX - 2, button.getHeight() - 8,
                          Justification::centredLeft, 10);
        */
    }
    
    pub fn draw_progress_bar(&mut self, 
        g:            &mut Graphics,
        progress_bar: &mut ProgressBar,
        width:        i32,
        height:       i32,
        progress:     f64,
        text_to_show: &String)  {
        
        todo!();
        /*
            if (progress < 0 || progress >= 1.0)
        {
            LookAndFeel_V2::drawProgressBar (g, progressBar, width, height, progress, textToShow);
        }
        else
        {
            const Colour background (progressBar.findColour (ProgressBar::backgroundColourId));
            const Colour foreground (progressBar.findColour (ProgressBar::foregroundColourId));

            g.fillAll (background);
            g.setColour (foreground);

            g.fillRect (1, 1,
                        jlimit (0, width - 2, roundToInt (progress * (width - 2))),
                        height - 2);

            if (textToShow.isNotEmpty())
            {
                g.setColour (Colour::contrasting (background, foreground));
                g.setFont ((float) height * 0.6f);

                g.drawText (textToShow, 0, 0, width, height, Justification::centred, false);
            }
        }
        */
    }
    
    pub fn draw_scrollbar_button(&mut self, 
        g:                                 &mut Graphics,
        bar:                               &mut ScrollBar,
        width:                             i32,
        height:                            i32,
        button_direction:                  i32,
        is_scrollbar_vertical:             bool,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            if (isScrollbarVertical)
            width -= 2;
        else
            height -= 2;

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
            g.setColour (Colours::white);
        else if (shouldDrawButtonAsHighlighted)
            g.setColour (Colours::white.withAlpha (0.7f));
        else
            g.setColour (bar.findColour (ScrollBar::thumbColourId).withAlpha (0.5f));

        g.fillPath (p);

        g.setColour (Colours::black.withAlpha (0.5f));
        g.strokePath (p, PathStrokeType (0.5f));
        */
    }
    
    pub fn draw_scrollbar(&mut self, 
        g:                     &mut Graphics,
        bar:                   &mut ScrollBar,
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
            g.fillAll (bar.findColour (ScrollBar::backgroundColourId));

        g.setColour (bar.findColour (ScrollBar::thumbColourId)
                        .withAlpha ((isMouseOver || isMouseDown) ? 0.4f : 0.15f));

        if ((float) thumbSize > 0.0f)
        {
            Rectangle<int> thumb;

            if (isScrollbarVertical)
            {
                width -= 2;
                g.fillRect (x + roundToInt ((float) width * 0.35f), y,
                            roundToInt ((float) width * 0.3f), height);

                thumb.setBounds (x + 1, thumbStartPosition,
                                 width - 2, thumbSize);
            }
            else
            {
                height -= 2;
                g.fillRect (x, y + roundToInt ((float) height * 0.35f),
                            width, roundToInt ((float) height * 0.3f));

                thumb.setBounds (thumbStartPosition, y + 1,
                                 thumbSize, height - 2);
            }

            g.setColour (bar.findColour (ScrollBar::thumbColourId)
                            .withAlpha ((isMouseOver || isMouseDown) ? 0.95f : 0.7f));

            g.fillRect (thumb);

            g.setColour (Colours::black.withAlpha ((isMouseOver || isMouseDown) ? 0.4f : 0.25f));
            g.drawRect (thumb.getX(), thumb.getY(), thumb.getWidth(), thumb.getHeight());

            if (thumbSize > 16)
            {
                for (int i = 3; --i >= 0;)
                {
                    const float linePos = (float) thumbStartPosition + (float) thumbSize * 0.5f + (float) (i - 1) * 4.0f;
                    g.setColour (Colours::black.withAlpha (0.15f));

                    if (isScrollbarVertical)
                    {
                        g.drawLine ((float) x + (float) width * 0.2f, linePos, (float) width * 0.8f, linePos);
                        g.setColour (Colours::white.withAlpha (0.15f));
                        g.drawLine ((float) width * 0.2f, linePos - 1.0f, (float) width * 0.8f, linePos - 1.0f);
                    }
                    else
                    {
                        g.drawLine (linePos, (float) height * 0.2f, linePos, (float) height * 0.8f);
                        g.setColour (Colours::white.withAlpha (0.15f));
                        g.drawLine (linePos - 1.0f, (float) height * 0.2f, linePos - 1.0f, (float) height * 0.8f);
                    }
                }
            }
        }
        */
    }
    
    pub fn get_scrollbar_effect(&mut self) -> *mut dyn ImageEffectFilter {
        
        todo!();
        /*
            return &scrollbarShadow;
        */
    }
    
    pub fn draw_popup_menu_background(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            g.fillAll (findColour (PopupMenu::backgroundColourId));

        g.setColour (Colours::black.withAlpha (0.6f));
        g.drawRect (0, 0, width, height);
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
            g.fillAll (menuBar.findColour (PopupMenu::backgroundColourId));
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
            g.setColour (textEditor.findColour (TextEditor::outlineColourId));
            g.drawRect (0, 0, width, height);
        }
        */
    }
    
    pub fn draw_combo_box(&mut self, 
        g:              &mut Graphics,
        width:          i32,
        height:         i32,
        is_button_down: bool,
        buttonx:        i32,
        buttony:        i32,
        buttonw:        i32,
        buttonh:        i32,
        box_:           &mut ComboBox)  {
        
        todo!();
        /*
            g.fillAll (box.findColour (ComboBox::backgroundColourId));

        g.setColour (box.findColour ((isButtonDown) ? ComboBox::buttonColourId
                                                    : ComboBox::backgroundColourId));
        g.fillRect (buttonX, buttonY, buttonW, buttonH);

        g.setColour (box.findColour (ComboBox::outlineColourId));
        g.drawRect (0, 0, width, height);

        const float arrowX = 0.2f;
        const float arrowH = 0.3f;

        const auto x = (float) buttonX;
        const auto y = (float) buttonY;
        const auto w = (float) buttonW;
        const auto h = (float) buttonH;

        if (box.isEnabled())
        {
            Path p;
            p.addTriangle (x + w * 0.5f,            y + h * (0.45f - arrowH),
                           x + w * (1.0f - arrowX), y + h * 0.45f,
                           x + w * arrowX,          y + h * 0.45f);

            p.addTriangle (x + w * 0.5f,            y + h * (0.55f + arrowH),
                           x + w * (1.0f - arrowX), y + h * 0.55f,
                           x + w * arrowX,          y + h * 0.55f);

            g.setColour (box.findColour ((isButtonDown) ? ComboBox::backgroundColourId
                                                        : ComboBox::buttonColourId));
            g.fillPath (p);
        }
        */
    }
    
    pub fn get_combo_box_font(&mut self, box_: &mut ComboBox) -> Font {
        
        todo!();
        /*
            Font f (jmin (15.0f, (float) box.getHeight() * 0.85f));
        f.setHorizontalScale (0.9f);
        return f;
        */
    }
    
    pub fn draw_linear_slider(
        &mut self, 
        g:              &mut Graphics,
        x:              i32,
        y:              i32,
        w:              i32,
        h:              i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        style:          SliderStyle,
        slider:         &mut Slider

    ) {
        
        todo!();
        /*
            g.fillAll (slider.findColour (Slider::backgroundColourId));

        if (style == Slider::LinearBar)
        {
            g.setColour (slider.findColour (Slider::thumbColourId));
            g.fillRect (x, y, (int) sliderPos - x, h);

            g.setColour (slider.findColour (Slider::textBoxTextColourId).withMultipliedAlpha (0.5f));
            g.drawRect (x, y, (int) sliderPos - x, h);
        }
        else
        {
            g.setColour (slider.findColour (Slider::trackColourId)
                               .withMultipliedAlpha (slider.isEnabled() ? 1.0f : 0.3f));

            if (slider.isHorizontal())
            {
                g.fillRect (x, y + roundToInt ((float) h * 0.6f),
                            w, roundToInt ((float) h * 0.2f));
            }
            else
            {
                g.fillRect (x + roundToInt ((float) w * 0.5f - jmin (3.0f, (float) w * 0.1f)), y,
                            jmin (4, roundToInt ((float) w * 0.2f)), h);
            }

            float alpha = 0.35f;

            if (slider.isEnabled())
                alpha = slider.isMouseOverOrDragging() ? 1.0f : 0.7f;

            const Colour fill (slider.findColour (Slider::thumbColourId).withAlpha (alpha));
            const Colour outline (Colours::black.withAlpha (slider.isEnabled() ? 0.7f : 0.35f));

            if (style == Slider::TwoValueVertical || style == Slider::ThreeValueVertical)
            {
                drawTriangle (g,
                              (float) x + (float) w * 0.5f + jmin (4.0f, (float) w * 0.3f), minSliderPos,
                              (float) x + (float) w * 0.5f - jmin (8.0f, (float) w * 0.4f), minSliderPos - 7.0f,
                              (float) x + (float) w * 0.5f - jmin (8.0f, (float) w * 0.4f), minSliderPos,
                              fill, outline);

                drawTriangle (g,
                              (float) x + (float) w * 0.5f + jmin (4.0f, (float) w * 0.3f), maxSliderPos,
                              (float) x + (float) w * 0.5f - jmin (8.0f, (float) w * 0.4f), maxSliderPos,
                              (float) x + (float) w * 0.5f - jmin (8.0f, (float) w * 0.4f), maxSliderPos + 7.0f,
                              fill, outline);
            }
            else if (style == Slider::TwoValueHorizontal || style == Slider::ThreeValueHorizontal)
            {
                drawTriangle (g,
                              minSliderPos, (float) y + (float) h * 0.6f - jmin (4.0f, (float) h * 0.3f),
                              minSliderPos - 7.0f, (float) y + (float) h * 0.9f,
                              minSliderPos, (float) y + (float) h * 0.9f,
                              fill, outline);

                drawTriangle (g,
                              maxSliderPos, (float) y + (float) h * 0.6f - jmin (4.0f, (float) h * 0.3f),
                              maxSliderPos, (float) y + (float) h * 0.9f,
                              maxSliderPos + 7.0f, (float) y + (float) h * 0.9f,
                              fill, outline);
            }

            if (style == Slider::LinearHorizontal || style == Slider::ThreeValueHorizontal)
            {
                drawTriangle (g,
                              sliderPos, (float) y + (float) h * 0.9f,
                              sliderPos - 7.0f, (float) y + (float) h * 0.2f,
                              sliderPos + 7.0f, (float) y + (float) h * 0.2f,
                              fill, outline);
            }
            else if (style == Slider::LinearVertical || style == Slider::ThreeValueVertical)
            {
                drawTriangle (g,
                              (float) x + (float) w * 0.5f - jmin (4.0f, (float) w * 0.3f), sliderPos,
                              (float) x + (float) w * 0.5f + jmin (8.0f, (float) w * 0.4f), sliderPos - 7.0f,
                              (float) x + (float) w * 0.5f + jmin (8.0f, (float) w * 0.4f), sliderPos + 7.0f,
                              fill, outline);
            }
        }
        */
    }
    
    pub fn create_slider_button(&mut self, 
        _0:           &mut Slider,
        is_increment: bool) -> *mut Button {
        
        todo!();
        /*
            if (isIncrement)
            return new ArrowButton ("u", 0.75f, Colours::white.withAlpha (0.8f));

        return new ArrowButton ("d", 0.25f, Colours::white.withAlpha (0.8f));
        */
    }
    
    pub fn get_slider_effect(&mut self, _0: &mut Slider) -> *mut dyn ImageEffectFilter {
        
        todo!();
        /*
            return &scrollbarShadow;
        */
    }
    
    pub fn get_slider_thumb_radius(&mut self, _0: &mut Slider) -> i32 {
        
        todo!();
        /*
            return 8;
        */
    }
    
    pub fn draw_corner_resizer(&mut self, 
        g:                 &mut Graphics,
        w:                 i32,
        h:                 i32,
        is_mouse_over:     bool,
        is_mouse_dragging: bool)  {
        
        todo!();
        /*
            g.setColour ((isMouseOver || isMouseDragging) ? Colours::lightgrey
                                                      : Colours::darkgrey);

        const float lineThickness = (float) jmin (w, h) * 0.1f;

        for (float i = 0.0f; i < 1.0f; i += 0.3f)
        {
            g.drawLine ((float) w * i,
                        (float) h + 1.0f,
                        (float) w + 1.0f,
                        (float) h * i,
                        lineThickness);
        }
        */
    }
    
    pub fn create_document_window_button(&mut self, button_type: i32) -> *mut Button {
        
        todo!();
        /*
            Path shape;

        if (buttonType == DocumentWindow::closeButton)
        {
            shape.addLineSegment (Line<float> (0.0f, 0.0f, 1.0f, 1.0f), 0.35f);
            shape.addLineSegment (Line<float> (1.0f, 0.0f, 0.0f, 1.0f), 0.35f);

            ShapeButton* const b = new ShapeButton ("close",
                                                    Colour (0x7fff3333),
                                                    Colour (0xd7ff3333),
                                                    Colour (0xf7ff3333));

            b->setShape (shape, true, true, true);
            return b;
        }
        else if (buttonType == DocumentWindow::minimiseButton)
        {
            shape.addLineSegment (Line<float> (0.0f, 0.5f, 1.0f, 0.5f), 0.25f);

            DrawableButton* b = new DrawableButton ("minimise", DrawableButton::ImageFitted);
            DrawablePath dp;
            dp.setPath (shape);
            dp.setFill (Colours::black.withAlpha (0.3f));
            b->setImages (&dp);
            return b;
        }
        else if (buttonType == DocumentWindow::maximiseButton)
        {
            shape.addLineSegment (Line<float> (0.5f, 0.0f, 0.5f, 1.0f), 0.25f);
            shape.addLineSegment (Line<float> (0.0f, 0.5f, 1.0f, 0.5f), 0.25f);

            DrawableButton* b = new DrawableButton ("maximise", DrawableButton::ImageFitted);
            DrawablePath dp;
            dp.setPath (shape);
            dp.setFill (Colours::black.withAlpha (0.3f));
            b->setImages (&dp);
            return b;
        }

        jassertfalse;
        return nullptr;
        */
    }
    
    pub fn position_document_window_buttons(&mut self, 
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
            titleBarY += titleBarH / 8;
        titleBarH -= titleBarH / 4;

        const int buttonW = titleBarH;

        int x = positionTitleBarButtonsOnLeft ? titleBarX + 4
                                              : titleBarX + titleBarW - buttonW - 4;

        if (closeButton != nullptr)
        {
            closeButton->setBounds (x, titleBarY, buttonW, titleBarH);
            x += positionTitleBarButtonsOnLeft ? buttonW + buttonW / 5
                                               : -(buttonW + buttonW / 5);
        }

        if (positionTitleBarButtonsOnLeft)
            std::swap (minimiseButton, maximiseButton);

        if (maximiseButton != nullptr)
        {
            maximiseButton->setBounds (x, titleBarY - 2, buttonW, titleBarH);
            x += positionTitleBarButtonsOnLeft ? buttonW : -buttonW;
        }

        if (minimiseButton != nullptr)
            minimiseButton->setBounds (x, titleBarY - 2, buttonW, titleBarH);
        */
    }
}

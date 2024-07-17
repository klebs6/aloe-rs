crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V4.h]

/**
  | A struct containing the set of colours
  | to apply to the GUI
  |
  */
pub struct LookAndFeelV4ColourScheme {
    palette: [Colour; LOOK_AND_FEEL_V4_COLOUR_SCHEME_UI_COLOUR_NUM_COLOURS],
}


/**
  | The standard set of colours to use.
  |
  */
pub enum LookAndFeelV4ColourSchemeUIColour
{
    windowBackground = 0,
    widgetBackground,
    menuBackground,
    outline,
    defaultText,
    defaultFill,
    highlightedText,
    highlightedFill,
    menuText,
}

pub const LOOK_AND_FEEL_V4_COLOUR_SCHEME_UI_COLOUR_NUM_COLOURS: usize = 9;

impl PartialEq<LookAndFeelV4ColourScheme> for LookAndFeelV4ColourScheme {
    
    /**
      | Returns true if two ColourPalette objects
      | contain the same colours.
      |
      */
    #[inline] fn eq(&self, other: &LookAndFeelV4ColourScheme) -> bool {
        todo!();
        /*
            for (auto i = 0; i < numColours; ++i)
                if (palette[i] != other.palette[i])
                    return false;

            return true;
        */
    }
}

impl Eq for LookAndFeelV4ColourScheme {}

impl LookAndFeelV4ColourScheme {
    
    pub fn new<ItemColours>(colours_to_use: ItemColours) -> Self {
    
        todo!();
        /*


            static_assert (sizeof... (coloursToUse) == numColours, "Must supply one colour for each LookAndFeelV4ColourSchemeUIColour item");
                const Colour c[] = { Colour (coloursToUse)... };

                for (int i = 0; i < numColours; ++i)
                    palette[i] = c[i];
        */
    }

    /**
      | Returns a colour from the scheme
      |
      */
    pub fn get_ui_colour(&self, index: LookAndFeelV4ColourSchemeUIColour) -> Colour {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, numColours))
                return palette[index];

            jassertfalse;
            return {};
        */
    }
    
    /**
      | Sets a scheme colour.
      |
      */
    pub fn set_ui_colour(&mut self, 
        index:      LookAndFeelV4ColourSchemeUIColour,
        new_colour: Colour)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, numColours))
                palette[index] = newColour;
            else
                jassertfalse;
        */
    }
}

/**
  | The latest Aloe look-and-feel style,
  | as introduced in 2017. @see LookAndFeel,
  | LookAndFeel_V1, LookAndFeel_V2,
  | LookAndFeel_V3
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LookAndFeel_V4<'a> {
    base:                  LookAndFeel_V3<'a>,
    current_colour_scheme: LookAndFeelV4ColourScheme,
}

impl<'a> Default for LookAndFeel_V4<'a> {
    
    /**
      | Creates a LookAndFeel_V4 object with
      | a default colour scheme.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/lookandfeel/aloe_LookAndFeel_V4.cpp]
impl<'a> LookAndFeel_V4<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        : current_colour_scheme(getDarkColourScheme()),

            initialiseColours();
        */
    }
}

impl<'a> LookAndFeel_V4<'a> {

    pub fn get_current_colour_scheme(&mut self) -> &mut LookAndFeelV4ColourScheme {
        
        todo!();
        /*
            return currentColourScheme;
        */
    }
    
    pub fn is_progress_bar_opaque(&mut self, _0: &mut ProgressBar) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    /**
      | Creates a LookAndFeel_V4 object with
      | a given colour scheme.
      |
      */
    pub fn new(scheme: LookAndFeelV4ColourScheme) -> Self {
    
        todo!();
        /*
        : current_colour_scheme(scheme),

            initialiseColours();
        */
    }
    
    pub fn set_colour_scheme(&mut self, new_colour_scheme: LookAndFeelV4ColourScheme)  {
        
        todo!();
        /*
            currentColourScheme = newColourScheme;
        initialiseColours();
        */
    }
    
    pub fn get_dark_colour_scheme(&mut self) -> LookAndFeelV4ColourScheme {
        
        todo!();
        /*
            return { 0xff323e44, 0xff263238, 0xff323e44,
                 0xff8e989b, 0xffffffff, 0xff42a2c8,
                 0xffffffff, 0xff181f22, 0xffffffff };
        */
    }
    
    pub fn get_midnight_colour_scheme(&mut self) -> LookAndFeelV4ColourScheme {
        
        todo!();
        /*
            return { 0xff2f2f3a, 0xff191926, 0xffd0d0d0,
                 0xff66667c, 0xc8ffffff, 0xffd8d8d8,
                 0xffffffff, 0xff606073, 0xff000000 };
        */
    }
    
    pub fn get_grey_colour_scheme(&mut self) -> LookAndFeelV4ColourScheme {
        
        todo!();
        /*
            return { 0xff505050, 0xff424242, 0xff606060,
                 0xffa6a6a6, 0xffffffff, 0xff21ba90,
                 0xff000000, 0xffffffff, 0xffffffff };
        */
    }
    
    pub fn get_light_colour_scheme(&mut self) -> LookAndFeelV4ColourScheme {
        
        todo!();
        /*
            return { 0xffefefef, 0xffffffff, 0xffffffff,
                 0xffdddddd, 0xff000000, 0xffa9a9a9,
                 0xffffffff, 0xff42a2c8, 0xff000000 };
        */
    }
    
    pub fn create_document_window_button(&mut self, button_type: i32) -> *mut Button {
        
        todo!();
        /*
            Path shape;
        auto crossThickness = 0.15f;

        if (buttonType == DocumentWindow::closeButton)
        {
            shape.addLineSegment ({ 0.0f, 0.0f, 1.0f, 1.0f }, crossThickness);
            shape.addLineSegment ({ 1.0f, 0.0f, 0.0f, 1.0f }, crossThickness);

            return new LookAndFeel_V4_DocumentWindowButton ("close", Colour (0xff9A131D), shape, shape);
        }

        if (buttonType == DocumentWindow::minimiseButton)
        {
            shape.addLineSegment ({ 0.0f, 0.5f, 1.0f, 0.5f }, crossThickness);

            return new LookAndFeel_V4_DocumentWindowButton ("minimise", Colour (0xffaa8811), shape, shape);
        }

        if (buttonType == DocumentWindow::maximiseButton)
        {
            shape.addLineSegment ({ 0.5f, 0.0f, 0.5f, 1.0f }, crossThickness);
            shape.addLineSegment ({ 0.0f, 0.5f, 1.0f, 0.5f }, crossThickness);

            Path fullscreenShape;
            fullscreenShape.startNewSubPath (45.0f, 100.0f);
            fullscreenShape.lineTo (0.0f, 100.0f);
            fullscreenShape.lineTo (0.0f, 0.0f);
            fullscreenShape.lineTo (100.0f, 0.0f);
            fullscreenShape.lineTo (100.0f, 45.0f);
            fullscreenShape.addRectangle (45.0f, 45.0f, 100.0f, 100.0f);
            PathStrokeType (30.0f).createStrokedPath (fullscreenShape, fullscreenShape);

            return new LookAndFeel_V4_DocumentWindowButton ("maximise", Colour (0xff0A830A), shape, fullscreenShape);
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
            auto buttonW = static_cast<int> (titleBarH * 1.2);

        auto x = positionTitleBarButtonsOnLeft ? titleBarX
                                               : titleBarX + titleBarW - buttonW;

        if (closeButton != nullptr)
        {
            closeButton->setBounds (x, titleBarY, buttonW, titleBarH);
            x += positionTitleBarButtonsOnLeft ? buttonW : -buttonW;
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
    
    pub fn draw_document_window_title_bar(&mut self, 
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

        auto isActive = window.isActiveWindow();

        g.setColour (getCurrentColourScheme().getUIColour (LookAndFeelV4ColourScheme::widgetBackground));
        g.fillAll();

        Font font ((float) h * 0.65f, Font::plain);
        g.setFont (font);

        auto textW = font.getStringWidth (window.getName());
        auto iconW = 0;
        auto iconH = 0;

        if (icon != nullptr)
        {
            iconH = static_cast<int> (font.getHeight());
            iconW = icon->getWidth() * iconH / icon->getHeight() + 4;
        }

        textW = jmin (titleSpaceW, textW + iconW);
        auto textX = drawTitleTextOnLeft ? titleSpaceX
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
            g.setColour (getCurrentColourScheme().getUIColour (LookAndFeelV4ColourScheme::defaultText));

        g.drawText (window.getName(), textX, 0, textW, h, Justification::centredLeft, true);
        */
    }
    
    pub fn get_text_button_font(&mut self, 
        _0:            &mut TextButton,
        button_height: i32) -> Font {
        
        todo!();
        /*
            return { jmin (16.0f, (float) buttonHeight * 0.6f) };
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
            auto cornerSize = 6.0f;
        auto bounds = button.getLocalBounds().toFloat().reduced (0.5f, 0.5f);

        auto baseColour = backgroundColour.withMultipliedSaturation (button.hasKeyboardFocus (true) ? 1.3f : 0.9f)
                                          .withMultipliedAlpha (button.isEnabled() ? 1.0f : 0.5f);

        if (shouldDrawButtonAsDown || shouldDrawButtonAsHighlighted)
            baseColour = baseColour.contrasting (shouldDrawButtonAsDown ? 0.2f : 0.05f);

        g.setColour (baseColour);

        auto flatOnLeft   = button.isConnectedOnLeft();
        auto flatOnRight  = button.isConnectedOnRight();
        auto flatOnTop    = button.isConnectedOnTop();
        auto flatOnBottom = button.isConnectedOnBottom();

        if (flatOnLeft || flatOnRight || flatOnTop || flatOnBottom)
        {
            Path path;
            path.addRoundedRectangle (bounds.getX(), bounds.getY(),
                                      bounds.getWidth(), bounds.getHeight(),
                                      cornerSize, cornerSize,
                                      ! (flatOnLeft  || flatOnTop),
                                      ! (flatOnRight || flatOnTop),
                                      ! (flatOnLeft  || flatOnBottom),
                                      ! (flatOnRight || flatOnBottom));

            g.fillPath (path);

            g.setColour (button.findColour (ComboBox::outlineColourId));
            g.strokePath (path, PathStrokeType (1.0f));
        }
        else
        {
            g.fillRoundedRectangle (bounds, cornerSize);

            g.setColour (button.findColour (ComboBox::outlineColourId));
            g.drawRoundedRectangle (bounds, cornerSize, 1.0f);
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
            auto fontSize = jmin (15.0f, (float) button.getHeight() * 0.75f);
        auto tickWidth = fontSize * 1.1f;

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
                          button.getLocalBounds().withTrimmedLeft (roundToInt (tickWidth) + 10)
                                                 .withTrimmedRight (2),
                          Justification::centredLeft, 10);
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
            ignoreUnused (isEnabled, shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown);

        Rectangle<float> tickBounds (x, y, w, h);

        g.setColour (component.findColour (ToggleButton::tickDisabledColourId));
        g.drawRoundedRectangle (tickBounds, 4.0f, 1.0f);

        if (ticked)
        {
            g.setColour (component.findColour (ToggleButton::tickColourId));
            auto tick = getTickShape (0.75f);
            g.fillPath (tick, tick.getTransformToScaleToFit (tickBounds.reduced (4, 5).toFloat(), false));
        }
        */
    }
    
    pub fn change_toggle_button_width_to_fit_text(&mut self, button: &mut ToggleButton)  {
        
        todo!();
        /*
            auto fontSize = jmin (15.0f, (float) button.getHeight() * 0.75f);
        auto tickWidth = fontSize * 1.1f;

        Font font (fontSize);

        button.setSize (font.getStringWidth (button.getButtonText()) + roundToInt (tickWidth) + 14, button.getHeight());
        */
    }
    
    pub fn create_alert_window(&mut self, 
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
            auto boundsOffset = 50;

        auto* aw = LookAndFeel_V2::createAlertWindow (title, message, button1, button2, button3,
                                                      iconType, numButtons, associatedComponent);

        auto bounds = aw->getBounds();
        bounds = bounds.withSizeKeepingCentre (bounds.getWidth() + boundsOffset, bounds.getHeight() + boundsOffset);
        aw->setBounds (bounds);

        for (auto* child : aw->getChildren())
            if (auto* button = dynamic_cast<TextButton*> (child))
                button->setBounds (button->getBounds() + Point<int> (25, 40));

        return aw;
        */
    }
    
    pub fn draw_alert_box(&mut self, 
        g:           &mut Graphics,
        alert:       &mut AlertWindow,
        text_area:   &Rectangle<i32>,
        text_layout: &mut TextLayout)  {
        
        todo!();
        /*
            auto cornerSize = 4.0f;

        g.setColour (alert.findColour (AlertWindow::outlineColourId));
        g.drawRoundedRectangle (alert.getLocalBounds().toFloat(), cornerSize, 2.0f);

        auto bounds = alert.getLocalBounds().reduced (1);
        g.reduceClipRegion (bounds);

        g.setColour (alert.findColour (AlertWindow::backgroundColourId));
        g.fillRoundedRectangle (bounds.toFloat(), cornerSize);

        auto iconSpaceUsed = 0;

        auto iconWidth = 80;
        auto iconSize = jmin (iconWidth + 50, bounds.getHeight() + 20);

        if (alert.containsAnyExtraComponents() || alert.getNumButtons() > 2)
            iconSize = jmin (iconSize, textArea.getHeight() + 50);

        Rectangle<int> iconRect (iconSize / -10, iconSize / -10,
                                 iconSize, iconSize);

        if (alert.getAlertType() != MessageBoxIconType::NoIcon)
        {
            Path icon;
            char character;
            uint32 colour;

            if (alert.getAlertType() == MessageBoxIconType::WarningIcon)
            {
                character = '!';

                icon.addTriangle ((float) iconRect.getX() + (float) iconRect.getWidth() * 0.5f, (float) iconRect.getY(),
                                  static_cast<float> (iconRect.getRight()), static_cast<float> (iconRect.getBottom()),
                                  static_cast<float> (iconRect.getX()), static_cast<float> (iconRect.getBottom()));

                icon = icon.createPathWithRoundedCorners (5.0f);
                colour = 0x66ff2a00;
            }
            else
            {
                colour = Colour (0xff00b0b9).withAlpha (0.4f).getARGB();
                character = alert.getAlertType() == MessageBoxIconType::InfoIcon ? 'i' : '?';

                icon.addEllipse (iconRect.toFloat());
            }

            GlyphArrangement ga;
            ga.addFittedText ({ (float) iconRect.getHeight() * 0.9f, Font::bold },
                              String::charToString ((aloe_wchar) (uint8) character),
                              static_cast<float> (iconRect.getX()), static_cast<float> (iconRect.getY()),
                              static_cast<float> (iconRect.getWidth()), static_cast<float> (iconRect.getHeight()),
                              Justification::centred, false);
            ga.createPath (icon);

            icon.setUsingNonZeroWinding (false);
            g.setColour (Colour (colour));
            g.fillPath (icon);

            iconSpaceUsed = iconWidth;
        }

        g.setColour (alert.findColour (AlertWindow::textColourId));

        Rectangle<int> alertBounds (bounds.getX() + iconSpaceUsed, 30,
                                    bounds.getWidth(), bounds.getHeight() - getAlertWindowButtonHeight() - 20);

        textLayout.draw (g, alertBounds.toFloat());
        */
    }
    
    pub fn get_alert_window_button_height(&mut self) -> i32 {
        
        todo!();
        /*
            return 40;
        */
    }
    
    pub fn get_alert_window_title_font(&mut self) -> Font {
        
        todo!();
        /*
            return { 18.0f, Font::bold };
        */
    }
    
    pub fn get_alert_window_message_font(&mut self) -> Font {
        
        todo!();
        /*
            return { 16.0f };
        */
    }
    
    pub fn get_alert_window_font(&mut self) -> Font {
        
        todo!();
        /*
            return { 14.0f };
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
            if (width == height)
            drawCircularProgressBar (g, progressBar, textToShow);
        else
            drawLinearProgressBar (g, progressBar, width, height, progress, textToShow);
        */
    }
    
    pub fn draw_linear_progress_bar(&mut self, 
        g:            &mut Graphics,
        progress_bar: &mut ProgressBar,
        width:        i32,
        height:       i32,
        progress:     f64,
        text_to_show: &String)  {
        
        todo!();
        /*
            auto background = progressBar.findColour (ProgressBar::backgroundColourId);
        auto foreground = progressBar.findColour (ProgressBar::foregroundColourId);

        auto barBounds = progressBar.getLocalBounds().toFloat();

        g.setColour (background);
        g.fillRoundedRectangle (barBounds, (float) progressBar.getHeight() * 0.5f);

        if (progress >= 0.0f && progress <= 1.0f)
        {
            Path p;
            p.addRoundedRectangle (barBounds, (float) progressBar.getHeight() * 0.5f);
            g.reduceClipRegion (p);

            barBounds.setWidth (barBounds.getWidth() * (float) progress);
            g.setColour (foreground);
            g.fillRoundedRectangle (barBounds, (float) progressBar.getHeight() * 0.5f);
        }
        else
        {
            // spinning bar..
            g.setColour (background);

            auto stripeWidth = height * 2;
            auto position = static_cast<int> (Time::getMillisecondCounter() / 15) % stripeWidth;

            Path p;

            for (auto x = static_cast<float> (-position); x < (float) (width + stripeWidth); x += (float) stripeWidth)
                p.addQuadrilateral (x, 0.0f,
                                    x + (float) stripeWidth * 0.5f, 0.0f,
                                    x, static_cast<float> (height),
                                    x - (float) stripeWidth * 0.5f, static_cast<float> (height));

            Image im (Image::ARGB, width, height, true);

            {
                Graphics g2 (im);
                g2.setColour (foreground);
                g2.fillRoundedRectangle (barBounds, (float) progressBar.getHeight() * 0.5f);
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
    
    pub fn draw_circular_progress_bar(&mut self, 
        g:             &mut Graphics,
        progress_bar:  &mut ProgressBar,
        progress_text: &String)  {
        
        todo!();
        /*
            auto background = progressBar.findColour (ProgressBar::backgroundColourId);
        auto foreground = progressBar.findColour (ProgressBar::foregroundColourId);

        auto barBounds = progressBar.getLocalBounds().reduced (2, 2).toFloat();

        auto rotationInDegrees  = static_cast<float> ((Time::getMillisecondCounter() / 10) % 360);
        auto normalisedRotation = rotationInDegrees / 360.0f;

        auto rotationOffset = 22.5f;
        auto maxRotation    = 315.0f;

        auto startInDegrees = rotationInDegrees;
        auto endInDegrees   = startInDegrees + rotationOffset;

        if (normalisedRotation >= 0.25f && normalisedRotation < 0.5f)
        {
            auto rescaledRotation = (normalisedRotation * 4.0f) - 1.0f;
            endInDegrees = startInDegrees + rotationOffset + (maxRotation * rescaledRotation);
        }
        else if (normalisedRotation >= 0.5f && normalisedRotation <= 1.0f)
        {
            endInDegrees = startInDegrees + rotationOffset + maxRotation;
            auto rescaledRotation = 1.0f - ((normalisedRotation * 2.0f) - 1.0f);
            startInDegrees = endInDegrees - rotationOffset - (maxRotation * rescaledRotation);
        }

        g.setColour (background);
        Path arcPath2;
        arcPath2.addCentredArc (barBounds.getCentreX(),
                                barBounds.getCentreY(),
                                barBounds.getWidth() * 0.5f,
                                barBounds.getHeight() * 0.5f, 0.0f,
                                0.0f,
                                MathConstants<float>::twoPi,
                                true);
        g.strokePath (arcPath2, PathStrokeType (4.0f));

        g.setColour (foreground);
        Path arcPath;
        arcPath.addCentredArc (barBounds.getCentreX(),
                               barBounds.getCentreY(),
                               barBounds.getWidth() * 0.5f,
                               barBounds.getHeight() * 0.5f,
                               0.0f,
                               degreesToRadians (startInDegrees),
                               degreesToRadians (endInDegrees),
                               true);

        arcPath.applyTransform (AffineTransform::rotation (normalisedRotation * MathConstants<float>::pi * 2.25f, barBounds.getCentreX(), barBounds.getCentreY()));
        g.strokePath (arcPath, PathStrokeType (4.0f));

        if (progressText.isNotEmpty())
        {
            g.setColour (progressBar.findColour (TextButton::textColourOffId));
            g.setFont ({ 12.0f, Font::italic });
            g.drawText (progressText, barBounds, Justification::centred, false);
        }
        */
    }
    
    pub fn get_default_scrollbar_width(&mut self) -> i32 {
        
        todo!();
        /*
            return 8;
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
            ignoreUnused (isMouseDown);

        Rectangle<int> thumbBounds;

        if (isScrollbarVertical)
            thumbBounds = { x, thumbStartPosition, width, thumbSize };
        else
            thumbBounds = { thumbStartPosition, y, thumbSize, height };

        auto c = scrollbar.findColour (ScrollBar::ColourIds::thumbColourId);
        g.setColour (isMouseOver ? c.brighter (0.25f) : c);
        g.fillRoundedRectangle (thumbBounds.reduced (1).toFloat(), 4.0f);
        */
    }
    
    pub fn get_tick_shape(&mut self, height: f32) -> Path {
        
        todo!();
        /*
            static const unsigned char pathData[] = { 110,109,32,210,202,64,126,183,148,64,108,39,244,247,64,245,76,124,64,108,178,131,27,65,246,76,252,64,108,175,242,4,65,246,76,252,
            64,108,236,5,68,65,0,0,160,180,108,240,150,90,65,21,136,52,63,108,48,59,16,65,0,0,32,65,108,32,210,202,64,126,183,148,64, 99,101,0,0 };

        Path path;
        path.loadPathFromData (pathData, sizeof (pathData));
        path.scaleToFit (0, 0, height * 2.0f, height, true);

        return path;
        */
    }
    
    pub fn get_cross_shape(&mut self, height: f32) -> Path {
        
        todo!();
        /*
            static const unsigned char pathData[] = { 110,109,51,51,255,66,0,0,0,0,108,205,204,13,67,51,51,99,65,108,0,0,170,66,205,204,141,66,108,51,179,13,67,52,51,255,66,108,0,0,255,
            66,205,204,13,67,108,205,204,141,66,0,0,170,66,108,52,51,99,65,51,179,13,67,108,0,0,0,0,51,51,255,66,108,205,204,98,66, 204,204,141,66,108,0,0,0,0,51,51,99,65,108,51,51,
            99,65,0,0,0,0,108,205,204,141,66,205,204,98,66,108,51,51,255,66,0,0,0,0,99,101,0,0 };

        Path path;
        path.loadPathFromData (pathData, sizeof (pathData));
        path.scaleToFit (0, 0, height * 2.0f, height, true);

        return path;
        */
    }
    
    pub fn fill_text_editor_background(&mut self, 
        g:           &mut Graphics,
        width:       i32,
        height:      i32,
        text_editor: &mut TextEditor)  {
        
        todo!();
        /*
            if (dynamic_cast<AlertWindow*> (textEditor.getParentComponent()) != nullptr)
        {
            g.setColour (textEditor.findColour (TextEditor::backgroundColourId));
            g.fillRect (0, 0, width, height);

            g.setColour (textEditor.findColour (TextEditor::outlineColourId));
            g.drawHorizontalLine (height - 1, 0.0f, static_cast<float> (width));
        }
        else
        {
            LookAndFeel_V2::fillTextEditorBackground (g, width, height, textEditor);
        }
        */
    }
    
    pub fn draw_text_editor_outline(&mut self, 
        g:           &mut Graphics,
        width:       i32,
        height:      i32,
        text_editor: &mut TextEditor)  {
        
        todo!();
        /*
            if (dynamic_cast<AlertWindow*> (textEditor.getParentComponent()) == nullptr)
        {
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
        }
        */
    }
    
    pub fn create_file_browser_go_up_button(&mut self) -> *mut Button {
        
        todo!();
        /*
            auto* goUpButton = new DrawableButton ("up", DrawableButton::ImageOnButtonBackground);

        Path arrowPath;
        arrowPath.addArrow ({ 50.0f, 100.0f, 50.0f, 0.0f }, 40.0f, 100.0f, 50.0f);

        DrawablePath arrowImage;
        arrowImage.setFill (goUpButton->findColour (TextButton::textColourOffId));
        arrowImage.setPath (arrowPath);

        goUpButton->setImages (&arrowImage);

        return goUpButton;
        */
    }
    
    pub fn layout_file_browser_component(&mut self, 
        browser_comp:        &mut FileBrowserComponent,
        file_list_component: *mut DirectoryContentsDisplayComponent,
        preview_comp:        *mut FilePreviewComponent,
        current_path_box:    *mut ComboBox,
        filename_box:        *mut TextEditor,
        go_up_button:        *mut Button)  {
        
        todo!();
        /*
            auto sectionHeight = 22;
        auto buttonWidth = 50;

        auto b = browserComp.getLocalBounds().reduced (20, 5);

        auto topSlice    = b.removeFromTop (sectionHeight);
        auto bottomSlice = b.removeFromBottom (sectionHeight);

        currentPathBox->setBounds (topSlice.removeFromLeft (topSlice.getWidth() - buttonWidth));

        topSlice.removeFromLeft (6);
        goUpButton->setBounds (topSlice);

        bottomSlice.removeFromLeft (20);
        filenameBox->setBounds (bottomSlice);

        if (previewComp != nullptr)
            previewComp->setBounds (b.removeFromRight (b.getWidth() / 3));

        if (auto* listAsComp = dynamic_cast<Component*> (fileListComponent))
            listAsComp->setBounds (b.reduced (0, 10));
        */
    }
    
    pub fn draw_file_browser_row(&mut self, 
        g:                     &mut Graphics,
        width:                 i32,
        height:                i32,
        file:                  &File,
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
            LookAndFeel_V2::drawFileBrowserRow (g, width, height, file, filename, icon,
                                            fileSizeDescription, fileTimeDescription,
                                            isDirectory, isItemSelected, itemIndex, dcc);
        */
    }
    
    pub fn draw_popup_menu_item(&mut self, 
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
        text_colour_to_use: *const Colour)  {
        
        todo!();
        /*
            if (isSeparator)
        {
            auto r  = area.reduced (5, 0);
            r.removeFromTop (roundToInt (((float) r.getHeight() * 0.5f) - 0.5f));

            g.setColour (findColour (PopupMenu::textColourId).withAlpha (0.3f));
            g.fillRect (r.removeFromTop (1));
        }
        else
        {
            auto textColour = (textColourToUse == nullptr ? findColour (PopupMenu::textColourId)
                                                          : *textColourToUse);

            auto r  = area.reduced (1);

            if (isHighlighted && isActive)
            {
                g.setColour (findColour (PopupMenu::highlightedBackgroundColourId));
                g.fillRect (r);

                g.setColour (findColour (PopupMenu::highlightedTextColourId));
            }
            else
            {
                g.setColour (textColour.withMultipliedAlpha (isActive ? 1.0f : 0.5f));
            }

            r.reduce (jmin (5, area.getWidth() / 20), 0);

            auto font = getPopupMenuFont();

            auto maxFontHeight = (float) r.getHeight() / 1.3f;

            if (font.getHeight() > maxFontHeight)
                font.setHeight (maxFontHeight);

            g.setFont (font);

            auto iconArea = r.removeFromLeft (roundToInt (maxFontHeight)).toFloat();

            if (icon != nullptr)
            {
                icon->drawWithin (g, iconArea, RectanglePlacement::centred | RectanglePlacement::onlyReduceInSize, 1.0f);
                r.removeFromLeft (roundToInt (maxFontHeight * 0.5f));
            }
            else if (isTicked)
            {
                auto tick = getTickShape (1.0f);
                g.fillPath (tick, tick.getTransformToScaleToFit (iconArea.reduced (iconArea.getWidth() / 5, 0).toFloat(), true));
            }

            if (hasSubMenu)
            {
                auto arrowH = 0.6f * getPopupMenuFont().getAscent();

                auto x = static_cast<float> (r.removeFromRight ((int) arrowH).getX());
                auto halfH = static_cast<float> (r.getCentreY());

                Path path;
                path.startNewSubPath (x, halfH - arrowH * 0.5f);
                path.lineTo (x + arrowH * 0.6f, halfH);
                path.lineTo (x, halfH + arrowH * 0.5f);

                g.strokePath (path, PathStrokeType (2.0f));
            }

            r.removeFromRight (3);
            g.drawFittedText (text, r, Justification::centredLeft, 1);

            if (shortcutKeyText.isNotEmpty())
            {
                auto f2 = font;
                f2.setHeight (f2.getHeight() * 0.75f);
                f2.setHorizontalScale (0.95f);
                g.setFont (f2);

                g.drawText (shortcutKeyText, r, Justification::centredRight, true);
            }
        }
        */
    }
    
    pub fn get_ideal_popup_menu_item_size(&mut self, 
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
            idealHeight = standardMenuItemHeight > 0 ? standardMenuItemHeight / 10 : 10;
        }
        else
        {
            auto font = getPopupMenuFont();

            if (standardMenuItemHeight > 0 && font.getHeight() > (float) standardMenuItemHeight / 1.3f)
                font.setHeight ((float) standardMenuItemHeight / 1.3f);

            idealHeight = standardMenuItemHeight > 0 ? standardMenuItemHeight : roundToInt (font.getHeight() * 1.3f);
            idealWidth = font.getStringWidth (text) + idealHeight * 2;
        }
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
            auto colour = menuBar.findColour (TextButton::buttonColourId).withAlpha (0.4f);

        Rectangle<int> r (width, height);

        g.setColour (colour.contrasting (0.15f));
        g.fillRect  (r.removeFromTop (1));
        g.fillRect  (r.removeFromBottom (1));

        g.setGradientFill (ColourGradient::vertical (colour, 0, colour.darker (0.2f), (float) height));
        g.fillRect (r);
        */
    }
    
    pub fn draw_menu_bar_item(&mut self, 
        g:                  &mut Graphics,
        width:              i32,
        height:             i32,
        item_index:         i32,
        item_text:          &String,
        is_mouse_over_item: bool,
        is_menu_open:       bool,
        is_mouse_over_bar:  bool,
        menu_bar:           &mut MenuBarComponent)  {
        
        todo!();
        /*
            if (! menuBar.isEnabled())
        {
            g.setColour (menuBar.findColour (TextButton::textColourOffId)
                                .withMultipliedAlpha (0.5f));
        }
        else if (isMenuOpen || isMouseOverItem)
        {
            g.fillAll   (menuBar.findColour (TextButton::buttonOnColourId));
            g.setColour (menuBar.findColour (TextButton::textColourOnId));
        }
        else
        {
            g.setColour (menuBar.findColour (TextButton::textColourOffId));
        }

        g.setFont (getMenuBarFont (menuBar, itemIndex, itemText));
        g.drawFittedText (itemText, 0, 0, width, height, Justification::centred, 1);
        */
    }
    
    pub fn draw_combo_box(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32,
        _3:     bool,
        _4:     i32,
        _5:     i32,
        _6:     i32,
        _7:     i32,
        box_:   &mut ComboBox)  {
        
        todo!();
        /*
            auto cornerSize = box.findParentComponentOfClass<ChoicePropertyComponent>() != nullptr ? 0.0f : 3.0f;
        Rectangle<int> boxBounds (0, 0, width, height);

        g.setColour (box.findColour (ComboBox::backgroundColourId));
        g.fillRoundedRectangle (boxBounds.toFloat(), cornerSize);

        g.setColour (box.findColour (ComboBox::outlineColourId));
        g.drawRoundedRectangle (boxBounds.toFloat().reduced (0.5f, 0.5f), cornerSize, 1.0f);

        Rectangle<int> arrowZone (width - 30, 0, 20, height);
        Path path;
        path.startNewSubPath ((float) arrowZone.getX() + 3.0f, (float) arrowZone.getCentreY() - 2.0f);
        path.lineTo ((float) arrowZone.getCentreX(), (float) arrowZone.getCentreY() + 3.0f);
        path.lineTo ((float) arrowZone.getRight() - 3.0f, (float) arrowZone.getCentreY() - 2.0f);

        g.setColour (box.findColour (ComboBox::arrowColourId).withAlpha ((box.isEnabled() ? 0.9f : 0.2f)));
        g.strokePath (path, PathStrokeType (2.0f));
        */
    }
    
    pub fn get_combo_box_font(&mut self, box_: &mut ComboBox) -> Font {
        
        todo!();
        /*
            return { jmin (16.0f, (float) box.getHeight() * 0.85f) };
        */
    }
    
    pub fn position_combo_box_text(&mut self, 
        box_:  &mut ComboBox,
        label: &mut Label)  {
        
        todo!();
        /*
            label.setBounds (1, 1,
                         box.getWidth() - 30,
                         box.getHeight() - 2);

        label.setFont (getComboBoxFont (box));
        */
    }
    
    pub fn get_slider_thumb_radius(&mut self, slider: &mut Slider) -> i32 {
        
        todo!();
        /*
            return jmin (12, slider.isHorizontal() ? static_cast<int> ((float) slider.getHeight() * 0.5f)
                                               : static_cast<int> ((float) slider.getWidth()  * 0.5f));
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
            if (slider.isBar())
        {
            g.setColour (slider.findColour (Slider::trackColourId));
            g.fillRect (slider.isHorizontal() ? Rectangle<float> (static_cast<float> (x), (float) y + 0.5f, sliderPos - (float) x, (float) height - 1.0f)
                                              : Rectangle<float> ((float) x + 0.5f, sliderPos, (float) width - 1.0f, (float) y + ((float) height - sliderPos)));
        }
        else
        {
            auto isTwoVal   = (style == Slider::SliderStyle::TwoValueVertical   || style == Slider::SliderStyle::TwoValueHorizontal);
            auto isThreeVal = (style == Slider::SliderStyle::ThreeValueVertical || style == Slider::SliderStyle::ThreeValueHorizontal);

            auto trackWidth = jmin (6.0f, slider.isHorizontal() ? (float) height * 0.25f : (float) width * 0.25f);

            Point<float> startPoint (slider.isHorizontal() ? (float) x : (float) x + (float) width * 0.5f,
                                     slider.isHorizontal() ? (float) y + (float) height * 0.5f : (float) (height + y));

            Point<float> endPoint (slider.isHorizontal() ? (float) (width + x) : startPoint.x,
                                   slider.isHorizontal() ? startPoint.y : (float) y);

            Path backgroundTrack;
            backgroundTrack.startNewSubPath (startPoint);
            backgroundTrack.lineTo (endPoint);
            g.setColour (slider.findColour (Slider::backgroundColourId));
            g.strokePath (backgroundTrack, { trackWidth, PathStrokeType::curved, PathStrokeType::rounded });

            Path valueTrack;
            Point<float> minPoint, maxPoint, thumbPoint;

            if (isTwoVal || isThreeVal)
            {
                minPoint = { slider.isHorizontal() ? minSliderPos : (float) width * 0.5f,
                             slider.isHorizontal() ? (float) height * 0.5f : minSliderPos };

                if (isThreeVal)
                    thumbPoint = { slider.isHorizontal() ? sliderPos : (float) width * 0.5f,
                                   slider.isHorizontal() ? (float) height * 0.5f : sliderPos };

                maxPoint = { slider.isHorizontal() ? maxSliderPos : (float) width * 0.5f,
                             slider.isHorizontal() ? (float) height * 0.5f : maxSliderPos };
            }
            else
            {
                auto kx = slider.isHorizontal() ? sliderPos : ((float) x + (float) width * 0.5f);
                auto ky = slider.isHorizontal() ? ((float) y + (float) height * 0.5f) : sliderPos;

                minPoint = startPoint;
                maxPoint = { kx, ky };
            }

            auto thumbWidth = getSliderThumbRadius (slider);

            valueTrack.startNewSubPath (minPoint);
            valueTrack.lineTo (isThreeVal ? thumbPoint : maxPoint);
            g.setColour (slider.findColour (Slider::trackColourId));
            g.strokePath (valueTrack, { trackWidth, PathStrokeType::curved, PathStrokeType::rounded });

            if (! isTwoVal)
            {
                g.setColour (slider.findColour (Slider::thumbColourId));
                g.fillEllipse (Rectangle<float> (static_cast<float> (thumbWidth), static_cast<float> (thumbWidth)).withCentre (isThreeVal ? thumbPoint : maxPoint));
            }

            if (isTwoVal || isThreeVal)
            {
                auto sr = jmin (trackWidth, (slider.isHorizontal() ? (float) height : (float) width) * 0.4f);
                auto pointerColour = slider.findColour (Slider::thumbColourId);

                if (slider.isHorizontal())
                {
                    drawPointer (g, minSliderPos - sr,
                                 jmax (0.0f, (float) y + (float) height * 0.5f - trackWidth * 2.0f),
                                 trackWidth * 2.0f, pointerColour, 2);

                    drawPointer (g, maxSliderPos - trackWidth,
                                 jmin ((float) (y + height) - trackWidth * 2.0f, (float) y + (float) height * 0.5f),
                                 trackWidth * 2.0f, pointerColour, 4);
                }
                else
                {
                    drawPointer (g, jmax (0.0f, (float) x + (float) width * 0.5f - trackWidth * 2.0f),
                                 minSliderPos - trackWidth,
                                 trackWidth * 2.0f, pointerColour, 1);

                    drawPointer (g, jmin ((float) (x + width) - trackWidth * 2.0f, (float) x + (float) width * 0.5f), maxSliderPos - sr,
                                 trackWidth * 2.0f, pointerColour, 3);
                }
            }
        }
        */
    }
    
    pub fn draw_rotary_slider(&mut self, 
        g:                  &mut Graphics,
        x:                  i32,
        y:                  i32,
        width:              i32,
        height:             i32,
        slider_pos:         f32,
        rotary_start_angle: f32,
        rotary_end_angle:   f32,
        slider:             &mut Slider)  {
        
        todo!();
        /*
            auto outline = slider.findColour (Slider::rotarySliderOutlineColourId);
        auto fill    = slider.findColour (Slider::rotarySliderFillColourId);

        auto bounds = Rectangle<int> (x, y, width, height).toFloat().reduced (10);

        auto radius = jmin (bounds.getWidth(), bounds.getHeight()) / 2.0f;
        auto toAngle = rotaryStartAngle + sliderPos * (rotaryEndAngle - rotaryStartAngle);
        auto lineW = jmin (8.0f, radius * 0.5f);
        auto arcRadius = radius - lineW * 0.5f;

        Path backgroundArc;
        backgroundArc.addCentredArc (bounds.getCentreX(),
                                     bounds.getCentreY(),
                                     arcRadius,
                                     arcRadius,
                                     0.0f,
                                     rotaryStartAngle,
                                     rotaryEndAngle,
                                     true);

        g.setColour (outline);
        g.strokePath (backgroundArc, PathStrokeType (lineW, PathStrokeType::curved, PathStrokeType::rounded));

        if (slider.isEnabled())
        {
            Path valueArc;
            valueArc.addCentredArc (bounds.getCentreX(),
                                    bounds.getCentreY(),
                                    arcRadius,
                                    arcRadius,
                                    0.0f,
                                    rotaryStartAngle,
                                    toAngle,
                                    true);

            g.setColour (fill);
            g.strokePath (valueArc, PathStrokeType (lineW, PathStrokeType::curved, PathStrokeType::rounded));
        }

        auto thumbWidth = lineW * 2.0f;
        Point<float> thumbPoint (bounds.getCentreX() + arcRadius * std::cos (toAngle - MathConstants<float>::halfPi),
                                 bounds.getCentreY() + arcRadius * std::sin (toAngle - MathConstants<float>::halfPi));

        g.setColour (slider.findColour (Slider::thumbColourId));
        g.fillEllipse (Rectangle<float> (thumbWidth, thumbWidth).withCentre (thumbPoint));
        */
    }
    
    pub fn draw_pointer(&mut self, 
        g:         &mut Graphics,
        x:         f32,
        y:         f32,
        diameter:  f32,
        colour:    &Colour,
        direction: i32)  {
        
        todo!();
        /*
            Path p;
        p.startNewSubPath (x + diameter * 0.5f, y);
        p.lineTo (x + diameter, y + diameter * 0.6f);
        p.lineTo (x + diameter, y + diameter);
        p.lineTo (x, y + diameter);
        p.lineTo (x, y + diameter * 0.6f);
        p.closeSubPath();

        p.applyTransform (AffineTransform::rotation ((float) direction * MathConstants<float>::halfPi,
                                                     x + diameter * 0.5f, y + diameter * 0.5f));
        g.setColour (colour);
        g.fillPath (p);
        */
    }
    
    pub fn create_slider_text_box(&mut self, slider: &mut Slider) -> *mut Label {
        
        todo!();
        /*
            auto* l = LookAndFeel_V2::createSliderTextBox (slider);

        if (getCurrentColourScheme() == LookAndFeel_V4::getGreyColourScheme() && (slider.getSliderStyle() == Slider::LinearBar
                                                                                   || slider.getSliderStyle() == Slider::LinearBarVertical))
        {
            l->setColour (Label::textColourId, Colours::black.withAlpha (0.7f));
        }

        return l;
        */
    }
    
    pub fn draw_tooltip(&mut self, 
        g:      &mut Graphics,
        text:   &String,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            Rectangle<int> bounds (width, height);
        auto cornerSize = 5.0f;

        g.setColour (findColour (TooltipWindow::backgroundColourId));
        g.fillRoundedRectangle (bounds.toFloat(), cornerSize);

        g.setColour (findColour (TooltipWindow::outlineColourId));
        g.drawRoundedRectangle (bounds.toFloat().reduced (0.5f, 0.5f), cornerSize, 1.0f);

        LookAndFeelHelpers::layoutTooltipText (text, findColour (TooltipWindow::textColourId))
                           .draw (g, { static_cast<float> (width), static_cast<float> (height) });
        */
    }
    
    pub fn draw_concertina_panel_header(&mut self, 
        g:             &mut Graphics,
        area:          &Rectangle<i32>,
        is_mouse_over: bool,
        is_mouse_down: bool,
        concertina:    &mut ConcertinaPanel,
        panel:         &mut Component)  {
        
        todo!();
        /*
            auto bounds = area.toFloat().reduced (0.5f);
        auto cornerSize = 4.0f;
        auto isTopPanel = (concertina.getPanel (0) == &panel);

        Path p;
        p.addRoundedRectangle (bounds.getX(), bounds.getY(), bounds.getWidth(), bounds.getHeight(),
                               cornerSize, cornerSize, isTopPanel, isTopPanel, false, false);

        g.setGradientFill (ColourGradient::vertical (Colours::white.withAlpha (isMouseOver ? 0.4f : 0.2f), static_cast<float> (area.getY()),
                                                     Colours::darkgrey.withAlpha (0.1f), static_cast<float> (area.getBottom())));
        g.fillPath (p);
        */
    }
    
    pub fn draw_level_meter(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32,
        level:  f32)  {
        
        todo!();
        /*
            auto outerCornerSize = 3.0f;
        auto outerBorderWidth = 2.0f;
        auto totalBlocks = 7;
        auto spacingFraction = 0.03f;

        g.setColour (findColour (ResizableWindow::backgroundColourId));
        g.fillRoundedRectangle (0.0f, 0.0f, static_cast<float> (width), static_cast<float> (height), outerCornerSize);

        auto doubleOuterBorderWidth = 2.0f * outerBorderWidth;
        auto numBlocks = roundToInt ((float) totalBlocks * level);

        auto blockWidth = ((float) width - doubleOuterBorderWidth) / static_cast<float> (totalBlocks);
        auto blockHeight = (float) height - doubleOuterBorderWidth;

        auto blockRectWidth = (1.0f - 2.0f * spacingFraction) * blockWidth;
        auto blockRectSpacing = spacingFraction * blockWidth;

        auto blockCornerSize = 0.1f * blockWidth;

        auto c = findColour (Slider::thumbColourId);

        for (auto i = 0; i < totalBlocks; ++i)
        {
            if (i >= numBlocks)
                g.setColour (c.withAlpha (0.5f));
            else
                g.setColour (i < totalBlocks - 1 ? c : Colours::red);

            g.fillRoundedRectangle (outerBorderWidth + ((float) i * blockWidth) + blockRectSpacing,
                                    outerBorderWidth,
                                    blockRectWidth,
                                    blockHeight,
                                    blockCornerSize);
        }
        */
    }
    
    pub fn paint_toolbar_background(&mut self, 
        g:       &mut Graphics,
        w:       i32,
        h:       i32,
        toolbar: &mut Toolbar)  {
        
        todo!();
        /*
            auto background = toolbar.findColour (Toolbar::backgroundColourId);

        g.setGradientFill ({ background, 0.0f, 0.0f,
                             background.darker (0.2f),
                             toolbar.isVertical() ? (float) w - 1.0f : 0.0f,
                             toolbar.isVertical() ? 0.0f : (float) h - 1.0f,
                             false });
        g.fillAll();
        */
    }
    
    pub fn paint_toolbar_button_label(&mut self, 
        g:         &mut Graphics,
        x:         i32,
        y:         i32,
        width:     i32,
        height:    i32,
        text:      &String,
        component: &mut dyn ToolbarItemComponent)  {
        
        todo!();
        /*
            auto baseTextColour = component.findParentComponentOfClass<PopupMenu::CustomComponent>() != nullptr
                                  ? component.findColour (PopupMenu::textColourId)
                                  : component.findColour (Toolbar::labelTextColourId);

        g.setColour (baseTextColour.withAlpha (component.isEnabled() ? 1.0f : 0.25f));

        auto fontHeight = jmin (14.0f, (float) height * 0.85f);
        g.setFont (fontHeight);

        g.drawFittedText (text,
                          x, y, width, height,
                          Justification::centred,
                          jmax (1, height / (int) fontHeight));
        */
    }
    
    pub fn draw_property_panel_section_header(&mut self, 
        g:       &mut Graphics,
        name:    &String,
        is_open: bool,
        width:   i32,
        height:  i32)  {
        
        todo!();
        /*
            auto buttonSize = (float) height * 0.75f;
        auto buttonIndent = ((float) height - buttonSize) * 0.5f;

        drawTreeviewPlusMinusBox (g, { buttonIndent, buttonIndent, buttonSize, buttonSize },
                                  findColour (ResizableWindow::backgroundColourId), isOpen, false);

        auto textX = static_cast<int> ((buttonIndent * 2.0f + buttonSize + 2.0f));

        g.setColour (findColour (PropertyComponent::labelTextColourId));

        g.setFont ({ (float) height * 0.7f, Font::bold });
        g.drawText (name, textX, 0, width - textX - 4, height, Justification::centredLeft, true);
        */
    }
    
    pub fn draw_property_component_background(&mut self, 
        g:         &mut Graphics,
        width:     i32,
        height:    i32,
        component: &mut PropertyComponent)  {
        
        todo!();
        /*
            g.setColour (component.findColour (PropertyComponent::backgroundColourId));
        g.fillRect  (0, 0, width, height - 1);
        */
    }
    
    pub fn draw_property_component_label(&mut self, 
        g:         &mut Graphics,
        width:     i32,
        height:    i32,
        component: &mut PropertyComponent)  {
        
        todo!();
        /*
            ignoreUnused (width);

        auto indent = getPropertyComponentIndent (component);

        g.setColour (component.findColour (PropertyComponent::labelTextColourId)
                              .withMultipliedAlpha (component.isEnabled() ? 1.0f : 0.6f));

        g.setFont ((float) jmin (height, 24) * 0.65f);

        auto r = getPropertyComponentContentPosition (component);

        g.drawFittedText (component.getName(),
                          indent, r.getY(), r.getX() - 5, r.getHeight(),
                          Justification::centredLeft, 2);
        */
    }
    
    pub fn get_property_component_indent(&mut self, component: &mut PropertyComponent) -> i32 {
        
        todo!();
        /*
            return jmin (10, component.getWidth() / 10);
        */
    }
    
    pub fn get_property_component_content_position(&mut self, component: &mut PropertyComponent) -> Rectangle<i32> {
        
        todo!();
        /*
            auto textW = jmin (200, component.getWidth() / 2);
        return { textW, 0, component.getWidth() - textW, component.getHeight() - 1 };
        */
    }
    
    pub fn draw_call_out_box_background(&mut self, 
        box_:         &mut CallOutBox,
        g:            &mut Graphics,
        path:         &Path,
        cached_image: &mut Image)  {
        
        todo!();
        /*
            if (cachedImage.isNull())
        {
            cachedImage = { Image::ARGB, box.getWidth(), box.getHeight(), true };
            Graphics g2 (cachedImage);

            DropShadow (Colours::black.withAlpha (0.7f), 8, { 0, 2 }).drawForPath (g2, path);
        }

        g.setColour (Colours::black);
        g.drawImageAt (cachedImage, 0, 0);

        g.setColour (currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).withAlpha (0.8f));
        g.fillPath (path);

        g.setColour (currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).withAlpha (0.8f));
        g.strokePath (path, PathStrokeType (2.0f));
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
            g.fillAll (currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).withAlpha (0.5f));
        */
    }
    
    pub fn initialise_colours(&mut self)  {
        
        todo!();
        /*
            const uint32 transparent = 0x00000000;

        const uint32 coloursToUse[] =
        {
            TextButton::buttonColourId,                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            TextButton::buttonOnColourId,               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),
            TextButton::textColourOnId,                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedText).getARGB(),
            TextButton::textColourOffId,                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            ToggleButton::textColourId,                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            ToggleButton::tickColourId,                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            ToggleButton::tickDisabledColourId,         currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).withAlpha (0.5f).getARGB(),

            TextEditor::backgroundColourId,             currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            TextEditor::textColourId,                   currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            TextEditor::highlightColourId,              currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).withAlpha (0.4f).getARGB(),
            TextEditor::highlightedTextColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedText).getARGB(),
            TextEditor::outlineColourId,                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            TextEditor::focusedOutlineColourId,         currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            TextEditor::shadowColourId,                 transparent,

            CaretComponent::caretColourId,              currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).getARGB(),

            Label::backgroundColourId,                  transparent,
            Label::textColourId,                        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            Label::outlineColourId,                     transparent,
            Label::textWhenEditingColourId,             currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            ScrollBar::backgroundColourId,              transparent,
            ScrollBar::thumbColourId,                   currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).getARGB(),
            ScrollBar::trackColourId,                   transparent,

            TreeView::linesColourId,                    transparent,
            TreeView::backgroundColourId,               transparent,
            TreeView::dragAndDropIndicatorColourId,     currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            TreeView::selectedItemBackgroundColourId,   transparent,
            TreeView::oddItemsColourId,                 transparent,
            TreeView::evenItemsColourId,                transparent,

            PopupMenu::backgroundColourId,              currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuBackground).getARGB(),
            PopupMenu::textColourId,                    currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuText).getARGB(),
            PopupMenu::headerTextColourId,              currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuText).getARGB(),
            PopupMenu::highlightedTextColourId,         currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedText).getARGB(),
            PopupMenu::highlightedBackgroundColourId,   currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),

            ComboBox::buttonColourId,                   currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            ComboBox::outlineColourId,                  currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            ComboBox::textColourId,                     currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            ComboBox::backgroundColourId,               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            ComboBox::arrowColourId,                    currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            ComboBox::focusedOutlineColourId,           currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            PropertyComponent::backgroundColourId,      currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            PropertyComponent::labelTextColourId,       currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            TextPropertyComponent::backgroundColourId,  currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            TextPropertyComponent::textColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            TextPropertyComponent::outlineColourId,     currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            BooleanPropertyComponent::backgroundColourId, currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            BooleanPropertyComponent::outlineColourId,    currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            ListBox::backgroundColourId,                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            ListBox::outlineColourId,                   currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            ListBox::textColourId,                      currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            Slider::backgroundColourId,                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            Slider::thumbColourId,                      currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).getARGB(),
            Slider::trackColourId,                      currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),
            Slider::rotarySliderFillColourId,           currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),
            Slider::rotarySliderOutlineColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            Slider::textBoxTextColourId,                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            Slider::textBoxBackgroundColourId,          currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).withAlpha (0.0f).getARGB(),
            Slider::textBoxHighlightColourId,           currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).withAlpha (0.4f).getARGB(),
            Slider::textBoxOutlineColourId,             currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            ResizableWindow::backgroundColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::windowBackground).getARGB(),

            DocumentWindow::textColourId,               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            AlertWindow::backgroundColourId,            currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            AlertWindow::textColourId,                  currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            AlertWindow::outlineColourId,               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            ProgressBar::backgroundColourId,            currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            ProgressBar::foregroundColourId,            currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),

            TooltipWindow::backgroundColourId,          currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),
            TooltipWindow::textColourId,                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedText).getARGB(),
            TooltipWindow::outlineColourId,             transparent,

            TabbedComponent::backgroundColourId,        transparent,
            TabbedComponent::outlineColourId,           currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            TabbedButtonBar::tabOutlineColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).withAlpha (0.5f).getARGB(),
            TabbedButtonBar::frontOutlineColourId,      currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            Toolbar::backgroundColourId,                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).withAlpha (0.4f).getARGB(),
            Toolbar::separatorColourId,                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            Toolbar::buttonMouseOverBackgroundColourId, currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).contrasting (0.2f).getARGB(),
            Toolbar::buttonMouseDownBackgroundColourId, currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).contrasting (0.5f).getARGB(),
            Toolbar::labelTextColourId,                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            Toolbar::editingModeOutlineColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            DrawableButton::textColourId,               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            DrawableButton::textColourOnId,             currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedText).getARGB(),
            DrawableButton::backgroundColourId,         transparent,
            DrawableButton::backgroundOnColourId,       currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),

            HyperlinkButton::textColourId,              currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).interpolatedWith (Colours::blue, 0.4f).getARGB(),

            GroupComponent::outlineColourId,            currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),
            GroupComponent::textColourId,               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            BubbleComponent::backgroundColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            BubbleComponent::outlineColourId,           currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            DirectoryContentsDisplayComponent::highlightColourId,          currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).getARGB(),
            DirectoryContentsDisplayComponent::textColourId,               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuText).getARGB(),
            DirectoryContentsDisplayComponent::highlightedTextColourId,    currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedText).getARGB(),

            0x1000440, /*LassoComponent::lassoFillColourId*/        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).getARGB(),
            0x1000441, /*LassoComponent::lassoOutlineColourId*/     currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::outline).getARGB(),

            0x1005000, /*MidiKeyboardComponent::whiteNoteColourId*/               0xffffffff,
            0x1005001, /*MidiKeyboardComponent::blackNoteColourId*/               0xff000000,
            0x1005002, /*MidiKeyboardComponent::keySeparatorLineColourId*/        0x66000000,
            0x1005003, /*MidiKeyboardComponent::mouseOverKeyOverlayColourId*/     0x80ffff00,
            0x1005004, /*MidiKeyboardComponent::keyDownOverlayColourId*/          0xffb6b600,
            0x1005005, /*MidiKeyboardComponent::textLabelColourId*/               0xff000000,
            0x1005006, /*MidiKeyboardComponent::upDownButtonBackgroundColourId*/  0xffd3d3d3,
            0x1005007, /*MidiKeyboardComponent::upDownButtonArrowColourId*/       0xff000000,
            0x1005008, /*MidiKeyboardComponent::shadowColourId*/                  0x4c000000,

            0x1004500, /*CodeEditorComponent::backgroundColourId*/                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            0x1004502, /*CodeEditorComponent::highlightColourId*/                 currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).withAlpha (0.4f).getARGB(),
            0x1004503, /*CodeEditorComponent::defaultTextColourId*/               currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            0x1004504, /*CodeEditorComponent::lineNumberBackgroundId*/            currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::highlightedFill).withAlpha (0.5f).getARGB(),
            0x1004505, /*CodeEditorComponent::lineNumberTextId*/                  currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).getARGB(),

            0x1007000, /*ColourSelector::backgroundColourId*/                     currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            0x1007001, /*ColourSelector::labelTextColourId*/                      currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            0x100ad00, /*KeyMappingEditorComponent::backgroundColourId*/          currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            0x100ad01, /*KeyMappingEditorComponent::textColourId*/                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            FileSearchPathListComponent::backgroundColourId,        currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuBackground).getARGB(),

            FileChooserDialogBox::titleTextColourId,                currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),

            SidePanel::backgroundColour,                            currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).getARGB(),
            SidePanel::titleTextColour,                             currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultText).getARGB(),
            SidePanel::shadowBaseColour,                            currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::widgetBackground).darker().getARGB(),
            SidePanel::dismissButtonNormalColour,                   currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).getARGB(),
            SidePanel::dismissButtonOverColour,                     currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).darker().getARGB(),
            SidePanel::dismissButtonDownColour,                     currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::defaultFill).brighter().getARGB(),

            FileBrowserComponent::currentPathBoxBackgroundColourId,    currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuBackground).getARGB(),
            FileBrowserComponent::currentPathBoxTextColourId,          currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuText).getARGB(),
            FileBrowserComponent::currentPathBoxArrowColourId,         currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuText).getARGB(),
            FileBrowserComponent::filenameBoxBackgroundColourId,       currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuBackground).getARGB(),
            FileBrowserComponent::filenameBoxTextColourId,             currentColourScheme.getUIColour (LookAndFeelV4ColourScheme::LookAndFeelV4ColourSchemeUIColour::menuText).getARGB(),
        };

        for (int i = 0; i < numElementsInArray (coloursToUse); i += 2)
            setColour ((int) coloursToUse [i], Colour ((uint32) coloursToUse [i + 1]));
        */
    }
}

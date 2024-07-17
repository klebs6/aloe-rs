crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ToolbarItemComponent.h]

/**
  | A component that can be used as one of
  | the items in a Toolbar.
  | 
  | Each of the items on a toolbar must be
  | a component derived from ToolbarItemComponent,
  | and these objects are always created
  | by a ToolbarItemFactory - see the ToolbarItemFactory
  | class for further info about creating
  | them.
  | 
  | The ToolbarItemComponent class is
  | actually a button, but can be used to
  | hold non-button components too. To
  | do this, set the value of isBeingUsedAsAButton
  | to false when calling the constructor,
  | and override contentAreaChanged(),
  | in which you can position any sub-components
  | you need to add.
  | 
  | To add basic buttons without writing
  | a special subclass, have a look at the
  | 
  | ToolbarButton class.
  | 
  | @see ToolbarButton, Toolbar, ToolbarItemFactory
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ToolbarItemComponent<'a> {
    base:                     Button<'a>,
    item_id:                  i32,
    mode:                     ToolbarEditingMode,
    toolbar_style:            ToolbarItemStyle,
    overlay_comp:             Box<Component<'a>>,
    drag_offsetx:             i32,
    drag_offsety:             i32,
    is_active:                bool,
    is_being_dragged:         bool,
    is_being_used_as_abutton: bool,
    content_area:             Rectangle<i32>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ToolbarItemComponent.cpp]
impl<'a> Drop for ToolbarItemComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            overlayComp.reset();
        */
    }
}

impl<'a> ToolbarItemComponent<'a> {

    /**
      | Returns the item type ID that this component
      | represents.
      | 
      | This value is in the constructor.
      |
      */
    pub fn get_item_id(&self) -> i32 {
        
        todo!();
        /*
            return itemId;
        */
    }

    /**
      | Returns the current style setting of
      | this item.
      | 
      | Styles are listed in the Toolbar::ToolbarItemStyle
      | enum. @see setStyle, Toolbar::getStyle
      |
      */
    pub fn get_style(&self) -> ToolbarItemStyle {
        
        todo!();
        /*
            return toolbarStyle;
        */
    }

    /**
      | Returns the area of the component that
      | should be used to display the button
      | image or other contents of the item.
      | 
      | This content area may change when the
      | item's style changes, and may leave
      | a space around the edge of the component
      | where the text label can be shown.
      | 
      | @see contentAreaChanged
      |
      */
    pub fn get_content_area(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return contentArea;
        */
    }

    /**
      | Returns the current editing mode of
      | this component.
      | 
      | This is used by the ToolbarItemPalette
      | and related classes for making the items
      | draggable, and is unlikely to be of much
      | use in end-user-code.
      |
      */
    pub fn get_editing_mode(&self) -> ToolbarEditingMode {
        
        todo!();
        /*
            return mode;
        */
    }
    
    /**
      | Constructor.
      | 
      | -----------
      | @param itemId
      | 
      | the ID of the type of toolbar item which
      | this represents
      | ----------
      | @param labelText
      | 
      | the text to display if the toolbar's
      | style is set to
      | 
      | Toolbar::iconsWithText or Toolbar::textOnly
      | ----------
      | @param isBeingUsedAsAButton
      | 
      | set this to false if you don't want the
      | button to draw itself with button over/down
      | states when the mouse moves over it or
      | clicks
      |
      */
    pub fn new(
        item_id:                  i32,
        label_text:               &String,
        is_being_used_as_abutton: bool) -> Self {
    
        todo!();
        /*
        : button(labelText),
        : item_id(itemId_),
        : mode(normalMode),
        : toolbar_style(Toolbar::iconsOnly),
        : drag_offsetx(0),
        : drag_offsety(0),
        : is_active(true),
        : is_being_dragged(false),
        : is_being_used_as_abutton(isBeingUsedAsAButton_),

            // Your item ID can't be 0!
        jassert (itemId_ != 0);
        */
    }
    
    /**
      | Returns the toolbar that contains this
      | component, or nullptr if it's not currently
      | inside one.
      |
      */
    pub fn get_toolbar(&self) -> *mut Toolbar {
        
        todo!();
        /*
            return dynamic_cast<Toolbar*> (getParentComponent());
        */
    }
    
    /**
      | Returns true if this component is currently
      | inside a toolbar which is vertical.
      | @see Toolbar::isVertical
      |
      */
    pub fn is_toolbar_vertical(&self) -> bool {
        
        todo!();
        /*
            const Toolbar* const t = getToolbar();
        return t != nullptr && t->isVertical();
        */
    }
    
    pub fn set_style(&mut self, new_style: &ToolbarItemStyle)  {
        
        todo!();
        /*
            if (toolbarStyle != newStyle)
        {
            toolbarStyle = newStyle;
            repaint();
            resized();
        }
        */
    }
    
    pub fn paint_button(&mut self, 
        g:    &mut Graphics,
        over: bool,
        down: bool)  {
        
        todo!();
        /*
            if (isBeingUsedAsAButton)
            getLookAndFeel().paintToolbarButtonBackground (g, getWidth(), getHeight(),
                                                           over, down, *this);

        if (toolbarStyle != Toolbar::iconsOnly)
        {
            auto indent = contentArea.getX();
            auto y = indent;
            auto h = getHeight() - indent * 2;

            if (toolbarStyle == Toolbar::iconsWithText)
            {
                y = contentArea.getBottom() + indent / 2;
                h -= contentArea.getHeight();
            }

            getLookAndFeel().paintToolbarButtonLabel (g, indent, y, getWidth() - indent * 2, h,
                                                      getButtonText(), *this);
        }

        if (! contentArea.isEmpty())
        {
            Graphics::ScopedSaveState ss (g);

            g.reduceClipRegion (contentArea);
            g.setOrigin (contentArea.getPosition());

            paintButtonArea (g, contentArea.getWidth(), contentArea.getHeight(), over, down);
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (toolbarStyle != Toolbar::textOnly)
        {
            const int indent = jmin (proportionOfWidth (0.08f),
                                     proportionOfHeight (0.08f));

            contentArea = Rectangle<int> (indent, indent,
                                          getWidth() - indent * 2,
                                          toolbarStyle == Toolbar::iconsWithText ? proportionOfHeight (0.55f)
                                                                                 : (getHeight() - indent * 2));
        }
        else
        {
            contentArea = {};
        }

        contentAreaChanged (contentArea);
        */
    }
    
    /**
      | Changes the editing mode of this component.
      | 
      | This is used by the ToolbarItemPalette
      | and related classes for making the items
      | draggable, and is unlikely to be of much
      | use in end-user-code.
      |
      */
    pub fn set_editing_mode(&mut self, new_mode: ToolbarEditingMode)  {
        
        todo!();
        /*
            if (mode != newMode)
        {
            mode = newMode;
            repaint();

            if (mode == normalMode)
            {
                overlayComp.reset();
            }
            else if (overlayComp == nullptr)
            {
                overlayComp.reset (new ItemDragAndDropOverlayComponent());
                addAndMakeVisible (overlayComp.get());
                overlayComp->parentSizeChanged();
            }

            resized();
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            const auto shouldItemBeAccessible = (itemId != ToolbarItemFactory::separatorBarId
                                          && itemId != ToolbarItemFactory::spacerId
                                          && itemId != ToolbarItemFactory::flexibleSpacerId);

        if (! shouldItemBeAccessible)
            return nullptr;

        return std::make_unique<ButtonAccessibilityHandler> (*this, AccessibilityRole::button);
        */
    }
}

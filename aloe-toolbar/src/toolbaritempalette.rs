crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ToolbarItemPalette.h]

/**
  | A component containing a list of toolbar
  | items, which the user can drag onto a
  | toolbar to add them.
  | 
  | You can use this class directly, but
  | it's a lot easier to call Toolbar::showCustomisationDialog(),
  | which automatically shows one of these
  | in a dialog box with lots of extra controls.
  | 
  | @see Toolbar
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ToolbarItemPalette<'a> {
    base:     Component<'a>,
    base2:    DragAndDropContainer<'a>,
    factory:  &'a mut dyn ToolbarItemFactory,
    toolbar:  &'a mut Toolbar<'a>,
    viewport: Viewport<'a>,
    items:    Vec<Box<dyn ToolbarItemComponent>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ToolbarItemPalette.cpp]
impl<'a> ToolbarItemPalette<'a> {

    /**
      | Creates a palette of items for a given
      | factory, with the aim of adding them
      | to the specified toolbar.
      | 
      | The ToolbarItemFactory::getAllToolbarItemIds()
      | method is used to create the set of items
      | that are shown in this palette.
      | 
      | The toolbar and factory must not be deleted
      | while this object exists.
      |
      */
    pub fn new(
        tbf: &mut dyn ToolbarItemFactory,
        bar: &mut Toolbar) -> Self {
    
        todo!();
        /*
        : factory(tbf),
        : toolbar(bar),

            auto* itemHolder = new Component();
        viewport.setViewedComponent (itemHolder);

        Vec<int> allIds;
        factory.getAllToolbarItemIds (allIds);

        for (auto& i : allIds)
            addComponent (i, -1);

        addAndMakeVisible (viewport);
        */
    }
    
    pub fn add_component(&mut self, 
        item_id: i32,
        index:   i32)  {
        
        todo!();
        /*
            if (auto* tc = Toolbar::createItem (factory, itemId))
        {
            items.insert (index, tc);
            viewport.getViewedComponent()->addAndMakeVisible (tc, index);
            tc->setEditingMode (ToolbarItemComponent::editableOnPalette);
        }
        else
        {
            jassertfalse;
        }
        */
    }
    
    pub fn replace_component(
        &mut self, 
        comp: &mut dyn ToolbarItemComponent

    ) {
        
        todo!();
        /*
            auto index = items.indexOf (&comp);
        jassert (index >= 0);
        items.removeObject (&comp, false);

        addComponent (comp.getItemId(), index);
        resized();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            viewport.setBoundsInset (BorderSize<int> (1));

        auto* itemHolder = viewport.getViewedComponent();

        const int indent = 8;
        const int preferredWidth = viewport.getWidth() - viewport.getScrollBarThickness() - indent;
        const int height = toolbar.getThickness();
        auto x = indent;
        auto y = indent;
        int maxX = 0;

        for (auto* tc : items)
        {
            tc->setStyle (toolbar.getStyle());

            int preferredSize = 1, minSize = 1, maxSize = 1;

            if (tc->getToolbarItemSizes (height, false, preferredSize, minSize, maxSize))
            {
                if (x + preferredSize > preferredWidth && x > indent)
                {
                    x = indent;
                    y += height;
                }

                tc->setBounds (x, y, preferredSize, height);

                x += preferredSize + 8;
                maxX = jmax (maxX, x);
            }
        }

        itemHolder->setSize (maxX, y + height + 8);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::group);
        */
    }
}

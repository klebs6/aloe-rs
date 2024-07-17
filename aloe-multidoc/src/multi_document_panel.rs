crate::ix!();

pub fn should_delete_comp(c: *mut Component) -> bool {
    
    todo!();
    /*
        return c->getProperties() ["mdiDocumentDelete_"];
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_MultiDocumentPanel.h]

/**
  | The different layout modes available.
  |
  */
pub enum MultiDocumentPanelLayoutMode
{
    /**
      | In this mode, there are overlapping
      | DocumentWindow components for each
      | document.
      |
      */
    FloatingWindows,            

    /**
      | In this mode, a TabbedComponent is used
      | to show one document at a time.
      |
      */
    MaximisedWindowsWithTabs,    
}

/**
  | A component that contains a set of other
  | components either in floating windows
  | or tabs.
  | 
  | This acts as a panel that can be used to
  | hold a set of open document windows,
  | with different layout modes.
  | 
  | Use addDocument() and closeDocument()
  | to add or remove components from the
  | panel - never use any of the Component
  | methods to access the panel's child
  | components directly, as these are managed
  | internally.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MultiDocumentPanel<'a> {
    base:                      Component<'a>,
    mode:                      MultiDocumentPanelLayoutMode, // default = MaximisedWindowsWithTabs
    components:                Vec<*mut Component<'a>>,
    tab_component:             Box<TabbedComponent<'a>>,
    background_colour:         Colour, // default = { Colours::lightblue  }
    maximum_num_documents:     i32, // default = 0
    num_docs_before_tabs_used: i32, // default = 0
}

impl<'a> ComponentListener for MultiDocumentPanel<'a> {

}

impl<'a> ComponentMovedOrResized for MultiDocumentPanel<'a> {

}

impl<'a> ComponentBroughtToFront for MultiDocumentPanel<'a> {

}

impl<'a> ComponentVisibilityChanged for MultiDocumentPanel<'a> {

}

impl<'a> ComponentChildrenChanged for MultiDocumentPanel<'a> {

}

impl<'a> ComponentParentHierarchyChanged for MultiDocumentPanel<'a> {

}

impl<'a> ComponentNameChanged for MultiDocumentPanel<'a> {

}

impl<'a> ComponentBeingDeleted for MultiDocumentPanel<'a> {

}

impl<'a> ComponentEnablementChanged for MultiDocumentPanel<'a> {

}

impl<'a> Default for MultiDocumentPanel<'a> {
    
    /**
      | Creates an empty panel.
      | 
      | Use addDocument() and closeDocument()
      | to add or remove components from the
      | panel - never use any of the Component
      | methods to access the panel's child
      | components directly, as these are managed
      | internally.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            setOpaque (true);
        */
    }
}

impl<'a> Drop for MultiDocumentPanel<'a> {

    /**
      | Destructor.
      | 
      | When deleted, this will call closeAllDocuments
      | (false) to make sure all its components
      | are deleted. If you need to make sure
      | all documents are saved before closing,
      | then you should call closeAllDocuments
      | (true) and check that it returns true
      | before deleting the panel.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_MultiDocumentPanel.cpp]

impl<'a> MultiDocumentPanel<'a> {

    /**
      | Returns the current layout mode.
      |
      */
    pub fn get_layout_mode(&self) -> MultiDocumentPanelLayoutMode {
        
        todo!();
        /*
            return mode;
        */
    }

    /**
      | Returns the current background colour.
      | 
      | @see setBackgroundColour
      |
      */
    pub fn get_background_colour(&self) -> Colour {
        
        todo!();
        /*
            return backgroundColour;
        */
    }

    /**
      | If the panel is being used in tabbed mode,
      | this returns the TabbedComponent that's
      | involved.
      |
      */
    pub fn get_current_tabbed_component(&self) -> *mut TabbedComponent {
        
        todo!();
        /*
            return tabComponent.get();
        */
    }
    
    /**
      | Tries to close all the documents.
      | 
      | If checkItsOkToCloseFirst is true,
      | then the tryToCloseDocument() method
      | will be called for each open document,
      | and any of these calls fails, this method
      | will stop and return false, leaving
      | some documents still open.
      | 
      | If checkItsOkToCloseFirst is false,
      | then all documents will be closed unconditionally.
      | 
      | @see closeDocument
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn close_all_documents(&mut self, check_its_ok_to_close_first: bool) -> bool {
        
        todo!();
        /*
            while (! components.isEmpty())
            if (! closeDocument (components.getLast(), checkItsOkToCloseFirst))
                return false;

        return true;
        */
    }
    
    pub fn close_last_document_recursive(&mut self, 
        parent:                      ComponentSafePointer<MultiDocumentPanel>,
        check_its_ok_to_close_first: bool,
        callback:                    fn(_0: bool) -> ())  {
        
        todo!();
        /*
            if (parent->components.isEmpty())
        {
            if (callback != nullptr)
                callback (true);

            return;
        }

        parent->closeDocumentAsync (parent->components.getLast(),
                                    checkItsOkToCloseFirst,
                                    [parent, checkItsOkToCloseFirst, callback] (bool closeResult)
        {
            if (parent == nullptr)
                return;

            if (! closeResult)
            {
                if (callback != nullptr)
                    callback (false);

                return;
            }

            parent->closeLastDocumentRecursive (parent, checkItsOkToCloseFirst, std::move (callback));
        });
        */
    }
    
    /**
      | Tries to close all the documents.
      | 
      | If checkItsOkToCloseFirst is true,
      | then the tryToCloseDocumentAsync()
      | method will be called for each open document,
      | and any of these calls fails, this method
      | will stop and provide an argument of
      | false to the callback, leaving some
      | documents still open.
      | 
      | If checkItsOkToCloseFirst is false,
      | then all documents will be closed unconditionally.
      | 
      | @see closeDocument
      |
      */
    pub fn close_all_documents_async(&mut self, 
        check_its_ok_to_close_first: bool,
        callback:                    fn(_0: bool) -> ())  {
        
        todo!();
        /*
            closeLastDocumentRecursive (this, checkItsOkToCloseFirst, std::move (callback));
        */
    }

    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn try_to_close_document(&mut self, _0: *mut Component) -> bool {
        
        todo!();
        /*
            // If you hit this assertion then you need to implement this method in a subclass.
        jassertfalse;
        return false;
        */
    }
    
    pub fn create_new_document_window(&mut self) -> *mut MultiDocumentPanelWindow {
        
        todo!();
        /*
            return new MultiDocumentPanelWindow (backgroundColour);
        */
    }
    
    pub fn add_window(&mut self, component: *mut Component)  {
        
        todo!();
        /*
            auto* dw = createNewDocumentWindow();

        dw->setResizable (true, false);
        dw->setContentNonOwned (component, true);
        dw->setName (component->getName());

        auto bkg = component->getProperties() ["mdiDocumentBkg_"];
        dw->setBackgroundColour (bkg.isVoid() ? backgroundColour : Colour ((uint32) static_cast<int> (bkg)));

        int x = 4;

        if (auto* topComp = getChildren().getLast())
            if (topComp->getX() == x && topComp->getY() == x)
                x += 16;

        dw->setTopLeftPosition (x, x);

        auto pos = component->getProperties() ["mdiDocumentPos_"];
        if (pos.toString().isNotEmpty())
            dw->restoreWindowStateFromString (pos.toString());

        addAndMakeVisible (dw);
        dw->toFront (true);
        */
    }
    
    /**
      | Adds a document component to the panel.
      | 
      | If the number of documents would exceed
      | the limit set by setMaximumNumDocuments()
      | then this will fail and return false.
      | (If it does fail, the component passed-in
      | will not be deleted, even if deleteWhenRemoved
      | was set to true).
      | 
      | The MultiDocumentPanel will deal with
      | creating a window border to go around
      | your component, so just pass in the bare
      | content component here, no need to give
      | it a ResizableWindow or DocumentWindow.
      | 
      | -----------
      | @param component
      | 
      | the component to add
      | ----------
      | @param backgroundColour
      | 
      | the background colour to use to fill
      | the component's window or tab
      | ----------
      | @param deleteWhenRemoved
      | 
      | if true, then when the component is removed
      | by closeDocument() or closeAllDocuments(),
      | then it will be deleted. If false, then
      | the caller must handle the component's
      | deletion
      |
      */
    pub fn add_document(&mut self, 
        component:           *mut Component,
        doc_colour:          Colour,
        delete_when_removed: bool) -> bool {
        
        todo!();
        /*
            // If you try passing a full DocumentWindow or ResizableWindow in here, you'll end up
        // with a frame-within-a-frame! Just pass in the bare content component.
        jassert (dynamic_cast<ResizableWindow*> (component) == nullptr);

        if (component == nullptr || (maximumNumDocuments > 0 && components.size() >= maximumNumDocuments))
            return false;

        components.add (component);
        component->getProperties().set ("mdiDocumentDelete_", deleteWhenRemoved);
        component->getProperties().set ("mdiDocumentBkg_", (int) docColour.getARGB());
        component->addComponentListener (this);

        if (mode == FloatingWindows)
        {
            if (isFullscreenWhenOneDocument())
            {
                if (components.size() == 1)
                {
                    addAndMakeVisible (component);
                }
                else
                {
                    if (components.size() == 2)
                        addWindow (components.getFirst());

                    addWindow (component);
                }
            }
            else
            {
               addWindow (component);
            }
        }
        else
        {
            if (tabComponent == nullptr && components.size() > numDocsBeforeTabsUsed)
            {
                tabComponent.reset (new MultiDocumentPanelTabbedComponentInternal());
                addAndMakeVisible (tabComponent.get());

                auto temp = components;

                for (auto& c : temp)
                    tabComponent->addTab (c->getName(), docColour, c, false);

                resized();
            }
            else
            {
                if (tabComponent != nullptr)
                    tabComponent->addTab (component->getName(), docColour, component, false);
                else
                    addAndMakeVisible (component);
            }

            setActiveDocument (component);
        }

        resized();
        activeDocumentChanged();
        return true;
        */
    }
    
    pub fn close_document_internal(&mut self, component: *mut Component)  {
        
        todo!();
        /*
            // Intellisense warns about component being uninitialised.
        // I'm not sure how a function argument could be uninitialised.
        ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6001)

        component->removeComponentListener (this);

        const bool shouldDelete = MultiDocHelpers::shouldDeleteComp (component);
        component->getProperties().remove ("mdiDocumentDelete_");
        component->getProperties().remove ("mdiDocumentBkg_");

        if (mode == FloatingWindows)
        {
            for (auto* child : getChildren())
            {
                if (auto* dw = dynamic_cast<MultiDocumentPanelWindow*> (child))
                {
                    if (dw->getContentComponent() == component)
                    {
                        std::unique_ptr<MultiDocumentPanelWindow> (dw)->clearContentComponent();
                        break;
                    }
                }
            }

            if (shouldDelete)
                delete component;

            components.removeFirstMatchingValue (component);

            if (isFullscreenWhenOneDocument() && components.size() == 1)
            {
                for (int i = getNumChildComponents(); --i >= 0;)
                {
                    std::unique_ptr<MultiDocumentPanelWindow> dw (dynamic_cast<MultiDocumentPanelWindow*> (getChildComponent (i)));

                    if (dw != nullptr)
                        dw->clearContentComponent();
                }

                addAndMakeVisible (components.getFirst());
            }
        }
        else
        {
            jassert (components.indexOf (component) >= 0);

            if (tabComponent != nullptr)
            {
                for (int i = tabComponent->getNumTabs(); --i >= 0;)
                    if (tabComponent->getTabContentComponent (i) == component)
                        tabComponent->removeTab (i);
            }
            else
            {
                removeChildComponent (component);
            }

            if (shouldDelete)
                delete component;

            if (tabComponent != nullptr && tabComponent->getNumTabs() <= numDocsBeforeTabsUsed)
                tabComponent.reset();

            components.removeFirstMatchingValue (component);

            if (components.size() > 0 && tabComponent == nullptr)
                addAndMakeVisible (components.getFirst());
        }

        resized();

        // This ensures that the active tab is painted properly when a tab is closed!
        if (auto* activeComponent = getActiveDocument())
            setActiveDocument (activeComponent);

        activeDocumentChanged();

        ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    /**
      | Closes one of the documents.
      | 
      | If checkItsOkToCloseFirst is true,
      | then the tryToCloseDocument() method
      | will be called, and if it fails, this
      | method will return false without closing
      | the document.
      | 
      | If checkItsOkToCloseFirst is false,
      | then the documents will be closed unconditionally.
      | 
      | The component will be deleted if the
      | deleteWhenRemoved parameter was set
      | to true when it was added with addDocument.
      | 
      | @see addDocument, closeAllDocuments
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn close_document(&mut self, 
        component:                   *mut Component,
        check_its_ok_to_close_first: bool) -> bool {
        
        todo!();
        /*
            // Intellisense warns about component being uninitialised.
        // I'm not sure how a function argument could be uninitialised.
        ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6001)

        if (component == nullptr)
            return true;

        if (components.contains (component))
        {
            if (checkItsOkToCloseFirst && ! tryToCloseDocument (component))
                return false;

            closeDocumentInternal (component);
        }
        else
        {
            jassertfalse;
        }

        return true;

        ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }
    
    /**
      | Closes one of the documents.
      | 
      | If checkItsOkToCloseFirst is true,
      | then the tryToCloseDocumentAsync()
      | method will be called, and if it fails,
      | this method will call the callback with
      | a false argument without closing the
      | document.
      | 
      | If checkItsOkToCloseFirst is false,
      | then the documents will be closed unconditionally.
      | 
      | The component will be deleted if the
      | deleteWhenRemoved parameter was set
      | to true when it was added with addDocument.
      | 
      | @see addDocument, closeAllDocuments
      |
      */
    pub fn close_document_async(&mut self, 
        component:                   *mut Component,
        check_its_ok_to_close_first: bool,
        callback:                    fn(_0: bool) -> ())  {
        
        todo!();
        /*
            // Intellisense warns about component being uninitialised.
        // I'm not sure how a function argument could be uninitialised.
        ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6001)

        if (component == nullptr)
        {
            if (callback != nullptr)
                callback (true);

            return;
        }

        if (components.contains (component))
        {
            if (checkItsOkToCloseFirst)
            {
                tryToCloseDocumentAsync (component,
                                         [parent = SafePointer<MultiDocumentPanel> { this }, component, callback] (bool closedSuccessfully)
                {
                    if (parent == nullptr)
                        return;

                    if (closedSuccessfully)
                        parent->closeDocumentInternal (component);

                    if (callback != nullptr)
                        callback (closedSuccessfully);
                });

                return;
            }
        }
        else
        {
            jassertfalse;
        }

        if (callback != nullptr)
            callback (true);

        ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }
    
    /**
      | Returns the number of open document
      | windows.
      | 
      | @see getDocument
      |
      */
    pub fn get_num_documents(&self) -> i32 {
        
        todo!();
        /*
            return components.size();
        */
    }
    
    /**
      | Returns one of the open documents.
      | 
      | The order of the documents in this array
      | may change when they are added, removed
      | or moved around.
      | 
      | @see getNumDocuments
      |
      */
    pub fn get_document(&self, index: i32) -> *mut Component {
        
        todo!();
        /*
            return components [index];
        */
    }
    
    /**
      | Returns the document component that
      | is currently focused or on top.
      | 
      | If currently using floating windows,
      | then this will be the component in the
      | currently active window, or the top
      | component if none are active.
      | 
      | If it's currently in tabbed mode, then
      | it'll return the component in the active
      | tab.
      | 
      | @see setActiveDocument
      |
      */
    pub fn get_active_document(&self) -> *mut Component {
        
        todo!();
        /*
            if (mode == FloatingWindows)
        {
            for (auto* child : getChildren())
                if (auto* dw = dynamic_cast<MultiDocumentPanelWindow*> (child))
                    if (dw->isActiveWindow())
                        return dw->getContentComponent();
        }

        return components.getLast();
        */
    }
    
    /**
      | Makes one of the components active and
      | brings it to the top.
      | 
      | @see getActiveDocument
      |
      */
    pub fn set_active_document(&mut self, component: *mut Component)  {
        
        todo!();
        /*
            jassert (component != nullptr);

        if (mode == FloatingWindows)
        {
            component = getContainerComp (component);

            if (component != nullptr)
                component->toFront (true);
        }
        else if (tabComponent != nullptr)
        {
            jassert (components.indexOf (component) >= 0);

            for (int i = tabComponent->getNumTabs(); --i >= 0;)
            {
                if (tabComponent->getTabContentComponent (i) == component)
                {
                    tabComponent->setCurrentTabIndex (i);
                    break;
                }
            }
        }
        else
        {
            component->grabKeyboardFocus();
        }
        */
    }
    
    pub fn active_document_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Sets a limit on how many windows can be
      | open at once.
      | 
      | If this is zero or less there's no limit
      | (the default). addDocument() will
      | fail if this number is exceeded.
      |
      */
    pub fn set_maximum_num_documents(&mut self, new_number: i32)  {
        
        todo!();
        /*
            maximumNumDocuments = newNumber;
        */
    }
    
    /**
      | Sets an option to make the document fullscreen
      | if there's only one document open.
      | 
      | If set to true, then if there's only one
      | document, it'll fill the whole of this
      | component without tabs or a window border.
      | If false, then tabs or a window will always
      | be shown, even if there's only one document.
      | If there's more than one document open,
      | then this option makes no difference.
      |
      */
    pub fn use_fullscreen_when_one_document(&mut self, should_use_tabs: bool)  {
        
        todo!();
        /*
            numDocsBeforeTabsUsed = shouldUseTabs ? 1 : 0;
        */
    }
    
    /**
      | Returns the result of the last time useFullscreenWhenOneDocument()
      | was called.
      |
      */
    pub fn is_fullscreen_when_one_document(&self) -> bool {
        
        todo!();
        /*
            return numDocsBeforeTabsUsed != 0;
        */
    }
    
    /**
      | Changes the panel's mode.
      | 
      | @see MultiDocumentPanelLayoutMode, getLayoutMode
      |
      */
    pub fn set_layout_mode(&mut self, new_layout_mode: MultiDocumentPanelLayoutMode)  {
        
        todo!();
        /*
            if (mode != newLayoutMode)
        {
            mode = newLayoutMode;

            if (mode == FloatingWindows)
            {
                tabComponent.reset();
            }
            else
            {
                for (int i = getNumChildComponents(); --i >= 0;)
                {
                    std::unique_ptr<MultiDocumentPanelWindow> dw (dynamic_cast<MultiDocumentPanelWindow*> (getChildComponent (i)));

                    if (dw != nullptr)
                    {
                        dw->getContentComponent()->getProperties().set ("mdiDocumentPos_", dw->getWindowStateAsString());
                        dw->clearContentComponent();
                    }
                }
            }

            resized();

            auto tempComps = components;
            components.clear();

            for (auto* c : tempComps)
                addDocument (c,
                             Colour ((uint32) static_cast<int> (c->getProperties().getWithDefault ("mdiDocumentBkg_", (int) Colours::white.getARGB()))),
                             MultiDocHelpers::shouldDeleteComp (c));
        }
        */
    }
    
    /**
      | Sets the background colour for the whole
      | panel.
      | 
      | Each document has its own background
      | colour, but this is the one used to fill
      | the areas behind them.
      |
      */
    pub fn set_background_colour(&mut self, new_background_colour: Colour)  {
        
        todo!();
        /*
            if (backgroundColour != newBackgroundColour)
        {
            backgroundColour = newBackgroundColour;
            setOpaque (newBackgroundColour.isOpaque());
            repaint();
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (backgroundColour);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (mode == MaximisedWindowsWithTabs || components.size() == numDocsBeforeTabsUsed)
        {
            for (auto* child : getChildren())
                child->setBounds (getLocalBounds());
        }

        setWantsKeyboardFocus (components.size() == 0);
        */
    }
    
    pub fn get_container_comp(&self, c: *mut Component) -> *mut Component {
        
        todo!();
        /*
            if (mode == FloatingWindows)
        {
            for (auto* child : getChildren())
                if (auto* dw = dynamic_cast<MultiDocumentPanelWindow*> (child))
                    if (dw->getContentComponent() == c)
                        return dw;
        }

        return c;
        */
    }
    
    pub fn component_name_changed(&mut self, _0: &mut Component)  {
        
        todo!();
        /*
            if (mode == FloatingWindows)
        {
            for (auto* child : getChildren())
                if (auto* dw = dynamic_cast<MultiDocumentPanelWindow*> (child))
                    dw->setName (dw->getContentComponent()->getName());
        }
        else if (tabComponent != nullptr)
        {
            for (int i = tabComponent->getNumTabs(); --i >= 0;)
                tabComponent->setTabName (i, tabComponent->getTabContentComponent (i)->getName());
        }
        */
    }
    
    pub fn update_order(&mut self)  {
        
        todo!();
        /*
            auto oldList = components;

        if (mode == FloatingWindows)
        {
            components.clear();

            for (auto* child : getChildren())
                if (auto* dw = dynamic_cast<MultiDocumentPanelWindow*> (child))
                    components.add (dw->getContentComponent());
        }
        else
        {
            if (tabComponent != nullptr)
            {
                if (auto* current = tabComponent->getCurrentContentComponent())
                {
                    components.removeFirstMatchingValue (current);
                    components.add (current);
                }
            }
        }

        if (components != oldList)
            activeDocumentChanged();
        */
    }
}

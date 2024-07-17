crate::ix!();

pub struct NavigableComponentsHolder<'a> {
    base: Component<'a>,
}

impl<'a> Default for NavigableComponentsHolder<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setTitle ("Navigable Components");
                setDescription ("A container of some navigable components.");
                setFocusContainerType (FocusContainerType::focusContainer);

                constexpr int numChildren = 12;

                for (int i = 1; i <= numChildren; ++i)
                {
                    children.push_back (std::make_unique<NavigableComponent> (i, numChildren, *this));
                    addAndMakeVisible (*children.back());
                
        */
    }
}

impl<'a> Resized for NavigableComponentsHolder<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)) };

                grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)) };

                for (auto& child : children)
                    grid.items.add (GridItem (*child).withMargin (5));

                grid.performLayout (getLocalBounds());
        */
    }
}

impl<'a> NavigableComponentsHolder<'a> {
    
    pub fn create_focus_traverser(&mut self) -> Box<dyn ComponentTraverser> {
        
        todo!();
        /*
            struct CustomTraverser  : public FocusTraverser
                {
                    explicit CustomTraverser (NavigableComponentsHolder& owner)
                        : navigableComponentsHolder (owner)  {}

                    Component* getDefaultComponent (Component*) override
                    {
                        for (auto& child : navigableComponentsHolder.children)
                            if (child->defaultToggle.getToggleState() && child->focusableToggle.getToggleState())
                                return child.get();

                        return nullptr;
                    }

                    Component* getNextComponent (Component* current) override
                    {
                        const auto& comps = navigableComponentsHolder.children;

                        const auto iter = std::find_if (comps.cbegin(), comps.cend(),
                                                        [current] (const std::unique_ptr<NavigableComponent>& c) { return c.get() == current; });

                        if (iter != comps.cend() && iter != std::prev (comps.cend()))
                            return std::next (iter)->get();

                        return nullptr;
                    }

                    Component* getPreviousComponent (Component* current) override
                    {
                        const auto& comps = navigableComponentsHolder.children;

                        const auto iter = std::find_if (comps.cbegin(), comps.cend(),
                                                        [current] (const std::unique_ptr<NavigableComponent>& c) { return c.get() == current; });

                        if (iter != comps.cend() && iter != comps.cbegin())
                            return std::prev (iter)->get();

                        return nullptr;
                    }

                    std::vector<Component*> getAllComponents (Component*) override
                    {
                        std::vector<Component*> comps;

                        for (auto& child : navigableComponentsHolder.children)
                            if (child->focusableToggle.getToggleState())
                                comps.push_back (child.get());

                        return comps;
                    }

                    NavigableComponentsHolder& navigableComponentsHolder;
                };

                return std::make_unique<CustomTraverser> (*this);
        */
    }
}

crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct FocusTraverserTests {
    base:      UnitTest,
    traverser: FocusTraverser,
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod focus_traverser_tests {
    use super::*;

    pub struct TestComponent {
        base:     Component<'a>,
        children: Vec<Component<'a>,10>,
    }

    impl Default for TestComponent {
        
        fn default() -> Self {
            todo!();
            /*


                for (auto& child : children)
                        addAndMakeVisible (child)
            */
        }
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for FocusTraverserTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("FocusTraverser", UnitTestCategories::gui
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl FocusTraverserTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            ScopedAloeInitialiser_GUI libraryInitialiser;

            beginTest ("Basic traversal");
            {
                TestComponent parent;

                expect (traverser.getDefaultComponent (&parent) == &parent.children.front());

                for (auto iter = parent.children.begin(); iter != parent.children.end(); ++iter)
                    expect (traverser.getNextComponent (&(*iter)) == (iter == std::prev (parent.children.cend()) ? nullptr
                                                                                                                 : &(*std::next (iter))));

                for (auto iter = parent.children.rbegin(); iter != parent.children.rend(); ++iter)
                    expect (traverser.getPreviousComponent (&(*iter)) == (iter == std::prev (parent.children.rend()) ? nullptr
                                                                                                                     : &(*std::next (iter))));
                auto allComponents = traverser.getAllComponents (&parent);

                expect (std::equal (allComponents.cbegin(), allComponents.cend(), parent.children.cbegin(),
                                    [] (const Component* c1, const Component& c2) { return c1 == &c2; }));
            }

            beginTest ("Disabled components are ignored");
            {
                checkIgnored ([] (Component& c) { c.setEnabled (false); });
            }

            beginTest ("Invisible components are ignored");
            {
                checkIgnored ([] (Component& c) { c.setVisible (false); });
            }

            beginTest ("Explicit focus order comes before unspecified");
            {
                TestComponent parent;

                auto& explicitFocusComponent = parent.children[2];

                explicitFocusComponent.setExplicitFocusOrder (1);
                expect (traverser.getDefaultComponent (&parent) == &explicitFocusComponent);

                expect (traverser.getAllComponents (&parent).front() == &explicitFocusComponent);
            }

            beginTest ("Explicit focus order comparison");
            {
                checkComponentProperties ([this] (Component& child) { child.setExplicitFocusOrder (getRandom().nextInt ({ 1, 100 })); },
                                          [] (const Component& c1, const Component& c2) { return c1.getExplicitFocusOrder()
                                                                                                   <= c2.getExplicitFocusOrder(); });
            }

            beginTest ("Left to right");
            {
                checkComponentProperties ([this] (Component& child) { child.setTopLeftPosition (getRandom().nextInt ({ 0, 100 }), 0); },
                                          [] (const Component& c1, const Component& c2) { return c1.getX() <= c2.getX(); });
            }

            beginTest ("Top to bottom");
            {
                checkComponentProperties ([this] (Component& child) { child.setTopLeftPosition (0, getRandom().nextInt ({ 0, 100 })); },
                                          [] (const Component& c1, const Component& c2) { return c1.getY() <= c2.getY(); });
            }

            beginTest ("Focus containers have their own focus");
            {
                Component root;

                TestComponent container;
                container.setFocusContainerType (Component::FocusContainerType::focusContainer);

                root.addAndMakeVisible (container);

                expect (traverser.getDefaultComponent (&root) == &container);
                expect (traverser.getNextComponent (&container) == nullptr);
                expect (traverser.getPreviousComponent (&container) == nullptr);

                expect (traverser.getDefaultComponent (&container) == &container.children.front());

                for (auto iter = container.children.begin(); iter != container.children.end(); ++iter)
                    expect (traverser.getNextComponent (&(*iter)) == (iter == std::prev (container.children.cend()) ? nullptr
                                                                                                                    : &(*std::next (iter))));

                for (auto iter = container.children.rbegin(); iter != container.children.rend(); ++iter)
                    expect (traverser.getPreviousComponent (&(*iter)) == (iter == std::prev (container.children.rend()) ? nullptr
                                                                                                                        : &(*std::next (iter))));

                expect (traverser.getAllComponents (&root).size() == 1);

                auto allContainerComponents = traverser.getAllComponents (&container);

                expect (std::equal (allContainerComponents.cbegin(), allContainerComponents.cend(), container.children.cbegin(),
                                    [] (const Component* c1, const Component& c2) { return c1 == &c2; }));
            }

            beginTest ("Non-focus containers pass-through focus");
            {
                Component root;

                TestComponent container;
                container.setFocusContainerType (Component::FocusContainerType::none);

                root.addAndMakeVisible (container);

                expect (traverser.getDefaultComponent (&root) == &container);
                expect (traverser.getNextComponent (&container) == &container.children.front());
                expect (traverser.getPreviousComponent (&container) == nullptr);

                expect (traverser.getDefaultComponent (&container) == &container.children.front());

                for (auto iter = container.children.begin(); iter != container.children.end(); ++iter)
                    expect (traverser.getNextComponent (&(*iter)) == (iter == std::prev (container.children.cend()) ? nullptr
                                                                                                                    : &(*std::next (iter))));

                for (auto iter = container.children.rbegin(); iter != container.children.rend(); ++iter)
                    expect (traverser.getPreviousComponent (&(*iter)) == (iter == std::prev (container.children.rend()) ? &container
                                                                                                                        : &(*std::next (iter))));

                expect (traverser.getAllComponents (&root).size() == container.children.size() + 1);
            }
        */
    }
    
    pub fn check_component_properties(&mut self, 
        child_fn:      fn(_0: &mut Component<'a>) -> (),
        test_property: fn(_0: &Component<'a>, _1: &Component<'a>) -> bool)  {
        
        todo!();
        /*
            TestComponent parent;

            for (auto& child : parent.children)
                childFn (child);

            auto* comp = traverser.getDefaultComponent (&parent);

            for (const auto& child : parent.children)
                if (&child != comp)
                    expect (testProperty (*comp, child));

            for (;;)
            {
                auto* next = traverser.getNextComponent (comp);

                if (next == nullptr)
                    break;

                expect (testProperty (*comp, *next));
                comp = next;
            }
        */
    }
    
    pub fn check_ignored(&mut self, make_ignored: &fn(_0: &mut Component<'a>) -> ())  {
        
        todo!();
        /*
            TestComponent parent;

            auto iter = parent.children.begin();

            makeIgnored (*iter);
            expect (traverser.getDefaultComponent (&parent) == std::addressof (*std::next (iter)));

            iter += 5;
            makeIgnored (*iter);
            expect (traverser.getNextComponent (std::addressof (*std::prev (iter))) == std::addressof (*std::next (iter)));
            expect (traverser.getPreviousComponent (std::addressof (*std::next (iter))) == std::addressof (*std::prev (iter)));

            auto allComponents = traverser.getAllComponents (&parent);

            expect (std::find (allComponents.cbegin(), allComponents.cend(), &parent.children.front()) == allComponents.cend());
            expect (std::find (allComponents.cbegin(), allComponents.cend(), std::addressof (*iter)) == allComponents.cend());
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static FocusTraverserTests focusTraverserTests;
    */
}

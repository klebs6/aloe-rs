crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct KeyboardFocusTraverserTests {
    base:      UnitTest,
    traverser: KeyboardFocusTraverser,
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod keyboard_focus_traverser_tests {
    use super::*;

    pub struct TestComponent<'a> {
        base:     Component<'a>,
        children: Vec<Component,10>,
    }

    impl<'a> Default for TestComponent<'a> {
        
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
impl Default for KeyboardFocusTraverserTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("KeyboardFocusTraverser", UnitTestCategories::gui
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl KeyboardFocusTraverserTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            ScopedAloeInitialiser_GUI libraryInitialiser;

            beginTest ("No child wants keyboard focus");
            {
                TestComponent parent;

                expect (traverser.getDefaultComponent (&parent) == nullptr);
                expect (traverser.getAllComponents (&parent).empty());
            }

            beginTest ("Single child wants keyboard focus");
            {
                TestComponent parent;

                parent.children[5].setWantsKeyboardFocus (true);

                auto* defaultComponent = traverser.getDefaultComponent (&parent);

                expect (defaultComponent == &parent.children[5]);
                expect (defaultComponent->getWantsKeyboardFocus());

                expect (traverser.getNextComponent (defaultComponent) == nullptr);
                expect (traverser.getPreviousComponent (defaultComponent) == nullptr);
                expect (traverser.getAllComponents (&parent).size() == 1);
            }

            beginTest ("Multiple children want keyboard focus");
            {
                TestComponent parent;

                Component* focusChildren[]
                {
                    &parent.children[1],
                    &parent.children[9],
                    &parent.children[3],
                    &parent.children[5],
                    &parent.children[8],
                    &parent.children[0]
                };

                for (auto* focusChild : focusChildren)
                    focusChild->setWantsKeyboardFocus (true);

                auto allComponents = traverser.getAllComponents (&parent);

                for (auto* focusChild : focusChildren)
                    expect (std::find (allComponents.cbegin(), allComponents.cend(), focusChild) != allComponents.cend());

                auto* componentToTest = traverser.getDefaultComponent (&parent);

                for (;;)
                {
                    expect (componentToTest->getWantsKeyboardFocus());
                    expect (std::find (std::begin (focusChildren), std::end (focusChildren), componentToTest) != std::end (focusChildren));

                    componentToTest = traverser.getNextComponent (componentToTest);

                    if (componentToTest == nullptr)
                        break;
                }

                int focusOrder = 1;
                for (auto* focusChild : focusChildren)
                    focusChild->setExplicitFocusOrder (focusOrder++);

                componentToTest = traverser.getDefaultComponent (&parent);

                for (auto* focusChild : focusChildren)
                {
                    expect (componentToTest == focusChild);
                    expect (componentToTest->getWantsKeyboardFocus());

                    componentToTest = traverser.getNextComponent (componentToTest);
                }
            }

            beginTest ("Single nested child wants keyboard focus");
            {
                TestComponent parent;
                Component grandparent;

                grandparent.addAndMakeVisible (parent);

                auto& focusChild = parent.children[5];

                focusChild.setWantsKeyboardFocus (true);

                expect (traverser.getDefaultComponent (&grandparent) == &focusChild);
                expect (traverser.getDefaultComponent (&parent) == &focusChild);
                expect (traverser.getNextComponent (&focusChild) == nullptr);
                expect (traverser.getPreviousComponent (&focusChild) == nullptr);
                expect (traverser.getAllComponents (&parent).size() == 1);
            }

            beginTest ("Multiple nested children want keyboard focus");
            {
                TestComponent parent;
                Component grandparent;

                grandparent.addAndMakeVisible (parent);

                Component* focusChildren[]
                {
                    &parent.children[1],
                    &parent.children[4],
                    &parent.children[5]
                };

                for (auto* focusChild : focusChildren)
                    focusChild->setWantsKeyboardFocus (true);

                auto allComponents = traverser.getAllComponents (&parent);

                expect (std::equal (allComponents.cbegin(), allComponents.cend(), focusChildren,
                                    [] (const Component* c1, const Component* c2) { return c1 == c2; }));

                const auto front = *focusChildren;
                const auto back  = *std::prev (std::end (focusChildren));

                expect (traverser.getDefaultComponent (&grandparent) == front);
                expect (traverser.getDefaultComponent (&parent) == front);
                expect (traverser.getNextComponent (front) == *std::next (std::begin (focusChildren)));
                expect (traverser.getPreviousComponent (back) == *std::prev (std::end (focusChildren), 2));

                std::array<Component, 3> otherParents;

                for (auto& p : otherParents)
                {
                    grandparent.addAndMakeVisible (p);
                    p.setWantsKeyboardFocus (true);
                }

                expect (traverser.getDefaultComponent (&grandparent) == front);
                expect (traverser.getDefaultComponent (&parent) == front);
                expect (traverser.getNextComponent (back) == &otherParents.front());
                expect (traverser.getNextComponent (&otherParents.back()) == nullptr);
                expect (traverser.getAllComponents (&grandparent).size() == numElementsInArray (focusChildren) + otherParents.size());
                expect (traverser.getAllComponents (&parent).size() == (size_t) numElementsInArray (focusChildren));

                for (auto* focusChild : focusChildren)
                    focusChild->setWantsKeyboardFocus (false);

                expect (traverser.getDefaultComponent (&grandparent) == &otherParents.front());
                expect (traverser.getDefaultComponent (&parent) == nullptr);
                expect (traverser.getAllComponents (&grandparent).size() == otherParents.size());
                expect (traverser.getAllComponents (&parent).empty());
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static KeyboardFocusTraverserTests keyboardFocusTraverserTests;
    */
}

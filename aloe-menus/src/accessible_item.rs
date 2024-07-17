crate::ix!();

pub struct MenuBarComponentAccessibleItemComponent<'a> {
    base:  Component<'a>,
    owner: &'a mut MenuBarComponent<'a>,
    name:  String,
}

impl<'a> MenuBarComponentAccessibleItemComponent<'a> {

    pub fn new(
        comp:           &mut MenuBarComponent,
        menu_item_name: &String) -> Self {
    
        todo!();
        /*
        : owner(comp),
        : name(menuItemName),

            setInterceptsMouseClicks (false, false);
        */
    }
    
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class ComponentHandler  : public AccessibilityHandler
                {
                
                    explicit ComponentHandler (MenuBarComponentAccessibleItemComponent& item)
                        : AccessibilityHandler (item,
                                                AccessibilityRole::menuItem,
                                                getAccessibilityActions (item)),
                          itemComponent (item)
                    {
                    }

                    String getTitle() const override  { return itemComponent.name; }

                
                    static AccessibilityActions getAccessibilityActions (MenuBarComponentAccessibleItemComponent& item)
                    {
                        auto showMenu = [&item] { item.owner.showMenu (item.owner.indexOfItemComponent (&item)); };

                        return AccessibilityActions().addAction (AccessibilityActionType::focus,
                                                                 [&item] { item.owner.setItemUnderMouse (item.owner.indexOfItemComponent (&item)); })
                                                     .addAction (AccessibilityActionType::press,    showMenu)
                                                     .addAction (AccessibilityActionType::showMenu, showMenu);
                    }

                    MenuBarComponentAccessibleItemComponent& itemComponent;
                };

                return std::make_unique<ComponentHandler> (*this);
        */
    }
}


crate::ix!();

pub struct AloeVst3EditControllerEditorContextMenu {
    context_menu: VstComSmartPtr<VstIContextMenu>,
}

impl HostProvidedContextMenu for AloeVst3EditControllerEditorContextMenu {

    fn get_equivalent_popup_menu(&self) -> aloe_menus::PopupMenu<'_> { todo!() }

    fn show_native_menu(&self, _: aloe_geometry::Point<i32>) { todo!() }
}

impl AloeVst3EditControllerEditorContextMenu {

    pub fn new(context_menu_in: VstComSmartPtr<VstIContextMenu>) -> Self {
    
        todo!();
        /*
        : context_menu(contextMenuIn),

        
        */
    }
    
    pub fn get_equivalent_popup_menu(&self) -> PopupMenu {
        
        todo!();
        /*
            using MenuItem   = VstIContextMenuItem;
                using MenuTarget = VstIContextMenuTarget;

                struct Submenu
                {
                    PopupMenu menu;
                    String name;
                    bool enabled;
                };

                std::vector<Submenu> menuStack (1);

                for (int32_t i = 0, end = contextMenu->getItemCount(); i < end; ++i)
                {
                    MenuItem item{};
                    MenuTarget* target = nullptr;
                    contextMenu->getItem (i, item, &target);

                    if ((item.flags & MenuItem::kIsGroupStart) == MenuItem::kIsGroupStart)
                    {
                        menuStack.push_back ({ PopupMenu{},
                                               toString (item.name),
                                               (item.flags & MenuItem::kIsDisabled) == 0 });
                    }
                    else if ((item.flags & MenuItem::kIsGroupEnd) == MenuItem::kIsGroupEnd)
                    {
                        const auto back = menuStack.back();
                        menuStack.pop_back();

                        if (menuStack.empty())
                        {
                            // malformed menu
                            jassertfalse;
                            return {};
                        }

                        menuStack.back().menu.addSubMenu (back.name, back.menu, back.enabled);
                    }
                    else if ((item.flags & MenuItem::kIsSeparator) == MenuItem::kIsSeparator)
                    {
                        menuStack.back().menu.addSeparator();
                    }
                    else
                    {
                        VSTComSmartPtr<MenuTarget> ownedTarget (target);
                        const auto tag = item.tag;
                        menuStack.back().menu.addItem (toString (item.name),
                                                       (item.flags & MenuItem::kIsDisabled) == 0,
                                                       (item.flags & MenuItem::kIsChecked) != 0,
                                                       [ownedTarget, tag] { ownedTarget->executeMenuItem (tag); });
                    }
                }

                if (menuStack.size() != 1)
                {
                    // malformed menu
                    jassertfalse;
                    return {};
                }

                return menuStack.back().menu;
        */
    }
    
    pub fn show_native_menu(&self, pos: Point<i32>)  {
        
        todo!();
        /*
            contextMenu->popup (pos.x, pos.y);
        */
    }
}

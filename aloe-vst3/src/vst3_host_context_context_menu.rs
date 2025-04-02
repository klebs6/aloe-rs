crate::ix!();

pub const Vst3_HOST_CONTEXT_MENU_ZERO_TAG_REPLACEMENT: usize = 0x7fffffff;

pub struct Vst3HostContextMenuItemAndTarget<Item>
{
    item:   Item,
    target: VstComSmartPtr<Box<dyn IContextMenuTarget>>,
}

#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[ALOE_DECLARE_Vst3_COM_QUERY_METHODS]
#[no_copy]
#[leak_detector]
pub struct Vst3HostContextContextMenu<'a, Item> {
    ref_count: Atomic<i32>,
    owner:     &'a mut Vst3PluginInstance<'a>,
    items:     Vec<Vst3HostContextMenuItemAndTarget<Item>>,
}

impl<'a, Item> IContextMenu for Vst3HostContextContextMenu<'a, Item> {

    fn get_item_count(&mut self) -> i32 { todo!() }

    fn get_item(&mut self, _: i32, _: &mut <Self as aloe_vst_edit::IContextMenu>::Item, _: *mut *mut (dyn aloe_vst_edit::IContextMenuTarget + 'static)) -> i32 { todo!() }

    fn add_item(&mut self, _: &<Self as aloe_vst_edit::IContextMenu>::Item, _: *mut (dyn aloe_vst_edit::IContextMenuTarget + 'static)) -> i32 { todo!() }

    fn remove_item(&mut self, _: &<Self as aloe_vst_edit::IContextMenu>::Item, _: *mut (dyn aloe_vst_edit::IContextMenuTarget + 'static)) -> i32 { todo!() }

    fn popup(&mut self, _: i32, _: i32) -> i32 { todo!() }
}

impl<'a, Item> FUnknown for Vst3HostContextContextMenu<'a, Item> {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl<'a, Item> Vst3HostContextContextMenu<'a, Item> {

    pub fn new(plugin_instance: &mut Vst3PluginInstance) -> Self {
    
        todo!();
        /*
        : owner(pluginInstance),

        
        */
    }
    
    pub fn get_item_count(&mut self) -> i32 {
        
        todo!();
        /*
            return (i32) items.size();
        */
    }
    
    pub fn add_item(&mut self, 
        item:   &Item,
        target: *mut dyn IContextMenuTarget) -> tresult {
        
        todo!();
        /*
            jassert (target != nullptr);

                Vst3HostContextMenuItemAndTarget newItem;
                newItem.item = item;
                newItem.target = target;

                items.add (newItem);
                return kResultOk;
        */
    }
    
    pub fn remove_item(&mut self, 
        to_remove: &Item,
        target:    *mut dyn IContextMenuTarget) -> tresult {
        
        todo!();
        /*
            for (int i = items.size(); --i >= 0;)
                {
                    auto& item = items.getReference(i);

                    if (item.item.tag == toRemove.tag && item.target == target)
                        items.remove (i);
                }

                return kResultOk;
        */
    }
    
    pub fn get_item(&mut self, 
        tag:    i32,
        result: &mut Item,
        target: *mut *mut dyn IContextMenuTarget) -> tresult {
        
        todo!();
        /*
            for (int i = 0; i < items.size(); ++i)
                {
                    auto& item = items.getReference(i);

                    if (item.item.tag == tag)
                    {
                        result = item.item;

                        if (target != nullptr)
                            *target = item.target;

                        return kResultTrue;
                    }
                }

                zerostruct (result);
                return kResultFalse;
        */
    }
    
    #[cfg(not(ALOE_MODAL_LOOPS_PERMITTED))]
    pub fn menu_finished(
        modal_result: i32,
        menu:         VstComSmartPtr<Vst3HostContextContextMenu<Item>>)  {
        
        todo!();
        /*
            menu->handleResult (modalResult);
        */
    }
    
    pub fn handle_result(&mut self, result: i32)  {
        
        todo!();
        /*
            if (result == 0)
                    return;

                if (result == zeroTagReplacement)
                    result = 0;

                for (int i = 0; i < items.size(); ++i)
                {
                    auto& item = items.getReference(i);

                    if ((int) item.item.tag == result)
                    {
                        if (item.target != nullptr)
                            item.target->executeMenuItem ((i32) result);

                        break;
                    }
                }
        */
    }
    
    pub fn popup(&mut self, 
        x: UCoord,
        y: UCoord) -> tresult {
        
        todo!();
        /*
            Vec<const Item*> subItemStack;
        Vec<Box<PopupMenu>> menuStack;
        PopupMenu* topLevelMenu = menuStack.add (new PopupMenu());

        for (int i = 0; i < items.size(); ++i)
        {
            auto& item = items.getReference (i).item;
            auto* menuToUse = menuStack.getLast();

            if (hasFlag (item.flags, Item::kIsGroupStart & ~Item::kIsDisabled))
            {
                subItemStack.add (&item);
                menuStack.add (new PopupMenu());
            }
            else if (hasFlag (item.flags, Item::kIsGroupEnd))
            {
                if (auto* subItem = subItemStack.getLast())
                {
                    if (auto* m = menuStack [menuStack.size() - 2])
                        m->addSubMenu (toString (subItem->name), *menuToUse,
                                       ! hasFlag (subItem->flags, Item::kIsDisabled),
                                       nullptr,
                                       hasFlag (subItem->flags, Item::kIsChecked));

                    menuStack.removeLast (1);
                    subItemStack.removeLast (1);
                }
            }
            else if (hasFlag (item.flags, Item::kIsSeparator))
            {
                menuToUse->addSeparator();
            }
            else
            {
                menuToUse->addItem (item.tag != 0 ? (int) item.tag : (int) zeroTagReplacement,
                                    toString (item.name),
                                    ! hasFlag (item.flags, Item::kIsDisabled),
                                    hasFlag (item.flags, Item::kIsChecked));
            }
        }

        PopupMenu::Options options;

        if (auto* ed = owner.getActiveEditor())
        {
           #if ALOE_WINDOWS && ALOE_WIN_PER_MONITOR_DPI_AWARE
            if (auto* peer = ed->getPeer())
            {
                auto scale = peer->getPlatformScaleFactor();

                x = roundToInt (x / scale);
                y = roundToInt (y / scale);
            }
           #endif

            options = options.withTargetScreenArea (ed->getScreenBounds().translated ((int) x, (int) y).withSize (1, 1));
        }

       #if ALOE_MODAL_LOOPS_PERMITTED
        // Unfortunately, Steinberg's docs explicitly say this should be modal..
        handleResult (topLevelMenu->showMenu (options));
       #else
        topLevelMenu->showMenuAsync (options, ModalCallbackFunction::create (menuFinished, VstComSmartPtr<Vst3HostContextContextMenu> (this)));
       #endif

        return kResultOk;
        */
    }
}

crate::ix!();

#[cfg(target_os="ios")]
pub struct PopoverDelegateClass {
    base: ObjCClass<NSObject<UIPopoverPresentationControllerDelegate>>,
}

#[cfg(target_os="ios")]
impl Default for PopoverDelegateClass {
    
    fn default() -> Self {
        todo!();
        /*

            : ObjCClass<NSObject<UIPopoverPresentationControllerDelegate>> ("PopoverDelegateClass_")
                    addMethod (@selector (popoverPresentationController:willRepositionPopoverToRect:inView:), willRepositionPopover, "v@:@@@");

                    registerClass()
        */
    }
}

#[cfg(target_os="ios")]
impl PopoverDelegateClass {

    pub fn will_reposition_popover(
        _0:   id,
        _1:   SEL,
        _2:   *mut UIPopoverPresentationController,
        rect: *mut CGRect,
        _4:   *mut UIView)  {
        
        todo!();
        /*
            auto screenBounds = [UIScreen mainScreen].bounds;

                    rect->origin.x = 0.f;
                    rect->origin.y = screenBounds.size.height - 10.f;
                    rect->size.width = screenBounds.size.width;
                    rect->size.height = 10.f;
        */
    }
}


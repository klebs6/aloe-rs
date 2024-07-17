crate::ix!();

pub const ANDROID_COMPONENT_PEER_GRAVITY_LEFT:                         i32 = 0x3;
pub const ANDROID_COMPONENT_PEER_GRAVITY_TOP:                          i32 = 0x30;
pub const ANDROID_COMPONENT_PEER_TYPE_APPLICATION:                     i32 = 0x2;
pub const ANDROID_COMPONENT_PEER_FLAG_NOT_TOUCH_MODAL:                 i32 = 0x20;
pub const ANDROID_COMPONENT_PEER_FLAG_LAYOUT_IN_SCREEN:                i32 = 0x100;
pub const ANDROID_COMPONENT_PEER_FLAG_LAYOUT_NO_LIMITS:                i32 = 0x200;
pub const ANDROID_COMPONENT_PEER_PIXEL_FORMAT_OPAQUE:                  i32 = -1;
pub const ANDROID_COMPONENT_PEER_PIXEL_FORMAT_TRANSPARENT:             i32 = -2;
pub const ANDROID_COMPONENT_PEER_LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS: i32 = 0x3;

///------------------
#[no_copy]
#[leak_detector]
#[cfg(target_os="android")]
pub struct AndroidComponentPeer<'a> {
    base:                 ComponentPeer<'a>,
    base2:                Timer,
    view:                 GlobalRef,
    view_group:           GlobalRef,
    buffer:               GlobalRef,
    view_group_is_window: bool, // default = false
    full_screen:          bool, // default = false
    nav_bars_hidden:      bool, // default = false
    size_allocated:       i32, // default = 0
    scale:                f32, //= (float) Desktop::getInstance().getDisplays().getPrimaryDisplay()->scale;
}

#[cfg(target_os="android")]
pub mod android_component_peer_ {

    use super::*;

    lazy_static!{
        /*
        static AndroidComponentPeer* frontWindow;
            static GlobalRef activityCallbackListener;
        */
    }

    lazy_static!{
        /*
        Point<float> AndroidComponentPeer::lastMousePos;
        int64 AndroidComponentPeer::touchesDown = 0;
        AndroidComponentPeer* AndroidComponentPeer::frontWindow = nullptr;
        GlobalRef AndroidComponentPeer::activityCallbackListener;
        AndroidComponentPeer::ComponentPeerView_Class AndroidComponentPeer::ComponentPeerView;
        */
    }

    lazy_static!{
        /*
        static Point<float> lastMousePos;
            static int64 touchesDown;
        */
    }
}

#[cfg(target_os="android")]
impl<'a> Drop for AndroidComponentPeer<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            stopTimer();

            auto* env = getEnv();

            env->CallVoidMethod (view, ComponentPeerView.clear);
            frontWindow = nullptr;

            GlobalRef localView (view);
            GlobalRef localViewGroup (viewGroup);

            callOnMessageThread ([env, localView, localViewGroup]
            {
                if (env->IsInstanceOf (localViewGroup.get(), AndroidActivity))
                    env->CallVoidMethod (localViewGroup.get(), AndroidActivity.setContentView, nullptr);
                else
                    env->CallVoidMethod (localViewGroup.get(), AndroidViewManager.removeView, localView.get());
            });
         */
    }
}

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
             METHOD   (create,                           "<init>",                        "(Landroid/content/Context;ZJ)V") 
             METHOD   (clear,                            "clear",                         "()V") 
             METHOD   (setViewName,                      "setViewName",                   "(Ljava/lang/String;)V") 
             METHOD   (setVisible,                       "setVisible",                    "(Z)V") 
             METHOD   (isVisible,                        "isVisible",                     "()Z") 
             METHOD   (containsPoint,                    "containsPoint",                 "(II)Z") 
             METHOD   (showKeyboard,                     "showKeyboard",                  "(Ljava/lang/String;)V") 
             METHOD   (setSystemUiVisibilityCompat,      "setSystemUiVisibilityCompat",   "(I)V") 
             CALLBACK (handlePaintJni,                   "handlePaint",                   "(JLandroid/graphics/Canvas;Landroid/graphics/Paint;)V") 
             CALLBACK (handleMouseDownJni,               "handleMouseDown",               "(JIFFJ)V") 
             CALLBACK (handleMouseDragJni,               "handleMouseDrag",               "(JIFFJ)V") 
             CALLBACK (handleMouseUpJni,                 "handleMouseUp",                 "(JIFFJ)V") 
             CALLBACK (handleKeyDownJni,                 "handleKeyDown",                 "(JII)V") 
             CALLBACK (handleKeyUpJni,                   "handleKeyUp",                   "(JII)V") 
             CALLBACK (handleBackButtonJni,              "handleBackButton",              "(J)V") 
             CALLBACK (handleKeyboardHiddenJni,          "handleKeyboardHidden",          "(J)V") 
             CALLBACK (viewSizeChangedJni,               "viewSizeChanged",               "(J)V") 
             CALLBACK (focusChangedJni,                  "focusChanged",                  "(JZ)V") 
             CALLBACK (handleAppPausedJni,               "handleAppPaused",               "(J)V") 
             CALLBACK (handleAppResumedJni,              "handleAppResumed",              "(J)V") 
             CALLBACK (populateAccessibilityNodeInfoJni, "populateAccessibilityNodeInfo", "(JILandroid/view/accessibility/AccessibilityNodeInfo;)Z") 
             CALLBACK (handlePerformActionJni,           "handlePerformAction",           "(JIILandroid/os/Bundle;)Z") 
             CALLBACK (getInputFocusViewIdJni,           "getInputFocusViewId",           "(J)Ljava/lang/Integer;") 
             CALLBACK (getAccessibilityFocusViewIdJni,   "getAccessibilityFocusViewId",   "(J)Ljava/lang/Integer;") 
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_bytecode!{
    ComponentPeerView, 
    "com/rmsl/aloe/ComponentPeerView", 
    16, 
    javaComponentPeerView, 
    sizeof (javaComponentPeerView)
}

// #undef JNI_CLASS_MEMBERS

#[cfg(target_os="android")]
impl<'a> AndroidComponentPeer<'a> {

    pub fn new(
        comp:               &mut Component<'a>,
        window_style_flags: i32,
        native_view_handle: *mut c_void) -> Self {
    
        todo!();
        /*
        : component_peer(comp, windowStyleFlags),

            auto* env = getEnv();

            // NB: must not put this in the initialiser list, as it invokes a callback,
            // which will fail if the peer is only half-constructed.
            view = GlobalRef (LocalRef<jobject> (env->NewObject (ComponentPeerView, ComponentPeerView.create,
                                                                 getAppContext().get(), (jboolean) component.isOpaque(),
                                                                 (jlong) this)));

            if (nativeViewHandle != nullptr)
            {
                viewGroupIsWindow = false;

                // we don't know if the user is holding on to a local ref to this, so
                // explicitly create a new one
                auto nativeView = LocalRef<jobject> (env->NewLocalRef (static_cast<jobject> (nativeViewHandle)));

                if (env->IsInstanceOf (nativeView.get(), AndroidActivity))
                {
                    viewGroup = GlobalRef (nativeView);
                    env->CallVoidMethod (viewGroup.get(), AndroidActivity.setContentView, view.get());
                }
                else if (env->IsInstanceOf (nativeView.get(), AndroidViewGroup))
                {
                    viewGroup = GlobalRef (nativeView);
                    LocalRef<jobject> layoutParams (env->NewObject (AndroidLayoutParams, AndroidLayoutParams.create, -2, -2));

                    env->CallVoidMethod (view.get(), AndroidView.setLayoutParams, layoutParams.get());
                    env->CallVoidMethod ((jobject) viewGroup.get(), AndroidViewGroup.addView, view.get());
                }
                else
                {
                    // the native handle you passed as a second argument to Component::addToDesktop must
                    // either be an Activity or a ViewGroup
                    jassertfalse;
                }
            }
            else
            {
                viewGroupIsWindow = true;

                LocalRef<jobject> viewLayoutParams (env->NewObject (AndroidLayoutParams, AndroidLayoutParams.create, -2, -2));
                env->CallVoidMethod (view.get(), AndroidView.setLayoutParams, viewLayoutParams.get());

                auto physicalBounds = (comp.getBoundsInParent().toFloat() * scale).toNearestInt();

                view.callVoidMethod (AndroidView.layout,
                                     physicalBounds.getX(), physicalBounds.getY(), physicalBounds.getRight(), physicalBounds.getBottom());

                LocalRef<jobject> windowLayoutParams (env->NewObject (AndroidWindowManagerLayoutParams, AndroidWindowManagerLayoutParams.create,
                                                                      physicalBounds.getWidth(), physicalBounds.getHeight(),
                                                                      physicalBounds.getX(), physicalBounds.getY(),
                                                                      TYPE_APPLICATION, FLAG_NOT_TOUCH_MODAL | FLAG_LAYOUT_IN_SCREEN | FLAG_LAYOUT_NO_LIMITS | FLAG_NOT_FOCUSABLE,
                                                                      component.isOpaque() ? PIXEL_FORMAT_OPAQUE : PIXEL_FORMAT_TRANSPARENT));

                env->SetIntField (windowLayoutParams.get(), AndroidWindowManagerLayoutParams.gravity, GRAVITY_LEFT | GRAVITY_TOP);
                env->SetIntField (windowLayoutParams.get(), AndroidWindowManagerLayoutParams.windowAnimations, 0x01030000 /* android.R.style.Animation */);

                if (supportsDisplayCutout())
                {
                    jfieldID layoutInDisplayCutoutModeFieldId = env->GetFieldID (AndroidWindowManagerLayoutParams,
                                                                                 "layoutInDisplayCutoutMode",
                                                                                 "I");

                    if (layoutInDisplayCutoutModeFieldId != nullptr)
                        env->SetIntField (windowLayoutParams.get(),
                                          layoutInDisplayCutoutModeFieldId,
                                          LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS);
                }

                if (Desktop::getInstance().getKioskModeComponent() != nullptr)
                    setNavBarsHidden (true);

                LocalRef<jobject> activity (getCurrentActivity());

                if (activity == nullptr)
                    activity = getMainActivity();

                viewGroup = GlobalRef (LocalRef<jobject> (env->CallObjectMethod (activity.get(), AndroidContext.getSystemService, javaString ("window").get())));
                env->CallVoidMethod (viewGroup.get(), AndroidViewManager.addView, view.get(), windowLayoutParams.get());
            }

            if (supportsDisplayCutout())
            {
                jmethodID setOnApplyWindowInsetsListenerMethodId = env->GetMethodID (AndroidView,
                                                                                     "setOnApplyWindowInsetsListener",
                                                                                     "(Landroid/view/View$OnApplyWindowInsetsListener;)V");

                if (setOnApplyWindowInsetsListenerMethodId != nullptr)
                    env->CallVoidMethod (view.get(), setOnApplyWindowInsetsListenerMethodId,
                                         CreateJavaInterface (new AndroidComponentPeerViewWindowInsetsListener,
                                                              "android/view/View$OnApplyWindowInsetsListener").get());
            }

            if (isFocused())
                handleFocusGain();
        */
    }
    
    pub fn get_native_handle(&self)  {
        
        todo!();
        /*
            return (void*) view.get();
        */
    }
    
    pub fn set_visible(&mut self, should_be_visible: bool)  {
        
        todo!();
        /*
            GlobalRef localView (view);

            callOnMessageThread ([localView, shouldBeVisible]
            {
                localView.callVoidMethod (ComponentPeerView.setVisible, shouldBeVisible);
            });
        */
    }
    
    pub fn set_title(&mut self, title: &String)  {
        
        todo!();
        /*
            view.callVoidMethod (ComponentPeerView.setViewName, javaString (title).get());
        */
    }
    
    pub fn set_bounds(&mut self, 
        user_rect:          &Rectangle<i32>,
        is_now_full_screen: bool)  {
        
        todo!();
        /*
            auto bounds = (userRect.toFloat() * scale).toNearestInt();

            if (MessageManager::getInstance()->isThisTheMessageThread())
            {
                fullScreen = isNowFullScreen;

                view.callVoidMethod (AndroidView.layout,
                                     bounds.getX(), bounds.getY(), bounds.getRight(), bounds.getBottom());

                if (viewGroup != nullptr && viewGroupIsWindow)
                {
                    auto* env = getEnv();

                    LocalRef<jobject> windowLayoutParams (env->NewObject (AndroidWindowManagerLayoutParams, AndroidWindowManagerLayoutParams.create,
                                                                          bounds.getWidth(), bounds.getHeight(), bounds.getX(), bounds.getY(),
                                                                          TYPE_APPLICATION, FLAG_NOT_TOUCH_MODAL | FLAG_LAYOUT_IN_SCREEN | FLAG_LAYOUT_NO_LIMITS,
                                                                          component.isOpaque() ? PIXEL_FORMAT_OPAQUE : PIXEL_FORMAT_TRANSPARENT));

                    env->SetIntField (windowLayoutParams.get(), AndroidWindowManagerLayoutParams.gravity, GRAVITY_LEFT | GRAVITY_TOP);
                    env->CallVoidMethod (viewGroup.get(), AndroidViewManager.updateViewLayout, view.get(), windowLayoutParams.get());
                }
            }
            else
            {
                GlobalRef localView (view);

                MessageManager::callAsync ([localView, bounds]
                {
                    localView.callVoidMethod (AndroidView.layout,
                                              bounds.getX(), bounds.getY(), bounds.getRight(), bounds.getBottom());
                });
            }
        */
    }
    
    pub fn get_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            Rectangle<int> bounds (view.callIntMethod (AndroidView.getLeft),
                                   view.callIntMethod (AndroidView.getTop),
                                   view.callIntMethod (AndroidView.getWidth),
                                   view.callIntMethod (AndroidView.getHeight));

            return (bounds.toFloat() / scale).toNearestInt();
        */
    }
    
    pub fn handle_screen_size_change(&mut self)  {
        
        todo!();
        /*
            ComponentPeer::handleScreenSizeChange();

            if (isFullScreen())
                setFullScreen (true);
        */
    }
    
    pub fn get_screen_position(&self) -> Point<i32> {
        
        todo!();
        /*
            auto* env = getEnv();

            LocalRef<jintArray> position (env->NewIntArray (2));
            env->CallVoidMethod (view.get(), AndroidView.getLocationOnScreen, position.get());

            jint* const screenPosition = env->GetIntArrayElements (position.get(), nullptr);
            Point<int> pos (screenPosition[0], screenPosition[1]);
            env->ReleaseIntArrayElements (position.get(), screenPosition, 0);

            return pos;
        */
    }
    
    pub fn local_to_global(&mut self, relative_position: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            return relativePosition + (getScreenPosition().toFloat() / scale);
        */
    }
    
    pub fn global_to_local(&mut self, screen_position: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            return screenPosition - (getScreenPosition().toFloat() / scale);
        */
    }
    
    pub fn set_minimised(&mut self, should_be_minimised: bool)  {
        
        todo!();
        /*
            // n/a
        */
    }
    
    pub fn is_minimised(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn set_full_screen(&mut self, should_be_full_screen: bool)  {
        
        todo!();
        /*
            if (shouldNavBarsBeHidden (shouldBeFullScreen))
            {
                if (isTimerRunning())
                    return;

                startTimer (500);
            }
            else
            {
                setNavBarsHidden (false);
            }

            auto newBounds = [&]
            {
                if (navBarsHidden || shouldBeFullScreen)
                    if (auto* display = Desktop::getInstance().getDisplays().getPrimaryDisplay())
                        return navBarsHidden ? display->totalArea
                                             : display->userArea;

                return lastNonFullscreenBounds.isEmpty() ? getBounds() : lastNonFullscreenBounds;
            }();

            if (! newBounds.isEmpty())
                setBounds (newBounds, shouldBeFullScreen);

            component.repaint();
        */
    }
    
    pub fn is_full_screen(&self) -> bool {
        
        todo!();
        /*
            return fullScreen;
        */
    }
    
    pub fn set_icon(&mut self, new_icon: &Image)  {
        
        todo!();
        /*
            // n/a
        */
    }
    
    pub fn contains(&self, 
        local_pos:                Point<i32>,
        true_if_in_achild_window: bool) -> bool {
        
        todo!();
        /*
            return isPositiveAndBelow (localPos.x, component.getWidth())
                && isPositiveAndBelow (localPos.y, component.getHeight())
                && ((! trueIfInAChildWindow) || view.callBooleanMethod (ComponentPeerView.containsPoint,
                                                                        (float) localPos.x * scale,
                                                                        (float) localPos.y * scale));
        */
    }
    
    pub fn get_frame_size(&self) -> BorderSize<i32> {
        
        todo!();
        /*
            // TODO
            return {};
        */
    }
    
    pub fn set_always_on_top(&mut self, always_on_top: bool) -> bool {
        
        todo!();
        /*
            // TODO
            return false;
        */
    }
    
    pub fn to_front(&mut self, make_active: bool)  {
        
        todo!();
        /*
            // Avoid calling bringToFront excessively: it's very slow
            if (frontWindow != this)
            {
                view.callVoidMethod (AndroidView.bringToFront);
                frontWindow = this;
            }

            if (makeActive)
                grabFocus();

            handleBroughtToFront();
        */
    }
    
    pub fn to_behind(&mut self, _0: *mut ComponentPeer)  {
        
        todo!();
        /*
            // TODO
        */
    }
    
    pub fn handle_mouse_down_callback(&mut self, 
        index:   i32,
        sys_pos: Point<f32>,
        time:    i64)  {
        
        todo!();
        /*
            lastMousePos = sysPos / scale;
            auto pos = globalToLocal (lastMousePos);

            // this forces a mouse-enter/up event, in case for some reason we didn't get a mouse-up before.
            handleMouseEvent (MouseInputSource::InputSourceType::touch,
                              pos,
                              ModifierKeys::currentModifiers.withoutMouseButtons(),
                              MouseInputSource::invalidPressure,
                              MouseInputSource::invalidOrientation,
                              time,
                              {},
                              index);

            if (isValidPeer (this))
                handleMouseDragCallback (index, sysPos, time);
        */
    }
    
    pub fn handle_mouse_drag_callback(&mut self, 
        index:   i32,
        sys_pos: Point<f32>,
        time:    i64)  {
        
        todo!();
        /*
            lastMousePos = sysPos / scale;
            auto pos = globalToLocal (lastMousePos);

            jassert (index < 64);
            touchesDown = (touchesDown | (1 << (index & 63)));

            ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withoutMouseButtons().withFlags (ModifierKeys::leftButtonModifier);

            handleMouseEvent (MouseInputSource::InputSourceType::touch,
                              pos,
                              ModifierKeys::currentModifiers.withoutMouseButtons().withFlags (ModifierKeys::leftButtonModifier),
                              MouseInputSource::invalidPressure,
                              MouseInputSource::invalidOrientation,
                              time,
                              {},
                              index);
        */
    }
    
    pub fn handle_mouse_up_callback(&mut self, 
        index:   i32,
        sys_pos: Point<f32>,
        time:    i64)  {
        
        todo!();
        /*
            lastMousePos = sysPos / scale;
            auto pos = globalToLocal (lastMousePos);

            jassert (index < 64);
            touchesDown = (touchesDown & ~(1 << (index & 63)));

            if (touchesDown == 0)
                ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withoutMouseButtons();

            handleMouseEvent (MouseInputSource::InputSourceType::touch,
                              pos,
                              ModifierKeys::currentModifiers.withoutMouseButtons(),
                              MouseInputSource::invalidPressure,
                              MouseInputSource::invalidOrientation,
                              time,
                              {},
                              index);
        */
    }
    
    pub fn handle_key_down_callback(&mut self, k: i32, kc: i32)  {
        
        todo!();
        /*
            handleKeyPress (k, static_cast<aloe_wchar> (kc));
        */
    }
    
    pub fn handle_key_up_callback(&mut self, k: i32, kc: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn handle_back_button_callback(&mut self)  {
        
        todo!();
        /*
            bool handled = false;

            if (auto* app = ALOEApplicationBase::getInstance())
                handled = app->backButtonPressed();

            if (isKioskModeComponent())
                setNavBarsHidden (navBarsHidden);

            if (! handled)
            {
                auto* env = getEnv();
                LocalRef<jobject> activity (getCurrentActivity());

                if (activity != nullptr)
                {
                    jmethodID finishMethod = env->GetMethodID (AndroidActivity, "finish", "()V");

                    if (finishMethod != nullptr)
                        env->CallVoidMethod (activity.get(), finishMethod);
                }
            }
        */
    }
    
    pub fn handle_keyboard_hidden_callback(&mut self)  {
        
        todo!();
        /*
            Component::unfocusAllComponents();
        */
    }
    
    pub fn handle_app_paused_callback(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn handle_app_resumed_callback(&mut self)  {
        
        todo!();
        /*
            if (isKioskModeComponent())
                setNavBarsHidden (navBarsHidden);
        */
    }
    
    pub fn get_native_handle_for_view_id(&self, virtual_view_id: i32) -> *mut AccessibilityNativeHandle {
        
        todo!();
        /*
            if (auto* handler = (virtualViewId == HOST_VIEW_ID
                                     ? component.getAccessibilityHandler()
                                     : AccessibilityNativeHandle::getAccessibilityHandlerForVirtualViewId (virtualViewId)))
            {
                return handler->getNativeImplementation();
            }

            return nullptr;
        */
    }
    
    pub fn populate_accessibility_node_info_callback(&self, 
        virtual_view_id: i32,
        info:            jobject) -> bool {
        
        todo!();
        /*
            if (auto* handle = getNativeHandleForViewId (virtualViewId))
            {
                handle->populateNodeInfo (info);
                return true;
            }

            return false;
        */
    }
    
    pub fn handle_perform_action_callback(&self, 
        virtual_view_id: i32,
        action:          i32,
        arguments:       jobject) -> bool {
        
        todo!();
        /*
            if (auto* handle = getNativeHandleForViewId (virtualViewId))
                return handle->performAction (action, arguments);

            return false;
        */
    }
    
    pub fn get_focus_view_id_for_handler(handler: *const AccessibilityHandler) -> jobject {
        
        todo!();
        /*
            if (handler != nullptr)
                return getEnv()->NewObject (JavaInteger,
                                            JavaInteger.constructor,
                                            handler->getNativeImplementation()->getVirtualViewId());

            return nullptr;
        */
    }
    
    pub fn get_input_focus_view_id_callback(&mut self) -> jobject {
        
        todo!();
        /*
            if (auto* comp = dynamic_cast<Component*> (findCurrentTextInputTarget()))
                return getFocusViewIdForHandler (comp->getAccessibilityHandler());

            return nullptr;
        */
    }
    
    pub fn get_accessibility_focus_view_id_callback(&self) -> jobject {
        
        todo!();
        /*
            if (auto* handler = component.getAccessibilityHandler())
            {
                if (auto* modal = Component::getCurrentlyModalComponent())
                {
                    if (! component.isParentOf (modal)
                         && component.isCurrentlyBlockedByAnotherModalComponent())
                    {
                        if (auto* modalHandler = modal->getAccessibilityHandler())
                        {
                            if (auto* focusChild = modalHandler->getChildFocus())
                                return getFocusViewIdForHandler (focusChild);

                            return getFocusViewIdForHandler (modalHandler);
                        }
                    }
                }

                if (auto* focusChild = handler->getChildFocus())
                    return getFocusViewIdForHandler (focusChild);
            }

            return nullptr;
        */
    }
    
    pub fn is_focused(&self) -> bool {
        
        todo!();
        /*
            if (view != nullptr)
                return view.callBooleanMethod (AndroidView.hasFocus);

            return false;
        */
    }
    
    pub fn grab_focus(&mut self)  {
        
        todo!();
        /*
            if (view != nullptr)
                view.callBooleanMethod (AndroidView.requestFocus);
        */
    }
    
    pub fn handle_focus_change_callback(&mut self, has_focus: bool)  {
        
        todo!();
        /*
            if (isFullScreen())
                setFullScreen (true);

            if (hasFocus)
                handleFocusGain();
            else
                handleFocusLoss();
        */
    }
    
    pub fn get_virtual_keyboard_type(ty: VirtualKeyboardType) -> *const u8 {
        
        todo!();
        /*
            switch (type)
            {
                case TextInputTarget::textKeyboard:          return "text";
                case TextInputTarget::numericKeyboard:       return "number";
                case TextInputTarget::decimalKeyboard:       return "numberDecimal";
                case TextInputTarget::urlKeyboard:           return "textUri";
                case TextInputTarget::emailAddressKeyboard:  return "textEmailAddress";
                case TextInputTarget::phoneNumberKeyboard:   return "phone";
                default:                                     jassertfalse; break;
            }

            return "text";
        */
    }
    
    pub fn text_input_required(
        &mut self, 
        _0:     Point<i32>,
        target: &mut dyn TextInputTarget
    ) {
        
        todo!();
        /*
            view.callVoidMethod (ComponentPeerView.showKeyboard,
                                 javaString (getVirtualKeyboardType (target.getKeyboardType())).get());
        */
    }
    
    pub fn dismiss_pending_text_input(&mut self)  {
        
        todo!();
        /*
            view.callVoidMethod (ComponentPeerView.showKeyboard, javaString ("").get());

            if (! isTimerRunning())
                startTimer (500);
        */
    }
    
    pub fn handle_paint_callback(&mut self, 
        canvas: jobject,
        paint:  jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();

            jobject rect = env->CallObjectMethod (canvas, AndroidCanvas.getClipBounds);
            auto left   = env->GetIntField (rect, AndroidRect.left);
            auto top    = env->GetIntField (rect, AndroidRect.top);
            auto right  = env->GetIntField (rect, AndroidRect.right);
            auto bottom = env->GetIntField (rect, AndroidRect.bottom);
            env->DeleteLocalRef (rect);

            auto clip = Rectangle<int>::leftTopRightBottom (left, top, right, bottom);

            if (clip.isEmpty())
                return;

            auto sizeNeeded = clip.getWidth() * clip.getHeight();

            if (sizeAllocated < sizeNeeded)
            {
                buffer.clear();
                sizeAllocated = sizeNeeded;
                buffer = GlobalRef (LocalRef<jobject> ((jobject) env->NewIntArray (sizeNeeded)));
            }

            if (jint* dest = env->GetIntArrayElements ((jintArray) buffer.get(), nullptr))
            {
                {
                    Image temp (new AndroidComponentPeerPreallocatedImage (clip.getWidth(), clip.getHeight(),
                                                       dest, ! component.isOpaque()));

                    {
                        LowLevelGraphicsSoftwareRenderer g (temp);
                        g.setOrigin (-clip.getPosition());
                        g.addTransform (AffineTransform::scale (scale));
                        handlePaint (g);
                    }
                }

                env->ReleaseIntArrayElements ((jintArray) buffer.get(), dest, 0);

                env->CallVoidMethod (canvas, AndroidCanvas.drawBitmap, (jintArray) buffer.get(), 0, clip.getWidth(),
                                     (jfloat) clip.getX(), (jfloat) clip.getY(),
                                     clip.getWidth(), clip.getHeight(), true, paint);
            }
        */
    }
    
    pub fn repaint(&mut self, user_area: &Rectangle<i32>)  {
        
        todo!();
        /*
            auto area = (userArea.toFloat() * scale).toNearestInt();

            GlobalRef localView (view);

            callOnMessageThread ([area, localView]
            {
                localView.callVoidMethod (AndroidView.invalidate,
                                          area.getX(), area.getY(), area.getRight(), area.getBottom());
            });
        */
    }
    
    pub fn perform_any_pending_repaints_now(&mut self)  {
        
        todo!();
        /*
            // TODO
        */
    }
    
    pub fn set_alpha(&mut self, new_alpha: f32)  {
        
        todo!();
        /*
            // TODO
        */
    }
    
    pub fn get_available_rendering_engines(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return Vec<String> ("Software Renderer");
        */
    }

    #[JNICALL]
    pub fn handle_paint_jni(
        _0:     *mut JNIEnv,
        view:   jobject,
        host:   i64,
        canvas: jobject,
        paint:  jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handlePaintCallback (canvas, paint);
        */
    }
    
    #[JNICALL]
    pub fn handle_mouse_down_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64,
        i:    i32,
        x:    f32,
        y:    f32,
        time: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleMouseDownCallback (i, Point<float> ((float) x, (float) y), (int64) time);
        */
    }
    
    #[JNICALL]
    pub fn handle_mouse_drag_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64,
        i:    i32,
        x:    f32,
        y:    f32,
        time: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleMouseDragCallback (i, Point<float> ((float) x, (float) y), (int64) time);
        */
    }
    
    #[JNICALL]
    pub fn handle_mouse_up_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64,
        i:    i32,
        x:    f32,
        y:    f32,
        time: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleMouseUpCallback   (i, Point<float> ((float) x, (float) y), (int64) time);
        */
    }
    
    #[JNICALL]
    pub fn view_size_changed_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleMovedOrResized();
        */
    }
    
    #[JNICALL]
    pub fn focus_changed_jni(
        _0:        *mut JNIEnv,
        view:      jobject,
        host:      i64,
        has_focus: bool)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleFocusChangeCallback (hasFocus);
        */
    }
    
    #[JNICALL]
    pub fn handle_key_down_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64,
        k:    i32,
        kc:   i32)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleKeyDownCallback ((int) k, (int) kc);
        */
    }
    
    #[JNICALL]
    pub fn handle_key_up_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64,
        k:    i32,
        kc:   i32)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleKeyUpCallback ((int) k, (int) kc);
        */
    }
    
    #[JNICALL]
    pub fn handle_back_button_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleBackButtonCallback();
        */
    }
    
    #[JNICALL]
    pub fn handle_keyboard_hidden_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleKeyboardHiddenCallback();
        */
    }
    
    #[JNICALL]
    pub fn handle_app_paused_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleAppPausedCallback();
        */
    }
    
    #[JNICALL]
    pub fn handle_app_resumed_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host)) myself->handleAppResumedCallback();
        */
    }
    
    #[JNICALL]
    pub fn populate_accessibility_node_info_jni(
        _0:              *mut JNIEnv,
        view:            jobject,
        host:            i64,
        virtual_view_id: i32,
        info:            jobject) -> bool {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host))
                return myself->populateAccessibilityNodeInfoCallback (virtualViewId, info);

            return false;
        */
    }
    
    #[JNICALL]
    pub fn handle_perform_action_jni(
        _0:              *mut JNIEnv,
        view:            jobject,
        host:            i64,
        virtual_view_id: i32,
        action:          i32,
        arguments:       jobject) -> bool {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host))
                return myself->handlePerformActionCallback (virtualViewId, action, arguments);

            return false;
        */
    }
    
    #[JNICALL]
    pub fn get_input_focus_view_id_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64) -> jobject {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host))
                return myself->getInputFocusViewIdCallback();

            return nullptr;
        */
    }
    
    #[JNICALL]
    pub fn get_accessibility_focus_view_id_jni(
        _0:   *mut JNIEnv,
        view: jobject,
        host: i64) -> jobject {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidComponentPeer*> (host))
                return myself->getAccessibilityFocusViewIdCallback();

            return nullptr;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            setNavBarsHidden (shouldNavBarsBeHidden (fullScreen));
            setFullScreen (fullScreen);
            stopTimer();
        */
    }
    
    pub fn is_kiosk_mode_component(&self) -> bool {
        
        todo!();
        /*
            if (auto* kiosk = Desktop::getInstance().getKioskModeComponent())
                return kiosk->getPeer() == this;

            return false;
        */
    }
    
    pub fn should_nav_bars_be_hidden(&self, should_be_full_screen: bool) -> bool {
        
        todo!();
        /*
            return (shouldBeFullScreen && isKioskModeComponent());
        */
    }
    
    pub fn set_nav_bars_hidden(&mut self, hidden: bool)  {
        
        todo!();
        /*
            if (navBarsHidden != hidden)
            {
                navBarsHidden = hidden;

                view.callVoidMethod (ComponentPeerView.setSystemUiVisibilityCompat,
                                     (navBarsHidden ? (jint) (fullScreenFlags) : (jint) (SYSTEM_UI_FLAG_VISIBLE)));
            }
        */
    }
    
    pub fn call_on_message_thread<Callback>(callback: Callback)  {
    
        todo!();
        /*
            if (MessageManager::getInstance()->isThisTheMessageThread())
                callback();
            else
                MessageManager::callAsync (std::forward<Callback> (callback));
        */
    }
}

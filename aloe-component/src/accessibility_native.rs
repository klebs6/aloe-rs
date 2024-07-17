crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AccessibilityNativeHandle<'a> {

    accessibility_handler: &'a mut AccessibilityHandler<'a>,
    virtual_view_id:       i32,
    in_populate_node_info: bool, // default = false
}

pub mod accessibility_native_handle {

    use super::*;

    lazy_static!{
        /*
        static std::unordered_map<int, AccessibilityHandler*> virtualViewIdMap;
        std::unordered_map<int, AccessibilityHandler*> AccessibilityNativeHandle::virtualViewIdMap;
        */
    }
}

impl<'a> Drop for AccessibilityNativeHandle<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            if (virtualViewId != HOST_VIEW_ID)
                virtualViewIdMap.erase (virtualViewId);
         */
    }
}

impl<'a> AccessibilityNativeHandle<'a> {

    pub fn get_accessibility_handler_for_virtual_view_id(virtual_view_id: i32) 
        -> *mut AccessibilityHandler<'a> {
        
        todo!();
        /*
            auto iter = virtualViewIdMap.find (virtualViewId);

            if (iter != virtualViewIdMap.end())
                return iter->second;

            return nullptr;
        */
    }
    
    pub fn new(h: &mut AccessibilityHandler<'a>) -> Self {
    
        todo!();
        /*
        : accessibility_handler(h),
        : virtual_view_id(getVirtualViewIdForHandler (accessibilityHandler)),

            loadSDKDependentMethods();

            if (virtualViewId != HOST_VIEW_ID)
                virtualViewIdMap[virtualViewId] = &accessibilityHandler;
        */
    }
    
    pub fn get_virtual_view_id(&self) -> i32 {
        
        todo!();
        /*
            return virtualViewId;
        */
    }
    
    pub fn populate_node_info(&mut self, info: jobject)  {
        
        todo!();
        /*
            const ScopedValueSetter<bool> svs (inPopulateNodeInfo, true);

            const auto sourceView = getSourceView (accessibilityHandler);

            if (sourceView == nullptr)
                return;

            auto* env = getEnv();
            auto appContext = getAppContext();

            if (appContext.get() == nullptr)
                return;

            {
                for (auto* child : accessibilityHandler.getChildren())
                    env->CallVoidMethod (info, AndroidAccessibilityNodeInfo.addChild,
                                         sourceView, child->getNativeImplementation()->getVirtualViewId());

                if (auto* parent = accessibilityHandler.getParent())
                    env->CallVoidMethod (info, AndroidAccessibilityNodeInfo.setVirtualParent,
                                         sourceView, parent->getNativeImplementation()->getVirtualViewId());
                else
                    env->CallVoidMethod (info, AndroidAccessibilityNodeInfo.setParent, sourceView);
            }

            {
                const auto scale = Desktop::getInstance().getDisplays().getPrimaryDisplay()->scale;

                const auto screenBounds = accessibilityHandler.getComponent().getScreenBounds() * scale;

                LocalRef<jobject> rect (env->NewObject (AndroidRect, AndroidRect.constructor,
                                                        screenBounds.getX(),     screenBounds.getY(),
                                                        screenBounds.getRight(), screenBounds.getBottom()));

                env->CallVoidMethod (info, AndroidAccessibilityNodeInfo.setBoundsInScreen, rect.get());

                const auto boundsInParent = accessibilityHandler.getComponent().getBoundsInParent() * scale;

                rect = LocalRef<jobject> (env->NewObject (AndroidRect, AndroidRect.constructor,
                                                          boundsInParent.getX(),     boundsInParent.getY(),
                                                          boundsInParent.getRight(), boundsInParent.getBottom()));

                env->CallVoidMethod (info, AndroidAccessibilityNodeInfo.setBoundsInParent, rect.get());
            }

            const auto state = accessibilityHandler.getCurrentState();

            env->CallVoidMethod (info,
                                 AndroidAccessibilityNodeInfo.setEnabled,
                                 ! state.isIgnored());
            env->CallVoidMethod (info,
                                 AndroidAccessibilityNodeInfo.setVisibleToUser,
                                 true);
            env->CallVoidMethod (info,
                                 AndroidAccessibilityNodeInfo.setPackageName,
                                 env->CallObjectMethod (appContext.get(),
                                                        AndroidContext.getPackageName));
            env->CallVoidMethod (info,
                                 AndroidAccessibilityNodeInfo.setSource,
                                 sourceView,
                                 virtualViewId);
            env->CallVoidMethod (info,
                                 AndroidAccessibilityNodeInfo.setClassName,
                                 javaString (getClassName (accessibilityHandler.getRole())).get());
            env->CallVoidMethod (info,
                                 AndroidAccessibilityNodeInfo.setContentDescription,
                                 getDescriptionString().get());

            if (state.isFocusable())
            {
                env->CallVoidMethod (info, AndroidAccessibilityNodeInfo.setFocusable, true);

                const auto& component = accessibilityHandler.getComponent();

                if (component.getWantsKeyboardFocus())
                {
                    const auto hasKeyboardFocus = component.hasKeyboardFocus (false);

                    env->CallVoidMethod (info,
                                         AndroidAccessibilityNodeInfo.setFocused,
                                         hasKeyboardFocus);
                    env->CallVoidMethod (info,
                                         AndroidAccessibilityNodeInfo.addAction,
                                         hasKeyboardFocus ? ACTION_CLEAR_FOCUS : ACTION_FOCUS);
                }

                const auto isAccessibleFocused = accessibilityHandler.hasFocus (false);

                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setAccessibilityFocused,
                                     isAccessibleFocused);

                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.addAction,
                                     isAccessibleFocused ? ACTION_CLEAR_ACCESSIBILITY_FOCUS
                                                         : ACTION_ACCESSIBILITY_FOCUS);
            }

            if (state.isCheckable())
            {
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setCheckable,
                                     true);
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setChecked,
                                     state.isChecked());
            }

            if (state.isSelectable() || state.isMultiSelectable())
            {
                const auto isSelected = state.isSelected();

                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setSelected,
                                     isSelected);
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.addAction,
                                     isSelected ? ACTION_CLEAR_SELECTION : ACTION_SELECT);
            }

            if (accessibilityHandler.getActions().contains (AccessibilityActionType::press))
            {
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setClickable,
                                     true);
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.addAction,
                                     ACTION_CLICK);
            }

            if (accessibilityHandler.getActions().contains (AccessibilityActionType::showMenu)
                && state.isExpandable())
            {
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.addAction,
                                     state.isExpanded() ? ACTION_COLLAPSE : ACTION_EXPAND);
            }

            if (auto* textInterface = accessibilityHandler.getTextInterface())
            {
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setText,
                                     javaString (textInterface->getText ({ 0, textInterface->getTotalNumCharacters() })).get());

                const auto isReadOnly = textInterface->isReadOnly();

                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setPassword,
                                     textInterface->isDisplayingProtectedText());

                if (nodeInfoSetEditable != nullptr)
                    env->CallVoidMethod (info, nodeInfoSetEditable, ! isReadOnly);

                const auto selection = textInterface->getSelection();

                if (nodeInfoSetTextSelection != nullptr && ! selection.isEmpty())
                    env->CallVoidMethod (info,
                                         nodeInfoSetTextSelection,
                                         selection.getStart(), selection.getEnd());

                if (nodeInfoSetLiveRegion != nullptr && accessibilityHandler.hasFocus (false))
                    env->CallVoidMethod (info,
                                         nodeInfoSetLiveRegion,
                                         ACCESSIBILITY_LIVE_REGION_POLITE);

                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.setMovementGranularities,
                                     ALL_GRANULARITIES);

                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.addAction,
                                     ACTION_NEXT_AT_MOVEMENT_GRANULARITY);
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.addAction,
                                     ACTION_PREVIOUS_AT_MOVEMENT_GRANULARITY);
                env->CallVoidMethod (info,
                                     AndroidAccessibilityNodeInfo.addAction,
                                     ACTION_SET_SELECTION);

                if (! isReadOnly)
                    env->CallVoidMethod (info, AndroidAccessibilityNodeInfo.addAction, ACTION_SET_TEXT);
            }

            if (auto* valueInterface = accessibilityHandler.getValueInterface())
            {
                if (! valueInterface->isReadOnly())
                {
                    const auto range = valueInterface->getRange();

                    if (range.isValid())
                    {
                        env->CallVoidMethod (info,
                                             AndroidAccessibilityNodeInfo.addAction,
                                             ACTION_SCROLL_FORWARD);
                        env->CallVoidMethod (info,
                                             AndroidAccessibilityNodeInfo.addAction,
                                             ACTION_SCROLL_BACKWARD);
                    }
                }
            }
        */
    }
    
    pub fn perform_action(&mut self, 
        action:    i32,
        arguments: jobject) -> bool {
        
        todo!();
        /*
            switch (action)
            {
                case ACTION_ACCESSIBILITY_FOCUS:
                {
                    const WeakReference<Component> safeComponent (&accessibilityHandler.getComponent());

                    accessibilityHandler.getActions().invoke (AccessibilityActionType::focus);

                    if (safeComponent != nullptr)
                        accessibilityHandler.grabFocus();

                    return true;
                }

                case ACTION_CLEAR_ACCESSIBILITY_FOCUS:
                {
                    accessibilityHandler.giveAwayFocus();
                    return true;
                }

                case ACTION_FOCUS:
                case ACTION_CLEAR_FOCUS:
                {
                    auto& component = accessibilityHandler.getComponent();

                    if (component.getWantsKeyboardFocus())
                    {
                        const auto hasFocus = component.hasKeyboardFocus (false);

                        if (hasFocus && action == ACTION_CLEAR_FOCUS)
                            component.giveAwayKeyboardFocus();
                        else if (! hasFocus && action == ACTION_FOCUS)
                            component.grabKeyboardFocus();

                        return true;
                    }

                    break;
                }

                case ACTION_CLICK:
                {
                    if (accessibilityHandler.getActions().invoke (AccessibilityActionType::press))
                    {
                        sendAccessibilityEventImpl (accessibilityHandler, TYPE_VIEW_CLICKED, 0);
                        return true;
                    }

                    break;
                }

                case ACTION_SELECT:
                case ACTION_CLEAR_SELECTION:
                {
                    const auto state = accessibilityHandler.getCurrentState();

                    if (state.isSelectable() || state.isMultiSelectable())
                    {
                        const auto isSelected = state.isSelected();

                        if ((isSelected && action == ACTION_CLEAR_SELECTION)
                            || (! isSelected && action == ACTION_SELECT))
                        {
                            return accessibilityHandler.getActions().invoke (AccessibilityActionType::toggle);
                        }

                    }

                    break;
                }

                case ACTION_EXPAND:
                case ACTION_COLLAPSE:
                {
                    const auto state = accessibilityHandler.getCurrentState();

                    if (state.isExpandable())
                    {
                        const auto isExpanded = state.isExpanded();

                        if ((isExpanded && action == ACTION_COLLAPSE)
                            || (! isExpanded && action == ACTION_EXPAND))
                        {
                            return accessibilityHandler.getActions().invoke (AccessibilityActionType::showMenu);
                        }
                    }

                    break;
                }

                case ACTION_NEXT_AT_MOVEMENT_GRANULARITY:      return moveCursor (arguments, true);
                case ACTION_PREVIOUS_AT_MOVEMENT_GRANULARITY:  return moveCursor (arguments, false);

                case ACTION_SET_SELECTION:
                {
                    if (auto* textInterface = accessibilityHandler.getTextInterface())
                    {
                        auto* env = getEnv();

                        const auto selection = [&]() -> Range<int>
                        {
                            const auto selectionStartKey = javaString ("ACTION_ARGUMENT_SELECTION_START_INT");
                            const auto selectionEndKey   = javaString ("ACTION_ARGUMENT_SELECTION_END_INT");

                            const auto hasKey = [&env, &arguments] (const auto& key)
                            {
                                return env->CallBooleanMethod (arguments, AndroidBundle.containsKey, key.get());
                            };

                            if (hasKey (selectionStartKey) && hasKey (selectionEndKey))
                            {
                                const auto getKey = [&env, &arguments] (const auto& key)
                                {
                                    return env->CallIntMethod (arguments, AndroidBundle.getInt, key.get());
                                };

                                return { getKey (selectionStartKey), getKey (selectionEndKey) };
                            }

                            return {};
                        }();

                        textInterface->setSelection (selection);

                        return true;
                    }

                    break;
                }

                case ACTION_SET_TEXT:
                {
                    if (auto* textInterface = accessibilityHandler.getTextInterface())
                    {
                        if (! textInterface->isReadOnly())
                        {
                            const auto charSequenceKey = javaString ("ACTION_ARGUMENT_SET_TEXT_CHARSEQUENCE");

                            auto* env = getEnv();

                            const auto text = [&]() -> String
                            {
                                if (env->CallBooleanMethod (arguments, AndroidBundle.containsKey, charSequenceKey.get()))
                                {
                                    LocalRef<jobject> charSequence (env->CallObjectMethod (arguments,
                                                                                           AndroidBundle.getCharSequence,
                                                                                           charSequenceKey.get()));
                                    LocalRef<jstring> textStringRef ((jstring) env->CallObjectMethod (charSequence,
                                                                                                      JavaCharSequence.toString));

                                    return aloeString (textStringRef.get());
                                }

                                return {};
                            }();

                            textInterface->setText (text);
                        }
                    }

                    break;
                }

                case ACTION_SCROLL_BACKWARD:
                case ACTION_SCROLL_FORWARD:
                {
                    if (auto* valueInterface = accessibilityHandler.getValueInterface())
                    {
                        if (! valueInterface->isReadOnly())
                        {
                            const auto range = valueInterface->getRange();

                            if (range.isValid())
                            {
                                const auto interval = action == ACTION_SCROLL_BACKWARD ? -range.getInterval()
                                                                                       : range.getInterval();
                                valueInterface->setValue (jlimit (range.getMinimumValue(),
                                                                  range.getMaximumValue(),
                                                                  valueInterface->getCurrentValue() + interval));

                                // required for Android to announce the new value
                                sendAccessibilityEventImpl (accessibilityHandler, TYPE_VIEW_SELECTED, 0);
                                return true;
                            }
                        }
                    }

                    break;
                }
            }

            return false;
        */
    }
    
    pub fn is_in_populate_node_info(&self) -> bool {
        
        todo!();
        /*
            return inPopulateNodeInfo;
        */
    }
    
    pub fn get_virtual_view_id_for_handler(handler: &AccessibilityHandler<'a>) -> i32 {
        
        todo!();
        /*
            static int counter = 0;

            if (handler.getComponent().isOnDesktop())
                return HOST_VIEW_ID;

            return counter++;
        */
    }
}

pub struct AccessibilityNativeImpl<'a> {
    base: AccessibilityNativeHandle<'a>,
}

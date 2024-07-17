crate::ix!();

#[cfg(target_os="android")]
pub trait AndroidAccessibilityGetDescriptionString {

    fn get_description_string(&self) -> LocalRef<jstring>;
}

#[cfg(target_os="android")]
pub trait AndroidAccessibilityMoveCursor {

    fn move_cursor(
        &mut self, 
        arguments: jobject,
        forwards:  bool) -> bool;
}

#[cfg(target_os="android")]
impl<'a> AndroidAccessibilityMoveCursor for AccessibilityNativeHandle<'a> {

    fn move_cursor(
        &mut self, 
        arguments: jobject,
        forwards:  bool) -> bool {
        
        todo!();
        /*
            if (auto* textInterface = accessibilityHandler.getTextInterface())
            {
                const auto granularityKey = javaString ("ACTION_ARGUMENT_MOVEMENT_GRANULARITY_INT");
                const auto extendSelectionKey = javaString ("ACTION_ARGUMENT_EXTEND_SELECTION_BOOLEAN");

                auto* env = getEnv();

                const auto boundaryType = [&]
                {
                    const auto granularity = env->CallIntMethod (arguments,
                                                                 AndroidBundle.getInt,
                                                                 granularityKey.get());

                    using BoundaryType = AccessibilityTextHelpers::BoundaryType;

                    switch (granularity)
                    {
                        case MOVEMENT_GRANULARITY_CHARACTER:  return BoundaryType::character;
                        case MOVEMENT_GRANULARITY_WORD:       return BoundaryType::word;
                        case MOVEMENT_GRANULARITY_LINE:       return BoundaryType::line;
                        case MOVEMENT_GRANULARITY_PARAGRAPH:
                        case MOVEMENT_GRANULARITY_PAGE:       return BoundaryType::document;
                    }

                    jassertfalse;
                    return BoundaryType::character;
                }();

                using Direction = AccessibilityTextHelpers::Direction;

                const auto cursorPos = AccessibilityTextHelpers::findTextBoundary (*textInterface,
                                                                                   textInterface->getTextInsertionOffset(),
                                                                                   boundaryType,
                                                                                   forwards ? Direction::forwards
                                                                                            : Direction::backwards);

                const auto newSelection = [&]() -> Range<int>
                {
                    const auto currentSelection = textInterface->getSelection();
                    const auto extendSelection = env->CallBooleanMethod (arguments,
                                                                         AndroidBundle.getBoolean,
                                                                         extendSelectionKey.get());

                    if (! extendSelection)
                        return { cursorPos, cursorPos };

                    const auto start = currentSelection.getStart();
                    const auto end = currentSelection.getEnd();

                    if (forwards)
                        return { start, jmax (start, cursorPos) };

                    return { jmin (start, cursorPos), end };
                }();

                textInterface->setSelection (newSelection);
                return true;
            }

            return false;
        */
    }
}

#[cfg(target_os="android")]
impl<'a> AndroidAccessibilityGetDescriptionString for AccessibilityNativeHandle<'a> {

    fn get_description_string(&self) -> LocalRef<jstring> {
        
        todo!();
        /*
            const auto valueString = [this]() -> String
            {
                if (auto* textInterface = accessibilityHandler.getTextInterface())
                    return textInterface->getText ({ 0, textInterface->getTotalNumCharacters() });

                if (auto* valueInterface = accessibilityHandler.getValueInterface())
                    return valueInterface->getCurrentValueAsString();

                return {};
            }();

            Vec<String> strings (accessibilityHandler.getTitle(),
                                 valueString,
                                 accessibilityHandler.getDescription(),
                                 accessibilityHandler.getHelp());

            strings.removeEmptyStrings();

            return javaString (strings.joinIntoString (","));
        */
    }
}

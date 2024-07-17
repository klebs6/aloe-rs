crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/accessibility/aloe_AccessibilityTextHelpers.h]

pub mod accessibility_text_helpers {
    use super::*;
    pub enum BoundaryType
    {
        character,
        word,
        line,
        document
    }

    pub enum Direction
    {
        forwards,
        backwards
    }

    pub fn find_text_boundary(
        text_interface:   &dyn AccessibilityTextInterface,
        current_position: i32,
        boundary:         BoundaryType,
        direction:        Direction) -> i32 {

        todo!();
        /*
            const auto numCharacters = textInterface.getTotalNumCharacters();
                const auto isForwards = (direction == Direction::forwards);

                auto offsetWithDirecton = [isForwards] (int input) { return isForwards ? input : -input; };

                switch (boundary)
                {
                    case BoundaryType::character:
                        return jlimit (0, numCharacters, currentPosition + offsetWithDirecton (1));

                    case BoundaryType::word:
                    case BoundaryType::line:
                    {
                        const auto text = [&]() -> String
                        {
                            if (isForwards)
                                return textInterface.getText ({ currentPosition, textInterface.getTotalNumCharacters() });

                            const auto str = textInterface.getText ({ 0, currentPosition });

                            auto start = str.getCharPointer();
                            auto end = start.findTerminatingNull();
                            const auto size = getAddressDifference (end.getAddress(), start.getAddress());

                            String reversed;

                            if (size > 0)
                            {
                                reversed.preallocateBytes ((size_t) size);

                                auto destPtr = reversed.getCharPointer();

                                for (;;)
                                {
                                    destPtr.write (*--end);

                                    if (end == start)
                                        break;
                                }

                                destPtr.writeNull();
                            }

                            return reversed;
                        }();

                        auto tokens = (boundary == BoundaryType::line ? StringArray::fromLines (text)
                                                                      : StringArray::fromTokens (text, false));

                        return currentPosition + offsetWithDirecton (tokens[0].length());
                    }

                    case BoundaryType::document:
                        return isForwards ? numCharacters : 0;
                }

                jassertfalse;
                return -1;
        */
    }
}

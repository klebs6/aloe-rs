crate::ix!();

pub fn get_display_orientation() -> *const u8 {
    
    todo!();
    /*
        switch (Desktop::getInstance().getCurrentOrientation())
        {
            case Desktop::upright:              return "Upright";
            case Desktop::upsideDown:           return "Upside-down";
            case Desktop::rotatedClockwise:     return "Rotated Clockwise";
            case Desktop::rotatedAntiClockwise: return "Rotated Anti-clockwise";
            case Desktop::allOrientations:      return "All";
            default: jassertfalse; break;
        }

        return nullptr;
    */
}

pub fn get_display_info() -> String {
    
    todo!();
    /*
        auto& displays = Desktop::getInstance().getDisplays();

        String displayDesc;

        for (int i = 0; i < displays.displays.size(); ++i)
        {
            auto display = displays.displays.getReference (i);

            displayDesc << "Display " << (i + 1) << (display.isMain ? " (main)" : "") << ":" << newLine
                        << "  Total area: " << display.totalArea.toString() << newLine
                        << "  User area:  " << display.userArea .toString() << newLine
                        << "  DPI: "        << display.dpi   << newLine
                        << "  Scale: "      << display.scale << newLine
                        << newLine;
        }

        displayDesc << "Orientation: " << getDisplayOrientation() << newLine;

        return displayDesc;
    */
}

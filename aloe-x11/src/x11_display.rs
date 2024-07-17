crate::ix!();

//=============================== X11 - Displays ===============================

pub fn x11_display_parse_xsettings<Callback>(
    data_ptr: *const u8,
    bytes:    usize,
    callback: Callback

) {

    todo!();
        /*
            struct Header
            {
                CARD8 byteOrder;
                CARD8 padding[3];
                CARD32 serial;
                CARD32 nSettings;
            };

            auto* data     = dataPtr;
            size_t byteNum = 0;

            const auto increment = [&] (size_t amount)
            {
                data    += amount;
                byteNum += amount;
            };

            const auto* header = reinterpret_cast<const Header*> (data);
            increment (sizeof (Header));

            const auto readCARD16 = [&]() -> CARD16
            {
                if (byteNum + sizeof (CARD16) > bytes)
                    return {};

                const auto value = header->byteOrder == MSBFirst ? ByteOrder::bigEndianShort (data)
                                                                 : ByteOrder::littleEndianShort (data);
                increment (sizeof (CARD16));
                return value;
            };

            const auto readCARD32 = [&]() -> CARD32
            {
                if (byteNum + sizeof (CARD32) > bytes)
                    return {};

                const auto value = header->byteOrder == MSBFirst ? ByteOrder::bigEndianInt (data)
                                                                 : ByteOrder::littleEndianInt (data);
                increment (sizeof (CARD32));
                return value;
            };

            const auto readString = [&] (size_t nameLen) -> std::string
            {
                const auto padded = (nameLen + 3) & (~(size_t) 3);

                if (byteNum + padded > bytes)
                    return {};

                auto* ptr = reinterpret_cast<const char*> (data);
                const std::string result (ptr, ptr + nameLen);
                increment (padded);
                return result;
            };

            CARD16 setting = 0;

            while (byteNum < bytes && setting < header->nSettings)
            {
                const auto type = *reinterpret_cast<const char*> (data);
                increment (2);

                const auto name   = readString (readCARD16());
                const auto serial = readCARD32();
                ignoreUnused (serial);

                enum { XSettingsTypeInteger, XSettingsTypeString, XSettingsTypeColor };

                switch (type)
                {
                    case XSettingsTypeInteger:
                    {
                        callback (name, (INT32) readCARD32());
                        break;
                    }

                    case XSettingsTypeString:
                    {
                        callback (name, readString (readCARD32()));
                        break;
                    }

                    case XSettingsTypeColor:
                    {
                        // Order is important, these should be kept as separate statements!
                        const auto r = readCARD16();
                        const auto g = readCARD16();
                        const auto b = readCARD16();
                        const auto a = readCARD16();
                        callback (name, r, g, b, a);
                        break;
                    }
                }

                setting += 1;
            }
        */
}

pub fn x11_display_get_scaling_factor_from_xsettings() -> f64 {
    
    todo!();
        /*
            if (auto* display = XWindowSystem::getInstance()->getDisplay())
            {
                using namespace XWindowSystemUtilities;

                ScopedXLock xLock;

                const auto selectionWindow = X11Symbols::getInstance()->xGetSelectionOwner (display,
                                                                                            XWindowSystemAtoms::getCreating (display, "_XSETTINGS_S0"));

                if (selectionWindow != None)
                {
                    const auto xsettingsSettingsAtom = XWindowSystemAtoms::getCreating (display, "_XSETTINGS_SETTINGS");

                    const GetXProperty prop { selectionWindow,
                                              xsettingsSettingsAtom,
                                              0L,
                                              std::numeric_limits<long>::max(),
                                              false,
                                              xsettingsSettingsAtom };

                    if (prop.success
                        && prop.actualType == xsettingsSettingsAtom
                        && prop.actualFormat == 8)
                    {
                        struct ExtractRelevantSettings
                        {
                            void operator() (const std::string& name, INT32 value)
                            {
                                if (name == "Gdk/WindowScalingFactor")
                                    scaleFactor = value;
                            }

                            void operator() (const std::string&, const std::string&) {}
                            void operator() (const std::string&, CARD16, CARD16, CARD16, CARD16) {}

                            INT32 scaleFactor = 0;
                        };

                        ExtractRelevantSettings callback;
                        parseXSettings (prop.data, prop.numItems, callback);

                        return (double) callback.scaleFactor;
                    }
                }
            }

            return 0.0;
        */
}

pub fn x11_display_get_displaydpi(
    display: *mut Display,
    index:   i32

) -> f64 {
    
    todo!();
        /*
            auto widthMM  = X11Symbols::getInstance()->xDisplayWidthMM  (display, index);
            auto heightMM = X11Symbols::getInstance()->xDisplayHeightMM (display, index);

            if (widthMM > 0 && heightMM > 0)
                return (((X11Symbols::getInstance()->xDisplayWidth (display, index) * 25.4) / widthMM)
                        + ((X11Symbols::getInstance()->xDisplayHeight (display, index) * 25.4) / heightMM)) / 2.0;

            return 96.0;
        */
}

pub fn x11_display_get_display_scale(
    name: &String,
    dpi:  f64

) -> f64 {
    
    todo!();
        /*
            const auto scaleFactorFromXSettings = getScalingFactorFromXSettings();

            if (scaleFactorFromXSettings > 0.0)
                return scaleFactorFromXSettings;

            if (name.isNotEmpty())
            {
                // Ubuntu and derived distributions now save a per-display scale factor as a configuration
                // variable. This can be changed in the Monitor system settings panel.
                ChildProcess dconf;

                if (File ("/usr/bin/dconf").existsAsFile()
                    && dconf.start ("/usr/bin/dconf read /com/ubuntu/user-interface/scale-factor", ChildProcess::wantStdOut))
                {
                    if (dconf.waitForProcessToFinish (200))
                    {
                        auto jsonOutput = dconf.readAllProcessOutput().replaceCharacter ('\'', '"');

                        if (dconf.getExitCode() == 0 && jsonOutput.isNotEmpty())
                        {
                            auto jsonVar = JSON::parse (jsonOutput);

                            if (auto* object = jsonVar.getDynamicObject())
                            {
                                auto scaleFactorVar = object->getProperty (name);

                                if (! scaleFactorVar.isVoid())
                                {
                                    auto scaleFactor = ((double) scaleFactorVar) / 8.0;

                                    if (scaleFactor > 0.0)
                                        return scaleFactor;
                                }
                            }
                        }
                    }
                }
            }

            {
                // Other gnome based distros now use gsettings for a global scale factor
                ChildProcess gsettings;

                if (File ("/usr/bin/gsettings").existsAsFile()
                    && gsettings.start ("/usr/bin/gsettings get org.gnome.desktop.interface scaling-factor", ChildProcess::wantStdOut))
                {
                    if (gsettings.waitForProcessToFinish (200))
                    {
                        auto gsettingsOutput = StringArray::fromTokens (gsettings.readAllProcessOutput(), true);

                        if (gsettingsOutput.size() >= 2 && gsettingsOutput[1].length() > 0)
                        {
                            auto scaleFactor = gsettingsOutput[1].getDoubleValue();

                            if (scaleFactor > 0.0)
                                return scaleFactor;

                            return 1.0;
                        }
                    }
                }
            }

            // If no scale factor is set by GNOME or Ubuntu then calculate from monitor dpi
            // We use the same approach as chromium which simply divides the dpi by 96
            // and then rounds the result
            return round (dpi / 96.0);
        */
}

#[cfg(ALOE_USE_XINERAMA)]
pub fn x11_display_xinerama_query_displays(display: *mut Display) -> Vec<XineramaScreenInfo> {
    
    todo!();
        /*
            int major_opcode, first_event, first_error;

            if (X11Symbols::getInstance()->xQueryExtension (display, "XINERAMA", &major_opcode, &first_event, &first_error)
                && (X11Symbols::getInstance()->xineramaIsActive (display) != 0))
            {
                int numScreens;

                if (auto xinfo = makeXFreePtr (X11Symbols::getInstance()->xineramaQueryScreens (display, &numScreens)))
                    return { xinfo.get(), numScreens };
            }

            return {};
        */
}

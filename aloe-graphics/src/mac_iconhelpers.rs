crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_mac_IconHelpers.cpp]

pub fn get_icon_from_icns_file(
    icns_file: &File,
    size:      i32) -> Image {
    
    todo!();
        /*
            FileInputStream stream (icnsFile);

        if (! stream.openedOk())
            return {};

        const int numHeaderSectionBytes = 4;
        char headerSection [numHeaderSectionBytes];

        if (stream.read (headerSection, numHeaderSectionBytes) != numHeaderSectionBytes
              || headerSection[0] != 'i'
              || headerSection[1] != 'c'
              || headerSection[2] != 'n'
              || headerSection[3] != 's')
            return {};

        if (stream.read (headerSection, numHeaderSectionBytes) != numHeaderSectionBytes)
            return {};

        const auto dataSize = ByteOrder::bigEndianInt (headerSection);

        if (dataSize <= 0)
            return {};

        Vec<Box<ImageFileFormat>> internalFormats;
        internalFormats.add (new  PNGImageFormat());
        internalFormats.add (new JPEGImageFormat());

        Vec<Image> images;
        auto maxWidth = 0;
        auto maxWidthIndex = -1;

        while (stream.getPosition() < dataSize)
        {
            const auto sectionStart = stream.getPosition();

            if (! stream.setPosition (sectionStart + 4))
                break;

            if (stream.read (headerSection, numHeaderSectionBytes) != numHeaderSectionBytes)
                break;

            const auto sectionSize = ByteOrder::bigEndianInt (headerSection);

            if (sectionSize <= 0)
                break;

            const auto sectionDataStart = stream.getPosition();

            for (auto* fmt : internalFormats)
            {
                if (fmt->canUnderstand (stream))
                {
                    stream.setPosition (sectionDataStart);

                    images.add (fmt->decodeImage (stream));

                    const auto lastImageIndex = images.size() - 1;
                    const auto lastWidth = images.getReference (lastImageIndex).getWidth();

                    if (lastWidth > maxWidth)
                    {
                        maxWidthIndex = lastImageIndex;
                        maxWidth = lastWidth;
                    }
                }

                stream.setPosition (sectionDataStart);
            }

            stream.setPosition (sectionStart + sectionSize);
        }

        return maxWidthIndex == -1 ? Image()
                                   : images.getReference (maxWidthIndex).rescaled (size, size, typename GraphicsResamplingQuality::highResamplingQuality);
        */
}

pub fn get_icon_from_application(
        application_path: &String,
        size:             i32) -> Image {
    
    todo!();
        /*
            if (auto pathCFString = CFUniquePtr<CFStringRef> (CFStringCreateWithCString (kCFAllocatorDefault, applicationPath.toRawUTF8(), kCFStringEncodingUTF8)))
        {
            if (auto url = CFUniquePtr<CFURLRef> (CFURLCreateWithFileSystemPath (kCFAllocatorDefault, pathCFString.get(), kCFURLPOSIXPathStyle, 1)))
            {
                if (auto appBundle = CFUniquePtr<CFBundleRef> (CFBundleCreate (kCFAllocatorDefault, url.get())))
                {
                    if (CFTypeRef infoValue = CFBundleGetValueForInfoDictionaryKey (appBundle.get(), CFSTR("CFBundleIconFile")))
                    {
                        if (CFGetTypeID (infoValue) == CFStringGetTypeID())
                        {
                            CFStringRef iconFilename = reinterpret_cast<CFStringRef> (infoValue);
                            CFStringRef resourceURLSuffix = CFStringHasSuffix (iconFilename, CFSTR(".icns")) ? nullptr : CFSTR("icns");

                            if (auto iconURL = CFUniquePtr<CFURLRef> (CFBundleCopyResourceURL (appBundle.get(), iconFilename, resourceURLSuffix, nullptr)))
                            {
                                if (auto iconPath = CFUniquePtr<CFStringRef> (CFURLCopyFileSystemPath (iconURL.get(), kCFURLPOSIXPathStyle)))
                                {
                                    File icnsFile (CFStringGetCStringPtr (iconPath.get(), CFStringGetSystemEncoding()));
                                    return getIconFromIcnsFile (icnsFile, size);
                                }
                            }
                        }
                    }
                }
            }
        }

        return {};
        */
}

crate::ix!();

#[cfg(any(target_os="macos",target_os="ios"))]
pub const core_audio_format_name: &'static str = "CoreAudio supported file";

#[cfg(any(target_os="macos",target_os="ios"))]
pub fn find_file_extensions_for_core_audio_codecs() -> StringArray {
    
    todo!();
        /*
            StringArray extensionsArray;
        CFObjectHolder<CFArrayRef> extensions;
        UInt32 sizeOfArray = sizeof (extensions.object);

        if (AudioFileGetGlobalInfo (kAudioFileGlobalInfo_AllExtensions, 0, nullptr, &sizeOfArray, &extensions.object) == noErr)
        {
            auto numValues = CFArrayGetCount (extensions.object);

            for (CFIndex i = 0; i < numValues; ++i)
                extensionsArray.add ("." + String::fromCFString ((CFStringRef) CFArrayGetValueAtIndex (extensions.object, i)));
        }

        return extensionsArray;
        */
}

crate::ix!();

pub fn ca_show_audio_channel_layout(
    file:   *mut libc::FILE,
    layout: *const AudioChannelLayout
) {

    todo!();
        /*
            if (layout == NULL)
        {
            fprintf (file, "\tNULL layout\n");
            return;
        }
        fprintf (file, "\tTag=0x%X, ", (int)layout->mChannelLayoutTag);
        if (layout->mChannelLayoutTag == kAudioChannelLayoutTag_UseChannelBitmap)
            fprintf (file, "Using Bitmap:0x%X\n", (int)layout->mChannelBitmap);
        else {
            fprintf (file, "Num Chan Descs=%d\n", (int)layout->mNumberChannelDescriptions);
            const AudioChannelDescription *desc = layout->mChannelDescriptions;
            for (unsigned int i = 0; i < layout->mNumberChannelDescriptions; ++i, ++desc) {
                fprintf (file, "\t\tLabel=%d, Flags=0x%X, ", (int)desc->mChannelLabel, (int)desc->mChannelFlags);
                fprintf (file, "[az=%f,el=%f,dist=%f]\n", desc->mCoordinates[0], desc->mCoordinates[1], desc->mCoordinates[2]);
            }
        }
        */
}

crate::ix!();

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(ALOE_UNIT_TESTS)]
macro_rules! define_channel_layout_dfl_entry {
    ($x:ident) => {
        /*
                CoreAudioChannelLayoutTag { x, #x, AudioChannelSet() }
        */
    }
}

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(ALOE_UNIT_TESTS)]
macro_rules! define_channel_layout_tag_entry {
    ($x:ident, $y:ident) => {
        /*
                CoreAudioChannelLayoutTag { x, #x, y }
        */
    }
}

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(ALOE_UNIT_TESTS)]
pub struct CoreAudioLayoutsUnitTest {
    base: UnitTest,
}

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(ALOE_UNIT_TESTS)]
pub mod core_audio_layouts_unit_test {
    use super::*;

    /**
       some ambisonic tags which are not
       explicitly defined
      */
    pub const AudioChannelLayoutTag_HOA_ACN_SN3D_0Order: usize = (190 << 16) | 1;
    pub const AudioChannelLayoutTag_HOA_ACN_SN3D_1Order: usize = (190 << 16) | 4;
    pub const AudioChannelLayoutTag_HOA_ACN_SN3D_2Order: usize = (190 << 16) | 9;
    pub const AudioChannelLayoutTag_HOA_ACN_SN3D_3Order: usize = (190 << 16) | 16;
    pub const AudioChannelLayoutTag_HOA_ACN_SN3D_4Order: usize = (190 << 16) | 25;
    pub const AudioChannelLayoutTag_HOA_ACN_SN3D_5Order: usize = (190 << 16) | 36;

    pub struct CoreAudioChannelLayoutTag
    {
        tag:                    AudioChannelLayoutTag,
        name:                   *const u8,

        /**
          | referred to this in the AudioChannelSet
          | documentation
          |
          */
        equivalent_channel_set: AudioChannelSet,
    }
}

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(ALOE_UNIT_TESTS)]
impl Default for CoreAudioLayoutsUnitTest {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Core Audio Layout <-> Aloe channel layout conversion", UnitTestCategories::audio)
        */
    }
}

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(ALOE_UNIT_TESTS)]
impl CoreAudioLayoutsUnitTest {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            auto& knownTags = getAllKnownLayoutTags();

            {
                // Check that all known tags defined in CoreAudio SDK version 10.12.4 are known to Aloe
                // Include all defined tags even if there are duplicates as Apple will sometimes change
                // definitions
                beginTest ("All CA tags handled");

                for (auto tagEntry : knownTags)
                {
                    auto labels = CoreAudioLayouts::fromCoreAudio (tagEntry.tag);

                    expect (! labels.isDiscreteLayout(), "Tag \"" + String (tagEntry.name) + "\" is not handled by Aloe");
                }
            }

            {
                beginTest ("Number of speakers");

                for (auto tagEntry : knownTags)
                {
                    auto labels = CoreAudioLayouts::getSpeakerLayoutForCoreAudioTag (tagEntry.tag);

                    expect (labels.size() == (tagEntry.tag & 0xffff), "Tag \"" + String (tagEntry.name) + "\" has incorrect channel count");
                }
            }

            {
                beginTest ("No duplicate speaker");

                for (auto tagEntry : knownTags)
                {
                    auto labels = CoreAudioLayouts::getSpeakerLayoutForCoreAudioTag (tagEntry.tag);
                    labels.sort();

                    for (int i = 0; i < (labels.size() - 1); ++i)
                        expect (labels.getReference (i) != labels.getReference (i + 1),
                                "Tag \"" + String (tagEntry.name) + "\" has the same speaker twice");
                }
            }

            {
                beginTest ("CA speaker list and aloe layouts are consistent");

                for (auto tagEntry : knownTags)
                    expect (AudioChannelSet::channelSetWithChannels (CoreAudioLayouts::getSpeakerLayoutForCoreAudioTag (tagEntry.tag))
                                == CoreAudioLayouts::fromCoreAudio (tagEntry.tag),
                            "Tag \"" + String (tagEntry.name) + "\" is not converted consistently by Aloe");
            }

            {
                beginTest ("AudioChannelSet documentation is correct");

                for (auto tagEntry : knownTags)
                {
                    if (tagEntry.equivalentChannelSet.isDisabled())
                        continue;

                    expect (CoreAudioLayouts::fromCoreAudio (tagEntry.tag) == tagEntry.equivalentChannelSet,
                            "Documentation for tag \"" + String (tagEntry.name) + "\" is incorrect");
                }
            }

            {
                beginTest ("CA tag reverse conversion");

                for (auto tagEntry : knownTags)
                {
                    if (tagEntry.equivalentChannelSet.isDisabled())
                        continue;

                    expect (CoreAudioLayouts::toCoreAudio (tagEntry.equivalentChannelSet) == tagEntry.tag,
                            "Incorrect reverse conversion for tag \"" + String (tagEntry.name) + "\"");
                }
            }
        */
    }
    
    pub fn get_all_known_layout_tags(&self) -> &[CoreAudioChannelLayoutTag] {
        
        todo!();
        /*
            static CoreAudioChannelLayoutTag tags[] = {
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_Mono,   AudioChannelSet::mono()),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_Stereo, AudioChannelSet::stereo()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_StereoHeadphones),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MatrixStereo),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MidSide),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_XY),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_Binaural),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_Ambisonic_B_Format),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_Quadraphonic, AudioChannelSet::quadraphonic()),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_Pentagonal, AudioChannelSet::pentagonal()),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_Hexagonal, AudioChannelSet::hexagonal()),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_Octagonal, AudioChannelSet::octagonal()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_Cube),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_1_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_2_0),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_MPEG_3_0_A, AudioChannelSet::createLCR()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_3_0_B),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_MPEG_4_0_A, AudioChannelSet::createLCRS()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_4_0_B),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_MPEG_5_0_A, AudioChannelSet::create5point0()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_5_0_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_5_0_C),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_5_0_D),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_MPEG_5_1_A, AudioChannelSet::create5point1()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_5_1_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_5_1_C),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_5_1_D),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_MPEG_6_1_A, AudioChannelSet::create6point1()),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_MPEG_7_1_A, AudioChannelSet::create7point1SDDS()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_MPEG_7_1_B),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_MPEG_7_1_C, AudioChannelSet::create7point1()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_Emagic_Default_7_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_SMPTE_DTV),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_1_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_2_0),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_ITU_2_1, AudioChannelSet::createLRS()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_2_2),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_3_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_3_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_3_2),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_3_2_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_ITU_3_4_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_2),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_3),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_4),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_5),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_6),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_7),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_8),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_9),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_10),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_11),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_12),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_13),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_14),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_15),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_16),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_17),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_18),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_19),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DVD_20),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_4),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_5),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_6),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_8),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_5_0),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_AudioUnit_6_0, AudioChannelSet::create6point0()),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_AudioUnit_7_0, AudioChannelSet::create7point0()),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_AudioUnit_7_0_Front, AudioChannelSet::create7point0SDDS()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_5_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_6_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_7_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AudioUnit_7_1_Front),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_3_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_Quadraphonic),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_4_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_5_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_5_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_6_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_6_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_7_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_7_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_7_1_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_7_1_C),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AAC_Octagonal),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_TMH_10_2_std),
                // DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_TMH_10_2_full), no indication on how to handle this tag
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AC3_1_0_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AC3_3_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AC3_3_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AC3_3_0_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AC3_2_1_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_AC3_3_1_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC_6_0_A),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC_7_0_A),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_6_1_A),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_6_1_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_6_1_C),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_A),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_C),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_D),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_E),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_F),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_G),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_EAC3_7_1_H),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_3_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_4_1),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_DTS_6_0_A, AudioChannelSet::create6point0Music()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_6_0_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_6_0_C),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_DTS_6_1_A, AudioChannelSet::create6point1Music()),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_6_1_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_6_1_C),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_7_0),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_7_1),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_8_0_A),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_8_0_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_8_1_A),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_8_1_B),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_6_1_D),
                DEFINE_CHANNEL_LAYOUT_DFL_ENTRY (kAudioChannelLayoutTag_DTS_6_1_D),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_HOA_ACN_SN3D_0Order,  AudioChannelSet::ambisonic (0)),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_HOA_ACN_SN3D_1Order,  AudioChannelSet::ambisonic (1)),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_HOA_ACN_SN3D_2Order,  AudioChannelSet::ambisonic (2)),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_HOA_ACN_SN3D_3Order, AudioChannelSet::ambisonic (3)),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_HOA_ACN_SN3D_4Order, AudioChannelSet::ambisonic (4)),
                DEFINE_CHANNEL_LAYOUT_TAG_ENTRY (kAudioChannelLayoutTag_HOA_ACN_SN3D_5Order, AudioChannelSet::ambisonic (5))
            };
            static Vec<CoreAudioChannelLayoutTag> knownTags (tags, sizeof (tags) / sizeof (CoreAudioChannelLayoutTag));

            return knownTags;
        */
    }
}

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static CoreAudioLayoutsUnitTest coreAudioLayoutsUnitTest;
    */
}

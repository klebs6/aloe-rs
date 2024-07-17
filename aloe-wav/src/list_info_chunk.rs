/*!
  | Reads a RIFF List Info chunk from a stream
  | positioned just after the size byte.
  |
  */
crate::ix!();

lazy_static!{
    /*
    static const char* const list_info_chunk_types[] =
            {
                WavAudioFormat::riffInfoArchivalLocation,
                WavAudioFormat::riffInfoArtist,
                WavAudioFormat::riffInfoBaseURL,
                WavAudioFormat::riffInfoCinematographer,
                WavAudioFormat::riffInfoComment,
                WavAudioFormat::riffInfoComments,
                WavAudioFormat::riffInfoComment2,
                WavAudioFormat::riffInfoCommissioned,
                WavAudioFormat::riffInfoCopyright,
                WavAudioFormat::riffInfoCostumeDesigner,
                WavAudioFormat::riffInfoCountry,
                WavAudioFormat::riffInfoCropped,
                WavAudioFormat::riffInfoDateCreated,
                WavAudioFormat::riffInfoDateTimeOriginal,
                WavAudioFormat::riffInfoDefaultAudioStream,
                WavAudioFormat::riffInfoDimension,
                WavAudioFormat::riffInfoDirectory,
                WavAudioFormat::riffInfoDistributedBy,
                WavAudioFormat::riffInfoDotsPerInch,
                WavAudioFormat::riffInfoEditedBy,
                WavAudioFormat::riffInfoEighthLanguage,
                WavAudioFormat::riffInfoEncodedBy,
                WavAudioFormat::riffInfoEndTimecode,
                WavAudioFormat::riffInfoEngineer,
                WavAudioFormat::riffInfoFifthLanguage,
                WavAudioFormat::riffInfoFirstLanguage,
                WavAudioFormat::riffInfoFourthLanguage,
                WavAudioFormat::riffInfoGenre,
                WavAudioFormat::riffInfoKeywords,
                WavAudioFormat::riffInfoLanguage,
                WavAudioFormat::riffInfoLength,
                WavAudioFormat::riffInfoLightness,
                WavAudioFormat::riffInfoLocation,
                WavAudioFormat::riffInfoLogoIconURL,
                WavAudioFormat::riffInfoLogoURL,
                WavAudioFormat::riffInfoMedium,
                WavAudioFormat::riffInfoMoreInfoBannerImage,
                WavAudioFormat::riffInfoMoreInfoBannerURL,
                WavAudioFormat::riffInfoMoreInfoText,
                WavAudioFormat::riffInfoMoreInfoURL,
                WavAudioFormat::riffInfoMusicBy,
                WavAudioFormat::riffInfoNinthLanguage,
                WavAudioFormat::riffInfoNumberOfParts,
                WavAudioFormat::riffInfoOrganisation,
                WavAudioFormat::riffInfoPart,
                WavAudioFormat::riffInfoProducedBy,
                WavAudioFormat::riffInfoProductName,
                WavAudioFormat::riffInfoProductionDesigner,
                WavAudioFormat::riffInfoProductionStudio,
                WavAudioFormat::riffInfoRate,
                WavAudioFormat::riffInfoRated,
                WavAudioFormat::riffInfoRating,
                WavAudioFormat::riffInfoRippedBy,
                WavAudioFormat::riffInfoSecondaryGenre,
                WavAudioFormat::riffInfoSecondLanguage,
                WavAudioFormat::riffInfoSeventhLanguage,
                WavAudioFormat::riffInfoSharpness,
                WavAudioFormat::riffInfoSixthLanguage,
                WavAudioFormat::riffInfoSoftware,
                WavAudioFormat::riffInfoSoundSchemeTitle,
                WavAudioFormat::riffInfoSource,
                WavAudioFormat::riffInfoSourceFrom,
                WavAudioFormat::riffInfoStarring_ISTR,
                WavAudioFormat::riffInfoStarring_STAR,
                WavAudioFormat::riffInfoStartTimecode,
                WavAudioFormat::riffInfoStatistics,
                WavAudioFormat::riffInfoSubject,
                WavAudioFormat::riffInfoTapeName,
                WavAudioFormat::riffInfoTechnician,
                WavAudioFormat::riffInfoThirdLanguage,
                WavAudioFormat::riffInfoTimeCode,
                WavAudioFormat::riffInfoTitle,
                WavAudioFormat::riffInfoTrackNo,
                WavAudioFormat::riffInfoTrackNumber,
                WavAudioFormat::riffInfoURL,
                WavAudioFormat::riffInfoVegasVersionMajor,
                WavAudioFormat::riffInfoVegasVersionMinor,
                WavAudioFormat::riffInfoVersion,
                WavAudioFormat::riffInfoWatermarkURL,
                WavAudioFormat::riffInfoWrittenBy,
                WavAudioFormat::riffInfoYear
            };
    */
}

pub fn list_info_chunk_is_matching_type_ignoring_case(
    value: i32,
    name:  *const u8

) -> bool {
    
    todo!();
        /*
            for (int i = 0; i < 4; ++i)
                    if ((aloe_wchar) name[i] != CharacterFunctions::toUpperCase ((aloe_wchar) ((value >> (i * 8)) & 0xff)))
                        return false;

                return true;
        */
}

pub fn list_info_chunk_add_to_metadata(
    values:    &mut StringPairArray,
    input:     &mut dyn Read,
    chunk_end: i64

) {
    
    todo!();
        /*
            while (input.getPosition() < chunkEnd)
                {
                    auto infoType = input.readInt();
                    auto infoLength = chunkEnd - input.getPosition();

                    if (infoLength > 0)
                    {
                        infoLength = jmin (infoLength, (int64) input.readInt());

                        if (infoLength <= 0)
                            return;

                        for (auto& type : types)
                        {
                            if (isMatchingTypeIgnoringCase (infoType, type))
                            {
                                MemoryBlock mb;
                                input.readIntoMemoryBlock (mb, (ssize_t) infoLength);
                                values.set (type, String::createStringFromData ((const char*) mb.getData(),
                                                                                (int) mb.getSize()));
                                break;
                            }
                        }
                    }
                }
        */
}

pub fn list_info_chunk_write_value(
    values:     &StringPairArray,
    out:        &mut MemoryOutputStream,
    param_name: *const u8

) -> bool {
    
    todo!();
        /*
            auto value = values.getValue (paramName, {});

                if (value.isEmpty())
                    return false;

                auto valueLength = (int) value.getNumBytesAsUTF8() + 1;
                auto chunkLength = valueLength + (valueLength & 1);

                out.writeInt (chunkName (paramName));
                out.writeInt (chunkLength);
                out.write (value.toUTF8(), (size_t) valueLength);

                if ((out.getDataSize() & 1) != 0)
                    out.writeByte (0);

                return true;
        */
}

pub fn list_info_chunk_create_from(values: &StringPairArray) -> MemoryBlock {
    
    todo!();
        /*
            MemoryOutputStream out;
                out.writeInt (chunkName ("INFO"));
                bool anyParamsDefined = false;

                for (auto& type : types)
                    if (writeValue (values, out, type))
                        anyParamsDefined = true;

                return anyParamsDefined ? out.getMemoryBlock() : MemoryBlock();
        */
}


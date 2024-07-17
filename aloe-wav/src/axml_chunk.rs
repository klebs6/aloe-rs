crate::ix!();

pub fn add_to_metadata(
    dest_values: &mut Vec<(String,String)>,
    source:      &String

)  {

    todo!();
        /*
            if (auto xml = parseXML (source))
                {
                    if (xml->hasTagName ("ebucore:ebuCoreMain"))
                    {
                        if (auto xml2 = xml->getChildByName ("ebucore:coreMetadata"))
                        {
                            if (auto xml3 = xml2->getChildByName ("ebucore:identifier"))
                            {
                                if (auto xml4 = xml3->getChildByName ("dc:identifier"))
                                {
                                    auto ISRCCode = xml4->getAllSubText().fromFirstOccurrenceOf ("ISRC:", false, true);

                                    if (ISRCCode.isNotEmpty())
                                        destValues.set (WavAudioFormat::ISRC, ISRCCode);
                                }
                            }
                        }
                    }
                }
        */
}

pub fn create_from(
    values: &Vec<(String,String)>

) -> MemoryBlock {

    todo!();
        /*
            auto ISRC = values.getValue (WavAudioFormat::ISRC, {});
                MemoryOutputStream xml;

                if (ISRC.isNotEmpty())
                {
                    xml << "<ebucore:ebuCoreMain xmlns:dc=\" http://purl.org/dc/elements/1.1/\" "
                                                "xmlns:ebucore=\"urn:ebu:metadata-schema:ebuCore_2012\">"
                             "<ebucore:coreMetadata>"
                               "<ebucore:identifier typeLabel=\"GUID\" "
                                                   "typeDefinition=\"Globally Unique Identifier\" "
                                                   "formatLabel=\"ISRC\" "
                                                   "formatDefinition=\"International Standard Recording Code\" "
                                                   "formatLink=\"http://www.ebu.ch/metadata/cs/ebu_IdentifierTypeCodeCS.xml#3.7\">"
                                 "<dc:identifier>ISRC:" << ISRC << "</dc:identifier>"
                               "</ebucore:identifier>"
                             "</ebucore:coreMetadata>"
                           "</ebucore:ebuCoreMain>";

                    xml.writeRepeatedByte (0, xml.getDataSize());  // ensures even size, null termination and room for future growing
                }

                return xml.getMemoryBlock();
        */
}

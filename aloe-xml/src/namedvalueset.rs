crate::ix!();

pub trait SetFromXmlAttributes {

    fn set_from_xml_attributes(&mut self, xml: &XmlElement);
}

pub trait CopyToXmlAttributes {

    fn copy_to_xml_attributes(&self, xml: &mut XmlElement);
}

impl SetFromXmlAttributes for NamedValueSet {

    /**
      | Sets properties to the values of all
      | of an XML element's attributes.
      |
      */
    fn set_from_xml_attributes(&mut self, xml: &XmlElement)  {
        
        todo!();
        /*
            values.clearQuick();

        for (auto* att = xml.attributes.get(); att != nullptr; att = att->nextListItem)
        {
            if (att->name.toString().startsWith ("base64:"))
            {
                MemoryBlock mb;

                if (mb.fromBase64Encoding (att->value))
                {
                    values.add ({ att->name.toString().substring (7), Var (mb) });
                    continue;
                }
            }

            values.add ({ att->name, Var (att->value) });
        }
        */
    }
}

impl CopyToXmlAttributes for NamedValueSet {

    /**
      | Sets attributes in an XML element corresponding
      | to each of this object's properties.
      |
      */
    fn copy_to_xml_attributes(&self, xml: &mut XmlElement)  {
        
        todo!();
        /*
            for (auto& i : values)
        {
            if (auto* mb = i.value.getBinaryData())
            {
                xml.setAttribute ("base64:" + i.name.toString(), mb->toBase64Encoding());
            }
            else
            {
                // These types can't be stored as XML!
                jassert (! i.value.isObject());
                jassert (! i.value.isMethod());
                jassert (! i.value.isArray());

                xml.setAttribute (i.name.toString(),
                                  i.value.toString());
            }
        }
        */
    }
}

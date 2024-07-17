crate::ix!();

pub struct KeyFileData
{
    licensee:         String,
    email:            String,
    appid:            String,
    machine_numbers:  StringArray,
    key_file_expires: bool,
    expiry_time:      Time,
}

/**
  | @note
  | 
  | There's a bit of light obfuscation in
  | this code, just to make things a bit more
  | annoying for crackers who try to reverse-engineer
  | your binaries, but nothing particularly
  | foolproof.
  |
  */
pub struct KeyFileUtils {

}

impl KeyFileUtils {

    pub fn create_key_file_content(
        app_name:                       &String,
        user_email:                     &String,
        user_name:                      &String,
        machine_numbers:                &String,
        machine_numbers_attribute_name: &String) -> XmlElement {
        
        todo!();
        /*
            XmlElement xml ("key");

            xml.setAttribute ("user", userName);
            xml.setAttribute ("email", userEmail);
            xml.setAttribute (machineNumbersAttributeName, machineNumbers);
            xml.setAttribute ("app", appName);
            xml.setAttribute ("date", String::toHexString (Time::getCurrentTime().toMilliseconds()));

            return xml;
        */
    }
    
    pub fn create_key_file_comment(
        app_name:        &String,
        user_email:      &String,
        user_name:       &String,
        machine_numbers: &String) -> String {
        
        todo!();
        /*
            String comment;
            comment << "Keyfile for " << appName << newLine;

            if (userName.isNotEmpty())
                comment << "User: " << userName << newLine;

            comment << "Email: " << userEmail << newLine
                    << "Machine numbers: " << machineNumbers << newLine
                    << "Created: " << Time::getCurrentTime().toString (true, true);

            return comment;
        */
    }
    
    pub fn encryptxml(
        xml:         &XmlElement,
        private_key: RSAKey) -> String {
        
        todo!();
        /*
            MemoryOutputStream text;
            text << xml.toString (XmlElement::TextFormat().singleLine());

            BigInteger val;
            val.loadFromMemoryBlock (text.getMemoryBlock());

            privateKey.applyToValue (val);

            return val.toString (16);
        */
    }
    
    pub fn create_key_file(
        comment:         String,
        xml:             &XmlElement,
        rsa_private_key: RSAKey) -> String {
        
        todo!();
        /*
            String asHex ("#" + encryptXML (xml, rsaPrivateKey));

            StringArray lines;
            lines.add (comment);
            lines.add (String());

            const int charsPerLine = 70;
            while (asHex.length() > 0)
            {
                lines.add (asHex.substring (0, charsPerLine));
                asHex = asHex.substring (charsPerLine);
            }

            lines.add (String());

            return lines.joinIntoString ("\r\n");
        */
    }
    
    pub fn decryptxml(
        hex_data:       String,
        rsa_public_key: RSAKey) -> XmlElement {
        
        todo!();
        /*
            BigInteger val;
            val.parseString (hexData, 16);

            RSAKey key (rsaPublicKey);
            jassert (key.isValid());

            std::unique_ptr<XmlElement> xml;

            if (! val.isZero())
            {
                key.applyToValue (val);

                auto mb = val.toMemoryBlock();

                if (CharPointer_UTF8::isValidString (static_cast<const char*> (mb.getData()), (int) mb.getSize()))
                    xml = parseXML (mb.toString());
            }

            return xml != nullptr ? *xml : XmlElement("key");
        */
    }
    
    pub fn get_xml_from_key_file(
        key_file_text:  String,
        rsa_public_key: RSAKey) -> XmlElement {
        
        todo!();
        /*
            return decryptXML (keyFileText.fromLastOccurrenceOf ("#", false, false).trim(), rsaPublicKey);
        */
    }
    
    pub fn get_machine_numbers(
        xml:            XmlElement,
        attribute_name: &str) -> StringArray {
        
        todo!();
        /*
            StringArray numbers;
            numbers.addTokens (xml.getStringAttribute (attributeName), ",; ", "");
            numbers.trim();
            numbers.removeEmptyStrings();
            return numbers;
        */
    }
    
    pub fn get_licensee(xml: &XmlElement) -> String {
        
        todo!();
        /*
            return xml.getStringAttribute ("user");
        */
    }
    
    pub fn get_email(xml: &XmlElement) -> String {
        
        todo!();
        /*
            return xml.getStringAttribute ("email");
        */
    }
    
    pub fn get_appid(xml: &XmlElement) -> String {
        
        todo!();
        /*
            return xml.getStringAttribute ("app");
        */
    }
    
    pub fn get_data_from_key_file(xml: XmlElement) -> KeyFileData {
        
        todo!();
        /*
            KeyFileData data;

            data.licensee = getLicensee (xml);
            data.email = getEmail (xml);
            data.appID = getAppID (xml);

            if (xml.hasAttribute ("expiryTime") && xml.hasAttribute ("expiring_mach"))
            {
                data.keyFileExpires = true;
                data.machineNumbers.addArray (getMachineNumbers (xml, "expiring_mach"));
                data.expiryTime = Time (xml.getStringAttribute ("expiryTime").getHexValue64());
            }
            else
            {
                data.keyFileExpires = false;
                data.machineNumbers.addArray (getMachineNumbers (xml, "mach"));
            }

            return data;
        */
    }
}

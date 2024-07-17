crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/utility/aloe_PluginUtilities.cpp]

#[cfg(AloePlugin_Build_Unity)]
pub fn aloe_is_running_in_unity() -> bool {
    
    todo!();
        /*
            return PluginHostType::getPluginLoadedAs() == AudioProcessor::wrapperType_Unity;
        */
}

#[cfg(not(ALOE_Vst3_CAN_REPLACE_Vst2))]
pub const ALOE_Vst3_CAN_REPLACE_Vst2: usize = 1;

#[cfg(all(all(AloePlugin_Build_Vst3,ALOE_Vst3_CAN_REPLACE_Vst2),any(any(target_os="macos",target_os="windows"),any(target_os="linux",target_os="bsd"))))]
pub const Vst3_REPLACEMENT_AVAILABLE: usize = 1;

/**
   NB: Nasty old-fashioned code in here because
   it's copied from the Steinberg example code.
  */
#[cfg(all(all(AloePlugin_Build_Vst3,ALOE_Vst3_CAN_REPLACE_Vst2),any(any(target_os="macos",target_os="windows"),any(target_os="linux",target_os="bsd"))))]
pub fn get_uuid_forvst2id(
        for_controlleruid: bool,
        uuid:              [u8; 16])  {
    
    todo!();
        /*
            char uidString[33];

         const int vstfxid = (('V' << 16) | ('S' << 8) | (forControllerUID ? 'E' : 'T'));
         char vstfxidStr[7] = { 0 };
         sprintf (vstfxidStr, "%06X", vstfxid);

         strcpy (uidString, vstfxidStr);

         char uidStr[9] = { 0 };
         sprintf (uidStr, "%08X", AloePlugin_VstUniqueID);
         strcat (uidString, uidStr);

         char nameidStr[3] = { 0 };
         const size_t len = strlen (AloePlugin_Name);

         for (size_t i = 0; i <= 8; ++i)
         {
             uint8 c = i < len ? static_cast<uint8> (AloePlugin_Name[i]) : 0;

             if (c >= 'A' && c <= 'Z')
                 c += 'a' - 'A';

             sprintf (nameidStr, "%02X", c);
             strcat (uidString, nameidStr);
         }

         unsigned long p0;
         unsigned int p1, p2;
         unsigned int p3[8];

        #if ! ALOE_MSVC
         sscanf
        #else
         sscanf_s
        #endif
         (uidString, "%08lX%04X%04X%02X%02X%02X%02X%02X%02X%02X%02X",
          &p0, &p1, &p2, &p3[0], &p3[1], &p3[2], &p3[3], &p3[4], &p3[5], &p3[6], &p3[7]);

         union q0_u {
             uint32 word;
             uint8 bytes[4];
         } q0;

         union q1_u {
             uint16 half;
             uint8 bytes[2];
         } q1, q2;

         q0.word = static_cast<uint32> (p0);
         q1.half = static_cast<uint16> (p1);
         q2.half = static_cast<uint16> (p2);

         // Vst3 doesn't use COM compatible UUIDs on non windows platforms
        #if ! ALOE_WINDOWS
         q0.word = ByteOrder::swap (q0.word);
         q1.half = ByteOrder::swap (q1.half);
         q2.half = ByteOrder::swap (q2.half);
        #endif

         for (int i = 0; i < 4; ++i)
             uuid[i+0] = q0.bytes[i];

         for (int i = 0; i < 2; ++i)
             uuid[i+4] = q1.bytes[i];

         for (int i = 0; i < 2; ++i)
             uuid[i+6] = q2.bytes[i];

         for (int i = 0; i < 8; ++i)
             uuid[i+8] = static_cast<uint8> (p3[i]);
        */
}

#[cfg(not(all(all(feature = "plugin-build-vst3",ALOE_Vst3_CAN_REPLACE_Vst2),any(any(target_os="macos",target_os="windows"),any(target_os="linux",target_os="bsd")))))]
pub const Vst3_REPLACEMENT_AVAILABLE: usize = 0;

#[cfg(AloePlugin_Build_Vst)]
pub fn handle_manufacturer_specific_vst2opcode(
        index: i32,
        value: PointerSizedInt,
        ptr:   *mut c_void,
        _3:    f32) -> bool {
    
    todo!();
        /*
            #if Vst3_REPLACEMENT_AVAILABLE
         if ((index == (int32) ByteOrder::bigEndianInt ("stCA") || index == (int32) ByteOrder::bigEndianInt ("stCa"))
             && value == (int32) ByteOrder::bigEndianInt ("FUID") && ptr != nullptr)
         {
             uint8 fuid[16];
             getUUIDForVst2ID  (false, fuid);
             ::memcpy (ptr, fuid, 16);
             return true;
         }
        #else
         ignoreUnused (index, value, ptr);
        #endif
         return false;
        */
}

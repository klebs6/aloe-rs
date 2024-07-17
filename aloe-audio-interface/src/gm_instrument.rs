crate::ix!();

pub trait GetGmInstrumentName {

    /**
      | Returns the standard name of a GM instrument,
      | or nullptr if unknown for this index.
      | 
      | -----------
      | @param midiInstrumentNumber
      | 
      | the program number 0 to 127
      | 
      | @see getProgramChangeNumber
      |
      */
    fn get_gm_instrument_name(&mut self, n: i32) -> *const u8;
}

pub trait GetGmInstrumentBankName {

    /**
      | Returns the name of a bank of GM instruments,
      | or nullptr if unknown for this bank number.
      | 
      | -----------
      | @param midiBankNumber
      | 
      | the bank, 0 to 15
      |
      */
    fn get_gm_instrument_bank_name(&mut self, n: i32) -> *const u8;
}

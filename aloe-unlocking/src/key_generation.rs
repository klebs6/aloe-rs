crate::ix!();

#[cfg(ALOE_MODULE_AVAILABLE_aloe_data_structures)]
pub const state_tag_name:    &'static str = "REG";

#[cfg(ALOE_MODULE_AVAILABLE_aloe_data_structures)]
pub const user_name_prop:    &'static str = "user";

#[cfg(ALOE_MODULE_AVAILABLE_aloe_data_structures)]
pub const keyfile_data_prop: &'static str = "key";

#[cfg(ALOE_MODULE_AVAILABLE_aloe_data_structures)]
pub fn machine_number_allowed(
    numbers_from_key_file: StringArray,
    local_machine_numbers: StringArray

) -> Var {
    
    todo!();
        /*
            var result;

        for (int i = 0; i < localMachineNumbers.size(); ++i)
        {
            auto localNumber = localMachineNumbers[i].trim();

            if (localNumber.isNotEmpty())
            {
                for (int j = numbersFromKeyFile.size(); --j >= 0;)
                {
                    var ok (localNumber.trim().equalsIgnoreCase (numbersFromKeyFile[j].trim()));
                    result.swapWith (ok);

                    if (result)
                        break;
                }

                if (result)
                    break;
            }
        }

        return result;
        */
}

crate::ix!();

macro_rules! acpi {
    () => {
        /*
                ((AudioComponentPlugInInstance *)self)
        */
    }
}

macro_rules! ac_imp {
    () => {
        /*
                ((ComponentBase *)&ACPI->mInstanceStorage)
        */
    }
}

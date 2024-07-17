crate::ix!();

pub struct AUBaseFactory<Implementor> {
    base: APFactory<AUBaseLookup,Implementor>,
}

pub struct AUOutputBaseFactory<Implementor> {
    base: APFactory<AUOutputLookup,Implementor>,
}

pub struct AUOutputComplexBaseFactory<Implementor> {
    base: APFactory<AUComplexOutputLookup,Implementor>,
}

pub struct AUBaseProcessFactory<Implementor> {
    base: APFactory<AUBaseProcessLookup,Implementor>,
}

pub struct AUBaseProcessMultipleFactory<Implementor> {
    base: APFactory<AUBaseProcessMultipleLookup,Implementor>,
}

pub struct AUBaseProcessAndMultipleFactory<Implementor> {
    base: APFactory<AUBaseProcessAndMultipleLookup,Implementor>,
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub struct AUMIDIEffectFactory<Implementor> {
    base: APFactory<AUMIDILookup,Implementor>,
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub struct AUMIDIProcessFactory<Implementor> {
    base: APFactory<AUMIDIProcessLookup,Implementor>,
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub struct AUMusicDeviceFactory<Implementor> {
    base: APFactory<AUMusicLookup,Implementor>,
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub struct AUAuxBaseFactory<Implementor> {
    base: APFactory<AUAuxBaseLookup,Implementor>,
}

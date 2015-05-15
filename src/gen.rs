use ffi::*;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum GenFlags {
    Unused = 0,
    Set, 
    AbsNrpn,
}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum GenType {
    StartAddrOffset = 0,
    EndAddrOffset, 
    StartLoopAddOffset,
    EndLoopAddrOffset,
    StartAddrCoarseOffset,
    ModLFOToPitch,
    VibLFOToPitch,
    ModEnvToPitch,
    FilterFC,
    FilterQ,
    ModLFOToFilterFC,
    ModEnvToFilterFC,
    EndAddrCoarseOffset,
    ModLFOToVol = 13,
    ChorusSend = 15,
    ReverbSend,
    Pan = 17,
    ModLFODelay = 21,
    ModLFOReq,
    VibLFODelay,
    VibLFOReq,
    ModEnvDelay,
    ModEnvAttack,
    ModEnvHold,
    ModEnvDecay,
    ModEnvSustain,
    ModEnvRelease,
    KeyToModEnvHold,
    KeyToModEnvDecay,
    VolEnvDelay,
    VolEnvAttack,
    VolEnvHold,
    VolEnvDecay,
    VolEnvSustain,
    VolEnvRelease,
    KeyToVolEnvHold,
    KeyToVolEnvDecay,
    Instrument = 41,
    KeyRange = 43,
    VelRange,
    StartLoopAddrCoarseOffset,
    KeyNum,
    Velocity,
    Attenuation = 48,
    EndLoopAddrCoarseOffset = 50,
    CoarseTune,
    FineTune,
    SampleID,
    SampleMode = 54,
    ScaleTune = 56,
    ExclusiveClass,
    OverrideRootKey,
    Pitch,
    Last,
}


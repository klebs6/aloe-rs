crate::ix!();

pub enum AdsrState { 
    idle, 
    attack, 
    decay, 
    sustain, 
    release 
}

/**
  | Holds the parameters being used by an
  | ADSR object.
  | 
  | @tags{Audio}
  |
  */
pub struct   AdsrParameters {
    attack:  f32,
    decay:   f32,
    sustain: f32,
    release: f32,
}

impl Default for AdsrParameters {

    fn default() -> Self {

        Self {
            attack:  0.1,
            decay:   0.1,
            sustain: 1.0,
            release: 0.1,
        }
    }
}

impl AdsrParameters {
    
    pub fn new(
        attack_time_seconds:  f32,
        decay_time_seconds:   f32,
        sustain_level:        f32,
        release_time_seconds: f32) -> Self {
    
        todo!();
        /*
        : attack(attackTimeSeconds),
        : decay(decayTimeSeconds),
        : sustain(sustainLevel),
        : release(releaseTimeSeconds),

        
        */
    }
}

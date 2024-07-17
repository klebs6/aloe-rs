crate::ix!();

/**
  | This is basically a sawtooth wave generator
  | 
  | - maps a value that bounces between 0.0
  | and 1.0 at a random speed
  |
  */
pub struct BouncingNumber {
    speed: f64,
    phase: f64,
}

impl Default for BouncingNumber {
    
    fn default() -> Self {
        todo!();
        /*
            : speed (0.0004 + 0.0007 * Random::getSystemRandom().nextDouble()),
              phase (Random::getSystemRandom().nextDouble()
        */
    }
}

impl BouncingNumber {

    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            double v = fmod (phase + speed * Time::getMillisecondCounterHiRes(), 2.0);
            return (float) (v >= 1.0 ? (2.0 - v) : v);
        */
    }
}

///-------------------------
pub struct SlowerBouncingNumber {
    base: BouncingNumber,
}

impl Default for SlowerBouncingNumber {
    
    fn default() -> Self {
        todo!();
        /*


            speed *= 0.3
        */
    }
}

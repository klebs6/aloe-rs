crate::ix!();

pub struct MathClass {
    base: DynamicObject,
}

impl Default for MathClass {
    
    fn default() -> Self {
        todo!();
        /*


            setMethod ("abs",       Math_abs);              setMethod ("round",     Math_round);
            setMethod ("random",    Math_random);           setMethod ("randInt",   Math_randInt);
            setMethod ("min",       Math_min);              setMethod ("max",       Math_max);
            setMethod ("range",     Math_range);            setMethod ("sign",      Math_sign);
            setMethod ("toDegrees", Math_toDegrees);        setMethod ("toRadians", Math_toRadians);
            setMethod ("sin",       Math_sin);              setMethod ("asin",      Math_asin);
            setMethod ("sinh",      Math_sinh);             setMethod ("asinh",     Math_asinh);
            setMethod ("cos",       Math_cos);              setMethod ("acos",      Math_acos);
            setMethod ("cosh",      Math_cosh);             setMethod ("acosh",     Math_acosh);
            setMethod ("tan",       Math_tan);              setMethod ("atan",      Math_atan);
            setMethod ("tanh",      Math_tanh);             setMethod ("atanh",     Math_atanh);
            setMethod ("log",       Math_log);              setMethod ("log10",     Math_log10);
            setMethod ("exp",       Math_exp);              setMethod ("pow",       Math_pow);
            setMethod ("sqr",       Math_sqr);              setMethod ("sqrt",      Math_sqrt);
            setMethod ("ceil",      Math_ceil);             setMethod ("floor",     Math_floor);

            setProperty ("PI",      MathConstants<double>::pi);
            setProperty ("E",       MathConstants<double>::euler);
            setProperty ("SQRT2",   MathConstants<double>::sqrt2);
            setProperty ("SQRT1_2", std::sqrt (0.5));
            setProperty ("LN2",     std::log (2.0));
            setProperty ("LN10",    std::log (10.0));
            setProperty ("LOG2E",   std::log (MathConstants<double>::euler) / std::log (2.0));
            setProperty ("LOG10E",  std::log (MathConstants<double>::euler) / std::log (10.0));
        */
    }
}

impl MathClass {
    
    pub fn math_random(_0: Args) -> Var {
        
        todo!();
        /*
            return Random::getSystemRandom().nextDouble();
        */
    }
    
    pub fn math_rand_int(a: Args) -> Var {
        
        todo!();
        /*
            return Random::getSystemRandom().nextInt (Range<int> (getInt (a, 0), getInt (a, 1)));
        */
    }
    
    pub fn math_abs(a: Args) -> Var {
        
        todo!();
        /*
            return isInt (a, 0) ? Var (std::abs   (getInt (a, 0))) : Var (std::abs   (getDouble (a, 0)));
        */
    }
    
    pub fn math_round(a: Args) -> Var {
        
        todo!();
        /*
            return isInt (a, 0) ? Var (roundToInt (getInt (a, 0))) : Var (roundToInt (getDouble (a, 0)));
        */
    }
    
    
    pub fn math_sign(a: Args) -> Var {
        
        todo!();
        /*
            return isInt (a, 0) ? Var (sign       (getInt (a, 0))) : Var (sign       (getDouble (a, 0)));
        */
    }
    
    
    pub fn math_range(a: Args) -> Var {
        
        todo!();
        /*
            return isInt (a, 0) ? Var (jlimit (getInt (a, 1), getInt (a, 2), getInt (a, 0))) : Var (jlimit (getDouble (a, 1), getDouble (a, 2), getDouble (a, 0)));
        */
    }
    
    
    pub fn math_min(a: Args) -> Var {
        
        todo!();
        /*
            return (isInt (a, 0) && isInt (a, 1)) ? Var (jmin (getInt (a, 0), getInt (a, 1))) : Var (jmin (getDouble (a, 0), getDouble (a, 1)));
        */
    }
    
    
    pub fn math_max(a: Args) -> Var {
        
        todo!();
        /*
            return (isInt (a, 0) && isInt (a, 1)) ? Var (jmax (getInt (a, 0), getInt (a, 1))) : Var (jmax (getDouble (a, 0), getDouble (a, 1)));
        */
    }
    
    
    pub fn math_to_degrees(a: Args) -> Var {
        
        todo!();
        /*
            return radiansToDegrees (getDouble (a, 0));
        */
    }
    
    
    pub fn math_to_radians(a: Args) -> Var {
        
        todo!();
        /*
            return degreesToRadians (getDouble (a, 0));
        */
    }
    
    
    pub fn math_sin(a: Args) -> Var {
        
        todo!();
        /*
            return std::sin   (getDouble (a, 0));
        */
    }
    
    
    pub fn math_asin(a: Args) -> Var {
        
        todo!();
        /*
            return std::asin  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_cos(a: Args) -> Var {
        
        todo!();
        /*
            return std::cos   (getDouble (a, 0));
        */
    }
    
    
    pub fn math_acos(a: Args) -> Var {
        
        todo!();
        /*
            return std::acos  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_sinh(a: Args) -> Var {
        
        todo!();
        /*
            return std::sinh  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_cosh(a: Args) -> Var {
        
        todo!();
        /*
            return std::cosh  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_tan(a: Args) -> Var {
        
        todo!();
        /*
            return std::tan   (getDouble (a, 0));
        */
    }
    
    
    pub fn math_tanh(a: Args) -> Var {
        
        todo!();
        /*
            return std::tanh  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_atan(a: Args) -> Var {
        
        todo!();
        /*
            return std::atan  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_log(a: Args) -> Var {
        
        todo!();
        /*
            return std::log   (getDouble (a, 0));
        */
    }
    
    
    pub fn math_log10(a: Args) -> Var {
        
        todo!();
        /*
            return std::log10 (getDouble (a, 0));
        */
    }
    
    
    pub fn math_exp(a: Args) -> Var {
        
        todo!();
        /*
            return std::exp   (getDouble (a, 0));
        */
    }
    
    
    pub fn math_pow(a: Args) -> Var {
        
        todo!();
        /*
            return std::pow   (getDouble (a, 0), getDouble (a, 1));
        */
    }
    
    
    pub fn math_sqr(a: Args) -> Var {
        
        todo!();
        /*
            return square (getDouble (a, 0));
        */
    }
    
    
    pub fn math_sqrt(a: Args) -> Var {
        
        todo!();
        /*
            return std::sqrt  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_ceil(a: Args) -> Var {
        
        todo!();
        /*
            return std::ceil  (getDouble (a, 0));
        */
    }
    
    
    pub fn math_floor(a: Args) -> Var {
        
        todo!();
        /*
            return std::floor (getDouble (a, 0));
        */
    }

    /**
      | We can't use the std namespace equivalents
      | of these functions without breaking
      | compatibility with older versions
      | of OS X.
      |
      */
    pub fn math_asinh(a: Args) -> Var {
        
        todo!();
        /*
            return asinh (getDouble (a, 0));
        */
    }
    
    pub fn math_acosh(a: Args) -> Var {
        
        todo!();
        /*
            return acosh (getDouble (a, 0));
        */
    }
    
    pub fn math_atanh(a: Args) -> Var {
        
        todo!();
        /*
            return atanh (getDouble (a, 0));
        */
    }
    
    pub fn get_class_name() -> Identifier {
        
        todo!();
        /*
            static const Identifier i ("Math"); return i;
        */
    }
    
    pub fn sign<Type>(n: Type) -> Type {
    
        todo!();
        /*
            return n > 0 ? (Type) 1 : (n < 0 ? (Type) -1 : 0);
        */
    }
}

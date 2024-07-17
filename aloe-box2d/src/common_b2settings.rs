/*!
   @file Global tuning constants based on
   meters-kilograms-seconds (MKS) units.
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Settings.h]

macro_rules! b2_not_used {
    ($x:ident) => {
        /*
                ((void)(x))
        */
    }
}

macro_rules! b2assert {
    ($A:ident) => {
        /*
                jassert(A)
        */
    }
}

pub type float32 = f32;
pub type float64 = f64;

macro_rules! b2_maxfloat {
    () => {
        /*
                FLT_MAX
        */
    }
}

macro_rules! b2_epsilon {
    () => {
        /*
                FLT_EPSILON
        */
    }
}

/* ------------------- Collision ------------------- */

/**
   The maximum number of contact points between
   two convex shapes. Do not change this value.
  */
pub const b2_maxManifoldPoints: usize = 2;

/**
  | The maximum number of vertices on a convex
  | polygon. You cannot increase this too much
  | because b2BlockAllocator has a maximum object
  | size.
  */
pub const b2_maxPolygonVertices: usize = 8;

/**
  | This is used to fatten AABBs in the dynamic
  | tree. This allows proxies to move by a small
  | amount without triggering a tree adjustment.
  | This is in meters.
  */
pub const b2_aabbExtension: f32 = 0.1;

/**
  | This is used to fatten AABBs in the dynamic
  | tree. This is used to predict the future
  | position based on the current displacement.
  | This is a dimensionless multiplier.
  */
pub const b2_aabbMultiplier: f32 = 2.0;

/**
  | A small length used as a collision and
  | constraint tolerance. Usually it is chosen to
  | be numerically significant, but visually
  | insignificant.
  */
pub const b2_linearSlop: f32 = 0.005;

/**
  | A small angle used as a collision and
  | constraint tolerance. Usually it is chosen to
  | be numerically significant, but visually
  | insignificant.
  */
pub const b2_angularSlop: f32 = 2.0 / 180.0 * std::f32::consts::PI;

/**
  | The radius of the polygon/edge shape
  | skin. This should not be modified. Making this
  | smaller means polygons will have an
  | insufficient buffer for continuous collision.
  | Making it larger may create artifacts for
  | vertex collision.
  */
pub const b2_polygonRadius: f32 = 2.0 * b2_linearSlop;

/**
   Maximum number of sub-steps per contact in
   continuous physics simulation.
  */
pub const b2_maxSubSteps: usize = 8;

/* ------------------- * Dynamics  ------------------- */

/**
   Maximum number of contacts to be handled to
   solve a TOI impact.
  */
pub const b2_maxTOIContacts: usize = 32;

/**
  | A velocity threshold for elastic
  | collisions. Any collision with a relative
  | linear velocity below this threshold will be
  | treated as inelastic.
  */
pub const b2_velocityThreshold: f32 = 1.0;

/**
   The maximum linear position correction used
   when solving constraints. This helps to
   prevent overshoot.
  */
pub const b2_maxLinearCorrection: f32 = 0.2;

/**
   The maximum angular position correction used
   when solving constraints. This helps to
   prevent overshoot.
  */
pub const b2_maxAngularCorrection: f32 = (8.0 / 180.0 * std::f32::consts::PI);

/**
  | The maximum linear velocity of a body. This
  | limit is very large and is used to prevent
  | numerical problems. You shouldn't need to
  | adjust this.
  */
pub const b2_maxTranslation:        f32 = 2.0;
pub const b2_maxTranslationSquared: f32 = b2_maxTranslation * b2_maxTranslation;

/**
  | The maximum angular velocity of a body. This
  | limit is very large and is used to prevent
  | numerical problems. You shouldn't need to
  | adjust this.
  */
pub const b2_maxRotation:        f32 = 0.5 * std::f32::consts::PI;
pub const b2_maxRotationSquared: f32 = b2_maxRotation * b2_maxRotation;

/**
  | This scale factor controls how fast overlap is
  | resolved. Ideally this would be 1 so that
  | overlap is removed in one time step. However
  | using values close to 1 often lead to
  | overshoot.
  */
pub const b2_baumgarte:   f32 = 0.2;
pub const b2_toiBaugarte: f32 = 0.75;

/* --------------------- * Sleep  --------------------- */

/**
   The time that a body must be still before it
   will go to sleep.
  */
pub const b2_timeToSleep: f32 = 0.5;

/**
   A body cannot sleep if its linear velocity is
   above this tolerance.
  */
pub const b2_linearSleepTolerance: f32 = 0.01;

/**
   A body cannot sleep if its angular velocity is
   above this tolerance.
  */
pub const b2_angularSleepTolerance: f32 = 2.0 / 180.0 * std::f32::consts::PI;

/* --------------- * Memory Allocation  --------------- */

/**
   Version numbering scheme.
   See http://en.wikipedia.org/wiki/Software_versioning
  */
pub struct b2Version {

    /**
      significant changes
      */
    major:    i32,

    /**
      incremental changes
      */
    minor:    i32,

    /**
      bug fixes
      */
    revision: i32,
}

/**
   Current version.
  */
lazy_static!{
    /*
    extern b2Version b2_version;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Settings.cpp]

lazy_static!{
    /*
    b2Version b2_version = {2, 2, 1};
    */
}

/**
  | Memory allocators. Modify these to use your own
  | allocator.
  */
pub fn b2alloc(size: i32)  {
    
    todo!();
    /*
        return malloc(size);
    */
}

/**
   If you implement b2Alloc, you should also
   implement this function.
  */
pub fn b2free(mem: *mut c_void)  {
    
    todo!();
    /*
        free(mem);
    */
}

/**
   You can modify this to use your logging
   facility.
  */
pub fn b2log(
        string: *const u8,
        args:   &[&str])  {
    
    todo!();
    /*
        va_list args;
        va_start(args, string);
        vprintf(string, args);
        va_end(args);
    */
}

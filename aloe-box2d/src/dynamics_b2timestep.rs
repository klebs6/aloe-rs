crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2TimeStep.h]

/**
   Profiling data. Times are in milliseconds.
  */
pub struct b2Profile 
{
    step:           f32,
    collide:        f32,
    solve:          f32,
    solve_init:     f32,
    solve_velocity: f32,
    solve_position: f32,
    broadphase:     f32,
    solvetoi:       f32,
}

/**
   This is an internal structure.
  */
pub struct b2TimeStep 
{

    /**
       time step
      */
    dt:                  f32,

    /**
       inverse time step (0 if dt == 0).
      */
    inv_dt:              f32,

    /**
       dt * inv_dt0
      */
    dt_ratio:            f32,

    velocity_iterations: i32,
    position_iterations: i32,
    warm_starting:       bool,
}

/**
   This is an internal structure.
  */
pub struct b2Position 
{
    c: b2Vec2,
    a: f32,
}

/**
   This is an internal structure.
  */
pub struct b2Velocity
{
    v: b2Vec2,
    w: f32,
}

/**
   Solver Data
  */
pub struct b2SolverData
{
    step:       b2TimeStep,
    positions:  *mut b2Position,
    velocities: *mut b2Velocity,
}

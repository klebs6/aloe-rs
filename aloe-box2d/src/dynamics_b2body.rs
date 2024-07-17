crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2Body.h]

/**
  | The body type.
  |
  | static: zero mass, zero velocity, may be
  | manually moved
  |
  | kinematic: zero mass, non-zero velocity set by
  | user, moved by solver
  |
  | dynamic: positive mass, non-zero velocity
  | determined by forces, moved by solver
  */
pub enum b2BodyType
{
    b2_staticBody = 0,
    b2_kinematicBody,
    b2_dynamicBody

    // TODO_ERIN
    //b2_bulletBody,
}

/**
  | A body definition holds all the data needed to
  | construct a rigid body.  You can safely re-use
  | body definitions. Shapes are added to a body
  | after construction.
  */
pub struct b2BodyDef {

    /**
      The body type: static, kinematic, or
      dynamic.

      Note: if a dynamic body would have zero
      mass, the mass is set to one.
      */
    ty:               b2BodyType,

    /**
      The world position of the body. Avoid
      creating bodies at the origin since this can
      lead to many overlapping shapes.
      */
    position:         b2Vec2,

    /**
      The world angle of the body in radians.
      */
    angle:            f32,

    /**
      The linear velocity of the body's origin in
      world co-ordinates.
      */
    linear_velocity:  b2Vec2,

    /**
      The angular velocity of the body.
      */
    angular_velocity: f32,

    /**
      | Linear damping is use to reduce the linear
      | velocity. The damping parameter can be
      | larger than 1.0f but the damping effect
      | becomes sensitive to the time step when the
      | damping parameter is large.
      */
    linear_damping:   f32,

    /**
      | Angular damping is use to reduce the angular
      | velocity. The damping parameter can be
      | larger than 1.0f but the damping effect
      | becomes sensitive to the time step when the
      | damping parameter is large.
      */
    angular_damping:  f32,

    /**
      | Set this flag to false if this body should
      | never fall asleep. Note that this increases
      | CPU usage.
      */
    allow_sleep:      bool,

    /**
      Is this body initially awake or sleeping?
      */
    awake:            bool,

    /**
      Should this body be prevented from rotating?
      Useful for characters.
      */
    fixed_rotation:   bool,

    /**
      | Is this a fast moving body that should
      | be prevented from tunneling through
      | other moving bodies? Note that all bodies
      | are prevented from tunneling through
      | kinematic and static bodies. This setting
      | is only considered on dynamic bodies.
      | 
      | -----------
      | @warning
      | 
      | You should use this flag sparingly since
      | it increases processing time.
      |
      */
    bullet:           bool,

    /**
      Does this body start out active?
      */
    active:           bool,

    /**
      Use this to store application specific body
      data.
      */
    user_data:        *mut c_void,

    /**
      Scale the gravity applied to this body.
      */
    gravity_scale:    f32,
}

impl Default for b2BodyDef {

    /**
       This constructor sets the body definition
       default values.
      */
    fn default() -> Self {
        todo!();
        /*


            userData = NULL;
            position.Set(0.0f, 0.0f);
            angle = 0.0f;
            linearVelocity.Set(0.0f, 0.0f);
            angularVelocity = 0.0f;
            linearDamping = 0.0f;
            angularDamping = 0.0f;
            allowSleep = true;
            awake = true;
            fixedRotation = false;
            bullet = false;
            type = b2_staticBody;
            active = true;
            gravityScale = 1.0f
        */
    }
}

pub const B2_BOXY_E_ISLAND_FLAG:         usize = 0x0001;
pub const B2_BOXY_E_AWAKE_FLAG:          usize = 0x0002;
pub const B2_BOXY_E_AUTO_SLEEP_FLAG:     usize = 0x0004;
pub const B2_BOXY_E_BULLET_FLAG:         usize = 0x0008;
pub const B2_BOXY_E_FIXED_ROTATION_FLAG: usize = 0x0010;
pub const B2_BOXY_E_ACTIVE_FLAG:         usize = 0x0020;
pub const B2_BOXY_E_TOI_FLAG:            usize = 0x0040;

/**
  | A rigid body. These are created via b2World::CreateBody.
  |
  */
pub struct b2Body {

    ty:               b2BodyType,
    flags:            u16,
    island_index:     i32,

    /**
      | the body origin transform
      |
      */
    xf:               b2Transform,

    /**
      | the swept motion for CCD
      |
      */
    sweep:            b2Sweep,

    linear_velocity:  b2Vec2,
    angular_velocity: f32,
    force:            b2Vec2,
    torque:           f32,
    world:            *mut b2World,
    prev:             *mut b2Body,
    next:             *mut b2Body,
    fixture_list:     *mut b2Fixture,
    fixture_count:    i32,
    joint_list:       *mut b2JointEdge,
    contact_list:     *mut b2ContactEdge,
    mass:             f32,
    inv_mass:         f32,

    /**
      | Rotational inertia about the center
      | of mass.
      |
      */
    i:                f32,

    /**
      | Rotational inertia about the center
      | of mass.
      |
      */
    invi:             f32,

    linear_damping:   f32,
    angular_damping:  f32,
    gravity_scale:    f32,
    sleep_time:       f32,
    user_data:        *mut c_void,
}

impl Drop for b2Body {

    fn drop(&mut self) {
        todo!();
        /* 
        // shapes and joints are destroyed in b2World::Destroy
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2Body.cpp]
impl b2Body {

    /**
      | Get the type of this body.
      |
      */
    #[inline] pub fn get_type(&self) -> b2BodyType {
        
        todo!();
        /*
            return m_type;
        */
    }
    
    /**
      | Get the body transform for the body's
      | origin.
      | 
      | -----------
      | @return
      | 
      | the world transform of the body's origin.
      |
      */
    #[inline] pub fn get_transform(&self) -> &b2Transform {
        
        todo!();
        /*
            return m_xf;
        */
    }
    
    /**
      | Get the world body origin position.
      | 
      | -----------
      | @return
      | 
      | the world position of the body's origin.
      |
      */
    #[inline] pub fn get_position(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_xf.p;
        */
    }
    
    /**
      | Get the angle in radians.
      | 
      | -----------
      | @return
      | 
      | the current world rotation angle in
      | radians.
      |
      */
    #[inline] pub fn get_angle(&self) -> f32 {
        
        todo!();
        /*
            return m_sweep.a;
        */
    }
    
    /**
      | Get the world position of the center
      | of mass.
      |
      */
    #[inline] pub fn get_world_center(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_sweep.c;
        */
    }
    
    /**
      | Get the local position of the center
      | of mass.
      |
      */
    #[inline] pub fn get_local_center(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_sweep.localCenter;
        */
    }
    
    /**
      | Set the linear velocity of the center
      | of mass.
      | 
      | -----------
      | @param v
      | 
      | the new linear velocity of the center
      | of mass.
      |
      */
    #[inline] pub fn set_linear_velocity(&mut self, v: &b2Vec2)  {
        
        todo!();
        /*
            if (m_type == b2_staticBody)
        {
            return;
        }

        if (b2Dot(v,v) > 0.0f)
        {
            SetAwake(true);
        }

        m_linearVelocity = v;
        */
    }
    
    /**
      | Get the linear velocity of the center
      | of mass.
      | 
      | -----------
      | @return
      | 
      | the linear velocity of the center of
      | mass.
      |
      */
    #[inline] pub fn get_linear_velocity(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_linearVelocity;
        */
    }
    
    /**
      | Set the angular velocity.
      | 
      | -----------
      | @param omega
      | 
      | the new angular velocity in radians/second.
      |
      */
    #[inline] pub fn set_angular_velocity(&mut self, w: f32)  {
        
        todo!();
        /*
            if (m_type == b2_staticBody)
        {
            return;
        }

        if (w * w > 0.0f)
        {
            SetAwake(true);
        }

        m_angularVelocity = w;
        */
    }
    
    /**
      | Get the angular velocity.
      | 
      | -----------
      | @return
      | 
      | the angular velocity in radians/second.
      |
      */
    #[inline] pub fn get_angular_velocity(&self) -> f32 {
        
        todo!();
        /*
            return m_angularVelocity;
        */
    }
    
    /**
      | Get the total mass of the body.
      | 
      | -----------
      | @return
      | 
      | the mass, usually in kilograms (kg).
      |
      */
    #[inline] pub fn get_mass(&self) -> f32 {
        
        todo!();
        /*
            return m_mass;
        */
    }
    
    /**
      | Get the rotational inertia of the body
      | about the local origin.
      | 
      | -----------
      | @return
      | 
      | the rotational inertia, usually in
      | kg-m^2.
      |
      */
    #[inline] pub fn get_inertia(&self) -> f32 {
        
        todo!();
        /*
            return m_I + m_mass * b2Dot(m_sweep.localCenter, m_sweep.localCenter);
        */
    }
    
    /**
      | Get the mass data of the body.
      | 
      | 
      | -----------
      | @return
      | 
      | a struct containing the mass, inertia
      | and center of the body.
      |
      */
    #[inline] pub fn get_mass_data(&self, data: *mut b2MassData)  {
        
        todo!();
        /*
            data->mass = m_mass;
        data->I = m_I + m_mass * b2Dot(m_sweep.localCenter, m_sweep.localCenter);
        data->center = m_sweep.localCenter;
        */
    }
    
    /**
      | Get the world coordinates of a point
      | given the local coordinates.
      | 
      | -----------
      | @param localPoint
      | 
      | a point on the body measured relative
      | the the body's origin.
      | 
      | -----------
      | @return
      | 
      | the same point expressed in world coordinates.
      |
      */
    #[inline] pub fn get_world_point(&self, local_point: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            return b2Mul(m_xf, localPoint);
        */
    }
    
    /**
      | Get the world coordinates of a vector
      | given the local coordinates.
      | 
      | -----------
      | @param localVector
      | 
      | a vector fixed in the body.
      | 
      | -----------
      | @return
      | 
      | the same vector expressed in world coordinates.
      |
      */
    #[inline] pub fn get_world_vector(&self, local_vector: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            return b2Mul(m_xf.q, localVector);
        */
    }
    
    /**
      | Gets a local point relative to the body's
      | origin given a world point.
      | 
      | -----------
      | @param a
      | 
      | point in world coordinates.
      | 
      | -----------
      | @return
      | 
      | the corresponding local point relative
      | to the body's origin.
      |
      */
    #[inline] pub fn get_local_point(&self, world_point: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            return b2MulT(m_xf, worldPoint);
        */
    }
    
    /**
      | Gets a local vector given a world vector.
      | 
      | -----------
      | @param a
      | 
      | vector in world coordinates.
      | 
      | -----------
      | @return
      | 
      | the corresponding local vector.
      |
      */
    #[inline] pub fn get_local_vector(&self, world_vector: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            return b2MulT(m_xf.q, worldVector);
        */
    }
    
    /**
      | Get the world linear velocity of a world
      | point attached to this body.
      | 
      | -----------
      | @param a
      | 
      | point in world coordinates.
      | 
      | -----------
      | @return
      | 
      | the world velocity of a point.
      |
      */
    #[inline] pub fn get_linear_velocity_from_world_point(&self, world_point: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            return m_linearVelocity + b2Cross(m_angularVelocity, worldPoint - m_sweep.c);
        */
    }
    
    /**
      | Get the world velocity of a local point.
      | 
      | -----------
      | @param a
      | 
      | point in local coordinates.
      | 
      | -----------
      | @return
      | 
      | the world velocity of a point.
      |
      */
    #[inline] pub fn get_linear_velocity_from_local_point(&self, local_point: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            return GetLinearVelocityFromWorldPoint(GetWorldPoint(localPoint));
        */
    }
    
    /**
      | Get the linear damping of the body.
      |
      */
    #[inline] pub fn get_linear_damping(&self) -> f32 {
        
        todo!();
        /*
            return m_linearDamping;
        */
    }
    
    /**
      | Set the linear damping of the body.
      |
      */
    #[inline] pub fn set_linear_damping(&mut self, linear_damping: f32)  {
        
        todo!();
        /*
            m_linearDamping = linearDamping;
        */
    }
    
    /**
      | Get the angular damping of the body.
      |
      */
    #[inline] pub fn get_angular_damping(&self) -> f32 {
        
        todo!();
        /*
            return m_angularDamping;
        */
    }
    
    /**
      | Set the angular damping of the body.
      |
      */
    #[inline] pub fn set_angular_damping(&mut self, angular_damping: f32)  {
        
        todo!();
        /*
            m_angularDamping = angularDamping;
        */
    }
    
    /**
      | Get the gravity scale of the body.
      |
      */
    #[inline] pub fn get_gravity_scale(&self) -> f32 {
        
        todo!();
        /*
            return m_gravityScale;
        */
    }
    
    /**
      | Set the gravity scale of the body.
      |
      */
    #[inline] pub fn set_gravity_scale(&mut self, scale: f32)  {
        
        todo!();
        /*
            m_gravityScale = scale;
        */
    }
    
    /**
      | Should this body be treated like a bullet
      | for continuous collision detection?
      |
      */
    #[inline] pub fn set_bullet(&mut self, flag: bool)  {
        
        todo!();
        /*
            if (flag)
        {
            m_flags |= B2_BOXY_E_BULLET_FLAG;
        }
        else
        {
            m_flags &= ~B2_BOXY_E_BULLET_FLAG;
        }
        */
    }
    
    /**
      | Is this body treated like a bullet for
      | continuous collision detection?
      |
      */
    #[inline] pub fn is_bullet(&self) -> bool {
        
        todo!();
        /*
            return (m_flags & B2_BOXY_E_BULLET_FLAG) == B2_BOXY_E_BULLET_FLAG;
        */
    }
    
    /**
      | Set the sleep state of the body. A sleeping
      | body has very low CPU cost.
      | 
      | -----------
      | @param flag
      | 
      | set to true to put body to sleep, false
      | to wake it.
      |
      */
    #[inline] pub fn set_awake(&mut self, flag: bool)  {
        
        todo!();
        /*
            if (flag)
        {
            if ((m_flags & B2_BOXY_E_AWAKE_FLAG) == 0)
            {
                m_flags |= B2_BOXY_E_AWAKE_FLAG;
                m_sleepTime = 0.0f;
            }
        }
        else
        {
            m_flags &= ~B2_BOXY_E_AWAKE_FLAG;
            m_sleepTime = 0.0f;
            m_linearVelocity.SetZero();
            m_angularVelocity = 0.0f;
            m_force.SetZero();
            m_torque = 0.0f;
        }
        */
    }
    
    /**
      | Get the sleeping state of this body.
      | 
      | -----------
      | @return
      | 
      | true if the body is sleeping.
      |
      */
    #[inline] pub fn is_awake(&self) -> bool {
        
        todo!();
        /*
            return (m_flags & B2_BOXY_E_AWAKE_FLAG) == B2_BOXY_E_AWAKE_FLAG;
        */
    }
    
    /**
      | Get the active state of the body.
      |
      */
    #[inline] pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return (m_flags & B2_BOXY_E_ACTIVE_FLAG) == B2_BOXY_E_ACTIVE_FLAG;
        */
    }
    
    /**
      | Set this body to have fixed rotation.
      | This causes the mass to be reset.
      |
      */
    #[inline] pub fn set_fixed_rotation(&mut self, flag: bool)  {
        
        todo!();
        /*
            if (flag)
        {
            m_flags |= B2_BOXY_E_FIXED_ROTATION_FLAG;
        }
        else
        {
            m_flags &= ~B2_BOXY_E_FIXED_ROTATION_FLAG;
        }

        ResetMassData();
        */
    }
    
    /**
      | Does this body have fixed rotation?
      |
      */
    #[inline] pub fn is_fixed_rotation(&self) -> bool {
        
        todo!();
        /*
            return (m_flags & B2_BOXY_E_FIXED_ROTATION_FLAG) == B2_BOXY_E_FIXED_ROTATION_FLAG;
        */
    }
    
    /**
      | You can disable sleeping on this body.
      | If you disable sleeping, the body will
      | be woken.
      |
      */
    #[inline] pub fn set_sleeping_allowed(&mut self, flag: bool)  {
        
        todo!();
        /*
            if (flag)
        {
            m_flags |= B2_BOXY_E_AUTO_SLEEP_FLAG;
        }
        else
        {
            m_flags &= ~B2_BOXY_E_AUTO_SLEEP_FLAG;
            SetAwake(true);
        }
        */
    }
    
    /**
      | Is this body allowed to sleep
      |
      */
    #[inline] pub fn is_sleeping_allowed(&self) -> bool {
        
        todo!();
        /*
            return (m_flags & B2_BOXY_E_AUTO_SLEEP_FLAG) == B2_BOXY_E_AUTO_SLEEP_FLAG;
        */
    }
    
    /**
      | Get the list of all fixtures attached
      | to this body.
      |
      */
    #[inline] pub fn get_fixture_list_mut(&mut self) -> *mut b2Fixture {
        
        todo!();
        /*
            return m_fixtureList;
        */
    }
    
    #[inline] pub fn get_fixture_list(&self) -> *const b2Fixture {
        
        todo!();
        /*
            return m_fixtureList;
        */
    }
    
    /**
      | Get the list of all joints attached to
      | this body.
      |
      */
    #[inline] pub fn get_joint_list_mut(&mut self) -> *mut b2JointEdge {
        
        todo!();
        /*
            return m_jointList;
        */
    }
    
    #[inline] pub fn get_joint_list(&self) -> *const b2JointEdge {
        
        todo!();
        /*
            return m_jointList;
        */
    }
    
    /**
      | Get the list of all contacts attached
      | to this body.
      | 
      | -----------
      | @warning
      | 
      | this list changes during the time step
      | and you may miss some collisions if you
      | don't use b2ContactListener.
      |
      */
    #[inline] pub fn get_contact_list_mut(&mut self) -> *mut b2ContactEdge {
        
        todo!();
        /*
            return m_contactList;
        */
    }
    
    #[inline] pub fn get_contact_list(&self) -> *const b2ContactEdge {
        
        todo!();
        /*
            return m_contactList;
        */
    }
    
    /**
      | Get the next body in the world's body
      | list.
      |
      */
    #[inline] pub fn get_next_mut(&mut self) -> *mut b2Body {
        
        todo!();
        /*
            return m_next;
        */
    }
    
    #[inline] pub fn get_next(&self) -> *const b2Body {
        
        todo!();
        /*
            return m_next;
        */
    }
    
    /**
      | Set the user data. Use this to store your
      | application specific data.
      |
      */
    #[inline] pub fn set_user_data(&mut self, data: *mut c_void)  {
        
        todo!();
        /*
            m_userData = data;
        */
    }
    
    /**
      | Get the user data pointer that was provided
      | in the body definition.
      |
      */
    #[inline] pub fn get_user_data(&self)  {
        
        todo!();
        /*
            return m_userData;
        */
    }
    
    /**
      | Apply a force at a world point. If the
      | force is not applied at the center of
      | mass, it will generate a torque and affect
      | the angular velocity. This wakes up
      | the body.
      | 
      | -----------
      | @param force
      | 
      | the world force vector, usually in Newtons
      | (N).
      | ----------
      | @param point
      | 
      | the world position of the point of application.
      |
      */
    #[inline] pub fn apply_force(&mut self, 
        force: &b2Vec2,
        point: &b2Vec2)  {
        
        todo!();
        /*
            if (m_type != b2_dynamicBody)
        {
            return;
        }

        if (IsAwake() == false)
        {
            SetAwake(true);
        }

        m_force += force;
        m_torque += b2Cross(point - m_sweep.c, force);
        */
    }
    
    /**
      | Apply a force to the center of mass. This
      | wakes up the body.
      | 
      | -----------
      | @param force
      | 
      | the world force vector, usually in Newtons
      | (N).
      |
      */
    #[inline] pub fn apply_force_to_center(&mut self, force: &b2Vec2)  {
        
        todo!();
        /*
            if (m_type != b2_dynamicBody)
        {
            return;
        }

        if (IsAwake() == false)
        {
            SetAwake(true);
        }

        m_force += force;
        */
    }
    
    /**
      | Apply a torque. This affects the angular
      | velocity without affecting the linear
      | velocity of the center of mass.
      | 
      | This wakes up the body.
      | 
      | -----------
      | @param torque
      | 
      | about the z-axis (out of the screen),
      | usually in N-m.
      |
      */
    #[inline] pub fn apply_torque(&mut self, torque: f32)  {
        
        todo!();
        /*
            if (m_type != b2_dynamicBody)
        {
            return;
        }

        if (IsAwake() == false)
        {
            SetAwake(true);
        }

        m_torque += torque;
        */
    }
    
    /**
      | Apply an impulse at a point. This immediately
      | modifies the velocity. It also modifies
      | the angular velocity if the point of
      | application is not at the center of mass.
      | This wakes up the body.
      | 
      | -----------
      | @param impulse
      | 
      | the world impulse vector, usually in
      | N-seconds or kg-m/s.
      | ----------
      | @param point
      | 
      | the world position of the point of application.
      |
      */
    #[inline] pub fn apply_linear_impulse(&mut self, 
        impulse: &b2Vec2,
        point:   &b2Vec2)  {
        
        todo!();
        /*
            if (m_type != b2_dynamicBody)
        {
            return;
        }

        if (IsAwake() == false)
        {
            SetAwake(true);
        }
        m_linearVelocity += m_invMass * impulse;
        m_angularVelocity += m_invI * b2Cross(point - m_sweep.c, impulse);
        */
    }
    
    /**
      | Apply an angular impulse.
      | 
      | -----------
      | @param impulse
      | 
      | the angular impulse in units of kg*m*m/s
      |
      */
    #[inline] pub fn apply_angular_impulse(&mut self, impulse: f32)  {
        
        todo!();
        /*
            if (m_type != b2_dynamicBody)
        {
            return;
        }

        if (IsAwake() == false)
        {
            SetAwake(true);
        }
        m_angularVelocity += m_invI * impulse;
        */
    }
    
    #[inline] pub fn synchronize_transform(&mut self)  {
        
        todo!();
        /*
            m_xf.q.Set(m_sweep.a);
        m_xf.p = m_sweep.c - b2Mul(m_xf.q, m_sweep.localCenter);
        */
    }
    
    #[inline] pub fn advance(&mut self, alpha: f32)  {
        
        todo!();
        /*
            // Advance to the new safe time. This doesn't sync the broad-phase.
        m_sweep.Advance(alpha);
        m_sweep.c = m_sweep.c0;
        m_sweep.a = m_sweep.a0;
        m_xf.q.Set(m_sweep.a);
        m_xf.p = m_sweep.c - b2Mul(m_xf.q, m_sweep.localCenter);
        */
    }
    
    /**
      | Get the parent world of this body.
      |
      */
    #[inline] pub fn get_world_mut(&mut self) -> *mut b2World {
        
        todo!();
        /*
            return m_world;
        */
    }
    
    #[inline] pub fn get_world(&self) -> *const b2World {
        
        todo!();
        /*
            return m_world;
        */
    }
    
    pub fn new(
        bd:    *const b2BodyDef,
        world: *mut b2World) -> Self {
    
        todo!();
        /*


            b2Assert(bd->position.IsValid());
        b2Assert(bd->linearVelocity.IsValid());
        b2Assert(b2IsValid(bd->angle));
        b2Assert(b2IsValid(bd->angularVelocity));
        b2Assert(b2IsValid(bd->angularDamping) && bd->angularDamping >= 0.0f);
        b2Assert(b2IsValid(bd->linearDamping) && bd->linearDamping >= 0.0f);

        m_flags = 0;

        if (bd->bullet)
        {
            m_flags |= B2_BOXY_E_BULLET_FLAG;
        }
        if (bd->fixedRotation)
        {
            m_flags |= B2_BOXY_E_FIXED_ROTATION_FLAG;
        }
        if (bd->allowSleep)
        {
            m_flags |= B2_BOXY_E_AUTO_SLEEP_FLAG;
        }
        if (bd->awake)
        {
            m_flags |= B2_BOXY_E_AWAKE_FLAG;
        }
        if (bd->active)
        {
            m_flags |= B2_BOXY_E_ACTIVE_FLAG;
        }

        m_world = world;

        m_xf.p = bd->position;
        m_xf.q.Set(bd->angle);

        m_sweep.localCenter.SetZero();
        m_sweep.c0 = m_xf.p;
        m_sweep.c = m_xf.p;
        m_sweep.a0 = bd->angle;
        m_sweep.a = bd->angle;
        m_sweep.alpha0 = 0.0f;

        m_jointList = NULL;
        m_contactList = NULL;
        m_prev = NULL;
        m_next = NULL;

        m_linearVelocity = bd->linearVelocity;
        m_angularVelocity = bd->angularVelocity;

        m_linearDamping = bd->linearDamping;
        m_angularDamping = bd->angularDamping;
        m_gravityScale = bd->gravityScale;

        m_force.SetZero();
        m_torque = 0.0f;

        m_sleepTime = 0.0f;

        m_type = bd->type;

        if (m_type == b2_dynamicBody)
        {
            m_mass = 1.0f;
            m_invMass = 1.0f;
        }
        else
        {
            m_mass = 0.0f;
            m_invMass = 0.0f;
        }

        m_I = 0.0f;
        m_invI = 0.0f;

        m_userData = bd->userData;

        m_fixtureList = NULL;
        m_fixtureCount = 0;
        */
    }
    
    /**
      | Set the type of this body. This may alter
      | the mass and velocity.
      |
      */
    pub fn set_type(&mut self, ty: b2BodyType)  {
        
        todo!();
        /*
            b2Assert(m_world->IsLocked() == false);
        if (m_world->IsLocked() == true)
        {
            return;
        }

        if (m_type == type)
        {
            return;
        }

        m_type = type;

        ResetMassData();

        if (m_type == b2_staticBody)
        {
            m_linearVelocity.SetZero();
            m_angularVelocity = 0.0f;
            m_sweep.a0 = m_sweep.a;
            m_sweep.c0 = m_sweep.c;
            SynchronizeFixtures();
        }

        SetAwake(true);

        m_force.SetZero();
        m_torque = 0.0f;

        // Since the body type changed, we need to flag contacts for filtering.
        for (b2Fixture* f = m_fixtureList; f; f = f->m_next)
        {
            f->Refilter();
        }
        */
    }
    
    /**
      | Creates a fixture and attach it to this
      | body. Use this function if you need to
      | set some fixture parameters, like friction.
      | Otherwise you can create the fixture
      | directly from a shape. If the density
      | is non-zero, this function automatically
      | updates the mass of the body. Contacts
      | are not created until the next time step.
      | 
      | -----------
      | @param def
      | 
      | the fixture definition.
      | 
      | -----------
      | @warning
      | 
      | This function is locked during callbacks.
      |
      */
    pub fn create_fixture(&mut self, def: *const b2FixtureDef) -> *mut b2Fixture {
        
        todo!();
        /*
            b2Assert(m_world->IsLocked() == false);
        if (m_world->IsLocked() == true)
        {
            return NULL;
        }

        b2BlockAllocator* allocator = &m_world->m_blockAllocator;

        void* memory = allocator->Allocate(sizeof(b2Fixture));
        b2Fixture* fixture = new (memory) b2Fixture;
        fixture->Create(allocator, this, def);

        if (m_flags & B2_BOXY_E_ACTIVE_FLAG)
        {
            b2BroadPhase* broadPhase = &m_world->m_contactManager.m_broadPhase;
            fixture->CreateProxies(broadPhase, m_xf);
        }

        fixture->m_next = m_fixtureList;
        m_fixtureList = fixture;
        ++m_fixtureCount;

        fixture->m_body = this;

        // Adjust mass properties if needed.
        if (fixture->m_density > 0.0f)
        {
            ResetMassData();
        }

        // Let the world know we have a new fixture. This will cause new contacts
        // to be created at the beginning of the next time step.
        m_world->m_flags |= b2World::e_newFixture;

        return fixture;
        */
    }
    
    /**
      | Creates a fixture from a shape and attach
      | it to this body. This is a convenience
      | function. Use b2FixtureDef if you need
      | to set parameters like friction, restitution,
      | user data, or filtering. If the density
      | is non-zero, this function automatically
      | updates the mass of the body.
      | 
      | -----------
      | @param shape
      | 
      | the shape to be cloned.
      | ----------
      | @param density
      | 
      | the shape density (set to zero for static
      | bodies).
      | 
      | -----------
      | @warning
      | 
      | This function is locked during callbacks.
      |
      */
    pub fn create_fixture_with_density(
        &mut self, 
        shape:   *const b2Shape,
        density: f32

    ) -> *mut b2Fixture {
        
        todo!();
        /*
            b2FixtureDef def;
        def.shape = shape;
        def.density = density;

        return CreateFixture(&def);
        */
    }
    
    /**
      | Destroy a fixture. This removes the
      | fixture from the broad-phase and destroys
      | all contacts associated with this fixture.
      | This will automatically adjust the
      | mass of the body if the body is dynamic
      | and the fixture has positive density.
      | 
      | All fixtures attached to a body are implicitly
      | destroyed when the body is destroyed.
      | 
      | -----------
      | @param fixture
      | 
      | the fixture to be removed.
      | 
      | -----------
      | @warning
      | 
      | This function is locked during callbacks.
      |
      */
    pub fn destroy_fixture(&mut self, fixture: *mut b2Fixture)  {
        
        todo!();
        /*
            b2Assert(m_world->IsLocked() == false);
        if (m_world->IsLocked() == true)
        {
            return;
        }

        b2Assert(fixture->m_body == this);

        // Remove the fixture from this body's singly linked list.
        b2Assert(m_fixtureCount > 0);
        b2Fixture** node = &m_fixtureList;
        bool found = false;
        while (*node != NULL)
        {
            if (*node == fixture)
            {
                *node = fixture->m_next;
                found = true;
                break;
            }

            node = &(*node)->m_next;
        }

        // You tried to remove a shape that is not attached to this body.
        b2Assert(found);
        ignoreUnused (found);

        // Destroy any contacts associated with the fixture.
        b2ContactEdge* edge = m_contactList;
        while (edge)
        {
            b2Contact* c = edge->contact;
            edge = edge->next;

            b2Fixture* fixtureA = c->GetFixtureA();
            b2Fixture* fixtureB = c->GetFixtureB();

            if (fixture == fixtureA || fixture == fixtureB)
            {
                // This destroys the contact and removes it from
                // this body's contact list.
                m_world->m_contactManager.Destroy(c);
            }
        }

        b2BlockAllocator* allocator = &m_world->m_blockAllocator;

        if (m_flags & B2_BOXY_E_ACTIVE_FLAG)
        {
            b2BroadPhase* broadPhase = &m_world->m_contactManager.m_broadPhase;
            fixture->DestroyProxies(broadPhase);
        }

        fixture->Destroy(allocator);
        fixture->m_body = NULL;
        fixture->m_next = NULL;
        fixture->~b2Fixture();
        allocator->Free(fixture, sizeof(b2Fixture));

        --m_fixtureCount;

        // Reset the mass data.
        ResetMassData();
        */
    }
    
    /**
      | This resets the mass properties to the
      | sum of the mass properties of the fixtures.
      | 
      | This normally does not need to be called
      | unless you called SetMassData to override
      | the mass and you later want to reset the
      | mass.
      |
      */
    pub fn reset_mass_data(&mut self)  {
        
        todo!();
        /*
            // Compute mass data from shapes. Each shape has its own density.
        m_mass = 0.0f;
        m_invMass = 0.0f;
        m_I = 0.0f;
        m_invI = 0.0f;
        m_sweep.localCenter.SetZero();

        // Static and kinematic bodies have zero mass.
        if (m_type == b2_staticBody || m_type == b2_kinematicBody)
        {
            m_sweep.c0 = m_xf.p;
            m_sweep.c = m_xf.p;
            m_sweep.a0 = m_sweep.a;
            return;
        }

        b2Assert(m_type == b2_dynamicBody);

        // Accumulate mass over all fixtures.
        b2Vec2 localCenter = b2Vec2_zero;
        for (b2Fixture* f = m_fixtureList; f; f = f->m_next)
        {
            if (f->m_density == 0.0f)
            {
                continue;
            }

            b2MassData massData;
            f->GetMassData(&massData);
            m_mass += massData.mass;
            localCenter += massData.mass * massData.center;
            m_I += massData.I;
        }

        // Compute center of mass.
        if (m_mass > 0.0f)
        {
            m_invMass = 1.0f / m_mass;
            localCenter *= m_invMass;
        }
        else
        {
            // Force all dynamic bodies to have a positive mass.
            m_mass = 1.0f;
            m_invMass = 1.0f;
        }

        if (m_I > 0.0f && (m_flags & B2_BOXY_E_FIXED_ROTATION_FLAG) == 0)
        {
            // Center the inertia about the center of mass.
            m_I -= m_mass * b2Dot(localCenter, localCenter);
            b2Assert(m_I > 0.0f);
            m_invI = 1.0f / m_I;

        }
        else
        {
            m_I = 0.0f;
            m_invI = 0.0f;
        }

        // Move center of mass.
        b2Vec2 oldCenter = m_sweep.c;
        m_sweep.localCenter = localCenter;
        m_sweep.c0 = m_sweep.c = b2Mul(m_xf, m_sweep.localCenter);

        // Update center of mass velocity.
        m_linearVelocity += b2Cross(m_angularVelocity, m_sweep.c - oldCenter);
        */
    }
    
    /**
      | Set the mass properties to override
      | the mass properties of the fixtures.
      | 
      | -----------
      | @note
      | 
      | that this changes the center of mass
      | position.
      | ----------
      | @note
      | 
      | that creating or destroying fixtures
      | can also alter the mass.
      | 
      | This function has no effect if the body
      | isn't dynamic.
      | 
      | -----------
      | @param massData
      | 
      | the mass properties.
      |
      */
    pub fn set_mass_data(&mut self, mass_data: *const b2MassData)  {
        
        todo!();
        /*
            b2Assert(m_world->IsLocked() == false);
        if (m_world->IsLocked() == true)
        {
            return;
        }

        if (m_type != b2_dynamicBody)
        {
            return;
        }

        m_invMass = 0.0f;
        m_I = 0.0f;
        m_invI = 0.0f;

        m_mass = massData->mass;
        if (m_mass <= 0.0f)
        {
            m_mass = 1.0f;
        }

        m_invMass = 1.0f / m_mass;

        if (massData->I > 0.0f && (m_flags & b2Body::B2_BOXY_E_FIXED_ROTATION_FLAG) == 0)
        {
            m_I = massData->I - m_mass * b2Dot(massData->center, massData->center);
            b2Assert(m_I > 0.0f);
            m_invI = 1.0f / m_I;
        }

        // Move center of mass.
        b2Vec2 oldCenter = m_sweep.c;
        m_sweep.localCenter =  massData->center;
        m_sweep.c0 = m_sweep.c = b2Mul(m_xf, m_sweep.localCenter);

        // Update center of mass velocity.
        m_linearVelocity += b2Cross(m_angularVelocity, m_sweep.c - oldCenter);
        */
    }
    
    /**
      | This is used to prevent connected bodies
      | from colliding. It may lie, depending
      | on the collideConnected flag.
      |
      */
    pub fn should_collide(&self, other: *const b2Body) -> bool {
        
        todo!();
        /*
            // At least one body should be dynamic.
        if (m_type != b2_dynamicBody && other->m_type != b2_dynamicBody)
        {
            return false;
        }

        // Does a joint prevent collision?
        for (b2JointEdge* jn = m_jointList; jn; jn = jn->next)
        {
            if (jn->other == other)
            {
                if (jn->joint->m_collideConnected == false)
                {
                    return false;
                }
            }
        }

        return true;
        */
    }
    
    /**
      | Set the position of the body's origin
      | and rotation. This breaks any contacts
      | and wakes the other bodies. Manipulating
      | a body's transform may cause non-physical
      | behavior.
      | 
      | -----------
      | @param position
      | 
      | the world position of the body's local
      | origin.
      | ----------
      | @param angle
      | 
      | the world rotation in radians.
      |
      */
    pub fn set_transform(&mut self, 
        position: &b2Vec2,
        angle:    f32)  {
        
        todo!();
        /*
            b2Assert(m_world->IsLocked() == false);
        if (m_world->IsLocked() == true)
        {
            return;
        }

        m_xf.q.Set(angle);
        m_xf.p = position;

        m_sweep.c = b2Mul(m_xf, m_sweep.localCenter);
        m_sweep.a = angle;

        m_sweep.c0 = m_sweep.c;
        m_sweep.a0 = angle;

        b2BroadPhase* broadPhase = &m_world->m_contactManager.m_broadPhase;
        for (b2Fixture* f = m_fixtureList; f; f = f->m_next)
        {
            f->Synchronize(broadPhase, m_xf, m_xf);
        }

        m_world->m_contactManager.FindNewContacts();
        */
    }
    
    pub fn synchronize_fixtures(&mut self)  {
        
        todo!();
        /*
            b2Transform xf1;
        xf1.q.Set(m_sweep.a0);
        xf1.p = m_sweep.c0 - b2Mul(xf1.q, m_sweep.localCenter);

        b2BroadPhase* broadPhase = &m_world->m_contactManager.m_broadPhase;
        for (b2Fixture* f = m_fixtureList; f; f = f->m_next)
        {
            f->Synchronize(broadPhase, xf1, m_xf);
        }
        */
    }
    
    /**
      | Set the active state of the body. An inactive
      | body is not simulated and cannot be collided
      | with or woken up.
      | 
      | If you pass a flag of true, all fixtures
      | will be added to the broad-phase.
      | 
      | If you pass a flag of false, all fixtures
      | will be removed from the broad-phase
      | and all contacts will be destroyed.
      | 
      | Fixtures and joints are otherwise unaffected.
      | You may continue to create/destroy
      | fixtures and joints on inactive bodies.
      | 
      | Fixtures on an inactive body are implicitly
      | inactive and will not participate in
      | collisions, ray-casts, or queries.
      | 
      | Joints connected to an inactive body
      | are implicitly inactive. An inactive
      | body is still owned by a b2World object
      | and remains in the body list.
      |
      */
    pub fn set_active(&mut self, flag: bool)  {
        
        todo!();
        /*
            b2Assert(m_world->IsLocked() == false);

        if (flag == IsActive())
        {
            return;
        }

        if (flag)
        {
            m_flags |= B2_BOXY_E_ACTIVE_FLAG;

            // Create all proxies.
            b2BroadPhase* broadPhase = &m_world->m_contactManager.m_broadPhase;
            for (b2Fixture* f = m_fixtureList; f; f = f->m_next)
            {
                f->CreateProxies(broadPhase, m_xf);
            }

            // Contacts are created the next time step.
        }
        else
        {
            m_flags &= ~B2_BOXY_E_ACTIVE_FLAG;

            // Destroy all proxies.
            b2BroadPhase* broadPhase = &m_world->m_contactManager.m_broadPhase;
            for (b2Fixture* f = m_fixtureList; f; f = f->m_next)
            {
                f->DestroyProxies(broadPhase);
            }

            // Destroy the attached contacts.
            b2ContactEdge* ce = m_contactList;
            while (ce)
            {
                b2ContactEdge* ce0 = ce;
                ce = ce->next;
                m_world->m_contactManager.Destroy(ce0->contact);
            }
            m_contactList = NULL;
        }
        */
    }
    
    /**
      | Dump this body to a log file
      |
      */
    pub fn dump(&mut self)  {
        
        todo!();
        /*
            int32 bodyIndex = m_islandIndex;

        b2Log("{\n");
        b2Log("  b2BodyDef bd;\n");
        b2Log("  bd.type = b2BodyType(%d);\n", m_type);
        b2Log("  bd.position.Set(%.15lef, %.15lef);\n", m_xf.p.x, m_xf.p.y);
        b2Log("  bd.angle = %.15lef;\n", m_sweep.a);
        b2Log("  bd.linearVelocity.Set(%.15lef, %.15lef);\n", m_linearVelocity.x, m_linearVelocity.y);
        b2Log("  bd.angularVelocity = %.15lef;\n", m_angularVelocity);
        b2Log("  bd.linearDamping = %.15lef;\n", m_linearDamping);
        b2Log("  bd.angularDamping = %.15lef;\n", m_angularDamping);
        b2Log("  bd.allowSleep = bool(%d);\n", m_flags & B2_BOXY_E_AUTO_SLEEP_FLAG);
        b2Log("  bd.awake = bool(%d);\n", m_flags & B2_BOXY_E_AWAKE_FLAG);
        b2Log("  bd.fixedRotation = bool(%d);\n", m_flags & B2_BOXY_E_FIXED_ROTATION_FLAG);
        b2Log("  bd.bullet = bool(%d);\n", m_flags & B2_BOXY_E_BULLET_FLAG);
        b2Log("  bd.active = bool(%d);\n", m_flags & B2_BOXY_E_ACTIVE_FLAG);
        b2Log("  bd.gravityScale = %.15lef;\n", m_gravityScale);
        b2Log("  bodies[%d] = m_world->CreateBody(&bd);\n", m_islandIndex);
        b2Log("\n");
        for (b2Fixture* f = m_fixtureList; f; f = f->m_next)
        {
            b2Log("  {\n");
            f->Dump(bodyIndex);
            b2Log("  }\n");
        }
        b2Log("}\n");
        */
    }
}

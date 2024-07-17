crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2MouseJoint.h]

/**
   Mouse joint definition. This requires a world
   target point, tuning parameters, and the time
   step.
  */
pub struct b2MouseJointDef {

    base:          b2JointDef,

    /**
      | The initial world target point. This is
      | assumed to coincide with the body anchor
      | initially.
      */
    target:        b2Vec2,

    /**
      | The maximum constraint force that can be
      | exerted to move the candidate body. Usually
      | you will express as some multiple of the
      | weight (multiplier * mass * gravity).
      */
    max_force:     f32,

    /**
      The response speed.
      */
    frequency_hz:  f32,

    /**
      The damping ratio. 0 = no damping,
      1 = critical damping.
      */
    damping_ratio: f32,
}

impl Default for b2MouseJointDef {
    
    fn default() -> Self {
        todo!();
        /*

            type = e_mouseJoint;
            target.Set(0.0f, 0.0f);
            maxForce = 0.0f;
            frequencyHz = 5.0f;
            dampingRatio = 0.7f
        */
    }
}

/**
  | A mouse joint is used to make a point on
  | a body track a specified world point. This
  | a soft constraint with a maximum force. This
  | allows the constraint to stretch and without
  | applying huge forces.
  |
  | NOTE: this joint is not documented in the
  | manual because it was developed to be used in
  | the testbed. If you want to learn how to use
  | the mouse joint, look at the testbed.
  */
pub struct b2MouseJoint {
    base:          b2Joint,
    local_anchorb: b2Vec2,
    targeta:       b2Vec2,
    frequency_hz:  f32,
    damping_ratio: f32,
    beta:          f32,

    /* ----------------- * Solver shared  ----------------- */
    impulse:       b2Vec2,

    max_force:     f32,
    gamma:         f32,

    /* ------------------ * Solver temp  ------------------ */
    indexa:        i32,

    indexb:        i32,
    rb:            b2Vec2,
    local_centerb: b2Vec2,
    inv_massb:     f32,
    invib:         f32,
    mass:          b2Mat22,
    c:             b2Vec2,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2MouseJoint.cpp]
impl b2MouseJoint {

    /**
      | The mouse joint does not support dumping.
      |
      */
    pub fn dump(&mut self)  {
        
        todo!();
        /*
            b2Log("Mouse joint dumping is not supported.\n");
        */
    }

    /**
      | p = attached point, m = mouse point
      | C = p - m
      | Cdot = v
      |      = v + cross(w, r)
      | J = [I r_skew]
      | Identity used:
      | w k % (rx i + ry j) = w * (-ry i + rx j)
      */
    pub fn new(def: *const b2MouseJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            b2Assert(def->target.IsValid());
        b2Assert(b2IsValid(def->maxForce) && def->maxForce >= 0.0f);
        b2Assert(b2IsValid(def->frequencyHz) && def->frequencyHz >= 0.0f);
        b2Assert(b2IsValid(def->dampingRatio) && def->dampingRatio >= 0.0f);

        m_targetA = def->target;
        m_localAnchorB = b2MulT(m_bodyB->GetTransform(), m_targetA);

        m_maxForce = def->maxForce;
        m_impulse.SetZero();

        m_frequencyHz = def->frequencyHz;
        m_dampingRatio = def->dampingRatio;

        m_beta = 0.0f;
        m_gamma = 0.0f;
        */
    }

    /**
      | Use this to update the target point.
      |
      */
    pub fn set_target(&mut self, target: &b2Vec2)  {
        
        todo!();
        /*
            if (m_bodyB->IsAwake() == false)
        {
            m_bodyB->SetAwake(true);
        }
        m_targetA = target;
        */
    }

    pub fn get_target(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_targetA;
        */
    }

    /**
      | Set/get the maximum force in Newtons.
      |
      */
    pub fn set_max_force(&mut self, force: f32)  {
        
        todo!();
        /*
            m_maxForce = force;
        */
    }

    pub fn get_max_force(&self) -> f32 {
        
        todo!();
        /*
            return m_maxForce;
        */
    }

    /**
      | Set/get the frequency in Hertz.
      |
      */
    pub fn set_frequency(&mut self, hz: f32)  {
        
        todo!();
        /*
            m_frequencyHz = hz;
        */
    }

    pub fn get_frequency(&self) -> f32 {
        
        todo!();
        /*
            return m_frequencyHz;
        */
    }

    /**
      | Set/get the damping ratio (dimensionless).
      |
      */
    pub fn set_damping_ratio(&mut self, ratio: f32)  {
        
        todo!();
        /*
            m_dampingRatio = ratio;
        */
    }

    pub fn get_damping_ratio(&self) -> f32 {
        
        todo!();
        /*
            return m_dampingRatio;
        */
    }

    pub fn init_velocity_constraints(&mut self, data: &b2SolverData)  {
        
        todo!();
        /*
            m_indexB = m_bodyB->m_islandIndex;
        m_localCenterB = m_bodyB->m_sweep.localCenter;
        m_invMassB = m_bodyB->m_invMass;
        m_invIB = m_bodyB->m_invI;

        b2Vec2 cB = data.positions[m_indexB].c;
        float32 aB = data.positions[m_indexB].a;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        b2Rot qB(aB);

        float32 mass = m_bodyB->GetMass();

        // Frequency
        float32 omega = 2.0f * b2_pi * m_frequencyHz;

        // Damping coefficient
        float32 d = 2.0f * mass * m_dampingRatio * omega;

        // Spring stiffness
        float32 k = mass * (omega * omega);

        // magic formulas
        // gamma has units of inverse mass.
        // beta has units of inverse time.
        float32 h = data.step.dt;
        b2Assert(d + h * k > b2_epsilon);
        m_gamma = h * (d + h * k);
        if (m_gamma != 0.0f)
        {
            m_gamma = 1.0f / m_gamma;
        }
        m_beta = h * k * m_gamma;

        // Compute the effective mass matrix.
        m_rB = b2Mul(qB, m_localAnchorB - m_localCenterB);

        // K    = [(1/m1 + 1/m2) * eye(2) - skew(r1) * invI1 * skew(r1) - skew(r2) * invI2 * skew(r2)]
        //      = [1/m1+1/m2     0    ] + invI1 * [r1.y*r1.y -r1.x*r1.y] + invI2 * [r1.y*r1.y -r1.x*r1.y]
        //        [    0     1/m1+1/m2]           [-r1.x*r1.y r1.x*r1.x]           [-r1.x*r1.y r1.x*r1.x]
        b2Mat22 K;
        K.ex.x = m_invMassB + m_invIB * m_rB.y * m_rB.y + m_gamma;
        K.ex.y = -m_invIB * m_rB.x * m_rB.y;
        K.ey.x = K.ex.y;
        K.ey.y = m_invMassB + m_invIB * m_rB.x * m_rB.x + m_gamma;

        m_mass = K.GetInverse();

        m_C = cB + m_rB - m_targetA;
        m_C *= m_beta;

        // Cheat with some damping
        wB *= 0.98f;

        if (data.step.warmStarting)
        {
            m_impulse *= data.step.dtRatio;
            vB += m_invMassB * m_impulse;
            wB += m_invIB * b2Cross(m_rB, m_impulse);
        }
        else
        {
            m_impulse.SetZero();
        }

        data.velocities[m_indexB].v = vB;
        data.velocities[m_indexB].w = wB;
        */
    }

    pub fn solve_velocity_constraints(&mut self, data: &b2SolverData)  {
        
        todo!();
        /*
            b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        // Cdot = v + cross(w, r)
        b2Vec2 Cdot = vB + b2Cross(wB, m_rB);
        b2Vec2 impulse = b2Mul(m_mass, -(Cdot + m_C + m_gamma * m_impulse));

        b2Vec2 oldImpulse = m_impulse;
        m_impulse += impulse;
        float32 maxImpulse = data.step.dt * m_maxForce;
        if (m_impulse.LengthSquared() > maxImpulse * maxImpulse)
        {
            m_impulse *= maxImpulse / m_impulse.Length();
        }
        impulse = m_impulse - oldImpulse;

        vB += m_invMassB * impulse;
        wB += m_invIB * b2Cross(m_rB, impulse);

        data.velocities[m_indexB].v = vB;
        data.velocities[m_indexB].w = wB;
        */
    }

    pub fn solve_position_constraints(&mut self, data: &b2SolverData) -> bool {
        
        todo!();
        /*
            B2_NOT_USED(data);
        return true;
        */
    }

    /**
      | Implements b2Joint.
      |
      */
    pub fn get_anchora(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_targetA;
        */
    }

    /**
      | Implements b2Joint.
      |
      */
    pub fn get_anchorb(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_bodyB->GetWorldPoint(m_localAnchorB);
        */
    }

    /**
      | Implements b2Joint.
      |
      */
    pub fn get_reaction_force(&self, inv_dt: f32) -> b2Vec2 {
        
        todo!();
        /*
            return inv_dt * m_impulse;
        */
    }

    /**
      | Implements b2Joint.
      |
      */
    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            return inv_dt * 0.0f;
        */
    }
}

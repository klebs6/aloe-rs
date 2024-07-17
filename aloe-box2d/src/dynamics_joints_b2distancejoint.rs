crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2DistanceJoint.h]

/**
  | Distance joint definition. This requires
  | defining an anchor point on both bodies
  | and the non-zero length of the distance
  | joint. The definition uses local anchor
  | points so that the initial configuration
  | can violate the constraint slightly.
  | This helps when saving and loading a
  | game.
  | 
  | -----------
  | @warning
  | 
  | Do not use a zero or short length.
  |
  */
pub struct b2DistanceJointDef {

    base:          b2JointDef,

    /**
      | The local anchor point relative to bodyA's
      | origin.
      |
      */
    local_anchora: b2Vec2,

    /**
      | The local anchor point relative to bodyB's
      | origin.
      |
      */
    local_anchorb: b2Vec2,

    /**
      | The natural length between the anchor
      | points.
      |
      */
    length:        f32,

    /**
      | The mass-spring-damper frequency
      | in Hertz. A value of 0 disables softness.
      |
      */
    frequency_hz:  f32,

    /**
      | The damping ratio. 0 = no damping, 1 =
      | critical damping.
      |
      */
    damping_ratio: f32,
}

impl Default for b2DistanceJointDef {
    
    fn default() -> Self {
        todo!();
        /*


            type = e_distanceJoint;
            localAnchorA.Set(0.0f, 0.0f);
            localAnchorB.Set(0.0f, 0.0f);
            length = 1.0f;
            frequencyHz = 0.0f;
            dampingRatio = 0.0f
        */
    }
}

/**
  | A distance joint constrains two points on two
  | bodies to remain at a fixed distance from each
  | other. You can view this as a massless, rigid
  | rod.
  */
pub struct b2DistanceJoint {
    base:          b2Joint,
    frequency_hz:  f32,
    damping_ratio: f32,
    bias:          f32,

    /**
       Solver shared
      */
    local_anchora: b2Vec2,

    local_anchorb: b2Vec2,
    gamma:         f32,
    impulse:       f32,
    length:        f32,

    /**
       Solver temp
      */
    indexa:        i32,

    indexb:        i32,
    u:             b2Vec2,
    ra:            b2Vec2,
    rb:            b2Vec2,
    local_centera: b2Vec2,
    local_centerb: b2Vec2,
    inv_massa:     f32,
    inv_massb:     f32,
    invia:         f32,
    invib:         f32,
    mass:          f32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2DistanceJoint.cpp]
impl b2DistanceJoint {
    
    /**
      | The local anchor point relative to bodyA's
      | origin.
      |
      */
    pub fn get_local_anchora(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_localAnchorA;
        */
    }

    /**
      | The local anchor point relative to bodyB's
      | origin.
      |
      */
    pub fn get_local_anchorb(&self) -> &b2Vec2 {
        
        todo!();
        /*
            return m_localAnchorB;
        */
    }

    /**
      | Set/get the natural length.
      | 
      | Manipulating the length can lead to
      | non-physical behavior when the frequency
      | is zero.
      |
      */
    #[inline] pub fn set_length(&mut self, length: f32)  {
        
        todo!();
        /*
            m_length = length;
        */
    }

    #[inline] pub fn get_length(&self) -> f32 {
        
        todo!();
        /*
            return m_length;
        */
    }

    /**
      | Set/get frequency in Hz.
      |
      */
    #[inline] pub fn set_frequency(&mut self, hz: f32)  {
        
        todo!();
        /*
            m_frequencyHz = hz;
        */
    }

    #[inline] pub fn get_frequency(&self) -> f32 {
        
        todo!();
        /*
            return m_frequencyHz;
        */
    }

    /**
      | Set/get damping ratio.
      |
      */
    #[inline] pub fn set_damping_ratio(&mut self, ratio: f32)  {
        
        todo!();
        /*
            m_dampingRatio = ratio;
        */
    }

    #[inline] pub fn get_damping_ratio(&self) -> f32 {
        
        todo!();
        /*
            return m_dampingRatio;
        */
    }
}

/**
  | 1-D constrained system
  | m (v2 - v1) = lambda
  | v2 + (beta/h) * x1 + gamma * lambda = 0, gamma has units of inverse mass.
  | x2 = x1 + h * v2
  -----------------
  | 1-D mass-damper-spring system
  | m (v2 - v1) + h * d * v2 + h * k *
  -----------------
  | C = norm(p2 - p1) - L
  | u = (p2 - p1) / norm(p2 - p1)
  | Cdot = dot(u, v2 + cross(w2, r2) - v1 - cross(w1, r1))
  | J = [-u -cross(r1, u) u cross(r2, u)]
  | K = J * invM * JT
  |   = invMass1 + invI1 * cross(r1, u)^2 + invMass2 + invI2 * cross(r2, u)^2
  */
impl b2DistanceJointDef {

    /**
      | Initialize the bodies, anchors, and
      | length using the world anchors.
      |
      */
    pub fn initialize(&mut self, 
        b1:      *mut b2Body,
        b2:      *mut b2Body,
        anchor1: &b2Vec2,
        anchor2: &b2Vec2)  {
        
        todo!();
        /*
            bodyA = b1;
        bodyB = b2;
        localAnchorA = bodyA->GetLocalPoint(anchor1);
        localAnchorB = bodyB->GetLocalPoint(anchor2);
        b2Vec2 d = anchor2 - anchor1;
        length = d.Length();
        */
    }

    pub fn new(def: *const b2DistanceJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            m_localAnchorA = def->localAnchorA;
        m_localAnchorB = def->localAnchorB;
        m_length = def->length;
        m_frequencyHz = def->frequencyHz;
        m_dampingRatio = def->dampingRatio;
        m_impulse = 0.0f;
        m_gamma = 0.0f;
        m_bias = 0.0f;
        */
    }

    pub fn init_velocity_constraints(&mut self, data: &b2SolverData)  {
        
        todo!();
        /*
            m_indexA = m_bodyA->m_islandIndex;
        m_indexB = m_bodyB->m_islandIndex;
        m_localCenterA = m_bodyA->m_sweep.localCenter;
        m_localCenterB = m_bodyB->m_sweep.localCenter;
        m_invMassA = m_bodyA->m_invMass;
        m_invMassB = m_bodyB->m_invMass;
        m_invIA = m_bodyA->m_invI;
        m_invIB = m_bodyB->m_invI;

        b2Vec2 cA = data.positions[m_indexA].c;
        float32 aA = data.positions[m_indexA].a;
        b2Vec2 vA = data.velocities[m_indexA].v;
        float32 wA = data.velocities[m_indexA].w;

        b2Vec2 cB = data.positions[m_indexB].c;
        float32 aB = data.positions[m_indexB].a;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        b2Rot qA(aA), qB(aB);

        m_rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        m_rB = b2Mul(qB, m_localAnchorB - m_localCenterB);
        m_u = cB + m_rB - cA - m_rA;

        // Handle singularity.
        float32 length = m_u.Length();
        if (length > b2_linearSlop)
        {
            m_u *= 1.0f / length;
        }
        else
        {
            m_u.Set(0.0f, 0.0f);
        }

        float32 crAu = b2Cross(m_rA, m_u);
        float32 crBu = b2Cross(m_rB, m_u);
        float32 invMass = m_invMassA + m_invIA * crAu * crAu + m_invMassB + m_invIB * crBu * crBu;

        // Compute the effective mass matrix.
        m_mass = invMass != 0.0f ? 1.0f / invMass : 0.0f;

        if (m_frequencyHz > 0.0f)
        {
            float32 C = length - m_length;

            // Frequency
            float32 omega = 2.0f * b2_pi * m_frequencyHz;

            // Damping coefficient
            float32 d = 2.0f * m_mass * m_dampingRatio * omega;

            // Spring stiffness
            float32 k = m_mass * omega * omega;

            // magic formulas
            float32 h = data.step.dt;
            m_gamma = h * (d + h * k);
            m_gamma = m_gamma != 0.0f ? 1.0f / m_gamma : 0.0f;
            m_bias = C * h * k * m_gamma;

            invMass += m_gamma;
            m_mass = invMass != 0.0f ? 1.0f / invMass : 0.0f;
        }
        else
        {
            m_gamma = 0.0f;
            m_bias = 0.0f;
        }

        if (data.step.warmStarting)
        {
            // Scale the impulse to support a variable time step.
            m_impulse *= data.step.dtRatio;

            b2Vec2 P = m_impulse * m_u;
            vA -= m_invMassA * P;
            wA -= m_invIA * b2Cross(m_rA, P);
            vB += m_invMassB * P;
            wB += m_invIB * b2Cross(m_rB, P);
        }
        else
        {
            m_impulse = 0.0f;
        }

        data.velocities[m_indexA].v = vA;
        data.velocities[m_indexA].w = wA;
        data.velocities[m_indexB].v = vB;
        data.velocities[m_indexB].w = wB;
        */
    }

    pub fn solve_velocity_constraints(&mut self, data: &b2SolverData)  {
        
        todo!();
        /*
            b2Vec2 vA = data.velocities[m_indexA].v;
        float32 wA = data.velocities[m_indexA].w;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        // Cdot = dot(u, v + cross(w, r))
        b2Vec2 vpA = vA + b2Cross(wA, m_rA);
        b2Vec2 vpB = vB + b2Cross(wB, m_rB);
        float32 Cdot = b2Dot(m_u, vpB - vpA);

        float32 impulse = -m_mass * (Cdot + m_bias + m_gamma * m_impulse);
        m_impulse += impulse;

        b2Vec2 P = impulse * m_u;
        vA -= m_invMassA * P;
        wA -= m_invIA * b2Cross(m_rA, P);
        vB += m_invMassB * P;
        wB += m_invIB * b2Cross(m_rB, P);

        data.velocities[m_indexA].v = vA;
        data.velocities[m_indexA].w = wA;
        data.velocities[m_indexB].v = vB;
        data.velocities[m_indexB].w = wB;
        */
    }

    pub fn solve_position_constraints(&mut self, data: &b2SolverData) -> bool {
        
        todo!();
        /*
            if (m_frequencyHz > 0.0f)
        {
            // There is no position correction for soft distance constraints.
            return true;
        }

        b2Vec2 cA = data.positions[m_indexA].c;
        float32 aA = data.positions[m_indexA].a;
        b2Vec2 cB = data.positions[m_indexB].c;
        float32 aB = data.positions[m_indexB].a;

        b2Rot qA(aA), qB(aB);

        b2Vec2 rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        b2Vec2 rB = b2Mul(qB, m_localAnchorB - m_localCenterB);
        b2Vec2 u = cB + rB - cA - rA;

        float32 length = u.Normalize();
        float32 C = length - m_length;
        C = b2Clamp(C, -b2_maxLinearCorrection, b2_maxLinearCorrection);

        float32 impulse = -m_mass * C;
        b2Vec2 P = impulse * u;

        cA -= m_invMassA * P;
        aA -= m_invIA * b2Cross(rA, P);
        cB += m_invMassB * P;
        aB += m_invIB * b2Cross(rB, P);

        data.positions[m_indexA].c = cA;
        data.positions[m_indexA].a = aA;
        data.positions[m_indexB].c = cB;
        data.positions[m_indexB].a = aB;

        return b2Abs(C) < b2_linearSlop;
        */
    }

    pub fn get_anchora(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_bodyA->GetWorldPoint(m_localAnchorA);
        */
    }

    pub fn get_anchorb(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_bodyB->GetWorldPoint(m_localAnchorB);
        */
    }

    /**
      | Get the reaction force given the inverse
      | time step. Unit is N.
      |
      */
    pub fn get_reaction_force(&self, inv_dt: f32) -> b2Vec2 {
        
        todo!();
        /*
            b2Vec2 F = (inv_dt * m_impulse) * m_u;
        return F;
        */
    }

    /**
      | Get the reaction torque given the inverse
      | time step. Unit is N*m. This is always
      | zero for a distance joint.
      |
      */
    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            B2_NOT_USED(inv_dt);
        return 0.0f;
        */
    }

    /**
      | Dump joint to dmLog
      |
      */
    pub fn dump(&mut self)  {
        
        todo!();
        /*
            int32 indexA = m_bodyA->m_islandIndex;
        int32 indexB = m_bodyB->m_islandIndex;

        b2Log("  b2DistanceJointDef jd;\n");
        b2Log("  jd.bodyA = bodies[%d];\n", indexA);
        b2Log("  jd.bodyB = bodies[%d];\n", indexB);
        b2Log("  jd.collideConnected = bool(%d);\n", m_collideConnected);
        b2Log("  jd.localAnchorA.Set(%.15lef, %.15lef);\n", m_localAnchorA.x, m_localAnchorA.y);
        b2Log("  jd.localAnchorB.Set(%.15lef, %.15lef);\n", m_localAnchorB.x, m_localAnchorB.y);
        b2Log("  jd.length = %.15lef;\n", m_length);
        b2Log("  jd.frequencyHz = %.15lef;\n", m_frequencyHz);
        b2Log("  jd.dampingRatio = %.15lef;\n", m_dampingRatio);
        b2Log("  joints[%d] = m_world->CreateJoint(&jd);\n", m_index);
        */
    }
}

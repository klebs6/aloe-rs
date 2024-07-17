crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2RopeJoint.h]

/**
  | Rope joint definition. This requires two body
  | anchor points and a maximum lengths.
  |
  | @note by default the connected objects will
  | not collide.  see collideConnected in
  | b2JointDef.
  */
pub struct b2RopeJointDef {
    base: b2JointDef,

    /**
      The local anchor point relative to bodyA's
      origin.
      */
    local_anchora: b2Vec2,

    /**
      The local anchor point relative to bodyB's
      origin.
      */
    local_anchorb: b2Vec2,

    /**
      | The maximum length of the rope.
      | 
      | -----------
      | @warning
      | 
      | this must be larger than b2_linearSlop
      | or the joint will have no effect.
      |
      */
    max_length:    f32,
}

impl Default for b2RopeJointDef {
    
    fn default() -> Self {
        todo!();
        /*

            type = e_ropeJoint;
            localAnchorA.Set(-1.0f, 0.0f);
            localAnchorB.Set(1.0f, 0.0f);
            maxLength = 0.0f
        */
    }
}

/**
  | A rope joint enforces a maximum distance
  | between two points on two bodies. It
  | has no other effect.
  | 
  | -----------
  | @warning
  | 
  | if you attempt to change the maximum
  | length during the simulation you will
  | get some non-physical behavior. A model
  | that would allow you to dynamically
  | modify the length would have some sponginess,
  | so I chose not to implement it that way.
  | See b2DistanceJoint if you want to dynamically
  | control length.
  |
  */
pub struct b2RopeJoint {
    base: b2Joint,

    /* ----------------- * Solver shared  ----------------- */
    local_anchora: b2Vec2,
    local_anchorb: b2Vec2,
    max_length:    f32,
    length:        f32,
    impulse:       f32,

    /* ------------------ * Solver temp  ------------------ */
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
    state:         b2LimitState,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2RopeJoint.cpp]
impl b2RopeJoint {

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
      | Set/Get the maximum length of the rope.
      |
      */
    pub fn set_max_length(&mut self, length: f32)  {
        
        todo!();
        /*
            m_maxLength = length;
        */
    }
    
    /**
      | Limit:
      | C = norm(pB - pA) - L
      | u = (pB - pA) / norm(pB - pA)
      | Cdot = dot(u, vB + cross(wB, rB) - vA - cross(wA, rA))
      | J = [-u -cross(rA, u) u cross(rB, u)]
      | K = J * invM * JT
      |   = invMassA + invIA * cross(rA, u)^2 + invMassB + invIB * cross(rB, u)^2
      */
    pub fn new(def: *const b2RopeJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            m_localAnchorA = def->localAnchorA;
        m_localAnchorB = def->localAnchorB;

        m_maxLength = def->maxLength;

        m_mass = 0.0f;
        m_impulse = 0.0f;
        m_state = e_inactiveLimit;
        m_length = 0.0f;
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

        m_length = m_u.Length();

        float32 C = m_length - m_maxLength;
        if (C > 0.0f)
        {
            m_state = e_atUpperLimit;
        }
        else
        {
            m_state = e_inactiveLimit;
        }

        if (m_length > b2_linearSlop)
        {
            m_u *= 1.0f / m_length;
        }
        else
        {
            m_u.SetZero();
            m_mass = 0.0f;
            m_impulse = 0.0f;
            return;
        }

        // Compute effective mass.
        float32 crA = b2Cross(m_rA, m_u);
        float32 crB = b2Cross(m_rB, m_u);
        float32 invMass = m_invMassA + m_invIA * crA * crA + m_invMassB + m_invIB * crB * crB;

        m_mass = invMass != 0.0f ? 1.0f / invMass : 0.0f;

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
        float32 C = m_length - m_maxLength;
        float32 Cdot = b2Dot(m_u, vpB - vpA);

        // Predictive constraint.
        if (C < 0.0f)
        {
            Cdot += data.step.inv_dt * C;
        }

        float32 impulse = -m_mass * Cdot;
        float32 oldImpulse = m_impulse;
        m_impulse = b2Min(0.0f, m_impulse + impulse);
        impulse = m_impulse - oldImpulse;

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
            b2Vec2 cA = data.positions[m_indexA].c;
        float32 aA = data.positions[m_indexA].a;
        b2Vec2 cB = data.positions[m_indexB].c;
        float32 aB = data.positions[m_indexB].a;

        b2Rot qA(aA), qB(aB);

        b2Vec2 rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        b2Vec2 rB = b2Mul(qB, m_localAnchorB - m_localCenterB);
        b2Vec2 u = cB + rB - cA - rA;

        float32 length = u.Normalize();
        float32 C = length - m_maxLength;

        C = b2Clamp(C, 0.0f, b2_maxLinearCorrection);

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

        return length - m_maxLength < b2_linearSlop;
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

    pub fn get_reaction_force(&self, inv_dt: f32) -> b2Vec2 {
        
        todo!();
        /*
            b2Vec2 F = (inv_dt * m_impulse) * m_u;
        return F;
        */
    }

    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            B2_NOT_USED(inv_dt);
        return 0.0f;
        */
    }

    pub fn get_max_length(&self) -> f32 {
        
        todo!();
        /*
            return m_maxLength;
        */
    }

    pub fn get_limit_state(&self) -> b2LimitState {
        
        todo!();
        /*
            return m_state;
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

        b2Log("  b2RopeJointDef jd;\n");
        b2Log("  jd.bodyA = bodies[%d];\n", indexA);
        b2Log("  jd.bodyB = bodies[%d];\n", indexB);
        b2Log("  jd.collideConnected = bool(%d);\n", m_collideConnected);
        b2Log("  jd.localAnchorA.Set(%.15lef, %.15lef);\n", m_localAnchorA.x, m_localAnchorA.y);
        b2Log("  jd.localAnchorB.Set(%.15lef, %.15lef);\n", m_localAnchorB.x, m_localAnchorB.y);
        b2Log("  jd.maxLength = %.15lef;\n", m_maxLength);
        b2Log("  joints[%d] = m_world->CreateJoint(&jd);\n", m_index);
        */
    }
}

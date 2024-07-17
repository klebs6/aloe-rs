crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2PulleyJoint.h]

pub const b2_minPulleyLength: f32 = 2.0;

/**
   Pulley joint definition. This requires two
   ground anchors, two dynamic body anchor
   points, and a pulley ratio.
  */
pub struct b2PulleyJointDef {

    base:           b2JointDef,

    /**
      The first ground anchor in world
      coordinates. This point never moves.
      */
    ground_anchora: b2Vec2,

    /**
      The second ground anchor in world
      coordinates. This point never moves.
      */
    ground_anchorb: b2Vec2,

    /**
      The local anchor point relative to bodyA's
      origin.
      */
    local_anchora:  b2Vec2,

    /**
      The local anchor point relative to bodyB's
      origin.
      */
    local_anchorb:  b2Vec2,

    /**
      The a reference length for the segment
      attached to bodyA.
      */
    lengtha:        f32,

    /**
      The a reference length for the segment
      attached to bodyB.
      */
    lengthb:        f32,

    /**
      The pulley ratio, used to simulate
      a block-and-tackle.
      */
    ratio:          f32,
}

impl Default for b2PulleyJointDef {
    
    fn default() -> Self {
        todo!();
        /*
            type = e_pulleyJoint;
            groundAnchorA.Set(-1.0f, 1.0f);
            groundAnchorB.Set(1.0f, 1.0f);
            localAnchorA.Set(-1.0f, 0.0f);
            localAnchorB.Set(1.0f, 0.0f);
            lengthA = 0.0f;
            lengthB = 0.0f;
            ratio = 1.0f;
            collideConnected = true
        */
    }
}

impl b2PulleyJointDef {

    /**
      | Initialize the bodies, anchors, lengths,
      | max lengths, and ratio using the world
      | anchors.
      |
      | Pulley:
      | length1 = norm(p1 - s1)
      | length2 = norm(p2 - s2)
      | C0 = (length1 + ratio * length2)_initial
      | C = C0 - (length1 + ratio * length2)
      | u1 = (p1 - s1) / norm(p1 - s1)
      | u2 = (p2 - s2) / norm(p2 - s2)
      | Cdot = -dot(u1, v1 + cross(w1, r1)) - ratio * dot(u2, v2 + cross(w2, r2))
      | J = -[u1 cross(r1, u1) ratio * u2  ratio * cross(r2, u2)]
      | K = J * invM * JT
      |   = invMass1 + invI1 * cross(r1, u1)^2 + ratio^2 * (invMass2 + invI2 * cross(r2, u2)^2)
      */
    pub fn initialize(&mut self, 
        ba:      *mut b2Body,
        bb:      *mut b2Body,
        grounda: &b2Vec2,
        groundb: &b2Vec2,
        anchora: &b2Vec2,
        anchorb: &b2Vec2,
        r:       f32)  {
        
        todo!();
        /*
            bodyA = bA;
        bodyB = bB;
        groundAnchorA = groundA;
        groundAnchorB = groundB;
        localAnchorA = bodyA->GetLocalPoint(anchorA);
        localAnchorB = bodyB->GetLocalPoint(anchorB);
        b2Vec2 dA = anchorA - groundA;
        lengthA = dA.Length();
        b2Vec2 dB = anchorB - groundB;
        lengthB = dB.Length();
        ratio = r;
        b2Assert(ratio > b2_epsilon);
        */
    }
}

/**
  | The pulley joint is connected to two bodies
  | and two fixed ground points.  The pulley
  | supports a ratio such that:
  |
  | length1 + ratio * length2 <= constant
  |
  | Yes, the force transmitted is scaled by the
  | ratio.
  |
  | @warning the pulley joint can get a bit
  | squirrelly by itself. They often work better
  | when combined with prismatic joints. You
  | should also cover the the anchor points with
  | static shapes to prevent one side from going
  | to zero length.
  */
pub struct b2PulleyJoint {
    base:           b2Joint,
    ground_anchora: b2Vec2,
    ground_anchorb: b2Vec2,
    lengtha:        f32,
    lengthb:        f32,

    /* ----------------- Solver shared  ----------------- */
    local_anchora:  b2Vec2,

    local_anchorb:  b2Vec2,
    constant:       f32,
    ratio:          f32,
    impulse:        f32,

    /* ------------------ Solver temp  ------------------ */
    indexa:         i32,

    indexb:         i32,
    ua:             b2Vec2,
    ub:             b2Vec2,
    ra:             b2Vec2,
    rb:             b2Vec2,
    local_centera:  b2Vec2,
    local_centerb:  b2Vec2,
    inv_massa:      f32,
    inv_massb:      f32,
    invia:          f32,
    invib:          f32,
    mass:           f32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2PulleyJoint.cpp]
impl b2PulleyJoint {

    pub fn new(def: *const b2PulleyJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            m_groundAnchorA = def->groundAnchorA;
        m_groundAnchorB = def->groundAnchorB;
        m_localAnchorA = def->localAnchorA;
        m_localAnchorB = def->localAnchorB;

        m_lengthA = def->lengthA;
        m_lengthB = def->lengthB;

        b2Assert(def->ratio != 0.0f);
        m_ratio = def->ratio;

        m_constant = def->lengthA + m_ratio * def->lengthB;

        m_impulse = 0.0f;
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

        // Get the pulley axes.
        m_uA = cA + m_rA - m_groundAnchorA;
        m_uB = cB + m_rB - m_groundAnchorB;

        float32 lengthA = m_uA.Length();
        float32 lengthB = m_uB.Length();

        if (lengthA > 10.0f * b2_linearSlop)
        {
            m_uA *= 1.0f / lengthA;
        }
        else
        {
            m_uA.SetZero();
        }

        if (lengthB > 10.0f * b2_linearSlop)
        {
            m_uB *= 1.0f / lengthB;
        }
        else
        {
            m_uB.SetZero();
        }

        // Compute effective mass.
        float32 ruA = b2Cross(m_rA, m_uA);
        float32 ruB = b2Cross(m_rB, m_uB);

        float32 mA = m_invMassA + m_invIA * ruA * ruA;
        float32 mB = m_invMassB + m_invIB * ruB * ruB;

        m_mass = mA + m_ratio * m_ratio * mB;

        if (m_mass > 0.0f)
        {
            m_mass = 1.0f / m_mass;
        }

        if (data.step.warmStarting)
        {
            // Scale impulses to support variable time steps.
            m_impulse *= data.step.dtRatio;

            // Warm starting.
            b2Vec2 PA = -(m_impulse) * m_uA;
            b2Vec2 PB = (-m_ratio * m_impulse) * m_uB;

            vA += m_invMassA * PA;
            wA += m_invIA * b2Cross(m_rA, PA);
            vB += m_invMassB * PB;
            wB += m_invIB * b2Cross(m_rB, PB);
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

        b2Vec2 vpA = vA + b2Cross(wA, m_rA);
        b2Vec2 vpB = vB + b2Cross(wB, m_rB);

        float32 Cdot = -b2Dot(m_uA, vpA) - m_ratio * b2Dot(m_uB, vpB);
        float32 impulse = -m_mass * Cdot;
        m_impulse += impulse;

        b2Vec2 PA = -impulse * m_uA;
        b2Vec2 PB = -m_ratio * impulse * m_uB;
        vA += m_invMassA * PA;
        wA += m_invIA * b2Cross(m_rA, PA);
        vB += m_invMassB * PB;
        wB += m_invIB * b2Cross(m_rB, PB);

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

        // Get the pulley axes.
        b2Vec2 uA = cA + rA - m_groundAnchorA;
        b2Vec2 uB = cB + rB - m_groundAnchorB;

        float32 lengthA = uA.Length();
        float32 lengthB = uB.Length();

        if (lengthA > 10.0f * b2_linearSlop)
        {
            uA *= 1.0f / lengthA;
        }
        else
        {
            uA.SetZero();
        }

        if (lengthB > 10.0f * b2_linearSlop)
        {
            uB *= 1.0f / lengthB;
        }
        else
        {
            uB.SetZero();
        }

        // Compute effective mass.
        float32 ruA = b2Cross(rA, uA);
        float32 ruB = b2Cross(rB, uB);

        float32 mA = m_invMassA + m_invIA * ruA * ruA;
        float32 mB = m_invMassB + m_invIB * ruB * ruB;

        float32 mass = mA + m_ratio * m_ratio * mB;

        if (mass > 0.0f)
        {
            mass = 1.0f / mass;
        }

        float32 C = m_constant - lengthA - m_ratio * lengthB;
        float32 linearError = b2Abs(C);

        float32 impulse = -mass * C;

        b2Vec2 PA = -impulse * uA;
        b2Vec2 PB = -m_ratio * impulse * uB;

        cA += m_invMassA * PA;
        aA += m_invIA * b2Cross(rA, PA);
        cB += m_invMassB * PB;
        aB += m_invIB * b2Cross(rB, PB);

        data.positions[m_indexA].c = cA;
        data.positions[m_indexA].a = aA;
        data.positions[m_indexB].c = cB;
        data.positions[m_indexB].a = aB;

        return linearError < b2_linearSlop;
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
            b2Vec2 P = m_impulse * m_uB;
        return inv_dt * P;
        */
    }

    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            B2_NOT_USED(inv_dt);
        return 0.0f;
        */
    }

    /**
      | Get the first ground anchor.
      |
      */
    pub fn get_ground_anchora(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_groundAnchorA;
        */
    }

    /**
      | Get the second ground anchor.
      |
      */
    pub fn get_ground_anchorb(&self) -> b2Vec2 {
        
        todo!();
        /*
            return m_groundAnchorB;
        */
    }

    /**
      | Get the current length of the segment
      | attached to bodyA.
      |
      */
    pub fn get_lengtha(&self) -> f32 {
        
        todo!();
        /*
            b2Vec2 p = m_bodyA->GetWorldPoint(m_localAnchorA);
        b2Vec2 s = m_groundAnchorA;
        b2Vec2 d = p - s;
        return d.Length();
        */
    }

    /**
      | Get the current length of the segment
      | attached to bodyB.
      |
      */
    pub fn get_lengthb(&self) -> f32 {
        
        todo!();
        /*
            b2Vec2 p = m_bodyB->GetWorldPoint(m_localAnchorB);
        b2Vec2 s = m_groundAnchorB;
        b2Vec2 d = p - s;
        return d.Length();
        */
    }

    /**
      | Get the pulley ratio.
      |
      */
    pub fn get_ratio(&self) -> f32 {
        
        todo!();
        /*
            return m_ratio;
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

        b2Log("  b2PulleyJointDef jd;\n");
        b2Log("  jd.bodyA = bodies[%d];\n", indexA);
        b2Log("  jd.bodyB = bodies[%d];\n", indexB);
        b2Log("  jd.collideConnected = bool(%d);\n", m_collideConnected);
        b2Log("  jd.groundAnchorA.Set(%.15lef, %.15lef);\n", m_groundAnchorA.x, m_groundAnchorA.y);
        b2Log("  jd.groundAnchorB.Set(%.15lef, %.15lef);\n", m_groundAnchorB.x, m_groundAnchorB.y);
        b2Log("  jd.localAnchorA.Set(%.15lef, %.15lef);\n", m_localAnchorA.x, m_localAnchorA.y);
        b2Log("  jd.localAnchorB.Set(%.15lef, %.15lef);\n", m_localAnchorB.x, m_localAnchorB.y);
        b2Log("  jd.lengthA = %.15lef;\n", m_lengthA);
        b2Log("  jd.lengthB = %.15lef;\n", m_lengthB);
        b2Log("  jd.ratio = %.15lef;\n", m_ratio);
        b2Log("  joints[%d] = m_world->CreateJoint(&jd);\n", m_index);
        */
    }
}

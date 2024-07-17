crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2FrictionJoint.h]

/**
  | Friction joint definition.
  |
  */
pub struct b2FrictionJointDef {

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
      | The maximum friction force in N.
      |
      */
    max_force:     f32,

    /**
      | The maximum friction torque in N-m.
      |
      */
    max_torque:    f32,
}

impl Default for b2FrictionJointDef {
    
    fn default() -> Self {
        todo!();
        /*


            type = e_frictionJoint;
            localAnchorA.SetZero();
            localAnchorB.SetZero();
            maxForce = 0.0f;
            maxTorque = 0.0f
        */
    }
}

impl b2FrictionJointDef {

    /**
      | Point-to-point constraint
      | Cdot = v2 - v1
      |      = v2 + cross(w2, r2) - v1 - cross(w1, r1)
      | J = [-I -r1_skew I r2_skew ]
      | Identity used:
      | w k % (rx i + ry j) = w * (-ry i + rx j)
      |
      | Angle constraint
      | Cdot = w2 - w1
      | J = [0 0 -1 0 0 1]
      | K = invI1 + invI2
      |
      | Initialize the bodies, anchors, axis, and
      | reference angle using the world anchor and
      | world axis.
      */
    pub fn initialize(&mut self, 
        ba:     *mut b2Body,
        bb:     *mut b2Body,
        anchor: &b2Vec2)  {
        
        todo!();
        /*
            bodyA = bA;
        bodyB = bB;
        localAnchorA = bodyA->GetLocalPoint(anchor);
        localAnchorB = bodyB->GetLocalPoint(anchor);
        */
    }
}

/**
  | Friction joint. This is used for top-down
  | friction.
  |
  | It provides 2D translational friction and
  | angular friction.
  */
pub struct b2FrictionJoint {
    base:            b2Joint,
    local_anchora:   b2Vec2,
    local_anchorb:   b2Vec2,

    /* ----------------- Solver shared  ----------------- */

    linear_impulse:  b2Vec2,
    angular_impulse: f32,
    max_force:       f32,
    max_torque:      f32,

    /* ------------------ Solver temp  ------------------ */
    indexa:          i32,
    indexb:          i32,

    ra:              b2Vec2,
    rb:              b2Vec2,
    local_centera:   b2Vec2,
    local_centerb:   b2Vec2,
    inv_massa:       f32,
    inv_massb:       f32,
    invia:           f32,
    invib:           f32,
    linear_mass:     b2Mat22,
    angular_mass:    f32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2FrictionJoint.cpp]
impl b2FrictionJoint {

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

    pub fn new(def: *const b2FrictionJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            m_localAnchorA = def->localAnchorA;
        m_localAnchorB = def->localAnchorB;

        m_linearImpulse.SetZero();
        m_angularImpulse = 0.0f;

        m_maxForce = def->maxForce;
        m_maxTorque = def->maxTorque;
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

        float32 aA = data.positions[m_indexA].a;
        b2Vec2 vA = data.velocities[m_indexA].v;
        float32 wA = data.velocities[m_indexA].w;

        float32 aB = data.positions[m_indexB].a;
        b2Vec2 vB = data.velocities[m_indexB].v;
        float32 wB = data.velocities[m_indexB].w;

        b2Rot qA(aA), qB(aB);

        // Compute the effective mass matrix.
        m_rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
        m_rB = b2Mul(qB, m_localAnchorB - m_localCenterB);

        // J = [-I -r1_skew I r2_skew]
        //     [ 0       -1 0       1]
        // r_skew = [-ry; rx]

        // Matlab
        // K = [ mA+r1y^2*iA+mB+r2y^2*iB,  -r1y*iA*r1x-r2y*iB*r2x,          -r1y*iA-r2y*iB]
        //     [  -r1y*iA*r1x-r2y*iB*r2x, mA+r1x^2*iA+mB+r2x^2*iB,           r1x*iA+r2x*iB]
        //     [          -r1y*iA-r2y*iB,           r1x*iA+r2x*iB,                   iA+iB]

        float32 mA = m_invMassA, mB = m_invMassB;
        float32 iA = m_invIA, iB = m_invIB;

        b2Mat22 K;
        K.ex.x = mA + mB + iA * m_rA.y * m_rA.y + iB * m_rB.y * m_rB.y;
        K.ex.y = -iA * m_rA.x * m_rA.y - iB * m_rB.x * m_rB.y;
        K.ey.x = K.ex.y;
        K.ey.y = mA + mB + iA * m_rA.x * m_rA.x + iB * m_rB.x * m_rB.x;

        m_linearMass = K.GetInverse();

        m_angularMass = iA + iB;
        if (m_angularMass > 0.0f)
        {
            m_angularMass = 1.0f / m_angularMass;
        }

        if (data.step.warmStarting)
        {
            // Scale impulses to support a variable time step.
            m_linearImpulse *= data.step.dtRatio;
            m_angularImpulse *= data.step.dtRatio;

            b2Vec2 P(m_linearImpulse.x, m_linearImpulse.y);
            vA -= mA * P;
            wA -= iA * (b2Cross(m_rA, P) + m_angularImpulse);
            vB += mB * P;
            wB += iB * (b2Cross(m_rB, P) + m_angularImpulse);
        }
        else
        {
            m_linearImpulse.SetZero();
            m_angularImpulse = 0.0f;
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

        float32 mA = m_invMassA, mB = m_invMassB;
        float32 iA = m_invIA, iB = m_invIB;

        float32 h = data.step.dt;

        // Solve angular friction
        {
            float32 Cdot = wB - wA;
            float32 impulse = -m_angularMass * Cdot;

            float32 oldImpulse = m_angularImpulse;
            float32 maxImpulse = h * m_maxTorque;
            m_angularImpulse = b2Clamp(m_angularImpulse + impulse, -maxImpulse, maxImpulse);
            impulse = m_angularImpulse - oldImpulse;

            wA -= iA * impulse;
            wB += iB * impulse;
        }

        // Solve linear friction
        {
            b2Vec2 Cdot = vB + b2Cross(wB, m_rB) - vA - b2Cross(wA, m_rA);

            b2Vec2 impulse = -b2Mul(m_linearMass, Cdot);
            b2Vec2 oldImpulse = m_linearImpulse;
            m_linearImpulse += impulse;

            float32 maxImpulse = h * m_maxForce;

            if (m_linearImpulse.LengthSquared() > maxImpulse * maxImpulse)
            {
                m_linearImpulse.Normalize();
                m_linearImpulse *= maxImpulse;
            }

            impulse = m_linearImpulse - oldImpulse;

            vA -= mA * impulse;
            wA -= iA * b2Cross(m_rA, impulse);

            vB += mB * impulse;
            wB += iB * b2Cross(m_rB, impulse);
        }

        data.velocities[m_indexA].v = vA;
        data.velocities[m_indexA].w = wA;
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
            return inv_dt * m_linearImpulse;
        */
    }

    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            return inv_dt * m_angularImpulse;
        */
    }

    /**
      | Set the maximum friction force in N.
      |
      */
    pub fn set_max_force(&mut self, force: f32)  {
        
        todo!();
        /*
            b2Assert(b2IsValid(force) && force >= 0.0f);
        m_maxForce = force;
        */
    }

    /**
      | Get the maximum friction force in N.
      |
      */
    pub fn get_max_force(&self) -> f32 {
        
        todo!();
        /*
            return m_maxForce;
        */
    }

    /**
      | Set the maximum friction torque in N*m.
      |
      */
    pub fn set_max_torque(&mut self, torque: f32)  {
        
        todo!();
        /*
            b2Assert(b2IsValid(torque) && torque >= 0.0f);
        m_maxTorque = torque;
        */
    }

    /**
      | Get the maximum friction torque in N*m.
      |
      */
    pub fn get_max_torque(&self) -> f32 {
        
        todo!();
        /*
            return m_maxTorque;
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

        b2Log("  b2FrictionJointDef jd;\n");
        b2Log("  jd.bodyA = bodies[%d];\n", indexA);
        b2Log("  jd.bodyB = bodies[%d];\n", indexB);
        b2Log("  jd.collideConnected = bool(%d);\n", m_collideConnected);
        b2Log("  jd.localAnchorA.Set(%.15lef, %.15lef);\n", m_localAnchorA.x, m_localAnchorA.y);
        b2Log("  jd.localAnchorB.Set(%.15lef, %.15lef);\n", m_localAnchorB.x, m_localAnchorB.y);
        b2Log("  jd.maxForce = %.15lef;\n", m_maxForce);
        b2Log("  jd.maxTorque = %.15lef;\n", m_maxTorque);
        b2Log("  joints[%d] = m_world->CreateJoint(&jd);\n", m_index);
        */
    }
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2RevoluteJoint.h]

/**
  | Revolute joint definition. This requires
  | defining an anchor point where the bodies are
  | joined. The definition uses local anchor
  | points so that the initial configuration can
  | violate the constraint slightly. You also need
  | to specify the initial relative angle for
  | joint limits. This helps when saving and
  | loading a game.
  |
  | The local anchor points are measured from the
  | body's origin rather than the center of mass
  | because:
  |
  | 1. you might not know where the center of mass
  |    will be.
  |
  | 2. if you add/remove shapes from a body and
  |    recompute the mass, the joints will be
  |    broken.
  */
pub struct b2RevoluteJointDef {

    base:             b2JointDef,

    /**
      | The local anchor point relative to bodyA's
      | origin.
      |
      */
    local_anchora:    b2Vec2,

    /**
      | The local anchor point relative to bodyB's
      | origin.
      |
      */
    local_anchorb:    b2Vec2,

    /**
      | The bodyB angle minus bodyA angle in
      | the reference state (radians).
      |
      */
    reference_angle:  f32,

    /**
      | A flag to enable joint limits.
      |
      */
    enable_limit:     bool,

    /**
      | The lower angle for the joint limit (radians).
      |
      */
    lower_angle:      f32,

    /**
      | The upper angle for the joint limit (radians).
      |
      */
    upper_angle:      f32,

    /**
      | A flag to enable the joint motor.
      |
      */
    enable_motor:     bool,

    /**
      | The desired motor speed. Usually in
      | radians per second.
      |
      */
    motor_speed:      f32,

    /**
      | The maximum motor torque used to achieve
      | the desired motor speed. Usually in
      | N-m.
      |
      */
    max_motor_torque: f32,
}

impl Default for b2RevoluteJointDef {
    
    fn default() -> Self {
        todo!();
        /*

            type = e_revoluteJoint;
            localAnchorA.Set(0.0f, 0.0f);
            localAnchorB.Set(0.0f, 0.0f);
            referenceAngle = 0.0f;
            lowerAngle = 0.0f;
            upperAngle = 0.0f;
            maxMotorTorque = 0.0f;
            motorSpeed = 0.0f;
            enableLimit = false;
            enableMotor = false
        */
    }
}

impl b2RevoluteJointDef {

    /**
      | Point-to-point constraint
      | C = p2 - p1
      | Cdot = v2 - v1
      |      = v2 + cross(w2, r2) - v1 - cross(w1, r1)
      | J = [-I -r1_skew I r2_skew ]
      | Identity used:
      | w k % (rx i + ry j) = w * (-ry i + rx j)
      |
      | Motor constraint
      | Cdot = w2 - w1
      | J = [0 0 -1 0 0 1]
      | K = invI1 + invI2
      |
      | Initialize the bodies, anchors, and
      | reference angle using a world anchor point.
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
        referenceAngle = bodyB->GetAngle() - bodyA->GetAngle();
        */
    }
}

/**
  | A revolute joint constrains two bodies to
  | share a common point while they are free to
  | rotate about the point. The relative rotation
  | about the shared point is the joint angle. You
  | can limit the relative rotation with a joint
  | limit that specifies a lower and upper
  | angle. You can use a motor to drive the
  | relative rotation about the shared
  | point. A maximum motor torque is provided so
  | that infinite forces are not generated.
  */
pub struct b2RevoluteJoint {
    base: b2Joint,

    /* ----------------- Solver shared  ----------------- */
    local_anchora:    b2Vec2,

    local_anchorb:    b2Vec2,
    impulse:          b2Vec3,
    motor_impulse:    f32,
    enable_motor:     bool,
    max_motor_torque: f32,
    motor_speed:      f32,
    enable_limit:     bool,
    reference_angle:  f32,
    lower_angle:      f32,
    upper_angle:      f32,

    /* ------------------ Solver temp  ------------------ */
    indexa:           i32,

    indexb:           i32,
    ra:               b2Vec2,
    rb:               b2Vec2,
    local_centera:    b2Vec2,
    local_centerb:    b2Vec2,
    inv_massa:        f32,
    inv_massb:        f32,
    invia:            f32,
    invib:            f32,

    /**
      | effective mass for point-to-point
      | constraint.
      |
      */
    mass:             b2Mat33,

    /**
      | effective mass for motor/limit angular
      | constraint.
      |
      */
    motor_mass:       f32,

    limit_state:      b2LimitState,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2RevoluteJoint.cpp]
impl b2RevoluteJoint {

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
      | Get the reference angle.
      |
      */
    pub fn get_reference_angle(&self) -> f32 {
        
        todo!();
        /*
            return m_referenceAngle;
        */
    }

    pub fn get_max_motor_torque(&self) -> f32 {
        
        todo!();
        /*
            return m_maxMotorTorque;
        */
    }

    /**
      | Get the motor speed in radians per second.
      |
      */
    #[inline] pub fn get_motor_speed(&self) -> f32 {
        
        todo!();
        /*
            return m_motorSpeed;
        */
    }

    pub fn new(def: *const b2RevoluteJointDef) -> Self {
    
        todo!();
        /*
        : b2joint(def),

            m_localAnchorA = def->localAnchorA;
        m_localAnchorB = def->localAnchorB;
        m_referenceAngle = def->referenceAngle;

        m_impulse.SetZero();
        m_motorImpulse = 0.0f;

        m_lowerAngle = def->lowerAngle;
        m_upperAngle = def->upperAngle;
        m_maxMotorTorque = def->maxMotorTorque;
        m_motorSpeed = def->motorSpeed;
        m_enableLimit = def->enableLimit;
        m_enableMotor = def->enableMotor;
        m_limitState = e_inactiveLimit;
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

        bool fixedRotation = (iA + iB == 0.0f);

        m_mass.ex.x = mA + mB + m_rA.y * m_rA.y * iA + m_rB.y * m_rB.y * iB;
        m_mass.ey.x = -m_rA.y * m_rA.x * iA - m_rB.y * m_rB.x * iB;
        m_mass.ez.x = -m_rA.y * iA - m_rB.y * iB;
        m_mass.ex.y = m_mass.ey.x;
        m_mass.ey.y = mA + mB + m_rA.x * m_rA.x * iA + m_rB.x * m_rB.x * iB;
        m_mass.ez.y = m_rA.x * iA + m_rB.x * iB;
        m_mass.ex.z = m_mass.ez.x;
        m_mass.ey.z = m_mass.ez.y;
        m_mass.ez.z = iA + iB;

        m_motorMass = iA + iB;
        if (m_motorMass > 0.0f)
        {
            m_motorMass = 1.0f / m_motorMass;
        }

        if (m_enableMotor == false || fixedRotation)
        {
            m_motorImpulse = 0.0f;
        }

        if (m_enableLimit && fixedRotation == false)
        {
            float32 jointAngle = aB - aA - m_referenceAngle;
            if (b2Abs(m_upperAngle - m_lowerAngle) < 2.0f * b2_angularSlop)
            {
                m_limitState = e_equalLimits;
            }
            else if (jointAngle <= m_lowerAngle)
            {
                if (m_limitState != e_atLowerLimit)
                {
                    m_impulse.z = 0.0f;
                }
                m_limitState = e_atLowerLimit;
            }
            else if (jointAngle >= m_upperAngle)
            {
                if (m_limitState != e_atUpperLimit)
                {
                    m_impulse.z = 0.0f;
                }
                m_limitState = e_atUpperLimit;
            }
            else
            {
                m_limitState = e_inactiveLimit;
                m_impulse.z = 0.0f;
            }
        }
        else
        {
            m_limitState = e_inactiveLimit;
        }

        if (data.step.warmStarting)
        {
            // Scale impulses to support a variable time step.
            m_impulse *= data.step.dtRatio;
            m_motorImpulse *= data.step.dtRatio;

            b2Vec2 P(m_impulse.x, m_impulse.y);

            vA -= mA * P;
            wA -= iA * (b2Cross(m_rA, P) + m_motorImpulse + m_impulse.z);

            vB += mB * P;
            wB += iB * (b2Cross(m_rB, P) + m_motorImpulse + m_impulse.z);
        }
        else
        {
            m_impulse.SetZero();
            m_motorImpulse = 0.0f;
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

        bool fixedRotation = (iA + iB == 0.0f);

        // Solve motor constraint.
        if (m_enableMotor && m_limitState != e_equalLimits && fixedRotation == false)
        {
            float32 Cdot = wB - wA - m_motorSpeed;
            float32 impulse = -m_motorMass * Cdot;
            float32 oldImpulse = m_motorImpulse;
            float32 maxImpulse = data.step.dt * m_maxMotorTorque;
            m_motorImpulse = b2Clamp(m_motorImpulse + impulse, -maxImpulse, maxImpulse);
            impulse = m_motorImpulse - oldImpulse;

            wA -= iA * impulse;
            wB += iB * impulse;
        }

        // Solve limit constraint.
        if (m_enableLimit && m_limitState != e_inactiveLimit && fixedRotation == false)
        {
            b2Vec2 Cdot1 = vB + b2Cross(wB, m_rB) - vA - b2Cross(wA, m_rA);
            float32 Cdot2 = wB - wA;
            b2Vec3 Cdot(Cdot1.x, Cdot1.y, Cdot2);

            b2Vec3 impulse = -m_mass.Solve33(Cdot);

            if (m_limitState == e_equalLimits)
            {
                m_impulse += impulse;
            }
            else if (m_limitState == e_atLowerLimit)
            {
                float32 newImpulse = m_impulse.z + impulse.z;
                if (newImpulse < 0.0f)
                {
                    b2Vec2 rhs = -Cdot1 + m_impulse.z * b2Vec2(m_mass.ez.x, m_mass.ez.y);
                    b2Vec2 reduced = m_mass.Solve22(rhs);
                    impulse.x = reduced.x;
                    impulse.y = reduced.y;
                    impulse.z = -m_impulse.z;
                    m_impulse.x += reduced.x;
                    m_impulse.y += reduced.y;
                    m_impulse.z = 0.0f;
                }
                else
                {
                    m_impulse += impulse;
                }
            }
            else if (m_limitState == e_atUpperLimit)
            {
                float32 newImpulse = m_impulse.z + impulse.z;
                if (newImpulse > 0.0f)
                {
                    b2Vec2 rhs = -Cdot1 + m_impulse.z * b2Vec2(m_mass.ez.x, m_mass.ez.y);
                    b2Vec2 reduced = m_mass.Solve22(rhs);
                    impulse.x = reduced.x;
                    impulse.y = reduced.y;
                    impulse.z = -m_impulse.z;
                    m_impulse.x += reduced.x;
                    m_impulse.y += reduced.y;
                    m_impulse.z = 0.0f;
                }
                else
                {
                    m_impulse += impulse;
                }
            }

            b2Vec2 P(impulse.x, impulse.y);

            vA -= mA * P;
            wA -= iA * (b2Cross(m_rA, P) + impulse.z);

            vB += mB * P;
            wB += iB * (b2Cross(m_rB, P) + impulse.z);
        }
        else
        {
            // Solve point-to-point constraint
            b2Vec2 Cdot = vB + b2Cross(wB, m_rB) - vA - b2Cross(wA, m_rA);
            b2Vec2 impulse = m_mass.Solve22(-Cdot);

            m_impulse.x += impulse.x;
            m_impulse.y += impulse.y;

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
            b2Vec2 cA = data.positions[m_indexA].c;
        float32 aA = data.positions[m_indexA].a;
        b2Vec2 cB = data.positions[m_indexB].c;
        float32 aB = data.positions[m_indexB].a;

        b2Rot qA(aA), qB(aB);

        float32 angularError = 0.0f;
        float32 positionError = 0.0f;

        bool fixedRotation = (m_invIA + m_invIB == 0.0f);

        // Solve angular limit constraint.
        if (m_enableLimit && m_limitState != e_inactiveLimit && fixedRotation == false)
        {
            float32 angle = aB - aA - m_referenceAngle;
            float32 limitImpulse = 0.0f;

            if (m_limitState == e_equalLimits)
            {
                // Prevent large angular corrections
                float32 C = b2Clamp(angle - m_lowerAngle, -b2_maxAngularCorrection, b2_maxAngularCorrection);
                limitImpulse = -m_motorMass * C;
                angularError = b2Abs(C);
            }
            else if (m_limitState == e_atLowerLimit)
            {
                float32 C = angle - m_lowerAngle;
                angularError = -C;

                // Prevent large angular corrections and allow some slop.
                C = b2Clamp(C + b2_angularSlop, -b2_maxAngularCorrection, 0.0f);
                limitImpulse = -m_motorMass * C;
            }
            else if (m_limitState == e_atUpperLimit)
            {
                float32 C = angle - m_upperAngle;
                angularError = C;

                // Prevent large angular corrections and allow some slop.
                C = b2Clamp(C - b2_angularSlop, 0.0f, b2_maxAngularCorrection);
                limitImpulse = -m_motorMass * C;
            }

            aA -= m_invIA * limitImpulse;
            aB += m_invIB * limitImpulse;
        }

        // Solve point-to-point constraint.
        {
            qA.Set(aA);
            qB.Set(aB);
            b2Vec2 rA = b2Mul(qA, m_localAnchorA - m_localCenterA);
            b2Vec2 rB = b2Mul(qB, m_localAnchorB - m_localCenterB);

            b2Vec2 C = cB + rB - cA - rA;
            positionError = C.Length();

            float32 mA = m_invMassA, mB = m_invMassB;
            float32 iA = m_invIA, iB = m_invIB;

            b2Mat22 K;
            K.ex.x = mA + mB + iA * rA.y * rA.y + iB * rB.y * rB.y;
            K.ex.y = -iA * rA.x * rA.y - iB * rB.x * rB.y;
            K.ey.x = K.ex.y;
            K.ey.y = mA + mB + iA * rA.x * rA.x + iB * rB.x * rB.x;

            b2Vec2 impulse = -K.Solve(C);

            cA -= mA * impulse;
            aA -= iA * b2Cross(rA, impulse);

            cB += mB * impulse;
            aB += iB * b2Cross(rB, impulse);
        }

        data.positions[m_indexA].c = cA;
        data.positions[m_indexA].a = aA;
        data.positions[m_indexB].c = cB;
        data.positions[m_indexB].a = aB;

        return positionError <= b2_linearSlop && angularError <= b2_angularSlop;
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
            b2Vec2 P(m_impulse.x, m_impulse.y);
        return inv_dt * P;
        */
    }

    /**
      | Get the reaction torque due to the joint
      | limit given the inverse time step. Unit
      | is N*m.
      |
      */
    pub fn get_reaction_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            return inv_dt * m_impulse.z;
        */
    }

    /**
      | Get the current joint angle in radians.
      |
      */
    pub fn get_joint_angle(&self) -> f32 {
        
        todo!();
        /*
            b2Body* bA = m_bodyA;
        b2Body* bB = m_bodyB;
        return bB->m_sweep.a - bA->m_sweep.a - m_referenceAngle;
        */
    }

    /**
      | Get the current joint angle speed in
      | radians per second.
      |
      */
    pub fn get_joint_speed(&self) -> f32 {
        
        todo!();
        /*
            b2Body* bA = m_bodyA;
        b2Body* bB = m_bodyB;
        return bB->m_angularVelocity - bA->m_angularVelocity;
        */
    }

    /**
      | Is the joint motor enabled?
      |
      */
    pub fn is_motor_enabled(&self) -> bool {
        
        todo!();
        /*
            return m_enableMotor;
        */
    }

    /**
      | Enable/disable the joint motor.
      |
      */
    pub fn enable_motor(&mut self, flag: bool)  {
        
        todo!();
        /*
            m_bodyA->SetAwake(true);
        m_bodyB->SetAwake(true);
        m_enableMotor = flag;
        */
    }

    /**
      | Get the current motor torque given the
      | inverse time step. Unit is N*m.
      |
      */
    pub fn get_motor_torque(&self, inv_dt: f32) -> f32 {
        
        todo!();
        /*
            return inv_dt * m_motorImpulse;
        */
    }

    /**
      | Set the motor speed in radians per second.
      |
      */
    pub fn set_motor_speed(&mut self, speed: f32)  {
        
        todo!();
        /*
            m_bodyA->SetAwake(true);
        m_bodyB->SetAwake(true);
        m_motorSpeed = speed;
        */
    }

    /**
      | Set the maximum motor torque, usually
      | in N-m.
      |
      */
    pub fn set_max_motor_torque(&mut self, torque: f32)  {
        
        todo!();
        /*
            m_bodyA->SetAwake(true);
        m_bodyB->SetAwake(true);
        m_maxMotorTorque = torque;
        */
    }

    /**
      | Is the joint limit enabled?
      |
      */
    pub fn is_limit_enabled(&self) -> bool {
        
        todo!();
        /*
            return m_enableLimit;
        */
    }

    /**
      | Enable/disable the joint limit.
      |
      */
    pub fn enable_limit(&mut self, flag: bool)  {
        
        todo!();
        /*
            if (flag != m_enableLimit)
        {
            m_bodyA->SetAwake(true);
            m_bodyB->SetAwake(true);
            m_enableLimit = flag;
            m_impulse.z = 0.0f;
        }
        */
    }

    /**
      | Get the lower joint limit in radians.
      |
      */
    pub fn get_lower_limit(&self) -> f32 {
        
        todo!();
        /*
            return m_lowerAngle;
        */
    }

    /**
      | Get the upper joint limit in radians.
      |
      */
    pub fn get_upper_limit(&self) -> f32 {
        
        todo!();
        /*
            return m_upperAngle;
        */
    }

    /**
      | Set the joint limits in radians.
      |
      */
    pub fn set_limits(&mut self, 
        lower: f32,
        upper: f32)  {
        
        todo!();
        /*
            b2Assert(lower <= upper);

        if (lower != m_lowerAngle || upper != m_upperAngle)
        {
            m_bodyA->SetAwake(true);
            m_bodyB->SetAwake(true);
            m_impulse.z = 0.0f;
            m_lowerAngle = lower;
            m_upperAngle = upper;
        }
        */
    }

    /**
      | Dump to b2Log.
      |
      */
    pub fn dump(&mut self)  {
        
        todo!();
        /*
            int32 indexA = m_bodyA->m_islandIndex;
        int32 indexB = m_bodyB->m_islandIndex;

        b2Log("  b2RevoluteJointDef jd;\n");
        b2Log("  jd.bodyA = bodies[%d];\n", indexA);
        b2Log("  jd.bodyB = bodies[%d];\n", indexB);
        b2Log("  jd.collideConnected = bool(%d);\n", m_collideConnected);
        b2Log("  jd.localAnchorA.Set(%.15lef, %.15lef);\n", m_localAnchorA.x, m_localAnchorA.y);
        b2Log("  jd.localAnchorB.Set(%.15lef, %.15lef);\n", m_localAnchorB.x, m_localAnchorB.y);
        b2Log("  jd.referenceAngle = %.15lef;\n", m_referenceAngle);
        b2Log("  jd.enableLimit = bool(%d);\n", m_enableLimit);
        b2Log("  jd.lowerAngle = %.15lef;\n", m_lowerAngle);
        b2Log("  jd.upperAngle = %.15lef;\n", m_upperAngle);
        b2Log("  jd.enableMotor = bool(%d);\n", m_enableMotor);
        b2Log("  jd.motorSpeed = %.15lef;\n", m_motorSpeed);
        b2Log("  jd.maxMotorTorque = %.15lef;\n", m_maxMotorTorque);
        b2Log("  joints[%d] = m_world->CreateJoint(&jd);\n", m_index);
        */
    }
}

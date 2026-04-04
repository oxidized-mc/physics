//! Jump physics.
//!
//! Implements jump impulse application matching
//! `LivingEntity.jumpFromGround()` from vanilla.

use glam::DVec3;

use super::constants::*;

/// Applies a jump impulse to the entity.
///
/// Sets vertical velocity to [`JUMP_POWER`] (0.42 blocks/tick), modified
/// by `jump_boost_level` (0 = no effect, 1 = Jump Boost I, etc.) and
/// `jump_factor` (from block below, e.g. 0.5 for honey blocks).
///
/// If `is_sprinting` is true, also applies a horizontal boost of 0.2
/// in the entity's facing direction (based on `yaw`).
///
/// # Examples
///
/// ```
/// use oxidized_physics::jump::apply_jump;
/// use glam::DVec3;
///
/// let mut vel = DVec3::ZERO;
/// apply_jump(&mut vel, 0.0, false, 0, 1.0);
/// assert!((vel.y - 0.42).abs() < 0.001);
/// ```
pub fn apply_jump(
    vel: &mut DVec3,
    yaw: f32,
    is_sprinting: bool,
    jump_boost_level: u8,
    jump_factor: f64,
) {
    let base = JUMP_POWER * jump_factor + f64::from(jump_boost_level) * JUMP_BOOST_PER_LEVEL;
    vel.y = base;

    if is_sprinting {
        // Sprint-jumping: horizontal boost in facing direction.
        let yaw_rad = f64::from(yaw).to_radians();
        vel.x -= yaw_rad.sin() * SPRINT_JUMP_BOOST;
        vel.z += yaw_rad.cos() * SPRINT_JUMP_BOOST;
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn test_jump_no_boost() {
        let mut vel = DVec3::ZERO;
        apply_jump(&mut vel, 0.0, false, 0, 1.0);
        assert!(
            (vel.y - JUMP_POWER).abs() < 0.0001,
            "Jump vy {} ≠ {}",
            vel.y,
            JUMP_POWER
        );
    }

    #[test]
    fn test_jump_with_boost_ii() {
        let mut vel = DVec3::ZERO;
        apply_jump(&mut vel, 0.0, false, 2, 1.0);
        let expected = JUMP_POWER + 2.0 * JUMP_BOOST_PER_LEVEL;
        assert!(
            (vel.y - expected).abs() < 0.0001,
            "Jump Boost II vy {} ≠ {}",
            vel.y,
            expected
        );
    }

    #[test]
    fn test_jump_honey_block_factor() {
        let mut vel = DVec3::ZERO;
        // Honey block jump factor is 0.5 (from block registry).
        apply_jump(&mut vel, 0.0, false, 0, 0.5);
        let expected = JUMP_POWER * 0.5;
        assert!(
            (vel.y - expected).abs() < 0.0001,
            "Honey jump vy {} ≠ {}",
            vel.y,
            expected
        );
    }

    #[test]
    fn test_sprint_jump_adds_horizontal_boost() {
        let mut vel = DVec3::ZERO;
        apply_jump(&mut vel, 0.0, true, 0, 1.0);

        // Facing 0° (yaw=0): sin(0)=0, cos(0)=1
        // vx -= 0 * 0.2 = 0, vz += 1 * 0.2 = 0.2
        assert!(
            vel.x.abs() < 0.001,
            "vx should be ~0 at yaw=0: {}",
            vel.x
        );
        assert!(
            (vel.z - SPRINT_JUMP_BOOST).abs() < 0.001,
            "vz should be ~0.2 at yaw=0: {}",
            vel.z
        );
    }

    #[test]
    fn test_no_sprint_boost_when_not_sprinting() {
        let mut vel = DVec3::ZERO;
        apply_jump(&mut vel, 90.0, false, 0, 1.0);

        assert!(vel.x.abs() < 0.0001, "No horizontal boost without sprint");
        assert!(vel.z.abs() < 0.0001, "No horizontal boost without sprint");
    }
}

use bevy::prelude::*;
use qgeometry::prelude::*;
use qmath::{dir::QDir, prelude::*, vec2::QVec2};
use std::hash::{Hash, Hasher};

#[derive(Default, Component, Debug, Clone, Copy)]
pub struct QObject {
    pub uuid: u64,
    pub entity: Option<Entity>,
}

impl Hash for QObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl PartialEq for QObject {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for QObject {}

/// Basic physics properties of a body
#[derive(Component, Debug, Clone)]
pub struct QPhysicsBody {
    /// Mass of the body in kg
    pub mass: Q64,
    /// Coefficient of restitution (bounciness), range [0, 1]
    pub restitution: Q64,
    /// Coefficient of friction, range [0, 1]
    pub friction: Q64,
}

impl QPhysicsBody {
    /// Create a new physics body with the given properties
    pub fn new(mass: Q64, restitution: Q64, friction: Q64) -> Self {
        Self {
            mass,
            restitution,
            friction,
        }
    }

    /// Create a static body (infinite mass, immovable)
    pub fn static_body(restitution: Q64, friction: Q64) -> Self {
        Self {
            mass: Q64::ZERO, // 0 mass indicates infinite mass (static)
            restitution,
            friction,
        }
    }

    /// Create a dynamic body with the given mass
    pub fn dynamic_body(mass: Q64, restitution: Q64, friction: Q64) -> Self {
        Self {
            mass,
            restitution,
            friction,
        }
    }

    /// Check if the body has infinite mass (is static)
    pub fn is_static(&self) -> bool {
        self.mass <= 0.0
    }

    /// Get the inverse mass (1/mass) of the body, or 0 for static bodies
    pub fn inverse_mass(&self) -> Q64 {
        if self.is_static() {
            Q64::ZERO
        } else {
            self.mass.saturating_recip()
        }
    }
}

/// Shape component for collision detection
#[derive(Component, Debug, Clone)]
pub enum QCollisionShape {
    Point(QPoint),
    Line(QLine),
    Circle(QCircle),
    Rectangle(QBbox),
    Polygon(QPolygon),
}

impl QCollisionShape {
    /// Convert to QPolygon for collision detection
    pub fn to_polygon(&self) -> QPolygon {
        match self {
            QCollisionShape::Point(point) => QPolygon::new(vec![*point]),
            QCollisionShape::Line(line) => QPolygon::new(line.points().clone()),
            QCollisionShape::Circle(circle) => circle.get_polygon(),
            QCollisionShape::Rectangle(rect) => rect.get_polygon(),
            QCollisionShape::Polygon(polygon) => polygon.clone(),
        }
    }

    /// Get bounding box of the shape
    pub fn get_bbox(&self) -> QBbox {
        match self {
            QCollisionShape::Point(point) => point.get_bbox(),
            QCollisionShape::Line(line) => line.get_bbox(),
            QCollisionShape::Circle(circle) => circle.get_bbox(),
            QCollisionShape::Rectangle(rect) => rect.get_bbox(),
            QCollisionShape::Polygon(polygon) => polygon.get_bbox(),
        }
    }

    /// Get centroid of the shape
    pub fn get_centroid(&self) -> QPoint {
        match self {
            QCollisionShape::Point(point) => point.get_centroid(),
            QCollisionShape::Line(line) => line.get_centroid(),
            QCollisionShape::Circle(circle) => circle.get_centroid(),
            QCollisionShape::Rectangle(rect) => rect.get_centroid(),
            QCollisionShape::Polygon(polygon) => polygon.get_centroid(),
        }
    }

    /// Check if a point is inside the shape
    pub fn is_point_inside(&self, point: &QPoint) -> bool {
        match self {
            QCollisionShape::Point(p) => p.is_point_inside(point),
            QCollisionShape::Line(l) => l.is_point_inside(point),
            QCollisionShape::Circle(c) => c.is_point_inside(point),
            QCollisionShape::Rectangle(r) => r.is_point_inside(point),
            QCollisionShape::Polygon(poly) => poly.is_point_inside(point),
        }
    }

    /// Check if this shape collides with another shape
    pub fn is_collide(&self, other: &QCollisionShape) -> bool {
        let self_polygon = self.to_polygon();
        let other_polygon = other.to_polygon();
        self_polygon.is_collide(&other_polygon)
    }

    /// Try to get separation vector between this shape and another shape
    pub fn try_get_separation_vector(&self, other: &QCollisionShape) -> Option<QVec2> {
        let self_polygon = self.to_polygon();
        let other_polygon = other.to_polygon();
        self_polygon.try_get_seperation_vector(&other_polygon)
    }
}

/// Motion state of a body
#[derive(Default, Component, Debug, Clone)]
pub struct QMotion {
    /// Linear velocity in units per second
    pub velocity: QVec2,
    /// Angular velocity in radians per second
    pub angular_velocity: Q64,
    /// Linear acceleration in units per second squared
    pub acceleration: QVec2,
}

impl QMotion {
    /// Create a new motion state
    pub fn new(velocity: QVec2, angular_velocity: Q64, acceleration: QVec2) -> Self {
        Self {
            velocity,
            angular_velocity,
            acceleration,
        }
    }

    /// Create motion with only linear velocity
    pub fn with_velocity(velocity: QVec2) -> Self {
        Self {
            velocity,
            ..Default::default()
        }
    }

    /// Create motion with only angular velocity
    pub fn with_angular_velocity(angular_velocity: Q64) -> Self {
        Self {
            angular_velocity,
            ..Default::default()
        }
    }
}

/// Collision flag for specifying collision behavior
#[derive(Component, Debug, Clone)]
pub struct QCollisionFlag {
    /// Whether this is a trigger (detects collisions but doesn't resolve them)
    pub is_trigger: bool,
    /// Collision layer this object belongs to
    pub collision_layer: u32,
    /// Collision mask specifying which layers this object can collide with
    pub collision_mask: u32,
}

impl Default for QCollisionFlag {
    fn default() -> Self {
        Self {
            is_trigger: false,
            collision_layer: 1,
            collision_mask: 0xFFFFFFFF,
        }
    }
}

impl QCollisionFlag {
    /// Create a new collision flag
    pub fn new(is_trigger: bool, collision_layer: u32, collision_mask: u32) -> Self {
        Self {
            is_trigger,
            collision_layer,
            collision_mask,
        }
    }

    /// Create a trigger flag
    pub fn trigger(collision_layer: u32, collision_mask: u32) -> Self {
        Self {
            is_trigger: true,
            collision_layer,
            collision_mask,
        }
    }

    /// Create a solid collision flag
    pub fn solid(collision_layer: u32, collision_mask: u32) -> Self {
        Self {
            is_trigger: false,
            collision_layer,
            collision_mask,
        }
    }

    /// Check if this object can collide with another based on layers
    pub fn can_collide_with(&self, other: &QCollisionFlag) -> bool {
        // Check if this object's mask includes the other's layer
        // and the other's mask includes this object's layer
        (self.collision_mask & other.collision_layer) != 0 && (other.collision_mask & self.collision_layer) != 0
    }
}

/// Describe the position of an 2d entity. If the entity has a parent, the position is relative
/// to its parent position.
#[derive(Clone, Copy, Component)]
pub struct QTransform {
    /// Position of the entity.
    pub position: QVec2,
    /// Rotation of the entity, in radians.
    pub rotation: QDir,
    /// Scale of the entity. X-Y dimension in same scale for simple calculations.
    pub scale: QVec2,
}

impl Default for QTransform {
    fn default() -> Self {
        Self {
            position: QVec2::ZERO,
            rotation: QDir::default(),
            scale: QVec2::ONE,
        }
    }
}

impl QTransform {
    pub fn apply_to(&self, shape: &QCollisionShape) -> QCollisionShape {
        match shape {
            QCollisionShape::Point(point) => {
                let pos = point.pos().saturating_mul(self.scale);
                let pos = self.rotation.rotate_vec(pos).saturating_add(self.position);
                QCollisionShape::Point(QPoint::new(pos))
            }
            QCollisionShape::Line(line) => {
                let start = line.start();
                let end = line.end();
                let s = self
                    .rotation
                    .rotate_vec(start.pos().saturating_mul(self.scale))
                    .saturating_add(self.position);
                let e = self
                    .rotation
                    .rotate_vec(end.pos().saturating_mul(self.scale))
                    .saturating_add(self.position);
                QCollisionShape::Line(QLine::new(QPoint::new(s), QPoint::new(e)))
            }
            QCollisionShape::Circle(circle) => {
                let center = circle.center();
                let center_pos = self
                    .rotation
                    .rotate_vec(center.pos().saturating_mul(self.scale))
                    .saturating_add(self.position);
                // Use geometric mean of absolute scales to scale radius (handles non-uniform scaling)
                let scale_mag = (self.scale.x.abs().saturating_mul(self.scale.y.abs())).saturating_sqrt();
                let mut radius = circle.radius().saturating_mul(scale_mag);
                if radius <= Q64::EPS {
                    radius = Q64::EPS;
                }
                QCollisionShape::Circle(QCircle::new(QPoint::new(center_pos), radius))
            }
            QCollisionShape::Rectangle(rect) => {
                let left_bottom = self
                    .rotation
                    .rotate_vec(rect.left_bottom().pos().saturating_mul(self.scale))
                    .saturating_add(self.position);
                let right_top = self
                    .rotation
                    .rotate_vec(rect.right_top().pos().saturating_mul(self.scale))
                    .saturating_add(self.position);
                QCollisionShape::Rectangle(QBbox::new_from_parts(left_bottom, right_top))
            }
            QCollisionShape::Polygon(polygon) => {
                let new_points: Vec<QPoint> = polygon
                    .points()
                    .iter()
                    .map(|p| {
                        QPoint::new(
                            self.rotation
                                .rotate_vec(p.pos().saturating_mul(self.scale))
                                .saturating_add(self.position),
                        )
                    })
                    .collect();
                QCollisionShape::Polygon(QPolygon::new(new_points))
            }
        }
    }
}

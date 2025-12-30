use bevy::prelude::*;
use qmath::vec2::QVec2;

pub fn qvec2vec(qvec: QVec2) -> Vec2 {
    Vec2::new(qvec.x.to_num::<f32>(), qvec.y.to_num::<f32>())
}

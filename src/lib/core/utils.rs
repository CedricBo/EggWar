use bevy::math::{Rect, Vec2};

pub struct IntersectDetails
{
    pub x: bool,
    pub y: bool,
}

pub fn is_intersect(a: (Vec2, Vec2), b: (Vec2, Vec2)) -> bool {
    let details = intersect_details(a, b);

    details.x && details.y
}

pub fn intersect_details(a: (Vec2, Vec2), b: (Vec2, Vec2)) -> IntersectDetails
{
    let (a_pos, a_size) = a;
    let (b_pos, b_size) = b;

    let sub = (a_pos - b_pos).abs();

    let size = (a_size + b_size) / 2.0;

    IntersectDetails { 
        x: sub.x <= size.x,
        y: sub.y <= size.y,
    }
}
use crate::{Color, Entity, Image, Plane, Pose, Texture, Vector, world::empty::Empty};

pub fn create_block_map(map: Image, texture_mapper: fn(Color) -> Option<Texture>) -> Entity {
    let mut root: Entity = Empty::default().into();

    let is_empty = |x: i32, y: i32| {
        if x < 0 || y < 0 || x >= map.width() || y >= map.height() {
            return true;
        }

        if texture_mapper(map.get(x, y)).is_some() {
            false
        } else {
            true
        }
    };

    for (x, y) in map.coordinates() {
        let color = map.get(x, y);
        let Some(texture) = texture_mapper(color) else {
            continue;
        };

        let directions = [
            (0, -1, Side::North),
            (0, 1, Side::South),
            (1, 0, Side::East),
            (-1, 0, Side::West),
        ];

        for (dx, dy, side) in directions {
            if is_empty(x + dx, y + dy) {
                let coordinates = get_plane_coordinates(x, y, side);
                let plane = Plane::new(coordinates.pos, coordinates.dir, texture.clone());
                let mut plane: Entity = plane.into();
                plane.set_parent(Some(&mut root));
            }
        }
    }

    root
}

enum Side {
    North,
    South,
    East,
    West,
}

fn get_plane_coordinates(x: i32, y: i32, side: Side) -> Pose {
    let x = x as f32;
    let y = y as f32;

    match side {
        Side::North => Pose::new((x + 1.0, -y), Vector::WEST),
        Side::South => Pose::new((x, -1.0 - y), Vector::EAST),
        Side::East => Pose::new((x + 1.0, -1.0 - y), Vector::NORTH),
        Side::West => Pose::new((x, y), Vector::SOUTH),
    };

    Pose::default()
}

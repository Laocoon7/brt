use bevy::prelude::*;

pub fn despawn_with<T: Component>(mut commands: Commands, q_entities: Query<Entity, With<T>>) {
    for entity in q_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

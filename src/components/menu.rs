use bevy::ecs::component::Component;



#[derive(Component)]
pub struct InputField;

#[derive(Component)]
pub struct InputValue(pub String);

#[derive(Component)]
pub struct Focused;


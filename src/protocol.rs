use bevy::math::Vec3;
use bevy::prelude::{default, Bundle, Color, Component, Deref, DerefMut, Entity, Vec2};
use bevy::utils::EntityHashSet;
use derive_more::{Add, Mul};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

// Player
#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    id: PlayerId,
    position: PlayerPosition,
    color: PlayerColor,
    replicate: Replicate,
}

impl PlayerBundle {
    pub(crate) fn new(id: ClientId, position: Vec3, color: Color) -> Self {
        Self {
            id: PlayerId(id),
            position: PlayerPosition(position),
            color: PlayerColor(color),
            replicate: Replicate {
                // prediction_target: NetworkTarget::None,
                prediction_target: NetworkTarget::Only(vec![id]),
                interpolation_target: NetworkTarget::AllExcept(vec![id]),
                ..default()
            },
        }
    }
}

// Components

#[derive(Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(pub (crate) ClientId);

/// Player position in 3D space. The world player z is always 0.
/// 
/// The x and y coordinates are the position in the 2D plane.
/// 
/// and the z coordinate is the angle of the camera in degrees.
/// 0 is x positive, 90 is y positive directions. CCW is + rotation(planned).
#[derive(
    Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul,
)]
pub struct PlayerPosition(Vec3);

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlayerColor(pub(crate) Color);

// Example of a component that contains an entity.
// This component, when replicated, needs to have the inner entity mapped from the Server world
// to the client World.
// This can be done by adding a `#[message(custom_map)]` attribute to the component, and then
// deriving the `MapEntities` trait for the component.
#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[message(custom_map)]
pub struct PlayerParent(Entity);

impl<'a> MapEntities<'a> for PlayerParent {
    fn map_entities(&mut self, entity_mapper: Box<dyn EntityMapper + 'a>) {
        self.0.map_entities(entity_mapper);
    }

    fn entities(&self) -> EntityHashSet<Entity> {
        EntityHashSet::from_iter(vec![self.0])
    }
}

#[component_protocol(protocol = "MyProtocol")]
pub enum Components {
    #[sync(once)]
    PlayerId(PlayerId),
    #[sync(full)]
    PlayerPosition(PlayerPosition),
    #[sync(once)]
    PlayerColor(PlayerColor),
}

// Channels

#[derive(Channel)]
pub struct Channel1;

// Messages

#[derive(Message, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);

#[message_protocol(protocol = "MyProtocol")]
pub enum Messages {
    Message1(Message1),
}

// Inputs

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Direction {
    pub(crate) up: bool, // it is forward
    pub(crate) down: bool, // it is backward
    pub(crate) left: bool, // it is turn left
    pub(crate) right: bool, // it is turn right
}

impl Direction {
    pub(crate) fn is_none(&self) -> bool {
        !self.up && !self.down && !self.left && !self.right
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Inputs {
    Direction(Direction),
    Delete,
    Spawn,
    None,
}

impl UserInput for Inputs {}

// Protocol

protocolize! {
    Self = MyProtocol,
    Message = Messages,
    Component = Components,
    Input = Inputs,
}

pub(crate) fn protocol() -> MyProtocol {
    let mut protocol = MyProtocol::default();
    protocol.add_channel::<Channel1>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        direction: ChannelDirection::Bidirectional,
    });
    protocol
}
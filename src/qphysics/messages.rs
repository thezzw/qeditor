use bevy::prelude::*;

/// Trigger events for detecting when objects enter/exit trigger areas
#[derive(Message, Debug, Clone)]
pub enum QTriggerEvent {
    /// Fired when an object enters a trigger area
    Enter(Entity, Entity),
    /// Fired while an object stays in a trigger area
    Stay(Entity, Entity),
    /// Fired when an object exits a trigger area
    Exit(Entity, Entity),
}

impl QTriggerEvent {
    /// Get the entities involved in this trigger event
    pub fn entities(&self) -> (Entity, Entity) {
        match self {
            QTriggerEvent::Enter(e1, e2) => (*e1, *e2),
            QTriggerEvent::Stay(e1, e2) => (*e1, *e2),
            QTriggerEvent::Exit(e1, e2) => (*e1, *e2),
        }
    }

    /// Check if this is an Enter event
    pub fn is_enter(&self) -> bool {
        matches!(self, QTriggerEvent::Enter(_, _))
    }

    /// Check if this is a Stay event
    pub fn is_stay(&self) -> bool {
        matches!(self, QTriggerEvent::Stay(_, _))
    }

    /// Check if this is an Exit event
    pub fn is_exit(&self) -> bool {
        matches!(self, QTriggerEvent::Exit(_, _))
    }
}

/// Collision events for detecting when objects collide
#[derive(Message, Debug, Clone)]
pub enum QCollisionEvent {
    /// Fired when two objects begin colliding
    Started(Entity, Entity),
    /// Fired while two objects are colliding
    Ongoing(Entity, Entity),
    /// Fired when two objects stop colliding
    Ended(Entity, Entity),
}

impl QCollisionEvent {
    /// Get the entities involved in this collision event
    pub fn entities(&self) -> (Entity, Entity) {
        match self {
            QCollisionEvent::Started(e1, e2) => (*e1, *e2),
            QCollisionEvent::Ongoing(e1, e2) => (*e1, *e2),
            QCollisionEvent::Ended(e1, e2) => (*e1, *e2),
        }
    }

    /// Check if this is a Started event
    pub fn is_started(&self) -> bool {
        matches!(self, QCollisionEvent::Started(_, _))
    }

    /// Check if this is an Ongoing event
    pub fn is_ongoing(&self) -> bool {
        matches!(self, QCollisionEvent::Ongoing(_, _))
    }

    /// Check if this is an Ended event
    pub fn is_ended(&self) -> bool {
        matches!(self, QCollisionEvent::Ended(_, _))
    }
}

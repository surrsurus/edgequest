//!
//! Metapackage to extend an interface to ai
//! 

pub mod ai;
pub use self::ai::AI;

// Needs a serious look at how AI is to be done, should there be 
// one monolithic AI that can interpret properties from enums (CompositeAI)
// or various different AI that extends from an AI trait? (Every other AI)

// pub mod composite;
// pub use self::composite::CompositeAI;

pub mod simple;
pub use self::simple::SimpleAI;

pub mod blink;
pub use self::blink::BlinkAI;

pub mod tracker;
pub use self::tracker::TrackerAI;
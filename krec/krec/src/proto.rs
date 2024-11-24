pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/protos/krec.rs"));
}

pub use proto::{
    ActuatorCommand, ActuatorConfig, ActuatorState, ImuQuaternion, ImuValues, KRecFrame, KRecHeader,
};

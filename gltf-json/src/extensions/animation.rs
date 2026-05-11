use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

/// A keyframe animation.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Animation {
    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Channel {}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Target {
    /// `KHR_animation_pointer` extension data.
    #[cfg(feature = "KHR_animation_pointer")]
    #[serde(
        default,
        rename = "KHR_animation_pointer",
        skip_serializing_if = "Option::is_none"
    )]
    pub khr_animation_pointer: Option<KhrAnimationPointer>,

    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}

/// `KHR_animation_pointer` extension data for an animation channel target.
///
/// This extension allows animation channels to target arbitrary glTF properties
/// using JSON pointers, rather than being limited to node transform and morph
/// target weight properties.
#[cfg(feature = "KHR_animation_pointer")]
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct KhrAnimationPointer {
    /// A JSON pointer (RFC 6901) to the property to animate.
    pub pointer: String,
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Sampler {}

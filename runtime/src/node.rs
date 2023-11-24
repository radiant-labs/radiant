use epaint::ClippedPrimitive;
use radiantkit_core::{
    RadiantComponent, RadiantComponentProvider, RadiantNode, RadiantRectangleNode,
    RadiantTessellatable, ScreenDescriptor,
};
use radiantkit_core::{RadiantDocumentNode, RadiantGroupNode};
use radiantkit_image::RadiantImageNode;
use radiantkit_macros::{RadiantComponentProvider, RadiantNode, RadiantTessellatable};
use radiantkit_path::RadiantPathNode;
use radiantkit_text::RadiantTextNode;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    RadiantComponentProvider,
    RadiantNode,
    RadiantTessellatable,
)]
pub enum RadiantNodeType {
    Document(RadiantDocumentNode<RadiantNodeType>),
    Artboard(RadiantGroupNode<RadiantNodeType>),
    Rectangle(RadiantRectangleNode),
    Path(RadiantPathNode),
    Image(RadiantImageNode),
    Text(RadiantTextNode),
    #[cfg(not(target_arch = "wasm32"))]
    Video(radiantkit_video::RadiantVideoNode),
}

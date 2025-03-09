use std::fmt::Debug;
use std::sync::Arc;

use crate::effects::node::EffectNode;

#[derive(Debug)]
pub struct EffectGraph<'a> {
    pub nodes: Vec<Arc<EffectNode<'a>>>,
}
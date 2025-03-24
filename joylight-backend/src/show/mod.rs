//! The show contains all information about the current state of the application, including
//! fixtures, programs, effects and other options.
//! 
//! ## Layers: Giving values to parameters
//! When an effect, playback etc. wants to set a value to a parameter, it does not do so directly.
//! Instead, it sends this information to the corresponding **layer**. Each parameter can be affected
//! my multiple layers. The priorities of these layers will define the final value of each
//! parameter.
//! 
//! ## Frames
//! Processing is done periodically, in frames. Each frame evaluates the status of playbacks, updates
//! the layers with new information, and sets the parameter values accordingly.

pub mod layer;
pub mod show;

pub use layer::*;
pub use show::*;

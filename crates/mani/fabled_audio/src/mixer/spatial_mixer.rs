// Notes for implementation.
// Should I make this a struct, impl from RawClip or just a pub function?

// Current implementation different audio transformation
// Buffered (Stores the source in a buffer in addition to returning it. This
// iterator can be cloned), Mix (Mixes this source with another one),
// repeat_infinite (Repeats this source forever might be required),
// take_duration (Takes a certain duration of this source and then stops),
// delay (Delays the sound by a certain duration) skip_duration (Immediately
// skips a certain duration of this source. If duration > source length then
// goes to the end of the source), amplify (Amplifies the sound by the given
// value.)

// If i make it a struct then i can just extract in once then store it then
// re-build the raw clip in the end and return it.
// Got to make use of generic since modifying the audio data will return a
// different struct with the same trait.

use crate::RawAmbisonicClip;

pub struct SpatialMixer<T: ambisonic::rodio::Source<Item = f32>>(RawAmbisonicClip<T>);

impl<T: ambisonic::rodio::Source<Item = f32>> SpatialMixer<T> {
    // create the transformation for the clip.
}

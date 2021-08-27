pub enum TextureBlending {
    /// Set horizontal (U) texture blending to true. Set vertical (V) texture blending to false
    BlendU,
    /// Set vertical (V) texture blending to true. Set horizontal (U) texture blending to false
    BlendV,
    // Set both horizontal and vertical (UV) texture blending to true.
    BlendUV,
}

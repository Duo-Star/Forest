// src/color.rs

/// A collection of beautiful, constant colors for rendering.
/// Each color is represented as [r, g, b, a] in f32 format (range 0.0 to 1.0).
///
/// Usage example:
/// ```rust
/// let background = Color::BLACK;
/// let curve = Color::BLUE;
/// let highlight = Color::YELLOW;
/// ```
pub struct Color;

/// A collection of beautiful, constant colors for rendering.
/// All colors are defined as static [f32; 4] arrays with full opacity (alpha = 1.0).
impl Color {
    // ==========================================
    // The Monochrome (单色系)
    // ==========================================

    /// **Obsidian Black** (黑曜石)
    /// The abyssal depth of space. A perfect absorber, defining boundaries with absolute certainty.
    /// Ideal for backgrounds that highlight the brilliance of data.
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    /// **Dark Matter** (暗物质)
    /// A subtle, charcoal grey. It whispers of structure without the harshness of absolute black.
    /// Perfect for grid lines that guide the eye without distraction.
    pub const DARK_GRAY: [f32; 4] = [0.1, 0.1, 0.12, 1.0];

    /// **Titanium White** (钛白)
    /// The absolute void of light's absence, or the totality of its presence.
    /// A blank canvas awaiting the stroke of mathematical creation.
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    // ==========================================
    // The Elements (自然元素系)
    // ==========================================

    /// **Crimson Ember** (绯红余烬)
    /// The burning heart of a dying star. Passionate, alert, and impossible to ignore.
    /// Use this to draw attention to critical singularities or errors.
    pub const RED: [f32; 4] = [0.9, 0.2, 0.2, 1.0];

    /// **Azure Sky** (蔚蓝苍穹)
    /// A fragment of the clear summer sky, suspended in eternal noon.
    /// Cool, calculated, and infinite. The standard for serene mathematical curves.
    pub const BLUE: [f32; 4] = [0.2, 0.5, 1.0, 1.0];

    /// **Emerald Forest** (翡翠森林)
    /// The verdant whisper of a spring meadow. A soothing tone of growth and algorithmic harmony.
    /// Represents stability and the natural order of functions.
    pub const GREEN: [f32; 4] = [0.2, 0.8, 0.3, 1.0];

    /// **Golden Sun** (金色暖阳)
    /// The radiant energy of a sunrise. A burst of optimism to highlight the peaks of your data.
    /// Warm, energetic, and enlightening.
    pub const YELLOW: [f32; 4] = [1.0, 0.85, 0.1, 1.0];

    // ==========================================
    // The Synthetic (合成霓虹系)
    // ==========================================

    /// **Cyber Cyan** (赛博青)
    /// The electric glow of a futuristic city interface. A sharp intersection of sky and sea.
    /// Perfect for parametric equations that weave through the digital void.
    pub const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

    /// **Neon Magenta** (霓虹洋红)
    /// A playful dance of synthetic light. The color of mathematical whimsy and complex numbers.
    /// Bold, unapologetic, and vibrantly alive.
    pub const MAGENTA: [f32; 4] = [1.0, 0.2, 0.8, 1.0];

    /// **Electric Purple** (电光紫)
    /// The majestic shroud of twilight mystery. Where imaginary numbers dream.
    /// Deep, royal, and profoundly sophisticated.
    pub const PURPLE: [f32; 4] = [0.6, 0.2, 1.0, 1.0];

    /// **Plasma Orange** (等离子橙)
    /// The high-energy bridge between red heat and yellow light.
    /// It creates a vibrant contrast against cool blue tones.
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];

    // ==========================================
    // The Soft & Pastel (柔和淡雅系)
    // ==========================================

    /// **Sakura Pink** (樱花粉)
    /// A gentle, fleeting touch of spring blossoms. Soft, unthreatening, and delicate.
    pub const SOFT_PINK: [f32; 4] = [1.0, 0.7, 0.75, 1.0];

    /// **Glacial Ice** (冰川蓝)
    /// The pale, translucent blue of ancient ice. Almost white, but holding a deep, cold secret.
    pub const ICE_BLUE: [f32; 4] = [0.7, 0.9, 1.0, 1.0];

    /// **Mint Cream** (薄荷奶油)
    /// A refreshing, light green that sits quietly in the background.
    pub const MINT: [f32; 4] = [0.6, 1.0, 0.7, 1.0];
}
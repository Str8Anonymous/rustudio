//! The Device Simulator's devices: Roblox Studio's built-in presets, the
//! rules Studio sizes them by, and the input each one reports.
//!
//! The table was read from Roblox Studio 0.748's built-in device table (its
//! EmulatedDevices XML) and measured in Studio's Play: the graphics quality
//! level each device picks (`settings().Rendering.QualityLevel`, the bar's
//! Q##) and `GuiService.ViewportDisplaySize`. The legacy rows Studio keeps
//! out of its device list (Default, Average Laptop) are left out.

use std::sync::OnceLock;

/// Studio's form factor (`device_form`): no form is a desktop, `vr` a headset.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Form {
    Phone,
    Tablet,
    Desktop,
    Console,
    Headset,
}

/// The simulator bar's four form buttons, in the bar's order.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FormButton {
    Mobile,
    Desktop,
    Console,
    Headset,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Platform {
    Ios,
    Android,
    Windows,
    MacOs,
    XboxOne,
    Ps4,
    Ps5,
    MetaOs,
}

/// Studio's `tier`: the dropdown's Flagship / Mid-range / Entry level groups.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Tier {
    Flagship,
    MidRange,
    EntryLevel,
}

/// Safe-area insets in device points, in the XML's order.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Insets {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Insets {
    pub const fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self { top, bottom, left, right }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub enum Orientation {
    #[default]
    Landscape,
    Portrait,
}

/// `UserInputService.PreferredInput`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Preferred {
    KeyboardAndMouse,
    Touch,
    Gamepad,
}

impl Preferred {
    pub fn enum_name(self) -> &'static str {
        match self {
            Self::KeyboardAndMouse => "KeyboardAndMouse",
            Self::Touch => "Touch",
            Self::Gamepad => "Gamepad",
        }
    }
}

/// What `UserInputService` and `GuiService:IsTenFootInterface()` report.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct InputFlags {
    pub touch: bool,
    pub keyboard: bool,
    pub mouse: bool,
    pub gamepad: bool,
    pub ten_foot: bool,
    pub preferred: Preferred,
}

impl InputFlags {
    /// Studio's Play with no device simulated: keyboard and mouse.
    pub const DESKTOP: Self = Self {
        touch: false,
        keyboard: true,
        mouse: true,
        gamepad: false,
        ten_foot: false,
        preferred: Preferred::KeyboardAndMouse,
    };
}

/// Where Studio's frame image puts the screen (`wireframe_screen_x/y`, in
/// image pixels), the image's scale and whether the screen has rounded
/// corners (`special_corners`).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FrameSpec {
    pub screen_offset: [f32; 2],
    pub image_scale: f32,
    pub special_corners: bool,
}

/// `GuiService.ViewportDisplaySize`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum DisplaySize {
    Small,
    Medium,
    Large,
}

impl DisplaySize {
    pub fn enum_name(self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub form: Form,
    pub platform: Platform,
    pub tier: Option<Tier>,
    pub memory_mb: u32,
    /// Width and height in points, landscape.
    pub size: [u32; 2],
    pub ppi: f32,
    pub resolution_scale: f32,
    pub safe_portrait: Insets,
    pub safe_landscape: Insets,
    /// On-screen keyboard height: portrait, landscape.
    pub keyboard_height: [u32; 2],
    pub frame: Option<FrameSpec>,
    /// The default graphics quality level (the bar's Q##).
    pub quality: u8,
    pub display_size: DisplaySize,
    /// Made in Manage devices, not one of Studio's presets.
    pub custom: bool,
}

struct Row {
    id: &'static str,
    name: &'static str,
    form: Form,
    platform: Platform,
    tier: Option<Tier>,
    memory_mb: u32,
    size: [u32; 2],
    ppi: f32,
    resolution_scale: f32,
    safe_portrait: [f32; 4],
    safe_landscape: [f32; 4],
    keyboard_height: [u32; 2],
    frame: Option<([f32; 2], f32, bool)>,
    quality: u8,
    display_size: DisplaySize,
}

use DisplaySize::{Large, Medium, Small};
use Form::{Console, Desktop, Headset, Phone, Tablet};
use Platform::{Android, Ios, MacOs, MetaOs, Ps4, Ps5, Windows, XboxOne};
use Tier::{EntryLevel, Flagship, MidRange};

/// Studio's presets in its table's order. The generic handhelds' XML names
/// (`Generic_Handheld_720`) are shown as Studio's device list shows them.
/// The Galaxy Tab A9 has no `wireframe_scale`: 0.5, as the other Android
/// tablets (estimate).
#[rustfmt::skip]
const ROWS: &[Row] = &[
    Row { id: "xbox", name: "Xbox One", form: Console, platform: XboxOne, tier: None, memory_mb: 8192, size: [1920, 1080], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 12, display_size: Large },
    Row { id: "ps4", name: "PS4", form: Console, platform: Ps4, tier: None, memory_mb: 8192, size: [1920, 1080], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 13, display_size: Large },
    Row { id: "ps5", name: "PS5", form: Console, platform: Ps5, tier: None, memory_mb: 16384, size: [1920, 1080], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 21, display_size: Large },
    Row { id: "android_tv_1080", name: "Android TV 1080p", form: Tablet, platform: Android, tier: None, memory_mb: 2048, size: [1920, 1080], ppi: 44.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [54.0, 54.0, 96.0, 96.0], keyboard_height: [0, 540], frame: None, quality: 1, display_size: Large },
    Row { id: "average_windows", name: "Average Windows", form: Desktop, platform: Windows, tier: None, memory_mb: 8192, size: [1366, 768], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 17, display_size: Medium },
    Row { id: "average_macbook", name: "Average MacBook", form: Desktop, platform: MacOs, tier: None, memory_mb: 8192, size: [1440, 900], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 21, display_size: Medium },
    Row { id: "hd_720", name: "HD 720", form: Desktop, platform: Windows, tier: None, memory_mb: 8192, size: [1280, 720], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 17, display_size: Medium },
    Row { id: "hd_1080", name: "HD 1080", form: Desktop, platform: Windows, tier: None, memory_mb: 8192, size: [1920, 1080], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 17, display_size: Medium },
    Row { id: "vga", name: "VGA", form: Desktop, platform: Windows, tier: None, memory_mb: 8192, size: [640, 480], ppi: 96.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 17, display_size: Small },
    Row { id: "meta_quest_2", name: "Meta Quest 2", form: Headset, platform: MetaOs, tier: None, memory_mb: 6144, size: [611, 640], ppi: 773.0, resolution_scale: 3.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 14, display_size: Medium },
    Row { id: "meta_quest_3", name: "Meta Quest 3", form: Headset, platform: MetaOs, tier: None, memory_mb: 8192, size: [688, 736], ppi: 1218.0, resolution_scale: 3.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 21, display_size: Medium },
    Row { id: "generic_handheld_720", name: "Generic Handheld HD 720", form: Console, platform: Windows, tier: None, memory_mb: 16384, size: [1280, 720], ppi: 274.0, resolution_scale: 1.5, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 21, display_size: Small },
    Row { id: "generic_handheld_1080", name: "Generic Handheld HD 1080", form: Console, platform: Windows, tier: None, memory_mb: 16384, size: [1920, 1080], ppi: 411.0, resolution_scale: 1.5, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [0, 0], frame: None, quality: 21, display_size: Small },
    Row { id: "iphone_6_Plus", name: "iPhone 6 Plus", form: Phone, platform: Ios, tier: Some(EntryLevel), memory_mb: 1024, size: [736, 414], ppi: 401.0, resolution_scale: 3.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [226, 162], frame: Some(([126.0, 34.0], 0.3333, false)), quality: 1, display_size: Small },
    Row { id: "iphone_7", name: "iPhone 7", form: Phone, platform: Ios, tier: Some(EntryLevel), memory_mb: 2048, size: [667, 375], ppi: 326.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [216, 162], frame: Some(([114.0, 30.0], 0.5, false)), quality: 12, display_size: Small },
    Row { id: "iphone_XR", name: "iPhone XR", form: Phone, platform: Ios, tier: Some(EntryLevel), memory_mb: 3072, size: [896, 414], ppi: 326.0, resolution_scale: 2.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [313, 186], frame: Some(([38.0, 46.0], 0.5, true)), quality: 21, display_size: Small },
    Row { id: "iphone_11", name: "iPhone 11", form: Phone, platform: Ios, tier: Some(EntryLevel), memory_mb: 4096, size: [896, 414], ppi: 326.0, resolution_scale: 2.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [313, 186], frame: Some(([38.0, 46.0], 0.5, true)), quality: 21, display_size: Small },
    Row { id: "samsung_galaxy_a06", name: "Samsung Galaxy A06", form: Phone, platform: Android, tier: Some(EntryLevel), memory_mb: 4096, size: [800, 360], ppi: 262.0, resolution_scale: 2.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [280, 162], frame: Some(([41.0, 27.0], 0.5, true)), quality: 1, display_size: Small },
    Row { id: "iphone_13", name: "iPhone 13", form: Phone, platform: Ios, tier: Some(MidRange), memory_mb: 4096, size: [844, 390], ppi: 460.0, resolution_scale: 3.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [295, 176], frame: Some(([24.0, 31.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "iphone_13_pro", name: "iPhone 13 Pro", form: Phone, platform: Ios, tier: Some(MidRange), memory_mb: 6144, size: [844, 390], ppi: 460.0, resolution_scale: 3.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [295, 176], frame: Some(([24.0, 31.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "iphone_13_pro_max", name: "iPhone 13 Pro Max", form: Phone, platform: Ios, tier: Some(MidRange), memory_mb: 6144, size: [926, 428], ppi: 458.0, resolution_scale: 3.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [324, 192], frame: Some(([24.0, 31.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "iphone_14", name: "iPhone 14", form: Phone, platform: Ios, tier: Some(MidRange), memory_mb: 6144, size: [844, 390], ppi: 460.0, resolution_scale: 3.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [295, 176], frame: Some(([24.0, 31.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "samsung_galaxy_a16", name: "Samsung Galaxy A16", form: Phone, platform: Android, tier: Some(MidRange), memory_mb: 6144, size: [780, 360], ppi: 385.0, resolution_scale: 3.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [273, 162], frame: Some(([41.0, 26.0], 0.3333, true)), quality: 10, display_size: Small },
    Row { id: "iphone_16", name: "iPhone 16", form: Phone, platform: Ios, tier: Some(Flagship), memory_mb: 8192, size: [852, 393], ppi: 460.0, resolution_scale: 3.0, safe_portrait: [59.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 59.0, 59.0], keyboard_height: [298, 177], frame: Some(([22.0, 29.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "iphone_16_pro", name: "iPhone 16 Pro", form: Phone, platform: Ios, tier: Some(Flagship), memory_mb: 8192, size: [874, 402], ppi: 460.0, resolution_scale: 3.0, safe_portrait: [62.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 62.0, 62.0], keyboard_height: [306, 181], frame: Some(([17.0, 25.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "iphone_16_pro_max", name: "iPhone 16 Pro Max", form: Phone, platform: Ios, tier: Some(Flagship), memory_mb: 8192, size: [956, 440], ppi: 460.0, resolution_scale: 3.0, safe_portrait: [62.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 62.0, 62.0], keyboard_height: [335, 198], frame: Some(([18.0, 25.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "iphone_17_pro", name: "iPhone 17 Pro", form: Phone, platform: Ios, tier: Some(Flagship), memory_mb: 12288, size: [874, 402], ppi: 460.0, resolution_scale: 3.0, safe_portrait: [62.0, 34.0, 0.0, 0.0], safe_landscape: [20.0, 20.0, 62.0, 62.0], keyboard_height: [306, 181], frame: Some(([17.0, 25.0], 0.3333, true)), quality: 21, display_size: Small },
    Row { id: "samsung_galaxy_s22_ultra", name: "Samsung Galaxy S22 Ultra", form: Phone, platform: Android, tier: Some(Flagship), memory_mb: 8192, size: [772, 360], ppi: 501.0, resolution_scale: 4.0, safe_portrait: [47.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [270, 162], frame: Some(([14.0, 18.0], 0.25, true)), quality: 15, display_size: Small },
    Row { id: "samsung_galaxy_s25_ultra", name: "Samsung Galaxy S25 Ultra", form: Phone, platform: Android, tier: Some(Flagship), memory_mb: 12288, size: [780, 360], ppi: 498.0, resolution_scale: 4.0, safe_portrait: [7.0, 34.0, 0.0, 0.0], safe_landscape: [0.0, 21.0, 47.0, 47.0], keyboard_height: [273, 162], frame: Some(([10.0, 18.0], 0.25, true)), quality: 21, display_size: Small },
    Row { id: "ipad_6th_generation", name: "iPad 6th Generation", form: Tablet, platform: Ios, tier: Some(EntryLevel), memory_mb: 1981, size: [1024, 768], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [320, 408], frame: Some(([135.0, 54.0], 0.5, false)), quality: 21, display_size: Small },
    Row { id: "ipad_8th_generation", name: "iPad 8th Generation", form: Tablet, platform: Ios, tier: Some(EntryLevel), memory_mb: 3072, size: [1080, 810], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [324, 408], frame: Some(([115.0, 58.0], 0.5, false)), quality: 21, display_size: Small },
    Row { id: "ipad_9th_generation", name: "iPad 9th Generation", form: Tablet, platform: Ios, tier: Some(EntryLevel), memory_mb: 3072, size: [1080, 810], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [324, 408], frame: Some(([115.0, 58.0], 0.5, false)), quality: 21, display_size: Small },
    Row { id: "xiaomi_redmi_pad_se", name: "Xiaomi Redmi Pad SE", form: Tablet, platform: Android, tier: Some(EntryLevel), memory_mb: 4096, size: [960, 600], ppi: 206.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [288, 302], frame: Some(([40.0, 45.0], 0.5, true)), quality: 2, display_size: Medium },
    Row { id: "amazon_fire_hd10_2023", name: "Amazon Fire HD 10 (2023)", form: Tablet, platform: Android, tier: Some(EntryLevel), memory_mb: 3072, size: [960, 600], ppi: 224.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [288, 302], frame: Some(([69.0, 64.0], 0.5, false)), quality: 2, display_size: Small },
    Row { id: "ipad_10th_generation", name: "iPad 10th Generation", form: Tablet, platform: Ios, tier: Some(MidRange), memory_mb: 4096, size: [1180, 820], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [354, 413], frame: Some(([54.0, 60.0], 0.5, true)), quality: 21, display_size: Medium },
    Row { id: "ipad_a16", name: "iPad A16", form: Tablet, platform: Ios, tier: Some(MidRange), memory_mb: 6144, size: [1180, 820], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [354, 413], frame: Some(([54.0, 60.0], 0.5, true)), quality: 21, display_size: Medium },
    Row { id: "samsung_galaxy_tab_a8", name: "Samsung Galaxy Tab A8", form: Tablet, platform: Android, tier: Some(MidRange), memory_mb: 4096, size: [960, 600], ppi: 216.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [288, 302], frame: Some(([44.0, 49.0], 0.5, true)), quality: 2, display_size: Medium },
    Row { id: "samsung_galaxy_tab_a9", name: "Samsung Galaxy Tab A9", form: Tablet, platform: Android, tier: Some(MidRange), memory_mb: 4096, size: [1340, 800], ppi: 179.0, resolution_scale: 1.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [402, 403], frame: Some(([81.0, 54.0], 0.5, true)), quality: 14, display_size: Small },
    Row { id: "ipad_air_5th_generation", name: "iPad Air 5th Generation", form: Tablet, platform: Ios, tier: Some(Flagship), memory_mb: 8192, size: [1180, 820], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [354, 413], frame: Some(([61.0, 61.0], 0.5, true)), quality: 21, display_size: Medium },
    Row { id: "ipad_pro_M4_11in", name: "iPad Pro M4 (11in)", form: Tablet, platform: Ios, tier: Some(Flagship), memory_mb: 8192, size: [1210, 834], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [363, 420], frame: Some(([48.0, 54.0], 0.5, true)), quality: 21, display_size: Medium },
    Row { id: "ipad_pro_M5_13in", name: "iPad Pro M5 (13in)", form: Tablet, platform: Ios, tier: Some(Flagship), memory_mb: 12288, size: [1376, 1032], ppi: 264.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [413, 520], frame: Some(([45.0, 51.0], 0.5, true)), quality: 21, display_size: Medium },
    Row { id: "samsung_galaxy_tab_a9+", name: "Samsung Galaxy Tab A9+", form: Tablet, platform: Android, tier: Some(Flagship), memory_mb: 8192, size: [960, 600], ppi: 206.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [288, 302], frame: Some(([45.0, 50.0], 0.5, true)), quality: 8, display_size: Medium },
    Row { id: "samsung_galaxy_tab_S11", name: "Samsung Galaxy Tab S11", form: Tablet, platform: Android, tier: Some(Flagship), memory_mb: 12288, size: [1280, 800], ppi: 274.0, resolution_scale: 2.0, safe_portrait: [0.0, 0.0, 0.0, 0.0], safe_landscape: [0.0, 0.0, 0.0, 0.0], keyboard_height: [384, 403], frame: Some(([52.0, 58.0], 0.5, true)), quality: 21, display_size: Medium },
];

fn insets(v: [f32; 4]) -> Insets {
    Insets::new(v[0], v[1], v[2], v[3])
}

/// Studio's 43 presets, in its table's order.
pub fn presets() -> &'static [Device] {
    static LIST: OnceLock<Vec<Device>> = OnceLock::new();
    LIST.get_or_init(|| {
        ROWS.iter()
            .map(|r| Device {
                id: r.id.to_string(),
                name: r.name.to_string(),
                form: r.form,
                platform: r.platform,
                tier: r.tier,
                memory_mb: r.memory_mb,
                size: r.size,
                ppi: r.ppi,
                resolution_scale: r.resolution_scale,
                safe_portrait: insets(r.safe_portrait),
                safe_landscape: insets(r.safe_landscape),
                keyboard_height: r.keyboard_height,
                frame: r.frame.map(|(screen_offset, image_scale, special_corners)| FrameSpec {
                    screen_offset,
                    image_scale,
                    special_corners,
                }),
                quality: r.quality,
                display_size: r.display_size,
                custom: false,
            })
            .collect()
    })
}

pub fn preset(id: &str) -> Option<&'static Device> {
    presets().iter().find(|d| d.id == id)
}

impl Device {
    pub fn button(&self) -> FormButton {
        match self.form {
            Form::Phone | Form::Tablet => FormButton::Mobile,
            Form::Desktop => FormButton::Desktop,
            Form::Console => FormButton::Console,
            Form::Headset => FormButton::Headset,
        }
    }

    /// Phones and tablets turn between landscape and portrait.
    pub fn rotates(&self) -> bool {
        matches!(self.form, Form::Phone | Form::Tablet)
    }

    /// The screen in points for an orientation (the table is landscape).
    pub fn points(&self, o: Orientation) -> [u32; 2] {
        match o {
            Orientation::Landscape => self.size,
            Orientation::Portrait if self.rotates() => [self.size[1], self.size[0]],
            Orientation::Portrait => self.size,
        }
    }

    /// The safe-area insets Studio applies. On iOS in landscape the bottom
    /// inset (the home indicator) is not taken off (measured: iPhone 14
    /// 749x389, not 749x368; Android uses all four, Galaxy S22 677x338).
    pub fn safe_insets(&self, o: Orientation) -> Insets {
        match o {
            Orientation::Portrait if self.rotates() => self.safe_portrait,
            _ => {
                let mut s = self.safe_landscape;
                if self.platform == Platform::Ios {
                    s.bottom = 0.0;
                }
                s
            }
        }
    }

    /// Studio's TV (Android TV 1080p): a touch device with a gamepad.
    fn is_tv(&self) -> bool {
        self.rotates() && self.tier.is_none() && !self.custom
    }

    /// The device's own input, with the Controller Emulator closed
    /// (measured in Studio's Play for every preset).
    pub fn flags(&self) -> InputFlags {
        let f = |touch, keyboard, mouse, gamepad, ten_foot, preferred| InputFlags {
            touch,
            keyboard,
            mouse,
            gamepad,
            ten_foot,
            preferred,
        };
        match self.form {
            Form::Phone | Form::Tablet => f(true, false, false, self.is_tv(), false, Preferred::Touch),
            Form::Desktop => InputFlags::DESKTOP,
            Form::Console if matches!(self.platform, Platform::XboxOne | Platform::Ps4 | Platform::Ps5) => {
                f(false, false, false, true, true, Preferred::Gamepad)
            }
            // The generic handhelds: everything, preferring the gamepad.
            Form::Console => f(true, true, true, true, false, Preferred::Gamepad),
            Form::Headset => f(false, false, true, true, false, Preferred::Gamepad),
        }
    }

    /// Gamepad1 is connected while this device is simulated (consoles,
    /// handhelds, headsets and the TV; measured).
    pub fn native_gamepad(&self) -> bool {
        matches!(self.form, Form::Console | Form::Headset) || self.is_tv()
    }
}

/// The screen size scripts see at `k_px` window pixels per device point:
/// the scaled screen is floored to whole pixels, then divided back and
/// floored (Studio: iPhone 14 844x390 shows as 843x389 when fitted).
pub fn logical_size(points: [u32; 2], k_px: f64) -> [f32; 2] {
    let axis = |p: u32| {
        if k_px <= 0.0 {
            return p as f32;
        }
        // (1e-4: the scale comes from f32 window sizes; 768 points at
        // 718/768 must give 718 pixels, not 717)
        let px = (p as f64 * k_px + 1e-4).floor();
        (px / k_px + 1e-4).floor() as f32
    };
    [axis(points[0]), axis(points[1])]
}

/// `GuiService.ViewportDisplaySize` for a device: fitted to the 43 measured
/// presets (it agrees with all of them); for custom devices it is an
/// estimate. Consoles are Large, headsets Medium; others by the physical
/// diagonal: 40 in and up Large (the 50 in TV), under 10.35 in Small.
pub fn display_size_for(form: Form, platform: Platform, size: [u32; 2], resolution_scale: f32, ppi: f32) -> DisplaySize {
    if matches!(platform, Platform::XboxOne | Platform::Ps4 | Platform::Ps5) {
        return DisplaySize::Large;
    }
    if form == Form::Headset {
        return DisplaySize::Medium;
    }
    let inches = (size[0] as f32).hypot(size[1] as f32) * resolution_scale / ppi.max(1.0);
    if inches >= 40.0 {
        DisplaySize::Large
    } else if inches < 10.35 {
        DisplaySize::Small
    } else {
        DisplaySize::Medium
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_are_studios_dropdown_list() {
        assert_eq!(presets().len(), 43);
        assert!(preset("default").is_none() && preset("average_laptop").is_none());
        let mut ids: Vec<_> = presets().iter().map(|d| &d.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 43);
    }

    #[test]
    fn iphone_14_row_matches_studio() {
        let d = preset("iphone_14").unwrap();
        assert_eq!((d.size, d.ppi, d.resolution_scale, d.memory_mb), ([844, 390], 460.0, 3.0, 6144));
        assert_eq!((d.form, d.platform, d.tier), (Form::Phone, Platform::Ios, Some(Tier::MidRange)));
        assert_eq!(d.safe_portrait, Insets::new(47.0, 34.0, 0.0, 0.0));
        assert_eq!(d.safe_landscape, Insets::new(0.0, 21.0, 47.0, 47.0));
        assert_eq!(d.keyboard_height, [295, 176]);
        assert_eq!(d.name, "iPhone 14");
        assert_eq!(preset("generic_handheld_720").unwrap().name, "Generic Handheld HD 720");
    }

    #[test]
    fn android_tv_keeps_its_tv_safe_zone() {
        let d = preset("android_tv_1080").unwrap();
        assert_eq!((d.form, d.platform, d.tier), (Form::Tablet, Platform::Android, None));
        assert_eq!(d.safe_landscape, Insets::new(54.0, 54.0, 96.0, 96.0));
    }

    #[test]
    fn ios_landscape_ignores_the_bottom_inset() {
        let o = Orientation::Landscape;
        assert_eq!(preset("iphone_14").unwrap().safe_insets(o), Insets::new(0.0, 0.0, 47.0, 47.0));
        assert_eq!(preset("samsung_galaxy_s22_ultra").unwrap().safe_insets(o), Insets::new(0.0, 21.0, 47.0, 47.0));
        assert_eq!(preset("iphone_14").unwrap().safe_insets(Orientation::Portrait), Insets::new(47.0, 34.0, 0.0, 0.0));
        // A console doesn't turn: portrait keeps the landscape screen.
        assert_eq!(preset("xbox").unwrap().points(Orientation::Portrait), [1920, 1080]);
    }

    #[test]
    fn logical_size_floors_like_studio() {
        assert_eq!(logical_size([844, 390], 1.5315), [843.0, 389.0]); // measured iPhone 14: 843 x 389
        assert_eq!(logical_size([1920, 1080], 718.0 / 1080.0), [1919.0, 1080.0]); // measured Xbox: 1919 x 1080
        assert_eq!(logical_size([844, 390], 1.0), [844.0, 390.0]);
        // measured desktops filling a 718 px panel (the scale as f32 math gives it)
        let k = |h: f32| f64::from(718.0f32 / h);
        assert_eq!(logical_size([1366, 768], k(768.0)), [1365.0, 768.0]); // Average Windows
        assert_eq!(logical_size([1280, 720], k(720.0)), [1279.0, 720.0]); // HD 720
        assert_eq!(logical_size([1920, 1080], k(1080.0)), [1919.0, 1080.0]); // HD 1080
    }

    #[test]
    fn flags_follow_the_device() {
        // measured in Studio's Play, the Controller Emulator closed
        let f = |id| preset(id).unwrap().flags();
        let fl = |touch, keyboard, mouse, gamepad, ten_foot, preferred| InputFlags { touch, keyboard, mouse, gamepad, ten_foot, preferred };
        use Preferred::*;
        assert_eq!(f("iphone_14"), fl(true, false, false, false, false, Touch));
        assert_eq!(f("ipad_10th_generation"), fl(true, false, false, false, false, Touch));
        assert_eq!(f("xbox"), fl(false, false, false, true, true, Gamepad));
        assert_eq!(f("ps5"), fl(false, false, false, true, true, Gamepad));
        assert_eq!(f("android_tv_1080"), fl(true, false, false, true, false, Touch)); // a TV, but not ten-foot
        assert_eq!(f("meta_quest_3"), fl(false, false, true, true, false, Gamepad));
        assert_eq!(f("generic_handheld_720"), fl(true, true, true, true, false, Gamepad));
        assert_eq!(f("hd_1080"), InputFlags::DESKTOP);
        let pads: Vec<_> = presets().iter().filter(|d| d.native_gamepad()).map(|d| d.id.as_str()).collect();
        assert_eq!(pads, ["xbox", "ps4", "ps5", "android_tv_1080", "meta_quest_2", "meta_quest_3", "generic_handheld_720", "generic_handheld_1080"]);
    }

    #[test]
    fn quality_and_display_size_match_studio() {
        // measured in Studio
        let q = |id| {
            let d = preset(id).unwrap();
            (d.quality, d.display_size)
        };
        use DisplaySize::*;
        assert_eq!(q("iphone_14"), (21, Small));
        assert_eq!(q("android_tv_1080"), (1, Large));
        assert_eq!(q("xiaomi_redmi_pad_se"), (2, Medium));
        assert_eq!(q("ipad_8th_generation"), (21, Small));
        assert_eq!(q("samsung_galaxy_tab_a9"), (14, Small));
        assert_eq!(q("hd_1080"), (17, Medium));
        assert_eq!(q("vga"), (17, Small));
        assert_eq!(q("meta_quest_2"), (14, Medium));
        assert_eq!(q("xbox"), (12, Large));
        // the fitted rule agrees with every measured row (it serves custom devices)
        for d in presets() {
            assert_eq!(display_size_for(d.form, d.platform, d.size, d.resolution_scale, d.ppi), d.display_size, "{}", d.id);
        }
    }
}

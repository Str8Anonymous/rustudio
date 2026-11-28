//! The Device Simulator's editor state: which device is shown, its
//! orientation and display scaling, kept in Rustudio's settings for runs a
//! person drives (Studio keeps them across sessions too).

use egui::Rect;
use rustudio_core::devices::{self, FormButton, Insets, Orientation};
use rustudio_player::device_view::{DeviceView, Scaling};
use serde_json::{json, Value};

const SETTINGS_KEY: &str = "device_simulator";

#[derive(Clone, Debug, PartialEq)]
pub struct DeviceSim {
    /// Test > Device Simulator is on.
    pub shown: bool,
    /// The simulated device; None = Current device (the window itself).
    pub device: Option<String>,
    /// Kept across devices (Studio: a phone left in portrait makes the next
    /// phone start in portrait).
    pub orientation: Orientation,
    pub scaling: Scaling,
    /// Calibrated screen density (Physical size).
    pub dpi: f32,
    /// The last device picked for each form button (Mobile, Desktop,
    /// Console, Headset).
    pub last_per_form: [Option<String>; 4],
    /// The Controller Emulator is on.
    pub controller: bool,
}

impl Default for DeviceSim {
    fn default() -> Self {
        Self {
            shown: false,
            device: None,
            orientation: Orientation::Landscape,
            scaling: Scaling::FitToWindow,
            dpi: 96.0,
            last_per_form: Default::default(),
            controller: false,
        }
    }
}

fn form_index(b: FormButton) -> usize {
    match b {
        FormButton::Mobile => 0,
        FormButton::Desktop => 1,
        FormButton::Console => 2,
        FormButton::Headset => 3,
    }
}

impl DeviceSim {
    /// The saved state (only in runs a person drives: rustudio_settings
    /// loads nothing otherwise).
    pub fn load() -> Self {
        let mut s = Self::default();
        let Some(v) = rustudio_core::rustudio_settings::saved_value(SETTINGS_KEY) else {
            return s;
        };
        s.shown = v["shown"].as_bool().unwrap_or(false);
        s.device = v["device"].as_str().filter(|id| devices::preset(id).is_some()).map(str::to_string);
        s.orientation = if v["portrait"].as_bool() == Some(true) { Orientation::Portrait } else { Orientation::Landscape };
        s.scaling = match v["scaling"].as_str() {
            Some("actual") => Scaling::ActualResolution,
            Some("physical") => Scaling::PhysicalSize,
            _ => Scaling::FitToWindow,
        };
        s.dpi = v["dpi"].as_f64().map_or(96.0, |d| d as f32);
        if let Some(list) = v["last_per_form"].as_array() {
            for (slot, id) in s.last_per_form.iter_mut().zip(list) {
                *slot = id.as_str().filter(|id| devices::preset(id).is_some()).map(str::to_string);
            }
        }
        s
    }

    pub fn save(&self) {
        let scaling = match self.scaling {
            Scaling::FitToWindow => "fit",
            Scaling::ActualResolution => "actual",
            Scaling::PhysicalSize => "physical",
        };
        let v: Value = json!({
            "shown": self.shown,
            "device": self.device,
            "portrait": self.orientation == Orientation::Portrait,
            "scaling": scaling,
            "dpi": self.dpi,
            "last_per_form": self.last_per_form,
        });
        rustudio_core::rustudio_settings::set(SETTINGS_KEY, Some(v));
    }

    /// Picks a device (None = Current device). The orientation stays.
    pub fn select(&mut self, id: Option<&str>) {
        let id = id.filter(|id| devices::preset(id).is_some());
        if let Some(d) = id.and_then(devices::preset) {
            self.last_per_form[form_index(d.button())] = Some(d.id.clone());
        }
        self.device = id.map(str::to_string);
    }

    pub fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Landscape => Orientation::Portrait,
            Orientation::Portrait => Orientation::Landscape,
        };
    }

    /// Where the device sits in the 3D panel; None while the simulator is
    /// off or shows Current device.
    pub fn view(&self, panel: Rect, ppp: f32) -> Option<DeviceView> {
        if !self.shown {
            return None;
        }
        let d = devices::preset(self.device.as_deref()?)?;
        Some(DeviceView::layout(panel, d, self.orientation, self.scaling, Insets::default(), self.dpi, ppp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{pos2, vec2};

    #[test]
    fn device_sim_keeps_orientation_across_phones() {
        let panel = Rect::from_min_size(pos2(0.0, 0.0), vec2(1914.0, 718.0));
        let mut s = DeviceSim { shown: true, ..Default::default() };
        s.select(Some("iphone_14"));
        s.rotate();
        s.select(Some("iphone_16"));
        assert_eq!(s.orientation, Orientation::Portrait);
        let v = s.view(panel, 1.0).unwrap();
        assert!(v.logical[0] < v.logical[1]);
        // a console doesn't turn
        s.select(Some("xbox"));
        let v = s.view(panel, 1.0).unwrap();
        assert!(v.logical[0] > v.logical[1]);
        s.select(Some("iphone_14"));
        assert!(s.view(panel, 1.0).unwrap().input().portrait);
        // Current device and the simulator off show the window itself
        s.select(None);
        assert!(s.view(panel, 1.0).is_none());
        s.select(Some("iphone_14"));
        s.shown = false;
        assert!(s.view(panel, 1.0).is_none());
        assert_eq!(s.last_per_form[0].as_deref(), Some("iphone_14"));
        assert_eq!(s.last_per_form[2].as_deref(), Some("xbox"));
    }

    #[test]
    fn unknown_devices_are_ignored() {
        let mut s = DeviceSim::default();
        s.select(Some("no_such_phone"));
        assert_eq!(s.device, None);
    }
}

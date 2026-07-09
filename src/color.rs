/// Pure color math — no dependencies, fully unit-tested.
/// RGB values are 0–255 u8. HSL/HSV use f32 with H in 0–360, S/L/V in 0–1.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    /// 0–360
    pub h: f32,
    /// 0–1
    pub s: f32,
    /// 0–1
    pub l: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    /// 0–360
    pub h: f32,
    /// 0–1
    pub s: f32,
    /// 0–1
    pub v: f32,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn to_rgb_string(self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn to_hsl(self) -> Hsl {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let l = (max + min) / 2.0;

        let s = if delta < 1e-6 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if delta < 1e-6 {
            0.0
        } else if (max - r).abs() < 1e-6 {
            60.0 * (((g - b) / delta) % 6.0)
        } else if (max - g).abs() < 1e-6 {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        Hsl { h, s, l }
    }

    pub fn to_hsv(self) -> Hsv {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let v = max;

        let s = if max < 1e-6 { 0.0 } else { delta / max };

        let h = if delta < 1e-6 {
            0.0
        } else if (max - r).abs() < 1e-6 {
            60.0 * (((g - b) / delta) % 6.0)
        } else if (max - g).abs() < 1e-6 {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        Hsv { h, s, v }
    }

    /// egui Color32
    pub fn to_egui_color(self) -> egui::Color32 {
        egui::Color32::from_rgb(self.r, self.g, self.b)
    }
}

impl Hsl {
    pub fn to_hsl_string(self) -> String {
        format!(
            "hsl({}, {}%, {}%)",
            self.h.round() as u32,
            (self.s * 100.0).round() as u32,
            (self.l * 100.0).round() as u32,
        )
    }
}

impl Hsv {
    pub fn to_hsv_string(self) -> String {
        format!(
            "hsv({}, {}%, {}%)",
            self.h.round() as u32,
            (self.s * 100.0).round() as u32,
            (self.v * 100.0).round() as u32,
        )
    }
}

/// A fully-formatted color with all representations pre-computed.
#[derive(Debug, Clone, PartialEq)]
pub struct PickedColor {
    pub rgb: Rgb,
    pub hex: String,
    pub rgb_string: String,
    pub hsl: Hsl,
    pub hsl_string: String,
    pub hsv: Hsv,
    pub hsv_string: String,
}

impl PickedColor {
    pub fn from_rgb(rgb: Rgb) -> Self {
        let hsl = rgb.to_hsl();
        let hsv = rgb.to_hsv();
        Self {
            hex: rgb.to_hex(),
            rgb_string: rgb.to_rgb_string(),
            hsl_string: hsl.to_hsl_string(),
            hsv_string: hsv.to_hsv_string(),
            rgb,
            hsl,
            hsv,
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.6
    }

    #[test]
    fn black() {
        let c = Rgb::new(0, 0, 0);
        assert_eq!(c.to_hex(), "#000000");
        let hsl = c.to_hsl();
        assert!(approx_eq(hsl.h, 0.0));
        assert!(approx_eq(hsl.s, 0.0));
        assert!(approx_eq(hsl.l, 0.0));
    }

    #[test]
    fn white() {
        let c = Rgb::new(255, 255, 255);
        assert_eq!(c.to_hex(), "#FFFFFF");
        let hsl = c.to_hsl();
        assert!(approx_eq(hsl.s, 0.0));
        assert!(approx_eq(hsl.l * 100.0, 100.0));
    }

    #[test]
    fn pure_red() {
        let c = Rgb::new(255, 0, 0);
        assert_eq!(c.to_hex(), "#FF0000");
        let hsl = c.to_hsl();
        assert!(approx_eq(hsl.h, 0.0));
        assert!(approx_eq(hsl.s * 100.0, 100.0));
        assert!(approx_eq(hsl.l * 100.0, 50.0));
    }

    #[test]
    fn pure_green() {
        let c = Rgb::new(0, 255, 0);
        assert_eq!(c.to_hex(), "#00FF00");
        let hsl = c.to_hsl();
        assert!(approx_eq(hsl.h, 120.0));
        assert!(approx_eq(hsl.s * 100.0, 100.0));
        assert!(approx_eq(hsl.l * 100.0, 50.0));
    }

    #[test]
    fn pure_blue() {
        let c = Rgb::new(0, 0, 255);
        assert_eq!(c.to_hex(), "#0000FF");
        let hsl = c.to_hsl();
        assert!(approx_eq(hsl.h, 240.0));
    }

    #[test]
    fn grey_eaeaea() {
        let c = Rgb::new(0xEA, 0xEA, 0xEA);
        assert_eq!(c.to_hex(), "#EAEAEA");
        let hsl = c.to_hsl();
        assert!(approx_eq(hsl.s, 0.0));
        assert!(approx_eq(hsl.l * 100.0, 91.8));
    }

    #[test]
    fn bgr_decode() {
        // Windows COLORREF for RGB(234,234,234) = 0x00EAEAEA
        let c = Rgb::from_u32_bgr(0x00EAEAEA);
        assert_eq!(c.r, 0xEA);
        assert_eq!(c.g, 0xEA);
        assert_eq!(c.b, 0xEA);
    }

    #[test]
    fn rgb_string() {
        let c = Rgb::new(10, 20, 30);
        assert_eq!(c.to_rgb_string(), "rgb(10, 20, 30)");
    }

    #[test]
    fn hsl_string() {
        let c = Rgb::new(255, 0, 0);
        assert_eq!(c.to_hsl().to_hsl_string(), "hsl(0, 100%, 50%)");
    }
}

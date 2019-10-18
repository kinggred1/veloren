pub const GIT_VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/githash"));

lazy_static::lazy_static! {
    pub static ref GIT_HASH: &'static str = include_str!(concat!(env!("OUT_DIR"), "/githash")).split(" ").nth(0).unwrap();
    pub static ref GIT_DATE: &'static str = include_str!(concat!(env!("OUT_DIR"), "/githash")).split(" ").nth(1).unwrap();
}

impl std::fmt::Display for GIT_HASH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Display for GIT_DATE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

use vek::{Mat3, Rgb, Rgba, Vec3};

/// This is a fast approximation of powf. This should only be used when minor accuracy loss is acceptable.
#[inline(always)]
#[allow(unsafe_code)]
fn approx_powf(b: f32, e: f32) -> f32 {
    unsafe {
        let b = b as f64;
        let e = e as f64;
        union Swagger {
            f: f64,
            a: [i32; 2],
        }
        let mut b = Swagger { f: b };
        b.a[1] = (e * (b.a[1] as f64 - 1072632447.0) + 1072632447.0) as i32;
        b.a[0] = 0;
        b.f as f32
    }
}

#[cfg(test)]
mod approx_powf_tests {
    fn close_ei(a: f32, b: f32) -> bool {
        (a - b < 1.0 && a - b > 0.0) || (b - a < 1.0 && b - a > 0.0)
    }

    #[test]
    fn accuracy_1() {
        let test_values: Vec<f32> = vec![3.0, 2.5, 1.5, 2.2];
        test_values.windows(2).for_each(|a| {
            assert!(close_ei(a[0].powf(a[1]), super::approx_powf(a[0], a[1])));
        });
    }
}

#[inline(always)]
pub fn srgb_to_linear(col: Rgb<f32>) -> Rgb<f32> {
    #[inline(always)]
    fn to_linear(x: f32) -> f32 {
        if x <= 0.04045 {
            x / 12.92
        } else {
            approx_powf((x + 0.055) / 1.055, 2.4)
        }
    }
    col.map(to_linear)
}
#[inline(always)]
pub fn linear_to_srgb(col: Rgb<f32>) -> Rgb<f32> {
    #[inline(always)]
    fn to_srgb(x: f32) -> f32 {
        if x <= 0.0031308 {
            x * 12.92
        } else {
            approx_powf(x, 1.0 / 2.4) * 1.055 - 0.055
        }
    }
    col.map(to_srgb)
}
#[inline(always)]
pub fn srgba_to_linear(col: Rgba<f32>) -> Rgba<f32> {
    Rgba::from_translucent(srgb_to_linear(Rgb::from(col)), col.a)
}
#[inline(always)]
pub fn linear_to_srgba(col: Rgba<f32>) -> Rgba<f32> {
    Rgba::from_translucent(linear_to_srgb(Rgb::from(col)), col.a)
}

/// Convert rgb to hsv. Expects rgb to be [0, 1].
#[inline(always)]
pub fn rgb_to_hsv(rgb: Rgb<f32>) -> Vec3<f32> {
    let (r, g, b) = rgb.into_tuple();
    let (max, min, diff, add) = {
        let (max, min, diff, add) = if r > g {
            (r, g, g - b, 0.0)
        } else {
            (g, r, b - r, 2.0)
        };
        if b > max {
            (b, min, r - g, 4.0)
        } else {
            (max, b.min(min), diff, add)
        }
    };

    let v = max;
    let h = if max == min {
        0.0
    } else {
        let mut h = 60.0 * (add + diff / (max - min));
        if h < 0.0 {
            h += 360.0;
        }
        h
    };
    let s = if max == 0.0 { 0.0 } else { (max - min) / max };

    Vec3::new(h, s, v)
}
/// Convert hsv to rgb. Expects h [0, 360], s [0, 1], v [0, 1]
#[inline(always)]
pub fn hsv_to_rgb(hsv: Vec3<f32>) -> Rgb<f32> {
    let (h, s, v) = hsv.into_tuple();
    let c = s * v;
    let h = h / 60.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h >= 0.0 && h <= 1.0 {
        (c, x, 0.0)
    } else if h <= 2.0 {
        (x, c, 0.0)
    } else if h <= 3.0 {
        (0.0, c, x)
    } else if h <= 4.0 {
        (0.0, x, c)
    } else if h <= 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Rgb::new(r + m, g + m, b + m)
}
/// Convert linear rgb to CIExyY
#[inline(always)]
pub fn rgb_to_xyy(rgb: Rgb<f32>) -> Vec3<f32> {
    // XYZ
    let xyz = Mat3::new(
        0.4124, 0.3576, 0.1805, 0.2126, 0.7152, 0.0722, 0.0193, 0.1192, 0.9504,
    ) * Vec3::from(rgb);

    let sum = xyz.sum();
    Vec3::new(xyz.x / sum, xyz.y / sum, xyz.y)
}
/// Convert to CIExyY to linear rgb
#[inline(always)]
pub fn xyy_to_rgb(xyy: Vec3<f32>) -> Rgb<f32> {
    let xyz = Vec3::new(
        xyy.z / xyy.y * xyy.x,
        xyy.z,
        xyy.z / xyy.y * (1.0 - xyy.x - xyy.y),
    );

    Rgb::from(
        Mat3::new(
            3.2406, -1.5372, -0.4986, -0.9689, 1.8758, 0.0415, 0.0557, -0.2040, 1.0570,
        ) * xyz,
    )
}

// TO-DO: speed this up
#[inline(always)]
pub fn saturate_srgb(col: Rgb<f32>, value: f32) -> Rgb<f32> {
    let mut hsv = rgb_to_hsv(srgb_to_linear(col));
    hsv.y *= 1.0 + value;
    linear_to_srgb(hsv_to_rgb(hsv).map(|e| e.min(1.0).max(0.0)))
}

/// Preserves the luma of one color while changing its chromaticty to match the other
#[inline(always)]
pub fn chromify_srgb(luma: Rgb<f32>, chroma: Rgb<f32>) -> Rgb<f32> {
    let l = rgb_to_xyy(srgb_to_linear(luma)).z;
    let mut xyy = rgb_to_xyy(srgb_to_linear(chroma));
    xyy.z = l;

    linear_to_srgb(xyy_to_rgb(xyy).map(|e| e.min(1.0).max(0.0)))
}

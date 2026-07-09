/// Windows screen pixel capture via GDI.
/// Uses BitBlt for loupe region capture and GetPixel for single-pixel reads.

use crate::color::Rgb;
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits,
    GetDC, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    SRCCOPY,
};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;


/// Get the current cursor position in screen coordinates.
pub fn cursor_pos() -> (i32, i32) {
    unsafe {
        let mut pt = POINT::default();
        let _ = GetCursorPos(&mut pt);
        (pt.x, pt.y)
    }
}

/// Capture a (2*half+1) × (2*half+1) region centered on (cx, cy).
/// Returns pixels row-major, top-left first, as Vec<Rgb>.
pub fn capture_region(cx: i32, cy: i32, half: i32) -> LoupeCapture {
    let size = (2 * half + 1) as u32;
    let x0 = cx - half;
    let y0 = cy - half;

    unsafe {
        // Get screen DC
        let hdc_screen = GetDC(None);

        // Create compatible memory DC + bitmap
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbitmap = CreateCompatibleBitmap(hdc_screen, size as i32, size as i32);
        let old_bmp = SelectObject(hdc_mem, hbitmap);

        // Blit region from screen into memory DC
        let _ = BitBlt(hdc_mem, 0, 0, size as i32, size as i32, hdc_screen, x0, y0, SRCCOPY);

        // Read back pixels via GetDIBits
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: size as i32,
                biHeight: -(size as i32), // negative = top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let pixel_count = (size * size) as usize;
        let mut buf: Vec<u32> = vec![0u32; pixel_count];

        let _ = GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            size,
            Some(buf.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        // Clean up GDI resources
        SelectObject(hdc_mem, old_bmp);
        let _ = DeleteObject(hbitmap);
        let _ = DeleteDC(hdc_mem);
        ReleaseDC(None, hdc_screen);

        // Convert BGRA u32 → Rgb (DIB 32-bit is BGRA, B in lowest byte)
        let pixels: Vec<Rgb> = buf
            .into_iter()
            .map(|bgra| Rgb {
                r: ((bgra >> 16) & 0xFF) as u8,
                g: ((bgra >> 8) & 0xFF) as u8,
                b: (bgra & 0xFF) as u8,
            })
            .collect();

        LoupeCapture {
            pixels,
            size: size as usize,
        }
    }
}

pub struct LoupeCapture {
    /// Row-major, top-left first.
    pub pixels: Vec<Rgb>,
    /// Width = height = size.
    pub size: usize,
}

impl LoupeCapture {
    /// Get pixel at (col, row) within the capture, 0-indexed.
    pub fn get(&self, col: usize, row: usize) -> Rgb {
        self.pixels.get(row * self.size + col).copied().unwrap_or(Rgb::new(0, 0, 0))
    }

    /// The center pixel (the actual picked color candidate).
    pub fn center(&self) -> Rgb {
        let mid = self.size / 2;
        self.get(mid, mid)
    }
}

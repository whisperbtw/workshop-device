use crate::device;
use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::{
    Foundation::{HANDLE, HWND, LPARAM, LRESULT, RECT, WPARAM},
    Graphics::Dwm::{
        DWMNCRP_DISABLED, DWMWA_BORDER_COLOR, DWMWA_NCRENDERING_POLICY, DwmSetWindowAttribute,
    },
    Graphics::Gdi::SetWindowRgn,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};
use windows::core::w;

const ORIGINAL_PROC: windows::core::PCWSTR = w!("PZWorkshopDownloader.OriginalProc");

unsafe extern "system" fn device_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_NCCALCSIZE {
        return LRESULT(0);
    }
    if message == WM_NCHITTEST {
        // Transparent shadow space must not act as an invisible rectangular frame.
        let mut rect = RECT::default();
        if unsafe { GetWindowRect(hwnd, &mut rect) }.is_ok() {
            let scale = (rect.right - rect.left) as f32 / device::WINDOW_WIDTH;
            let x = ((lparam.0 & 0xffff) as i16 as i32 - rect.left) as f32 / scale;
            let y = (((lparam.0 >> 16) & 0xffff) as i16 as i32 - rect.top) as f32 / scale;
            if !device::contains(x, y) {
                return LRESULT(HTTRANSPARENT as isize);
            }
        }
    }
    unsafe {
        let original = GetPropW(hwnd, ORIGINAL_PROC).0 as isize;
        if original == 0 {
            return DefWindowProcW(hwnd, message, wparam, lparam);
        }
        if message == WM_NCDESTROY {
            let _ = RemovePropW(hwnd, ORIGINAL_PROC);
        }
        CallWindowProcW(
            std::mem::transmute::<isize, WNDPROC>(original),
            hwnd,
            message,
            wparam,
            lparam,
        )
    }
}

fn hwnd(window: &Window) -> Option<HWND> {
    match HasWindowHandle::window_handle(window).ok()?.as_raw() {
        RawWindowHandle::Win32(handle) => Some(HWND(handle.hwnd.get() as *mut _)),
        _ => None,
    }
}

/// Configures only this application's window, never another process's HWND.
pub fn configure(window: &Window) {
    let Some(hwnd) = hwnd(window) else { return };
    let address = hwnd.0 as usize;
    let scale = window.scale_factor();
    // Match the device dimensions, excluding the initial native frame.
    let width = (device::WINDOW_WIDTH * scale).round() as i32;
    let height = (device::WINDOW_HEIGHT * scale).round() as i32;
    // Win32 messages must run after GPUI releases the current App borrow.
    // A worker thread marshals window changes back to the native UI thread.
    std::thread::spawn(move || configure_handle(HWND(address as *mut _), width, height));
}

fn configure_handle(hwnd: HWND, width: i32, height: i32) {
    unsafe {
        eprintln!("Window: configuring native device");
        if !IsWindow(Some(hwnd)).as_bool() {
            return;
        }
        // Resource 1 is the multi-resolution application icon embedded by build.rs.
        if let Ok(module) = GetModuleHandleW(None)
            && let Ok(icon) = LoadIconW(
                Some(module.into()),
                windows::core::PCWSTR(std::ptr::without_provenance(1)),
            )
        {
            SendMessageW(
                hwnd,
                WM_SETICON,
                Some(WPARAM(ICON_BIG as usize)),
                Some(LPARAM(icon.0 as isize)),
            );
            SendMessageW(
                hwnd,
                WM_SETICON,
                Some(WPARAM(ICON_SMALL as usize)),
                Some(LPARAM(icon.0 as isize)),
            );
        }
        if GetPropW(hwnd, ORIGINAL_PROC).0.is_null() {
            let original = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
            if SetPropW(hwnd, ORIGINAL_PROC, Some(HANDLE(original as *mut _))).is_err() {
                return;
            }
            SetWindowLongPtrW(hwnd, GWLP_WNDPROC, device_proc as *const () as isize);
        }
        let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
        let border = WS_CAPTION.0 | WS_THICKFRAME.0 | WS_BORDER.0 | WS_DLGFRAME.0;
        SetWindowLongW(hwnd, GWL_STYLE, (style & !border) as i32);
        let policy = DWMNCRP_DISABLED;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_NCRENDERING_POLICY,
            &policy as *const _ as _,
            std::mem::size_of_val(&policy) as u32,
        );
        let no_border: u32 = 0xfffffffe;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_BORDER_COLOR,
            &no_border as *const _ as _,
            std::mem::size_of_val(&no_border) as u32,
        );
        let _ = SetWindowPos(
            hwnd,
            None,
            0,
            0,
            width,
            height,
            SWP_NOMOVE | SWP_NOZORDER | SWP_FRAMECHANGED,
        );
        // Keep the outline in the alpha surface: GDI regions have binary edges.
        SetWindowRgn(hwnd, None, true);
        let _ = ShowWindowAsync(hwnd, SW_SHOW);
        let mut client = RECT::default();
        let _ = GetClientRect(hwnd, &mut client);
        eprintln!(
            "Window: alpha outline, client={}x{}",
            client.right, client.bottom
        );
    }
}

// #![no_std]
#![no_main]
#![allow(unused_variables, non_camel_case_types, non_snake_case)]

extern crate alloc;
use alloc::vec::Vec;
// use core::ffi::c_size_t;
use core::ffi::*;
// use std::{os::windows::raw::HANDLE, ffi::*};

// pub type APIENTRY = WINAPI;
pub type ATOM = WORD;
pub type BOOL = c_int;
pub type BOOLEAN = BYTE;
pub type BYTE = c_uchar;
// pub type CALLBACK
pub type CCHAR = c_char;
pub type CHAR = c_char;
pub type COLORREF = DWORD;
// pub type CONST
pub type DWORD = c_ulong;
pub type DWORDLONG = c_ulonglong;
pub type DWORD_PTR = ULONG_PTR;
pub type DWORD32 = c_uint;
pub type DWORD64 = c_ulonglong;
pub type FLOAT = c_float;
pub type HACCEL = HANDLE;
pub type HALF_PTR = c_int;
/* todo!
#ifdef _WIN64
 typedef int HALF_PTR;
#else
 typedef short HALF_PTR;
#endif */
pub type HANDLE = PVOID;
pub type HBITMAP = HANDLE;
pub type HBRUSH = HANDLE;
pub type HCOLORSPACE = HANDLE;
pub type HCONV = HANDLE;
pub type HCONVLIST = HANDLE;
pub type HCURSOR = HICON;
pub type HDC = HANDLE;
pub type HDDEDATA = HANDLE;
pub type HDESK = HANDLE;
pub type HDROP = HANDLE;
pub type HDWP = HANDLE;
pub type HENHMETAFILE = HANDLE;
pub type HFILE = c_int;
pub type HFONT = HANDLE;
pub type HGDIOBJ = HANDLE;
pub type HGLOBAL = HANDLE;
pub type HHOOK = HANDLE;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HKEY = HANDLE;
pub type HKL = HANDLE;
pub type HLOCAL = HANDLE;
pub type HMENU = HANDLE;
pub type HMETAFILE = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HMONITOR = HANDLE; // todo! if (WINVER >= 0x0500)
pub type HPALETTE = HANDLE;
pub type HPEN = HANDLE;
pub type HRESULT = LONG;
pub type HRGN = HANDLE;
pub type HRSRC = HANDLE;
pub type HSZ = HANDLE;
pub type HWINSTA = HANDLE;
pub type HWND = HANDLE;
pub type INT = c_int;
pub type INT_PTR = c_int;
/* todo!
#if defined(_WIN64)
 typedef __int64 INT_PTR;
#else
 typedef int INT_PTR;
#endif */
pub type INT8 = c_char;
pub type INT16 = c_short;
pub type INT32 = c_int;
pub type INT64 = c_long;
pub type LANGID = WORD;
pub type LCID = DWORD;
pub type LCTYPE = DWORD;
pub type LGRPID = DWORD;
pub type LONG = c_long;
pub type LONGLONG = c_double;
/* todo!
#if !defined(_M_IX86)
 typedef __int64 LONGLONG;
#else
 typedef double LONGLONG;
#endif */
pub type LONG_PTR = c_long;
/* todo!
#if defined(_WIN64)
 typedef __int64 LONG_PTR;
#else
 typedef long LONG_PTR;
#endif */
pub type LONG32 = c_int;
pub type LONG64 = c_longlong;
pub type LPARAM = LONG_PTR;
pub type LPBOOL = *mut BOOL;
pub type LPBYTE = *mut BYTE;
pub type LPCOLORREF = *mut DWORD;
// pub type LPCSTR
pub type LPCTSTR = LPCWSTR;
/* todo!
#ifdef UNICODE
 typedef LPCWSTR LPCTSTR;
#else
 typedef LPCSTR LPCTSTR;
#endif */
pub type LPCVOID = *const c_void;
pub type LPCWSTR = *const u16;
pub type LPDWORD = *mut DWORD;
pub type LPHANDLE = *mut HANDLE;
pub type LPINT = *mut c_int;
pub type LPLONG = *mut c_long;
pub type LPSTR = *mut CHAR;
// pub type LPTSTR = LPWSTR;
pub type LPVOID = *mut c_void;
pub type LPWORD = *mut WORD;
pub type LPWSTR = *mut u16;
pub type LRESULT = isize;

//
//
//
//

pub type PVOID = *mut c_void;
pub type ULONG_PTR = c_ulong;
/* todo!
#if defined(_WIN64)
 typedef unsigned __int64 ULONG_PTR;
#else
 typedef unsigned long ULONG_PTR;
#endif */
pub type UINT = c_uint;
pub type WPARAM = usize;
pub type LPMSG = *mut MSG;
pub type PBYTE = *mut BYTE;
pub type LPPOINT = *mut POINT;
pub type RGBQUAD = tagRGBQUAD;
pub type WORD = c_ushort;
pub type SIZE_T = usize; // c_size_t;

pub type CURSORINFO = tagCURSORINFO;
pub type PCURSORINFO = *mut CURSORINFO;
pub type LPCURSORINFO = *mut CURSORINFO;

// other types
pub type PIXELFORMATDESCRIPTOR = tagPIXELFORMATDESCRIPTOR;
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;

/// Button Styles
pub const BS_PUSHBUTTON: u32 = 0x00000000;
pub const BS_FLAT: u32 = 0x00008000;

// Window Class Styles
/// Aligns the window's client area on a byte boundary (in the x direction). This style affects the width of the window and its horizontal placement on the display.
pub const CS_BYTEALIGNCLIENT: u32 = 0x1000;
pub const CS_BYTEALIGNWINDOW: u32 = 0x2000;
pub const CS_CLASSDC: u32 = 0x0040;
pub const CS_DBLCLKS: u32 = 0x0008;
pub const CS_DROPSHADOW: u32 = 0x00020000;
pub const CS_GLOBALCLASS: u32 = 0x4000;
/// Redraws the entire window if a movement or size adjustment changes the
/// width of the client area.
pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_NOCLOSE: u32 = 0x0200;
/// Allocates a unique device context for each window in the class.
pub const CS_OWNDC: u32 = 0x0020;
pub const CS_PARENTDC: u32 = 0x0080;
pub const CS_SAVEBITS: u32 = 0x0800;
/// Redraws the entire window if a movement or size adjustment changes the
/// height of the client area.
pub const CS_VREDRAW: u32 = 0x0001;

pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

// pub const IDI_APPLICATION: LPWSTR = MAKEINTRESOURCEW(32512);
// pub const IDI_WINLOGO: LPWSTR = MAKEINTRESOURCEW(32517);

pub const IMAGE_BITMAP: UINT = 0;
pub const IMAGE_CURSOR: UINT = 2;
pub const IMAGE_ICON: UINT = 1;

// Values for fuLoad parameter
pub const LR_CREATEDIBSECTION: UINT = 0x00002000;
pub const LR_DEFAULTCOLOR: UINT = 0x00000000;
pub const LR_DEFAULTSIZE: UINT = 0x00000040;
pub const LR_LOADFROMFILE: UINT = 0x00000010;
pub const LR_LOADMAP3DCOLORS: UINT = 0x00001000;
pub const LR_LOADTRANSPARENT: UINT = 0x00000020;
pub const LR_MONOCHROME: UINT = 0x00000001;
pub const LR_SHARED: UINT = 0x00008000;
pub const LR_VGACOLOR: UINT = 0x00000080;

pub const SW_HIDE: c_int = 0;
pub const SW_SHOWNORMAL: c_int = 1;
pub const SW_SHOWMINIMIZED: c_int = 2;
pub const SW_SHOWMAXIMIZED: c_int = 3;
pub const SW_SHOWNOACTIVATE: c_int = 4;
pub const SW_SHOW: c_int = 5;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_SHOWMINNOACTIVE: c_int = 7;
pub const SW_SHOWNA: c_int = 8;
pub const SW_RESTORE: c_int = 9;
pub const SW_SHOWDEFAULT: c_int = 10;
pub const SW_FORCEMINIMIZE: c_int = 11;

pub const SWP_ASYNCWINDOWPOS: UINT = 0x4000;
pub const SWP_DEFERERASE: UINT = 0x2000;
pub const SWP_DRAWFRAME: UINT = 0x0020;
pub const SWP_FRAMECHANGED: UINT = 0x0020;
pub const SWP_HIDEWINDOW: UINT = 0x0080;
pub const SWP_NOACTIVATE: UINT = 0x0010;
pub const SWP_NOCOPYBITS: UINT = 0x0100;
pub const SWP_NOMOVE: UINT = 0x0002;
pub const SWP_NOOWNERZORDER: UINT = 0x0200;
pub const SWP_NOREDRAW: UINT = 0x0008;
pub const SWP_NOREPOSITION: UINT = 0x0200;
pub const SWP_NOSENDCHANGING: UINT = 0x0400;
pub const SWP_NOSIZE: UINT = 0x0001;
pub const SWP_NOZORDER: UINT = 0x0004;
pub const SWP_SHOWWINDOW: UINT = 0x0040;

pub const MB_OKCANCEL: u32 = 1;
pub const MB_YESNO: u32 = 4;

// MessageBeep values
pub const MB_ICONASTERISK: u32 = 0x00000040;
pub const MB_ICONEXCLAMATION: u32 = 0x00000030;
pub const MB_ICONERROR: u32 = 0x00000010;
pub const MB_ICONHAND: u32 = 0x00000010;
pub const MB_ICONINFORMATION: u32 = 0x00000040;
pub const MB_ICONQUESTION: u32 = 0x00000020;
pub const MB_ICONSTOP: u32 = 0x00000010;
pub const MB_ICONWARNING: u32 = 0x00000030;
pub const MB_OK: u32 = 0x00000000;

pub const MEM_DECOMMIT: u32 = 0x00004000;
pub const MEM_RELEASE: u32 = 0x00008000;

pub const MEM_COALESCE_PLACEHOLDERS: u32 = 0x00000001;
pub const MEM_PRESERVE_PLACEHOLDER: u32 = 0x00000002;

pub const MEM_COMMIT: u32 = 0x00001000;
pub const MEM_RESERVE: u32 = 0x00002000;
pub const MEM_RESET: u32 = 0x00080000;
pub const MEM_RESET_UNDO: u32 = 0x1000000;

pub const MEM_LARGE_PAGES: u32 = 0x20000000;
pub const MEM_PHYSICAL: u32 = 0x00400000;
pub const MEM_TOP_DOWN: u32 = 0x00100000;
pub const MEM_WRITE_WATCH: u32 = 0x00200000;

pub const MF_BITMAP: u32 = 0x00000004;
pub const MF_CHECKED: u32 = 0x00000008;
pub const MF_DISABLED: u32 = 0x00000010;
pub const MF_ENABLED: u32 = 0x00000000;
pub const MF_GRAYED: u32 = 0x00000001;
pub const MF_MENUBARBREAK: u32 = 0x00000020;
pub const MF_MENUBREAK: u32 = 0x00000040;
pub const MF_OWNERDRAW: u32 = 0x00000100;
pub const MF_POPUP: u32 = 0x00000010;
pub const MF_SEPARATOR: u32 = 0x00000800;
pub const MF_STRING: u32 = 0x00000000;
pub const MF_UNCHECKED: u32 = 0x00000000;

pub const PAGE_READONLY: u32 = 0x02;
pub const PAGE_READWRITE: u32 = 0x04;

pub const IDOK: c_int = 1;
pub const IDYES: c_int = 6;

pub const COLOR_BACKGROUND: u32 = 1;
pub const COLOR_DESKTOP: u32 = 1;
pub const COLOR_WINDOW: u32 = 5;

pub const rgbRed: COLORREF = 0x000000FF;
pub const rgbGreen: COLORREF = 0x0000FF00;
pub const rgbBlue: COLORREF = 0x00FF0000;
pub const rgbBlack: COLORREF = 0x00000000;
pub const rgbWhite: COLORREF = 0x00FFFFFF;

pub const PM_NOREMOVE: u32 = 0x0000;
pub const PM_REMOVE: u32 = 0x0001;
pub const PM_NOYIELD: u32 = 0x0002;

/// [`PIXELFORMATDESCRIPTOR`] pixel type
pub const PFD_TYPE_RGBA: u8 = 0;
/// [`PIXELFORMATDESCRIPTOR`] pixel type
pub const PFD_TYPE_COLORINDEX: u8 = 1;

/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_MAIN_PLANE: u8 = 0;
/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_OVERLAY_PLANE: u8 = 1;
/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_UNDERLAY_PLANE: u8 = u8::MAX /* was (-1) */;

pub const PFD_DOUBLEBUFFER: u32 = 0x00000001;
pub const PFD_STEREO: u32 = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: u32 = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: u32 = 0x00000008;
pub const PFD_SUPPORT_GDI: u32 = 0x00000010;
pub const PFD_SUPPORT_OPENGL: u32 = 0x00000020;
pub const PFD_GENERIC_FORMAT: u32 = 0x00000040;
pub const PFD_NEED_PALETTE: u32 = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: u32 = 0x00000100;
pub const PFD_SWAP_EXCHANGE: u32 = 0x00000200;
pub const PFD_SWAP_COPY: u32 = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: u32 = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: u32 = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: u32 = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: u32 = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: u32 = 0x00008000;

/// use with [`ChoosePixelFormat`] only
pub const PFD_DEPTH_DONTCARE: u32 = 0x20000000;
/// use with [`ChoosePixelFormat`] only
pub const PFD_DOUBLEBUFFER_DONTCARE: u32 = 0x40000000;
/// use with [`ChoosePixelFormat`] only
pub const PFD_STEREO_DONTCARE: u32 = 0x80000000;

pub const PS_SOLID: i32 = 0;
pub const PS_DASH: i32 = 1;
pub const PS_DOT: i32 = 2;
pub const PS_DASHDOT: i32 = 3;
pub const PS_DASHDOTDOT: i32 = 4;
pub const PS_NULL: i32 = 5;
pub const PS_INSIDEFRAME: i32 = 6;

pub const WM_COMMAND: u32 = 0x0111;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;

// Window Styles (https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles)
/// Description:
/// The window has a thin-line border
pub const WS_BORDER: DWORD = 0x00800000;
/// Description:
/// The window has a title bar (includes the WS_BORDER style).
pub const WS_CAPTION: DWORD = 0x00C00000;
/// Description:
/// The window is a child window. A window with this style cannot have a menu bar. This style cannot be used with the WS_POPUP style.
pub const WS_CHILD: DWORD = 0x40000000;
pub const WS_CHILDWINDOW: DWORD = 0x40000000;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_DISABLED: u32 = 0x08000000;
pub const WS_DLGFRAME: u32 = 0x00400000;
pub const WS_GROUP: u32 = 0x00020000;
pub const WS_HSCROLL: u32 = 0x00100000;
pub const WS_ICONIC: u32 = 0x20000000;
pub const WS_MAXIMIZE: u32 = 0x01000000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_MINIMIZE: DWORD = 0x20000000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_POPUP: u32 = 0x80000000;
pub const WS_POPUPWINDOW: u32 = WS_POPUP | WS_BORDER | WS_SYSMENU;
pub const WS_SIZEBOX: u32 = 0x00040000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_TABSTOP: u32 = 0x00010000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_TILED: u32 = 0x00000000;
pub const WS_TILEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_VSCROLL: u32 = 0x00200000;

// Extended Window Styles (https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)
/// Description:
/// The window accepts drag-drop files.
pub const WS_EX_ACCEPTFILES: DWORD = 0x00000010;
/// Description:
/// Forces a top-level window onto the taskbar when the window is visible.
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
/// Description:
/// The window has a border with a sunken edge.
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
pub const WS_EX_COMPOSITED: DWORD = 0x02000000;
pub const WS_EX_CONTEXTHELP: DWORD = 0x00000400;
pub const WS_EX_CONTROLPARENT: DWORD = 0x00010000;
pub const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;
pub const WS_EX_LAYERED: DWORD = 0x00080000;
pub const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
pub const WS_EX_LEFT: DWORD = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;
pub const WS_EX_LTRREADING: DWORD = 0x00000000;
pub const WS_EX_MDICHILD: DWORD = 0x00000040;
pub const WS_EX_NOACTIVATE: DWORD = 0x08000000;
pub const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;
pub const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;
pub const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const WS_EX_PALETTEWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
pub const WS_EX_RIGHT: DWORD = 0x00001000;
pub const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;
pub const WS_EX_RTLREADING: DWORD = 0x00002000;
pub const WS_EX_STATICEDGE: DWORD = 0x00020000;
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub const WS_EX_TOPMOST: DWORD = 0x00000008;
pub const WS_EX_TRANSPARENT: DWORD = 0x00000020;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;

//
pub const CURSOR_SHOWING: DWORD = 0x00000001;
pub const CURSOR_SUPPRESSED: DWORD = 0x00000002;

macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

pub type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

#[repr(C)]
pub struct tagPIXELFORMATDESCRIPTOR {
    nSize: WORD,
    nVersion: WORD,
    dwFlags: DWORD,
    iPixelType: BYTE,
    cColorBits: BYTE,
    cRedBits: BYTE,
    cRefShift: BYTE,
    cGreenBits: BYTE,
    cGreenShift: BYTE,
    cBlueBits: BYTE,
    cBlueShift: BYTE,
    cAlphaBits: BYTE,
    cAlphaShift: BYTE,
    cAccumBits: BYTE,
    cAccumRedBits: BYTE,
    cAccumGreenBits: BYTE,
    cAccumBlueBits: BYTE,
    cAccumAplhaBits: BYTE,
    cDepthBits: BYTE,
    cStencilBits: BYTE,
    cAuxBuffers: BYTE,
    iLayerType: BYTE,
    bReserved: BYTE,
    dwLayerMask: DWORD,
    dwVisibleMask: DWORD,
    dwDamageMask: DWORD,
}

impl Default for PIXELFORMATDESCRIPTOR {
    #[inline]
    #[must_use]
    fn default() -> Self {
        let mut out: Self = unsafe { core::mem::zeroed() };
        out.nSize = core::mem::size_of::<Self>() as WORD;
        out.nVersion = 1;
        out
    }
}

#[repr(C)]
pub struct tagBITMAP {
    bmType: LONG,
    bmWidth: LONG,
    bmHeight: LONG,
    bmWidthBytes: LONG,
    bmPlanes: WORD,
    bmBitsPixel: WORD,
    bmBits: LPVOID,
}
unsafe_impl_default_zeroed!(tagBITMAP);

#[repr(C)]
pub struct WNDCLASSW {
    pub style: c_uint,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct POINT {
    pub x: c_long,
    pub y: c_long,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: c_uint,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

#[repr(C)]
pub struct RECT {
    pub left: c_long,
    pub top: c_long,
    pub right: c_long,
    pub bottom: c_long,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: bool,
    pub rcPaint: RECT,
    pub fRestore: bool,
    pub fIncUpdate: bool,
    pub rgbReserved: [u8; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct tagRGBQUAD {
    rgbBlue: BYTE,
    rgbGreen: BYTE,
    rgbRed: BYTE,
    rgbReserved: BYTE,
}

/// [CURSORINFO structure](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-cursorinfo)
#[repr(C)]
pub struct tagCURSORINFO {
    cbSize: DWORD,
    flags: DWORD,
    hCursor: HCURSOR,
    ptScreenPos: POINT,
}

#[link(name = "Gdi32")]
extern "system" {
    // ['CreateSolidBrush'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
    // ['Rectangle'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
    pub fn Rectangle(
        hdc: HDC,
        nLeftRect: c_int,
        nTopRect: c_int,
        nRightRect: c_int,
        nBottomRect: c_int,
    ) -> BOOL;
    // ['CreatePen'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
    pub fn CreatePen(nPenStyle: c_int, nWidth: c_int, color: COLORREF) -> HPEN;
    // ['SelectObject'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
    pub fn SelectObject(hdc: HDC, hgdiobj: HGDIOBJ) -> HGDIOBJ;
    // ['SetBkColor'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkcolor)
    pub fn SetBkColor(hdc: HDC, color: COLORREF) -> COLORREF;
    // ['DeleteObject'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
    pub fn DeleteObject(hObject: HGDIOBJ) -> BOOL;
    // [SetPixel] (https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixel)
    pub fn SetPixel(hdc: HDC, c: c_int, y: c_int, color: COLORREF);
    /// [ChoosePixelFormat](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat)
    pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;
    // pub unsafe fn choose_pixel_format(
    //     hdc: HDC,
    //     ppfd: &PIXELFORMATDESCRIPTOR,
    // ) -> Result<c_int, Win32Error> {
    //     let index = ChoosePixelFormat(hdc, ppfd);
    //     if index != 0 {
    //         Ok(index)
    //     } else {
    //         Err(get_last_error())
    //     }
    // }
    /// [SetPixelFormat](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixelformat)
    pub fn SetPixelFormat(hdc: HDC, format: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;
    /// [DescribePixelFormat](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
    pub fn DescribePixelFormat(
        hdc: HDC,
        iPixelFormat: c_int,
        nBytes: UINT,
        ppfd: LPPIXELFORMATDESCRIPTOR,
    ) -> c_int;
}

#[link(name = "Kernel32")]
extern "system" {
    /// [GetLastError](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
    /// [GetModuleHandleExW](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexw)
    pub fn GetModuleHandleExW(dwFlags: DWORD, lpModuleName: LPCWSTR) -> HINSTANCE;
    /// If the function succeeds, the return value is a handle to the specified module.
    /// If lpModuleName is NULL, GetModuleHandle returns a handle to the file used to create the calling process (.exe file).
    /// If the function fails, the return value is NULL. To get extended error information, call GetLastError.
    ///
    /// [GetModuleHandleW](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
    /// [`VirtualFree`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree)
    /// If the function succeeds, the return value is nonzero.
    /// If the function fails, the return value is 0 (zero). To get extended error information, call GetLastError.
    pub fn VirtualFree(lpAddress: LPVOID, dwSize: SIZE_T, dwFreeType: DWORD) -> BOOL;
    /// [`VirtualAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc)
    pub fn VirtualAlloc(
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        flAllocationType: DWORD,
        flProtect: DWORD,
    ) -> LPVOID;
    /// [`FormatMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
    pub fn FormatMessageW(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPWSTR,
        nSize: DWORD,
        // Arguments: va_list, Todo!
    ) -> DWORD;

}

#[link(name = "User32")]
extern "system" {
    // ['ShowWindow'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    // ['GetWindowRect'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
    pub fn GetWindowRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    // ['TranslateMessage'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lpMsg: LPMSG) -> BOOL;
    // ['DispatchMessageW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lpmsg: LPMSG) -> LRESULT;
    // ['CreateWindowExW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
        dwExStyle: c_ulong,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: c_ulong,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    // ['CreateWindowW'](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindoww)
    pub fn CreateWindowW(
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    // ['GetMessageW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    // ['PeekMessageW'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
    pub fn PeekMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    // ['SendMessageW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
    pub fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    // ['RegisterClassW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    // ['DefWindowProcW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWND, Msg: c_uint, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    // ['CloseWindow'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
    pub fn CloseWindow(hWnd: HWND) -> BOOL;
    // ['DestroyWindow'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    // ['PostQuitMessage'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExitCode: c_int);
    // ['LoadCursorW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    // ['BeginPaint'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
    pub fn BeginPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> HDC;
    // ['FillRect'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn FillRect(hDc: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    // ['EndPaint'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    // ['MessageBox'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox)
    pub fn MessageBox(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint) -> c_int;
    /// [GetClientRect](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
    pub fn GetClientRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    // ['MessageBoxW'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint) -> c_int;
    // ['CreateMenu'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
    pub fn CreateMenu() -> HMENU;
    // ['AppendMenuW'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
    pub fn AppendMenuW(hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT, lpNewItem: LPCWSTR) -> BOOL;
    // ['SetMenu'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
    pub fn SetMenu(hWnd: HWND, hMenu: HMENU) -> BOOL;
    /// [GetCursorInfo](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorinfo)
    pub fn GetCursorInfo(pci: PCURSORINFO) -> BOOL;
    /// [GetCursorPos](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
    pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;
    // ['ScreenToClient'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
    pub fn ScreenToClient(hwnd: HWND, lpPoint: LPPOINT) -> BOOL;
    // ['GetKeyState'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeystate)
    pub fn GetKeyState(nVirtKey: c_int) -> c_short;
    // ['GetKeyboardState'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardstate)
    pub fn GetKeyboardState(lpKeyState: PBYTE) -> BOOL;
    // ['GetDC'] (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
    pub fn GetDC(hWnd: HWND) -> HDC;
    /// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
    /// [MessageBeep](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebeep)
    pub fn MessageBeep(uType: UINT) -> BOOL;
    /// [`UpdateWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
    pub fn UpdateWindow(hWnd: HWND) -> BOOL;
    /// [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos?redirectedfrom=MSDN)
    pub fn SetWindowPos(
        hWnd: HWND,
        hWndInsertAfter: HWND,
        X: c_int,
        Y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    /// [`FindWindowW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
    pub fn FindWindowW(lpClassName: LPCWSTR, lpWindowName: LPCWSTR) -> HWND;
    /// [`LoadIconW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)
    pub fn LoadIconW(hInstance: HINSTANCE, lpIconName: LPCWSTR) -> HICON;
    /// [`LoadImageW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
    pub fn LoadImageW(
        hInst: HINSTANCE,
        name: LPCWSTR,
        image_type: UINT,
        cx: c_int,
        cy: c_int,
        fuLoad: UINT,
    ) -> HANDLE;
}

#[link(name = "WinUser")]
extern "system" {
    pub fn MAKEINTRESOURCEW(i: c_int) -> LPWSTR;
    // {
    //     ((i as WORD) as ULONG_PTR) as LPWSTR
    // }
}

pub fn wide_null(string: &str) -> Vec<u16> {
    string.encode_utf16().chain(Some(0)).collect()
}

pub fn rgb(r: BYTE, g: BYTE, b: BYTE) -> u32 {
    let color: COLORREF =
        r as COLORREF | ((g as WORD) << 8) as COLORREF | ((b as DWORD) << 16) as COLORREF;
    return color;
}

#[test]
fn rgb_test() {
    assert_eq!(rgb(255, 0, 0), 255);
    assert_eq!(rgb(0, 255, 0), 0b00000000_00000000_11111111_00000000);
    assert_eq!(rgb(0, 0, 255), 0b00000000_11111111_00000000_00000000);
}

// wrappers

// needs more wrapping
pub fn get_last_error() -> DWORD {
    unsafe { GetLastError() }
}

// pub fn register_class_w(window_class: &WNDCLASSW) -> Result<ATOM, ()> {
//     let atom = unsafe { RegisterClassW(windows_class) };
//     if atom == 0 {
//         Err(())
//     } else {
//         Ok(atom)
//     }
// }

// done:
pub fn get_module_handle_w(lp_module_nsame: LPCWSTR) -> HMODULE {
    let module: HMODULE = unsafe { GetModuleHandleW(lp_module_nsame) };
    if module.is_null() {
        panic!("Failed to get module handle: {}", unsafe { GetLastError() });
    } else {
        module
    }
}

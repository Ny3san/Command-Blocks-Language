//! Native controls; callbacks queue events, never borrow the interpreter.
#[cfg(windows)]
use crate::diagnostic::{Error, Result, Span};

#[cfg(windows)]
mod native {
    use super::*;
    use std::{
        cell::RefCell,
        collections::{BTreeMap, VecDeque},
        ffi::c_void,
        ptr::null_mut,
    };
    type Handle = *mut c_void;

    fn failure(span: Span) -> Error {
        Error::new(
            "CB401",
            span,
            "Não foi possível executar a operação de janela.",
            "Could not perform window operation.",
        )
    }
    #[repr(C)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[repr(C)]
    struct Message {
        hwnd: Handle,
        message: u32,
        wparam: usize,
        lparam: isize,
        time: u32,
        point: Point,
        private: u32,
    }
    #[repr(C)]
    struct Class {
        style: u32,
        procedure: Option<unsafe extern "system" fn(Handle, u32, usize, isize) -> isize>,
        extra: i32,
        window_extra: i32,
        instance: Handle,
        icon: Handle,
        cursor: Handle,
        background: Handle,
        menu: *const u16,
        name: *const u16,
    }
    #[link(name = "user32")]
    unsafe extern "system" {
        fn RegisterClassW(class: *const Class) -> u16;
        fn CreateWindowExW(
            ex: u32,
            class: *const u16,
            title: *const u16,
            style: u32,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            parent: Handle,
            menu: Handle,
            instance: Handle,
            param: Handle,
        ) -> Handle;
        fn DefWindowProcW(hwnd: Handle, msg: u32, w: usize, l: isize) -> isize;
        fn DestroyWindow(hwnd: Handle) -> i32;
        fn IsWindow(hwnd: Handle) -> i32;
        fn ShowWindow(hwnd: Handle, command: i32) -> i32;
        fn PeekMessageW(
            message: *mut Message,
            hwnd: Handle,
            min: u32,
            max: u32,
            remove: u32,
        ) -> i32;
        fn TranslateMessage(message: *const Message) -> i32;
        fn DispatchMessageW(message: *const Message) -> isize;
        fn GetWindowTextLengthW(hwnd: Handle) -> i32;
        fn GetWindowTextW(hwnd: Handle, buffer: *mut u16, count: i32) -> i32;
        fn SetWindowTextW(hwnd: Handle, text: *const u16) -> i32;
        #[cfg(test)]
        fn SendMessageW(hwnd: Handle, msg: u32, w: usize, l: isize) -> isize;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleW(name: *const u16) -> Handle;
    }
    thread_local! { static EVENTS: RefCell<VecDeque<(usize,i32)>> = const { RefCell::new(VecDeque::new()) }; }
    unsafe extern "system" fn procedure(hwnd: Handle, msg: u32, w: usize, l: isize) -> isize {
        if msg == 0x111 && (w >> 16) == 0 && l != 0 {
            EVENTS.with(|q| {
                let mut q = q.borrow_mut();
                if q.len() < 1024 {
                    q.push_back((hwnd as usize, (w & 0xffff) as i32));
                }
            });
            return 0;
        }
        unsafe { DefWindowProcW(hwnd, msg, w, l) }
    }
    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(Some(0)).collect()
    }
    pub struct Window {
        handle: Handle,
        controls: BTreeMap<i32, Handle>,
        events: BTreeMap<i32, String>,
        next: i32,
    }
    impl Window {
        pub fn open(
            title: &str,
            width: i32,
            height: i32,
            visible: bool,
            span: Span,
        ) -> Result<Self> {
            if !(100..=4096).contains(&width)
                || !(100..=4096).contains(&height)
                || title.contains('\0')
            {
                return Err(failure(span));
            }
            let name = wide("CommandBlocksNativeWindow");
            unsafe {
                let instance = GetModuleHandleW(std::ptr::null());
                let class = Class {
                    style: 0,
                    procedure: Some(procedure),
                    extra: 0,
                    window_extra: 0,
                    instance,
                    icon: null_mut(),
                    cursor: null_mut(),
                    background: 16usize as Handle,
                    menu: std::ptr::null(),
                    name: name.as_ptr(),
                };
                // The class may already be registered by a previous window on this thread.
                RegisterClassW(&class);
                let title = wide(title);
                let handle = CreateWindowExW(
                    0,
                    name.as_ptr(),
                    title.as_ptr(),
                    0x00cf0000,
                    100,
                    100,
                    width,
                    height,
                    null_mut(),
                    null_mut(),
                    instance,
                    null_mut(),
                );
                if handle.is_null() {
                    return Err(failure(span));
                }
                if visible {
                    ShowWindow(handle, 5);
                }
                Ok(Self {
                    handle,
                    controls: BTreeMap::new(),
                    events: BTreeMap::new(),
                    next: 1,
                })
            }
        }
        pub fn control(
            &mut self,
            kind: &str,
            text: &str,
            event: &str,
            rect: [i32; 4],
            span: Span,
        ) -> Result<i32> {
            if self.next > 1000
                || text.contains('\0')
                || rect[2] <= 0
                || rect[3] <= 0
                || rect.iter().any(|v| v.abs_diff(0) > 16384)
                || !self.is_open()
            {
                return Err(failure(span));
            }
            let class = wide(match kind {
                "button" => "BUTTON",
                "input" => "EDIT",
                _ => "STATIC",
            });
            let text = wide(text);
            let id = self.next;
            let style = 0x50000000
                | if kind == "input" {
                    0x00810080
                } else if kind == "button" {
                    0x00010000
                } else {
                    0
                };
            let handle = unsafe {
                CreateWindowExW(
                    0,
                    class.as_ptr(),
                    text.as_ptr(),
                    style,
                    rect[0],
                    rect[1],
                    rect[2],
                    rect[3],
                    self.handle,
                    id as usize as Handle,
                    GetModuleHandleW(std::ptr::null()),
                    null_mut(),
                )
            };
            if handle.is_null() {
                return Err(failure(span));
            }
            self.next += 1;
            self.controls.insert(id, handle);
            if kind == "button" {
                self.events.insert(id, event.into());
            }
            Ok(id)
        }
        pub fn text(&self, id: i32, span: Span) -> Result<String> {
            let handle = *self.controls.get(&id).ok_or_else(|| failure(span))?;
            unsafe {
                let n = GetWindowTextLengthW(handle);
                if n > 1_000_000 {
                    return Err(failure(span));
                }
                let mut buffer = vec![0u16; n as usize + 1];
                let count = GetWindowTextW(handle, buffer.as_mut_ptr(), buffer.len() as i32);
                Ok(String::from_utf16_lossy(&buffer[..count as usize]))
            }
        }
        pub fn set_text(&self, id: i32, text: &str, span: Span) -> Result<()> {
            if text.contains('\0') {
                return Err(failure(span));
            }
            let handle = *self.controls.get(&id).ok_or_else(|| failure(span))?;
            if unsafe { SetWindowTextW(handle, wide(text).as_ptr()) } == 0 {
                return Err(failure(span));
            }
            Ok(())
        }
        pub fn poll(&mut self) -> Option<String> {
            unsafe {
                let mut message: Message = std::mem::zeroed();
                for _ in 0..128 {
                    if PeekMessageW(&mut message, self.handle, 0, 0, 1) == 0 {
                        break;
                    }
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
            EVENTS.with(|q| {
                let mut q = q.borrow_mut();
                let index = q.iter().position(|(h, _)| *h == self.handle as usize)?;
                let (_, id) = q.remove(index)?;
                self.events.get(&id).cloned()
            })
        }
        pub fn is_open(&self) -> bool {
            unsafe { IsWindow(self.handle) != 0 }
        }
        pub fn close(&mut self) {
            if self.is_open() {
                unsafe {
                    DestroyWindow(self.handle);
                }
            }
        }
    }
    impl Drop for Window {
        fn drop(&mut self) {
            self.close();
            EVENTS.with(|q| q.borrow_mut().retain(|(h, _)| *h != self.handle as usize));
        }
    }
    #[test]
    fn native_controls_and_click_events() {
        let span = Span::default();
        let mut w = Window::open("Teste", 320, 240, false, span).unwrap();
        let input = w
            .control("input", "Olá", "", [10, 10, 200, 24], span)
            .unwrap();
        assert_eq!(w.text(input, span).unwrap(), "Olá");
        w.set_text(input, "Alex", span).unwrap();
        assert_eq!(w.text(input, span).unwrap(), "Alex");
        let button = w
            .control("button", "Executar", "clicked", [10, 50, 100, 30], span)
            .unwrap();
        unsafe {
            SendMessageW(w.controls[&button], 0x00f5, 0, 0);
        }
        assert_eq!(w.poll().as_deref(), Some("clicked"));
        w.close();
        assert!(!w.is_open());
    }
}
#[cfg(windows)]
pub use native::Window;

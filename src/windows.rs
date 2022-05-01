use std::collections::HashMap;
use std::mem;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::HOT_KEY_MODIFIERS;
use windows::Win32::UI::WindowsAndMessaging;

pub mod modifiers {
    use windows::Win32::UI::Input::KeyboardAndMouse;

    pub const ALT: u32 = KeyboardAndMouse::MOD_ALT.0;
    pub const CONTROL: u32 = KeyboardAndMouse::MOD_CONTROL.0;
    pub const SHIFT: u32 = KeyboardAndMouse::MOD_SHIFT.0;
    pub const SUPER: u32 = KeyboardAndMouse::MOD_WIN.0;
}

pub mod keys {
    use windows::Win32::UI::Input::KeyboardAndMouse;

    pub const BACKSPACE: u32 = KeyboardAndMouse::VK_BACK.0 as u32;
    pub const TAB: u32 = KeyboardAndMouse::VK_TAB.0 as u32;
    pub const ENTER: u32 = KeyboardAndMouse::VK_RETURN.0 as u32;
    pub const CAPS_LOCK: u32 = KeyboardAndMouse::VK_CAPITAL.0 as u32;
    pub const ESCAPE: u32 = KeyboardAndMouse::VK_ESCAPE.0 as u32;
    pub const SPACEBAR: u32 = KeyboardAndMouse::VK_SPACE.0 as u32;
    pub const PAGE_UP: u32 = KeyboardAndMouse::VK_PRIOR.0 as u32;
    pub const PAGE_DOWN: u32 = KeyboardAndMouse::VK_NEXT.0 as u32;
    pub const END: u32 = KeyboardAndMouse::VK_END.0 as u32;
    pub const HOME: u32 = KeyboardAndMouse::VK_HOME.0 as u32;
    pub const ARROW_LEFT: u32 = KeyboardAndMouse::VK_LEFT.0 as u32;
    pub const ARROW_RIGHT: u32 = KeyboardAndMouse::VK_RIGHT.0 as u32;
    pub const ARROW_UP: u32 = KeyboardAndMouse::VK_UP.0 as u32;
    pub const ARROW_DOWN: u32 = KeyboardAndMouse::VK_DOWN.0 as u32;
    pub const PRINT_SCREEN: u32 = KeyboardAndMouse::VK_SNAPSHOT.0 as u32;
    pub const INSERT: u32 = KeyboardAndMouse::VK_INSERT.0 as u32;
    pub const DELETE: u32 = KeyboardAndMouse::VK_DELETE.0 as u32;
}

pub type ListenerID = i32;

pub struct Listener {
    last_id: i32,
    handlers: HashMap<ListenerID, Box<dyn Fn()>>,
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            last_id: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn register_hotkey<CB: 'static + Fn()>(
        &mut self,
        modifiers: u32,
        key: u32,
        handler: CB,
    ) -> Result<ListenerID, String> {
        unsafe {
            self.last_id += 1;
            let id = self.last_id;
            let result =
                KeyboardAndMouse::RegisterHotKey(HWND(0), id, HOT_KEY_MODIFIERS(modifiers), key);
            if !result.as_bool() {
                return Err("Failed to register hotkey".to_string());
            }

            self.handlers.insert(id, Box::new(handler));
            Ok(id)
        }
    }

    pub fn listen(self) {
        unsafe {
            loop {
                let mut msg = mem::MaybeUninit::uninit().assume_init();
                while WindowsAndMessaging::GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {
                    if !msg.wParam.is_invalid() {
                        if let Some(handler) = self.handlers.get(&(msg.wParam.0 as i32)) {
                            handler();
                        }
                    }
                }
            }
        }
    }
}

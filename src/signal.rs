use inputbot::{from_keybd_key, KeybdKey, KeybdKey::*};

#[derive(Copy, Clone, Debug)]
pub struct Signal {
    pub key: KeybdKey,
    pub is_capslock: bool,
    pub is_shift: bool,
    pub is_ctrl: bool,
    pub is_alt: bool,
}

impl Signal {
    fn is_uppercase_letter(&self) -> bool {
        (self.is_capslock || self.is_shift) && !(self.is_capslock && self.is_shift)
    }

    fn shift_modifier_keys(&self) -> Option<String> {
        if !(self.is_alt || self.is_ctrl) {
            return None;
        }

        let mut modifier = String::from(" ");

        if self.is_alt {
            modifier.push_str("⎇ + ");
        }

        if self.is_ctrl {
            modifier.push_str("✲ + ");
        }

        if (self.is_alt || self.is_ctrl) && self.is_shift {
            modifier.push_str("⇧ + ");
        }

        Some(modifier)
    }

    fn modifier(&self, val: &str) -> String {
        let mut modifier = String::new();

        if self.is_alt || self.is_ctrl || self.is_shift {
            modifier.push_str(" ");
        }

        if self.is_alt {
            modifier.push_str("⎇ + ");
        }

        if self.is_ctrl {
            modifier.push_str("✲ + ");
        }

        if self.is_shift {
            modifier.push_str("⇧ + ");
        }

        modifier.push_str(val);

        if self.is_alt || self.is_ctrl || self.is_shift {
            modifier.push_str(" ");
        }

        modifier
    }

    fn alternate_modifier(&self, val: &str, alt_val: &str) -> String {
        if let Some(mut modifier) = self.shift_modifier_keys() {
            modifier.push_str(&format!("{} ", val));
            modifier
        } else {
            if self.is_shift {
                alt_val.to_string()
            } else {
                val.to_string()
            }
        }
    }

    pub fn unicode_character(&self) -> String {
        match self.key {
            BackspaceKey => self.modifier("⌫"),
            TabKey => self.modifier("↹"),
            EnterKey => self.modifier("⏎"),
            EscapeKey => self.modifier("⎋"),
            SpaceKey => self.modifier("⎵"),
            PageUpKey => self.modifier("⇞"),
            PageDownKey => self.modifier("⇟"),
            EndKey => self.modifier("↘"),
            HomeKey => self.modifier("↖"),
            LeftKey => self.modifier("←"),
            UpKey => self.modifier("↑"),
            RightKey => self.modifier("→"),
            DownKey => self.modifier("↓"),
            InsertKey => self.modifier("⎀"),
            DeleteKey => self.modifier("⌦"),
            Numrow0Key => self.alternate_modifier("0", ")"),
            Numrow1Key => self.alternate_modifier("1", "!"),
            Numrow2Key => self.alternate_modifier("2", "@"),
            Numrow3Key => self.alternate_modifier("3", "#"),
            Numrow4Key => self.alternate_modifier("4", "$"),
            Numrow5Key => self.alternate_modifier("5", "%"),
            Numrow6Key => self.alternate_modifier("6", "^"),
            Numrow7Key => self.alternate_modifier("7", "&"),
            Numrow8Key => self.alternate_modifier("8", "*"),
            Numrow9Key => self.alternate_modifier("9", "("),
            AKey | BKey | CKey | DKey | EKey | FKey | GKey | HKey | IKey | JKey | KKey | LKey
            | MKey | NKey | OKey | PKey | QKey | RKey | SKey | TKey | UKey | VKey | WKey | XKey
            | YKey | ZKey => match from_keybd_key(self.key) {
                Some(e) => {
                    if let Some(mut modifier) = self.shift_modifier_keys() {
                        modifier.push_str(&format!("{} ", e.to_uppercase()));
                        modifier
                    } else {
                        if self.is_uppercase_letter() {
                            e.to_uppercase().to_string()
                        } else {
                            e.to_string()
                        }
                    }
                }
                _ => "".to_string(),
            },
            LSuper => self.modifier("❖"),
            RSuper => self.modifier("❖"),
            Numpad0Key => self.alternate_modifier("0", "0"),
            Numpad1Key => self.alternate_modifier("1", "1"),
            Numpad2Key => self.alternate_modifier("2", "2"),
            Numpad3Key => self.alternate_modifier("3", "3"),
            Numpad4Key => self.alternate_modifier("4", "4"),
            Numpad5Key => self.alternate_modifier("5", "5"),
            Numpad6Key => self.alternate_modifier("6", "6"),
            Numpad7Key => self.alternate_modifier("7", "7"),
            Numpad8Key => self.alternate_modifier("8", "8"),
            Numpad9Key => self.alternate_modifier("9", "9"),
            F1Key => self.modifier("F1"),
            F2Key => self.modifier("F2"),
            F3Key => self.modifier("F3"),
            F4Key => self.modifier("F4"),
            F5Key => self.modifier("F5"),
            F6Key => self.modifier("F6"),
            F7Key => self.modifier("F7"),
            F8Key => self.modifier("F8"),
            F9Key => self.modifier("F9"),
            F10Key => self.modifier("F10"),
            F11Key => self.modifier("F11"),
            F12Key => self.modifier("F12"),
            F13Key => self.modifier("F13"),
            F14Key => self.modifier("F14"),
            F15Key => self.modifier("F15"),
            F16Key => self.modifier("F16"),
            F17Key => self.modifier("F17"),
            F18Key => self.modifier("F18"),
            F19Key => self.modifier("F19"),
            F20Key => self.modifier("F20"),
            F21Key => self.modifier("F21"),
            F22Key => self.modifier("F22"),
            F23Key => self.modifier("F23"),
            F24Key => self.modifier("F24"),
            NumLockKey => self.modifier("⇭"),
            ScrollLockKey => self.modifier("⇳"),
            CapsLockKey => self.modifier("⇬"),
            LShiftKey => self.modifier("⇧"),
            RShiftKey => self.modifier("⇧"),
            LControlKey => self.modifier("✲"),
            RControlKey => self.modifier("✲"),
            LAltKey => self.modifier("⎇"),
            RAltKey => self.modifier("⎇"),
            BrowserBackKey => self.modifier("⎗"),
            BrowserForwardKey => self.modifier("⎘"),
            BrowserRefreshKey => self.modifier("⟲"),
            VolumeMuteKey => self.modifier(""),
            VolumeDownKey => self.modifier(""),
            VolumeUpKey => self.modifier(""),
            MediaNextTrackKey => self.modifier(""),
            MediaPrevTrackKey => self.modifier(""),
            MediaStopKey => self.modifier("◼"),
            MediaPlayPauseKey => self.modifier(""),
            BackquoteKey => self.alternate_modifier("`", "~"),
            SlashKey => self.alternate_modifier("/", "?"),
            BackslashKey => self.alternate_modifier("\\", "|"),
            CommaKey => self.alternate_modifier(",", "<"),
            PeriodKey => self.alternate_modifier(".", ">"),
            MinusKey => self.alternate_modifier("-", "_"),
            QuoteKey => self.alternate_modifier("'", "\""),
            SemicolonKey => self.alternate_modifier(";", ":"),
            LBracketKey => self.alternate_modifier("[", "{"),
            RBracketKey => self.alternate_modifier("]", "}"),
            EqualKey => self.alternate_modifier("=", "+"),
            _ => "".to_string(),
        }
    }
}

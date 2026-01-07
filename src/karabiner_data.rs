use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BundleIdentifier {
    #[serde(rename = "com.googlecode.iterm2")]
    ITerm2,
    #[serde(rename = "com.microsoft.VSCode")]
    VSCode,
    #[serde(rename = "com.todesktop.230313mzl4w4u92")]
    Cursor,
    #[serde(rename = "io.dynalist")]
    Dynalist, // https://help.dynalist.io/article/91-keyboard-shortcut-reference
    #[serde(rename = "com.tinyspeck.slackmacgap")]
    Slack,
    #[serde(rename = "com.google.Chrome")]
    GoogleChrome,
    #[serde(rename = "notion.id")]
    Notion,
    #[serde(rename = "com.openai.chat")]
    ChatGPT,
}

// https://karabiner-elements.pqrs.org/docs/json/root-data-structure/#custom-json-file-in-configkarabinerassetscomplex_modifications
#[derive(Debug, serde::Serialize)]
pub struct ComplexModifications<'a> {
    pub title: &'a str,
    pub rules: &'a Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub description: String,
    pub manipulators: Vec<Manipulator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Condition {
    OnApplication {
        r#type: ConditionType,
        bundle_identifiers: Vec<BundleIdentifier>,
    },
    WithVirtualKey {
        r#type: ConditionType,
        name: VirtualKey,
        value: u8,
    },
    InputSource {
        r#type: ConditionType,
        input_sources: Vec<InputSource>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSource {
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    FrontmostApplicationIf,
    VariableIf,
    InputSourceIf,
}

impl Condition {
    pub fn on_app(bundle_identifier: BundleIdentifier) -> Condition {
        Condition::OnApplication {
            r#type: ConditionType::FrontmostApplicationIf,
            bundle_identifiers: vec![bundle_identifier],
        }
    }

    pub fn with_vk1() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk1)
    }

    pub fn with_vk2() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk2)
    }

    pub fn with_vk3() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk3)
    }

    pub fn with_vk4() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk4)
    }

    pub fn with_shingeta_mode() -> Condition {
        Self::with_virtual_key(VirtualKey::ShingetaMode)
    }

    pub fn with_virtual_key(virtual_key: VirtualKey) -> Condition {
        Condition::WithVirtualKey {
            r#type: ConditionType::VariableIf,
            name: virtual_key,
            value: 1,
        }
    }

    pub fn without_virtual_key(virtual_key: VirtualKey) -> Condition {
        Condition::WithVirtualKey {
            r#type: ConditionType::VariableIf,
            name: virtual_key,
            value: 0,
        }
    }

    pub fn with_japanese_input() -> Condition {
        Condition::InputSource {
            r#type: ConditionType::InputSourceIf,
            input_sources: vec![InputSource {
                language: "ja".to_string(),
            }],
        }
    }

    pub fn with_english_input() -> Condition {
        Condition::InputSource {
            r#type: ConditionType::InputSourceIf,
            input_sources: vec![InputSource {
                language: "en".to_string(),
            }],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimultaneousKey {
    pub key_code: KeyCode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimultaneousOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_key_down_uninterruptedly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_down_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_up_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_up_when: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<To>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum From {
    Single {
        key_code: KeyCode,
        #[serde(skip_serializing_if = "Option::is_none")]
        modifiers: Option<FromModifier>,
    },
    Simultaneous {
        simultaneous: Vec<SimultaneousKey>,
        #[serde(skip_serializing_if = "Option::is_none")]
        simultaneous_options: Option<SimultaneousOptions>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FromModifier {
    Optional(Vec<ModifierKey>),
    Mandatory(Vec<ModifierKey>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetVariable {
    pub name: VirtualKey,
    pub value: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MouseKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_wheel: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_wheel: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged, rename_all = "snake_case")]
pub enum To {
    Variable {
        set_variable: SetVariable,
    },
    Key {
        key_code: KeyCode,
        #[serde(skip_serializing_if = "Option::is_none")]
        modifiers: Option<Vec<ModifierKey>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        repeat: Option<bool>,
    },
    Mouse {
        mouse_key: MouseKey,
    },
    Click {
        pointing_button: PointingButton,
    },
    Command {
        shell_command: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PointingButton {
    Button1,
    Button2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manipulator {
    pub r#type: ManipulatorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,

    pub from: From,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<To>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<ToAfterKeyUp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_if_alone: Option<Vec<ToIfAlone>>,
}

impl Manipulator {
    pub fn builder() -> ManipulatorInitBuilder {
        ManipulatorInitBuilder::default()
    }
}

pub struct ManipulatorInitBuilder<S = WithoutFrom> {
    conditions: Option<Vec<Condition>>,
    from: Option<From>,
    to: Vec<To>,
    to_after_key_up: Option<Vec<ToAfterKeyUp>>,
    to_if_alone: Option<Vec<ToIfAlone>>,
    _phantom: std::marker::PhantomData<S>,
}

pub struct WithoutFrom;
pub struct WithFrom;

impl Default for ManipulatorInitBuilder<WithoutFrom> {
    fn default() -> Self {
        Self {
            conditions: None,
            from: None,
            to: Vec::new(),
            to_after_key_up: None,
            to_if_alone: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S> ManipulatorInitBuilder<S> {
    pub fn condition(mut self, condition: Condition) -> Self {
        self.conditions.get_or_insert(vec![]).push(condition);
        self
    }

    pub fn conditions(mut self, conditions: Vec<Condition>) -> Self {
        self.conditions = Some(conditions);
        self
    }

    pub fn to_key(mut self, key_code: KeyCode, modifiers: Option<Vec<ModifierKey>>) -> Self {
        self.to.push(To::Key {
            key_code,
            modifiers,
            repeat: None,
        });
        self
    }

    pub fn to_key_with_repeat(
        mut self,
        key_code: KeyCode,
        modifiers: Option<Vec<ModifierKey>>,
        repeat: bool,
    ) -> Self {
        self.to.push(To::Key {
            key_code,
            modifiers,
            repeat: Some(repeat),
        });
        self
    }

    pub fn to_command(mut self, command: &str) -> Self {
        self.to.push(To::Command {
            shell_command: command.to_string(),
        });
        self
    }

    pub fn to_mouse(mut self, mouse_key: MouseKey) -> Self {
        self.to.push(To::Mouse { mouse_key });
        self
    }

    pub fn to_click(mut self, pointing_button: PointingButton) -> Self {
        self.to.push(To::Click { pointing_button });
        self
    }

    pub fn to_variable(mut self, set_variable: SetVariable) -> Self {
        self.to.push(To::Variable { set_variable });
        self
    }

    pub fn to_after_key_up(mut self, set_variable: SetVariable) -> Self {
        self.to_after_key_up
            .get_or_insert(vec![])
            .push(ToAfterKeyUp { set_variable });
        self
    }

    pub fn to_if_alone(mut self, key_code: KeyCode) -> Self {
        self.to_if_alone
            .get_or_insert(vec![])
            .push(ToIfAlone { key_code });
        self
    }
}

impl ManipulatorInitBuilder<WithoutFrom> {
    pub fn from_key(self, key_code: KeyCode) -> ManipulatorInitBuilder<WithFrom> {
        ManipulatorInitBuilder {
            conditions: self.conditions,
            from: Some(From::Single {
                key_code,
                modifiers: None,
            }),
            to: self.to,
            to_after_key_up: self.to_after_key_up,
            to_if_alone: self.to_if_alone,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn from_key_with_modifiers(
        self,
        key_code: KeyCode,
        modifiers: FromModifier,
    ) -> ManipulatorInitBuilder<WithFrom> {
        ManipulatorInitBuilder {
            conditions: self.conditions,
            from: Some(From::Single {
                key_code,
                modifiers: Some(modifiers),
            }),
            to: self.to,
            to_after_key_up: self.to_after_key_up,
            to_if_alone: self.to_if_alone,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn from_simultaneous_keys(
        self,
        key_codes: Vec<KeyCode>,
    ) -> ManipulatorInitBuilder<WithFrom> {
        ManipulatorInitBuilder {
            conditions: self.conditions,
            from: Some(From::Simultaneous {
                simultaneous: key_codes
                    .into_iter()
                    .map(|key_code| SimultaneousKey { key_code })
                    .collect(),
                simultaneous_options: None,
            }),
            to: self.to,
            to_after_key_up: self.to_after_key_up,
            to_if_alone: self.to_if_alone,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn from_simultaneous_keys_with_options(
        self,
        key_codes: Vec<KeyCode>,
        options: SimultaneousOptions,
    ) -> ManipulatorInitBuilder<WithFrom> {
        ManipulatorInitBuilder {
            conditions: self.conditions,
            from: Some(From::Simultaneous {
                simultaneous: key_codes
                    .into_iter()
                    .map(|key_code| SimultaneousKey { key_code })
                    .collect(),
                simultaneous_options: Some(options),
            }),
            to: self.to,
            to_after_key_up: self.to_after_key_up,
            to_if_alone: self.to_if_alone,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl ManipulatorInitBuilder<WithFrom> {
    pub fn build(self) -> Manipulator {
        Manipulator {
            r#type: ManipulatorType::Basic,
            conditions: self.conditions,
            from: self.from.expect("from key should be set"),
            to: self.to,
            to_after_key_up: self.to_after_key_up,
            to_if_alone: self.to_if_alone,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ManipulatorType {
    #[default]
    Basic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToIfAlone {
    pub key_code: KeyCode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToAfterKeyUp {
    pub set_variable: SetVariable,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VirtualKey {
    Vk1,
    Vk2,
    Vk3,
    Vk4,
    ShingetaMode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ModifierKey {
    Any,
    #[serde(rename = "control")]
    Ctrl,
    Shift,
    #[serde(rename = "option")]
    Opt,
    #[serde(rename = "command")]
    Cmd,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
// See https://github.com/pqrs-org/Karabiner-Elements/blob/a9154a6b073a3396631f43ed11f6dc603c28ea7b/src/share/types/key_code.hpp#L146-L360
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    #[serde(rename = "1")]
    Key1,
    #[serde(rename = "2")]
    Key2,
    #[serde(rename = "3")]
    Key3,
    #[serde(rename = "4")]
    Key4,
    #[serde(rename = "5")]
    Key5,
    #[serde(rename = "6")]
    Key6,
    #[serde(rename = "7")]
    Key7,
    #[serde(rename = "8")]
    Key8,
    #[serde(rename = "9")]
    Key9,
    #[serde(rename = "0")]
    Key0,
    ReturnOrEnter,
    Escape,
    DeleteOrBackspace,
    Tab,
    Spacebar,
    Hyphen,
    EqualSign,
    OpenBracket,
    CloseBracket,
    Backslash,
    NonUsPound,
    Semicolon,
    Quote,
    GraveAccentAndTilde,
    Comma,
    Period,
    Slash,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    DeleteForward,
    End,
    PageDown,
    RightArrow,
    LeftArrow,
    DownArrow,
    UpArrow,
    KeypadNumLock,
    KeypadSlash,
    KeypadAsterisk,
    KeypadHyphen,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadPeriod,
    NonUsBackslash,
    Application,
    Power,
    KeypadEqualSign,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeDecrement,
    VolumeIncrement,
    LockingCapsLock,
    LockingNumLock,
    LockingScrollLock,
    KeypadComma,
    KeypadEqualSignAs400,
    International1,
    International2,
    International3,
    International4,
    International5,
    International6,
    International7,
    International8,
    International9,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AlternateErase,
    SysReqOrAttention,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,
    Out,
    Oper,
    ClearOrAgain,
    CrSelOrProps,
    ExSel,
    LeftControl,
    LeftShift,
    LeftAlt,
    LeftGui,
    RightControl,
    RightShift,
    RightAlt,
    RightGui,
    VkNone,
    Fn,
    DisplayBrightnessDecrement,
    DisplayBrightnessIncrement,
    MissionControl,
    Launchpad,
    Dashboard,
    IlluminationDecrement,
    IlluminationIncrement,
    Rewind,
    PlayOrPause,
    Fastforward,
    Eject,
    AppleDisplayBrightnessDecrement,
    AppleDisplayBrightnessIncrement,
    AppleTopCaseDisplayBrightnessDecrement,
    AppleTopCaseDisplayBrightnessIncrement,
    LeftOption,
    LeftCommand,
    RightOption,
    RightCommand,
    JapaneseEisuu,
    JapaneseKana,
    JapanesePcNfer,
    JapanesePcXfer,
    JapanesePcKatakana,
    VkConsumerBrightnessDown,
    VkConsumerBrightnessUp,
    VkMissionControl,
    VkLaunchpad,
    VkDashboard,
    VkConsumerIlluminationDown,
    VkConsumerIlluminationUp,
    VkConsumerPrevious,
    VkConsumerPlay,
    VkConsumerNext,
    VolumeDown,
    VolumeUp,
}

/// Represents the structure of karabiner.json for type-safe operations
#[derive(Debug, Serialize, Deserialize)]
pub struct KarabinerConfig {
    pub global: Global,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Global {
    pub check_for_updates_on_startup: bool,
    pub show_in_menu_bar: bool,
    pub show_profile_name_in_menu_bar: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub complex_modifications: ComplexModificationsProfile,
    pub devices: Vec<serde_json::Value>,
    pub fn_function_keys: Vec<serde_json::Value>,
    pub name: String,
    pub parameters: serde_json::Value,
    pub selected: bool,
    pub simple_modifications: Vec<serde_json::Value>,
    pub virtual_hid_keyboard: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexModificationsProfile {
    pub parameters: serde_json::Value,
    pub rules: Vec<Rule>,
}

impl Default for KarabinerConfig {
    fn default() -> Self {
        Self {
            global: Global {
                check_for_updates_on_startup: true,
                show_in_menu_bar: true,
                show_profile_name_in_menu_bar: false,
            },
            profiles: vec![Profile {
                complex_modifications: ComplexModificationsProfile {
                    parameters: serde_json::json!({
                        "basic.simultaneous_threshold_milliseconds": 50,
                        "basic.to_delayed_action_delay_milliseconds": 500,
                        "basic.to_if_alone_timeout_milliseconds": 1000,
                        "basic.to_if_held_down_threshold_milliseconds": 500,
                        "mouse_motion_to_scroll.speed": 100
                    }),
                    rules: vec![],
                },
                devices: vec![],
                fn_function_keys: vec![],
                name: "Default profile".to_string(),
                parameters: serde_json::json!({
                    "delay_milliseconds_before_open_device": 1000
                }),
                selected: true,
                simple_modifications: vec![],
                virtual_hid_keyboard: serde_json::json!({
                    "country_code": 0,
                    "indicate_sticky_modifier_keys_state": true,
                    "mouse_key_xy_scale": 100,
                    "keyboard_type_v2": "jis",
                }),
            }],
        }
    }
}

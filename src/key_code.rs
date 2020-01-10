use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum KeyCode {
    Lang1,
    Lang2,
    International1,
    International2,
    International3,
    International4,
    International5,
    #[serde(rename = "0")]
    Key0,
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
    LeftArrow,
    DownArrow,
    UpArrow,
    RightArrow,
    Backslash,
    CapsLock,
    CloseBracket,
    Comma,
    DeleteForward,
    DeleteOrBackspace,
    DisplayBrightnessDecrement,
    DisplayBrightnessIncrement,
    EqualSign,
    Escape,
    Hyphen,
    JapaneseEisuu,
    JapaneseKana,
    NonUsPound,
    OpenBracket,
    Period,
    Quote,
    ReturnOrEnter,
    RightGui,
    Semicolon,
    Slash,
    Tab,
    VkNone,
    VolumeDecrement,
    VolumeIncrement,
}

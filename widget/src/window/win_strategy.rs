#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStrategy {
    Close(CloseStrategy),
    Minimize(MinimizeStrategy),
    Maximize(MaximizeStrategy),
    GainFocus(GainFocusStrategy),
    LoseFocus(LoseFocusStrategy),
    Show(ShowStrategy),
    Hide(HideStrategy),
    Hotkey(GlobalHotKeyStrategy),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseStrategy {
    Close,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinimizeStrategy {
    Minimize,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaximizeStrategy {
    Maximize,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GainFocusStrategy {
    GainFocus,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoseFocusStrategy {
    LoseFocus,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowStrategy {
    Show,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HideStrategy {
    Hide,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalHotKeyStrategy {
    StopSpread,
    Ignore,
}

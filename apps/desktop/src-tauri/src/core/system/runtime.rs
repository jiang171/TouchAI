// Copyright (c) 2026. 千诚. Licensed under GPL v3.

//! 运行时模式与环境开关。

use std::path::PathBuf;

const TOUCHAI_APP_ROOT_ENV: &str = "TOUCHAI_APP_ROOT";
const TOUCHAI_E2E_ENV: &str = "TOUCHAI_E2E";

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    pub is_e2e_test_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchWindowLaunchAction {
    Show,
    Toggle,
}

impl RuntimeInfo {
    pub fn current() -> Self {
        Self {
            is_e2e_test_mode: is_e2e_test_mode(),
        }
    }
}

pub fn is_e2e_test_mode() -> bool {
    matches!(
        std::env::var(TOUCHAI_E2E_ENV)
            .ok()
            .map(|value| value.trim().to_ascii_lowercase())
            .as_deref(),
        Some("1" | "true" | "yes" | "on")
    )
}

pub fn should_enable_single_instance() -> bool {
    !is_e2e_test_mode()
}

pub fn search_window_launch_action_from_args<I, S>(args: I) -> Option<SearchWindowLaunchAction>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    args.into_iter().find_map(|arg| match arg.as_ref() {
        "--show" | "show" => Some(SearchWindowLaunchAction::Show),
        "--toggle" | "toggle" => Some(SearchWindowLaunchAction::Toggle),
        _ => None,
    })
}

pub fn current_search_window_launch_action() -> Option<SearchWindowLaunchAction> {
    search_window_launch_action_from_args(std::env::args())
}

pub fn resolve_app_root_override() -> Option<PathBuf> {
    std::env::var(TOUCHAI_APP_ROOT_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::{search_window_launch_action_from_args, SearchWindowLaunchAction};

    #[test]
    fn search_window_launch_action_detects_show_flag() {
        assert_eq!(
            search_window_launch_action_from_args(["TouchAI", "--show"]),
            Some(SearchWindowLaunchAction::Show)
        );
    }

    #[test]
    fn search_window_launch_action_detects_toggle_flag() {
        assert_eq!(
            search_window_launch_action_from_args(["TouchAI", "--toggle"]),
            Some(SearchWindowLaunchAction::Toggle)
        );
    }

    #[test]
    fn search_window_launch_action_detects_single_instance_args_without_binary_name() {
        assert_eq!(
            search_window_launch_action_from_args(["--toggle"]),
            Some(SearchWindowLaunchAction::Toggle)
        );
    }

    #[test]
    fn search_window_launch_action_ignores_unknown_flags() {
        assert_eq!(
            search_window_launch_action_from_args(["TouchAI", "--minimized"]),
            None
        );
    }
}

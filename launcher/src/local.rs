//! Local-first launcher policy, read once rather than on every UI frame.
//! Local identities, installation downloads and decorative rendering are independent choices.
use std::sync::LazyLock;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Policy {
    /// Use local player names and leave Microsoft services, telemetry and shops off.
    pub local_only: bool,
    /// Explicitly refuse all launcher downloads; installed instances still launch.
    pub offline: bool,
    pub low_resource: bool,
}

impl Policy {
    fn read(online: Option<&str>, lightweight: Option<&str>, offline: Option<&str>) -> Self {
        Self {
            local_only: !matches!(online, Some("1")),
            offline: matches!(offline, Some("1")),
            low_resource: !matches!(lightweight, Some("0")),
        }
    }

    /// Background result delivery; input itself remains event-driven by GPUI.
    pub fn poll_interval(self, busy: bool) -> Duration {
        Duration::from_millis(match (self.low_resource, busy) {
            (false, _) => 80,
            (true, true) => 100,
            (true, false) => 500,
        })
    }
}

pub fn policy() -> &'static Policy {
    static POLICY: LazyLock<Policy> = LazyLock::new(|| {
        Policy::read(
            std::env::var("HORIZON_LAUNCHER_ONLINE").ok().as_deref(),
            std::env::var("HORIZON_LAUNCHER_LIGHTWEIGHT")
                .ok()
                .as_deref(),
            std::env::var("HORIZON_LAUNCHER_OFFLINE").ok().as_deref(),
        )
    });
    &POLICY
}

/// Installation downloads are allowed with a local identity. Explicit offline mode fails
/// before opening a socket; it never requires enabling Microsoft services.
pub fn require_online() -> Result<(), String> {
    require_online_with(*policy())
}

fn require_online_with(policy: Policy) -> Result<(), String> {
    if policy.offline {
        Err(crate::i18n::tr("Offline mode: install this version with internet first. Restart without HORIZON_LAUNCHER_OFFLINE=1 to allow downloads.").to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn online_and_visual_effects_are_independent_opt_ins() {
        let local = Policy::read(None, None, None);
        assert!(local.local_only && local.low_resource);
        assert!(!local.offline);
        let online = Policy::read(Some("1"), None, None);
        assert!(!online.local_only && online.low_resource);
        let decorated = Policy::read(None, Some("0"), None);
        assert!(decorated.local_only && !decorated.low_resource);
        let invalid = Policy::read(Some("yes"), Some("yes"), Some("yes"));
        assert!(invalid.local_only && invalid.low_resource);
    }

    #[test]
    fn offline_requests_fail_before_opening_a_connection() {
        assert!(require_online_with(Policy::read(None, None, None)).is_ok());
        assert!(require_online_with(Policy::read(None, None, Some("1"))).is_err());
        assert!(require_online_with(Policy::read(Some("1"), None, Some("1"))).is_err());
    }

    #[test]
    fn idle_polling_yields_to_the_game_without_delaying_busy_progress() {
        let lite = Policy::read(None, None, None);
        assert_eq!(lite.poll_interval(false), Duration::from_millis(500));
        assert_eq!(lite.poll_interval(true), Duration::from_millis(100));
        assert_eq!(
            Policy::read(None, Some("0"), None).poll_interval(false),
            Duration::from_millis(80)
        );
    }
}

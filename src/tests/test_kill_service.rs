use crate::models::Kill;
use crate::services::KillService;

#[test]
fn test_new_kill() {
    let kill = Kill::new(
        "Player1".to_string(),
        "Player2".to_string(),
        "MOD_RAILGUN".to_string(),
    );

    assert_eq!(kill.killer, "Player1");
    assert_eq!(kill.victim, "Player2");
    assert_eq!(kill.weapon, "MOD_RAILGUN");
}

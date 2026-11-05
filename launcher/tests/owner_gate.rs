//! A local launch needs prepared files, never a Microsoft account. No real player data is used.
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const MISSING: &str = "is not prepared yet";

/// Fresh homes with one installed (not prepared) version and an offline name, as a player who
/// played offline before the gate has.
fn home(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("horizon-owner-gate-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let version = root.join("launcher/versions/v2026.10.27.01");
    std::fs::create_dir_all(&version).unwrap();
    std::fs::write(version.join(".installed"), "").unwrap();
    std::fs::write(
        root.join("launcher/accounts.json"),
        r#"{"accounts":[],"selected":null,"offline":["Builder_1"],"offline_selected":"Builder_1"}"#,
    )
    .unwrap();
    root
}

fn launcher(root: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_horizon-launcher"));
    cmd.args(args)
        .env("HORIZON_LAUNCHER_ONLINE", "0")
        .env("HORIZON_LAUNCHER_OFFLINE", "1")
        .env("HORIZON_LAUNCHER_LIGHTWEIGHT", "1")
        .env("HORIZON_HOME", root.join("game"))
        .env("HORIZON_LAUNCHER_HOME", root.join("launcher"))
        .env("XDG_DATA_HOME", root.join("data"))
        .env("XDG_CONFIG_HOME", root.join("config"))
        .env("HORIZON_LAUNCHER_NO_SELF_UPDATE", "1")
        .env("HORIZON_NO_AUDIO", "1")
        .env("HORIZON_TELEMETRY_URL", "off")
        // Memory only: the test never reads the player's keychain.
        .env("HORIZON_NO_KEYCHAIN", "1")
        .env("HORIZON_MSA_CLIENT_ID", "test-client");
    cmd
}

fn text(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr)
}

#[test]
fn the_cli_reports_missing_resources_instead_of_asking_for_an_account() {
    let root = home("cli");
    let out = launcher(&root, &["play"]).output().unwrap();
    let said = text(&out);
    assert!(!out.status.success(), "{said}");
    assert!(said.contains(MISSING), "{said}");
    assert!(!said.contains("Sign in with a Microsoft account"), "{said}");
    assert!(said.contains("horizon-launcher prepare"), "{said}");
    // Nothing of the game ran, and the offline name is still there.
    assert!(!root.join("game/sessions").exists());
    let accounts = std::fs::read_to_string(root.join("launcher/accounts.json")).unwrap();
    assert!(accounts.contains("Builder_1"), "{accounts}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_mcp_server_reports_missing_resources_without_an_account_gate() {
    let root = home("mcp");
    let mut child = launcher(&root, &["mcp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let requests = [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"play","arguments":{}}}"#,
    ];
    let mut stdin = child.stdin.take().unwrap();
    for r in requests {
        writeln!(stdin, "{r}").unwrap();
    }
    drop(stdin);
    let out = child.wait_with_output().unwrap();
    let reply = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap())
        .find(|v| v["id"] == 2)
        .expect("a reply to play");
    assert_eq!(reply["result"]["isError"], true, "{reply}");
    let said = reply["result"]["content"][0]["text"].as_str().unwrap();
    assert!(said.contains(MISSING), "{said}");
    assert!(!said.contains("Sign in with a Microsoft account"), "{said}");
    let _ = std::fs::remove_dir_all(&root);
}

/// A harmless stand-in records the actual process arguments and telemetry policy. It has no
/// game code or assets: this proves the CLI reaches process launch with only local files.
#[cfg(unix)]
#[test]
fn prepared_local_game_starts_without_tokens_and_keeps_the_offline_name() {
    check_prepared_local_game(false);
}

#[cfg(unix)]
#[test]
fn a_saved_microsoft_selection_is_ignored_without_overwriting_the_accounts_file() {
    check_prepared_local_game(true);
}

#[cfg(unix)]
fn check_prepared_local_game(saved_account: bool) {
    use std::os::unix::fs::PermissionsExt;
    let root = home(if saved_account {
        "saved-account"
    } else {
        "local-ready"
    });
    if saved_account {
        std::fs::write(root.join("launcher/accounts.json"),
            r#"{"accounts":[{"uuid":"069a79f444e94726a5befca90e38aaf5","name":"SavedPlayer"}],"selected":"069a79f444e94726a5befca90e38aaf5","offline":["Builder_1"],"offline_selected":"Builder_1"}"#).unwrap();
    }
    let version = root.join("launcher/versions/v2026.10.27.01");
    let resources = version.join(".local/horizon-client/26.3");
    std::fs::create_dir_all(&resources).unwrap();
    std::fs::write(resources.join("horizon-resources.json"), "{}").unwrap();
    std::fs::write(
        version.join("research-lock.json"),
        r#"{"minecraft":{"version":"26.3"}}"#,
    )
    .unwrap();
    let program = version.join("horizon-client");
    std::fs::write(&program, r#"#!/bin/sh
printf '%s\n' "$@" > "$HORIZON_HOME/probe-args.txt"
printf '%s|%s|%s\n' "$HORIZON_TELEMETRY_URL" "$HORIZON_ACCOUNT_SESSIONS" "$HORIZON_LAUNCHER_EXE" > "$HORIZON_HOME/probe-env.txt"
"#).unwrap();
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o700)).unwrap();
    let accounts = std::fs::read(root.join("launcher/accounts.json")).unwrap();
    let out = launcher(&root, &["play"])
        .env("HORIZON_ACCOUNT_SESSIONS", "must-not-reach-the-game")
        .env("HORIZON_LAUNCHER_EXE", "must-not-reach-the-game")
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", text(&out));
    let args = std::fs::read_to_string(root.join("game/probe-args.txt")).unwrap();
    assert!(args.contains("--name\nBuilder_1\n"), "{args}");
    assert!(!args.contains("--session-file"), "{args}");
    assert_eq!(
        std::fs::read_to_string(root.join("game/probe-env.txt")).unwrap(),
        "off||\n"
    );
    assert!(!root.join("game/sessions").exists());
    assert_eq!(
        std::fs::read(root.join("launcher/accounts.json")).unwrap(),
        accounts
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// Installed Java instances launch twice with the same local identity while launcher network
/// access is explicitly disabled. A stand-in Java records the command; no game files are used.
#[cfg(unix)]
#[test]
fn installed_java_plays_offline_without_reading_or_overwriting_a_saved_microsoft_account() {
    use std::os::unix::fs::PermissionsExt;
    let root = home("java-local");
    let minecraft = root.join("launcher/minecraft");
    let version = minecraft.join("versions/1.21.4");
    std::fs::create_dir_all(&version).unwrap();
    std::fs::write(minecraft.join("version_manifest_v2.json"), r#"{
        "latest":{"release":"1.21.4"},
        "versions":[{"id":"1.21.4","type":"release","url":"unused","sha1":"unused"}]
    }"#).unwrap();
    std::fs::write(version.join(".downloaded"), "").unwrap();
    std::fs::write(version.join("1.21.4.jar"), "stand-in client").unwrap();
    std::fs::write(version.join("1.21.4.json"), r#"{
        "id":"1.21.4","mainClass":"test.Main","libraries":[],
        "javaVersion":{"majorVersion":21,"component":"java-runtime-delta"},
        "arguments":{"jvm":["-cp","${classpath}"],"game":[
            "--username","${auth_player_name}","--uuid","${auth_uuid}",
            "--accessToken","${auth_access_token}","--userType","${user_type}"]}
    }"#).unwrap();
    let java_home = root.join("fake-java");
    std::fs::create_dir_all(java_home.join("bin")).unwrap();
    std::fs::write(java_home.join("release"), "JAVA_VERSION=\"21.0.5\"\n").unwrap();
    let java = java_home.join("bin/java");
    std::fs::write(&java, "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$HORIZON_HOME/java-args.txt\"\n").unwrap();
    std::fs::set_permissions(&java, std::fs::Permissions::from_mode(0o700)).unwrap();
    std::fs::write(root.join("launcher/accounts.json"),
        r#"{"accounts":[{"uuid":"069a79f444e94726a5befca90e38aaf5","name":"SavedPlayer"}],"selected":"069a79f444e94726a5befca90e38aaf5","offline":["Builder_1"],"offline_selected":"Builder_1"}"#).unwrap();
    let accounts = std::fs::read(root.join("launcher/accounts.json")).unwrap();
    for args in [
        vec!["instance", "create", "Vanilla", "--minecraft", "1.21.4"],
        vec!["instance", "java", "Vanilla", "--home", java_home.to_str().unwrap()],
    ] {
        let out = launcher(&root, &args).output().unwrap();
        assert!(out.status.success(), "{}", text(&out));
    }
    let mut first = String::new();
    for attempt in 0..2 {
        let out = launcher(&root, &["play", "--instance", "Vanilla"]).output().unwrap();
        assert!(out.status.success(), "{}", text(&out));
        let args = std::fs::read_to_string(root.join("game/java-args.txt")).unwrap();
        assert!(args.contains("--username\nBuilder_1\n"), "{args}");
        assert!(args.contains("--accessToken\n0\n"), "{args}");
        assert!(args.contains("--userType\nlegacy\n"), "{args}");
        assert!(!args.contains("SavedPlayer"), "{args}");
        if attempt == 0 {
            first = args;
        } else {
            assert_eq!(args, first);
        }
    }
    assert_eq!(std::fs::read(root.join("launcher/accounts.json")).unwrap(), accounts);
    assert!(!root.join("game/sessions").exists());
    std::fs::remove_dir_all(root).unwrap();
}

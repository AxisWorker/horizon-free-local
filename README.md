# Horizon Free Local

A lightweight launcher fork of [0CYBER/horizon-free](https://github.com/0CYBER/horizon-free), based on [Horizon](https://github.com/horizon-rust/horizon).

**Choose a Minecraft Java version, create an instance, and press Play.** The first
launch downloads that version, its libraries, assets and the required Java runtime.
Once installation is complete, the instance can launch offline with a local player
name. Microsoft sign-in is not required in the default local-account mode.

## What changed in local-v0.2.0

- Separated local identities from network access: installation downloads are now
  allowed without turning on Microsoft account services.
- New-instance dialogs default to **Vanilla**. Select the desired Minecraft Java
  version from Mojang's catalog, create the instance and press **Play**.
- Java instances use the selected local name, a stable Minecraft offline UUID and
  a placeholder token. Saved Microsoft credentials are not read or renewed.
- Downloads retain manifest and file hash verification. Incomplete assets fail the
  installation instead of marking it complete; retrying keeps verified files.
- Minecraft libraries/assets and the matching Java runtime are downloaded on
  demand. Installed Java instances do not need a catalog refresh to start.
- Explicit `HORIZON_LAUNCHER_OFFLINE=1` blocks launcher downloads for users who
  want a strict offline session. Default local-account mode permits installation.
- The Horizon build catalog can load at startup, without automatic installation,
  self-update, shop polling, live player counts or Microsoft startup workers.

The original lightweight changes remain: no animated background, parallax, panel
blur or entrance animations; reduced idle polling; no scan of every version's
resources at startup; minimize on play when no preference has been saved.
Launcher telemetry is disabled in local-account mode, and Horizon game processes
receive `HORIZON_TELEMETRY_URL=off` without inherited account sessions.

## First installation

1. Download the Windows x64 or Linux x64 launcher from [Releases](https://github.com/AxisWorker/horizon-free-local/releases).
2. Open it with internet access. Choose a local player name in the player switcher
   (the default is `Player` if no name has been selected).
3. Create a **New instance**, choose **Vanilla**, select the Minecraft Java version
   you want, and create it.
4. Press **Play**. Wait for the game files, assets and required Java runtime to finish
   installing. The game then starts in the instance's own folder.
5. Future launches of that completely installed instance can run without internet.

Fabric, Quilt, Forge and NeoForge options use the existing loader flow. Availability
and compatibility depend on the selected Minecraft and loader versions. This fork
does not promise that every historical version or third-party modpack will work.
Online-authenticated multiplayer services still need a valid account.

**Minecraft Java instances and Horizon builds are different.** Java instances run
the selected original Minecraft version on Java. Horizon is a Rust reimplementation
with its own supported Minecraft version and preparation flow; selecting an older
Java version does not make Horizon emulate it. This work reduces launcher overhead,
not the game's hardware requirements. No performance benchmarks were performed.

No Minecraft jars, assets, Java runtimes or prepared resources are bundled with
this repository or the launcher downloads; installation fetches the required files.

## Command line

```sh
horizon-launcher account offline YourName
horizon-launcher instance versions
horizon-launcher instance create "Vanilla 1.21.4" --minecraft 1.21.4
horizon-launcher play --instance "Vanilla 1.21.4"
```

The first Play installs missing files. Subsequent launches reuse them. `play` runs
without creating the graphical launcher window. On Windows, use
`.\horizon-free-local-windows-x64.exe` instead of `horizon-launcher`.

For a completely installed instance with launcher downloads explicitly blocked:

```sh
HORIZON_LAUNCHER_OFFLINE=1 horizon-launcher play --instance "Vanilla 1.21.4"
```

For Horizon preparation from existing local files, `HORIZON_SERVER_JAR` still
accepts your server jar path and `prepare --minecraft /path/to/minecraft` selects
its Minecraft folder. Missing installation files require internet unless supplied
locally. `HORIZON_LAUNCHER_HOME` and `HORIZON_HOME` select the launcher and game data
folders, respectively.

## Optional modes

| Setting | Behavior |
| --- | --- |
| No overrides | Local player names, installation downloads, lightweight UI |
| `HORIZON_LAUNCHER_OFFLINE=1` | Refuse launcher downloads; use installed files |
| `HORIZON_LAUNCHER_ONLINE=1` | Opt into the original Microsoft account and online services flow |
| `HORIZON_LAUNCHER_LIGHTWEIGHT=0` | Restore decorative launcher effects |

The offline flag controls the launcher's shared download path. It is not a firewall
for games, mods, external programs or multiplayer connections.

## Builds and verification

The **Local launcher - Windows and Linux x64** workflow runs native builds and
launcher tests before publishing the Windows `.exe`/`.zip`, Linux `.tar.gz` and
`SHA256SUMS.txt`. A configured workflow alone does not mean a release has passed;
check the corresponding run and published release.

```sh
cargo build --release --locked --manifest-path launcher/Cargo.toml
```

Tests cover the independent network/account policy, verified downloads and resume,
Java launch arguments, local names and repeated offline process launches using
made-up fixtures. No actual Minecraft gameplay, physical GPU compatibility or
performance is certified by those fixtures. The earlier validation record is in
[docs/launcher-local-validation.md](docs/launcher-local-validation.md).

## Credits and license

Original Horizon work: the Horizon authors and contributors. Offline-play fork:
0CYBER. Local installation and lightweight launcher changes were prepared for
AxisWorker with OpenAI Codex. Original attribution and notices are preserved.

[MIT license](LICENSE) for the project's original work; dependencies retain their
own licenses. Unofficial; not affiliated with Mojang or Microsoft.

<details>
<summary>Original upstream README (reference; its online setup instructions differ from this fork)</summary>

> [!WARNING]
> This repository is not in any way affiliated with Horizon original devs or Microsoft/Mojang.
>
> The goal of this fork is to return local account play with **YOUR** minecraft assets without Microsoft profile authentication
>
> You can download it's builds in [RELEASES](https://github.com/0CYBER/horizon-free/releases/latest) section
>
> Lower part of the README is fully original

<br><br><br><br>

<p align="center">
  <img src="docs/media/banner.webp" alt="The Horizon website: the game you know, rebuilt in Rust" width="100%">
</p>

<p align="center">
  <a href="https://github.com/horizon-rust/horizon/actions/workflows/ci.yml"><img src="https://github.com/horizon-rust/horizon/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI"></a>
  <img src="https://img.shields.io/badge/Minecraft-Java%2026.3-5D9C3F" alt="Minecraft Java 26.3">
  <img src="https://img.shields.io/badge/Rust-stable-B7410E?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/vanilla%20parity-100%25-F0795A" alt="Vanilla parity 100%">
  <a href="#project-status"><img src="https://img.shields.io/badge/player--visible%20gaps-0%20open-F0795A" alt="Player-visible gaps open"></a>
  <img src="https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-2A2470" alt="Linux, macOS and Windows">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-3B2F7E" alt="MIT license"></a>
</p>

<p align="center">
  <a href="https://playhorizon.dev"><b>Website</b></a> ·
  <a href="#download"><b>Download</b></a> ·
  <a href="https://playhorizon.dev/docs/"><b>Documentation</b></a> ·
  <a href="https://github.com/horizon-rust/horizon/releases"><b>Release notes</b></a> ·
  <a href="ROADMAP.md"><b>Roadmap</b></a> ·
  <a href="#project-status"><b>Project status</b></a> ·
  <a href="#contributing"><b>Contributing</b></a>
</p>

# Horizon

Minecraft: Java Edition 26.3, rewritten in Rust. Same worlds, same rules, same terrain, checked against the Java game by more than 300 differential harnesses. Free, open source and unofficial.

> Not an official Minecraft product, nor approved by or associated with Mojang or Microsoft. Horizon contains no Minecraft code, assets or data: you need your own copy of Java Edition 26.3.

## Download

Get **the Horizon launcher** from [playhorizon.dev](https://playhorizon.dev) or [the launcher releases](https://github.com/horizon-rust/horizon/releases?q=launcher-v&expanded=true) (Linux x86-64, macOS on Apple Silicon, macOS on Intel from the next Nightly, Windows x64). It installs and updates the game, prepares its files from your copy of Minecraft, and updates itself.

**You need** a Microsoft account that owns Java Edition ([why](docs/microsoft-account.md)) and a GPU with Vulkan or OpenGL 3.3 (Linux), Vulkan or DirectX 12 (Windows), or a Mac (Apple Silicon, or Intel from the next Nightly). An existing Minecraft install (official launcher, Prism, MultiMC...) is reused; without one, Play downloads 26.3 from Mojang for your account. Step by step: [Install and first launch](https://playhorizon.dev/docs/install).

<table>
  <tr>
    <td width="50%"><img src="docs/media/launcher-play.webp" alt="The launcher's Home page: the selected instance ready to play, the version picker, Join a friend and the instance's tabs"></td>
    <td width="50%"><img src="docs/media/launcher-settings.webp" alt="The launcher's Settings page: Microsoft account, offline name, updates, appearance and accent colour"></td>
  </tr>
</table>

## What works today

- **The game:** vanilla world generation, mobs, villagers, raids, redstone, every command, the HUD and the recipe book, on vanilla saves. Worlds from 1.8 to 26.2 are upgraded. [Status](#project-status).
- **Extras:** new menus, graphics presets, distant terrain up to 1024 chunks, controllers, zoom, minimap and more, off by default where they change the vanilla look. [Settings](https://playhorizon.dev/docs/settings).
- **Packs:** resource packs, and data packs with functions, predicates, item modifiers and world generation.
- **Multiplayer:** invite friends with a code, Open to LAN, join servers (online mode included). [Multiplayer](https://playhorizon.dev/docs/multiplayer).
- **Mods:** content mods (blocks, items, data) and native Rust mods, built by the launcher with nothing to install. Java mod jars do not run. [Mods](https://playhorizon.dev/docs/mods).

In the Rust client, with vanilla assets and no mods or shaders:

<table>
  <tr>
    <td colspan="2"><img src="docs/media/sunset.webp" alt="Sunset over a savanna river valley and a plains village"></td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/media/survival.webp" alt="Survival at dusk: a zombie walks up, with the hotbar, health, hunger and experience"></td>
    <td width="50%"><img src="docs/media/settings.webp" alt="Vanilla's Video Settings screen with Horizon's own buttons at the top: Horizon Graphics and Advanced Graphics"></td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/media/village.webp" alt="A plains village among savanna hills, with farms, villagers and an iron golem"></td>
    <td width="50%"><img src="docs/media/nether.webp" alt="A crimson forest in the Nether with a zombified piglin"></td>
  </tr>
</table>

## Host a server

The launcher's **Servers** page creates, starts, backs up and updates a server, and invites friends over Tailscale. On a VPS, `horizon-server` installs with one command and needs no Java. [Host a server](https://playhorizon.dev/docs/hosting) ([reference](docs/hosting.md)).

## Modpacks

The launcher imports Modrinth and CurseForge packs and plays the mods Horizon can carry over: today, content mods made of data and assets, not Java code. [Modpacks](https://playhorizon.dev/docs/modpacks).

## Agent control

AI agents and scripts can play Horizon. `horizon-ctl`, shipped with the game, reads its state as JSON and acts like a player; `horizon-ctl mcp` serves the same commands over MCP.

```sh
horizon-ctl launch --world Survival --seed 5 --wait   # finds the version the launcher installed
horizon-ctl state                                       # position, health, food, target, screen...
horizon-ctl find-block birch_log                        # the nearest logs
horizon-ctl goto -0.5 65 -2.5 --near 2.5                # walks there along a path
horizon-ctl break -1 65 -3                              # mines it; replies with the drops
horizon-ctl open-inventory && horizon-ctl click-slot 0  # menus: slots, buttons by label, text fields
```

Commands block until done and reply with the result; `horizon-ctl help` lists them all. [Agent control](docs/native-client.md#agent-control-horizon-ctl-and-mcp).

## Build from source

Needs Rust 1.90+, Python 3.10+, JDK 25, and on Linux `libasound2-dev libudev-dev`. `horizon-prepare` builds the game's resources from **your own copy** of Java Edition 26.3 (checked against `research-lock.json`) and never downloads it:

1. **Client:** play 26.3 once in the official launcher. Horizon reads `~/.minecraft` (or `--minecraft-dir` / `HORIZON_MINECRAFT_DIR`).
2. **Server jar:** the 26.3 server jar (`downloads.server.url` in `versions/26.3/26.3.json`), passed once with `--server-jar` or `HORIZON_SERVER_JAR`.

```sh
git clone https://github.com/horizon-rust/horizon.git
cd horizon
cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar   # needs JDK 25
cargo run --release -p horizon-prepare -- sounds   # optional: sounds from your launcher install
cargo run --release -p horizon-client
```

Prepared files land in the gitignored `.local/`: never commit or share them. Options, controls and architecture: [the native client guide](docs/native-client.md); all developer docs: [docs/README.md](docs/README.md).

## Roadmap

What is in progress, what comes next and where to help: [ROADMAP.md](ROADMAP.md), or [playhorizon.dev/hub](https://playhorizon.dev/hub) with the work board and what shipped.

## Contributing

Issues and pull requests are welcome: read [CONTRIBUTING.md](CONTRIBUTING.md) (vanilla first, small pull requests). Questions: [Discord](https://discord.gg/playhorizon). Before a pull request, run the self-check and paste its output:

```sh
git config core.hooksPath .githooks
python3 tools/pr_ready.py
```

It runs rustfmt, clippy, the touched crates' tests, the Python tests, the public-tree check and the vanilla harnesses your change affects. Never commit game files, decompiled source or prepared resources.

## Project status

**Player-visible gaps: 0 open** of the 833 the [port ledger](tools/port_ledger.py) found, reading every vanilla 26.3 method against Horizon's code: vanilla behaviour a player would notice that Horizon doesn't have yet. By area: Animals [0](https://github.com/horizon-rust/horizon/issues/3512); Monsters and bosses [0](https://github.com/horizon-rust/horizon/issues/3513); Entities, projectiles, vehicles and decorations [0](https://github.com/horizon-rust/horizon/issues/3514); Items, enchantments and inventories [0](https://github.com/horizon-rust/horizon/issues/3515); Blocks, redstone and fluids [0](https://github.com/horizon-rust/horizon/issues/3516); World, world generation and saves [0](https://github.com/horizon-rust/horizon/issues/3517); Screens, HUD and client packets [0](https://github.com/horizon-rust/horizon/issues/3518); Entity and block rendering, models [0](https://github.com/horizon-rust/horizon/issues/3519); Server and players on the server [0](https://github.com/horizon-rust/horizon/issues/3520); Mob AI, villagers and raids [0](https://github.com/horizon-rust/horizon/issues/3521). This number goes down as gaps close (`tools/parity_gaps.py`); the harness results below say that what is tested matches.

The table is the project's own audit: every subsystem matches vanilla in every harness that compares it. 100% means 100% of what the harnesses compare, not that nothing else differs.

### Java parity: 100%

Scores: 0 (absent), 0.25 (started), 0.5 (playable, major features missing), 0.75 (mostly complete) or 1 (complete and verified by differential tests); the percentage is their average. Last audited 2026-10-13: the harnesses pass on Linux and macOS. The [port ledger](tools/port_ledger.py) adds a method-by-method view of what is ported.

| Subsystem | Score | Main gaps |
| --- | ---: | --- |
| Window, input and game loop | 1 | None: game loop, options, camera, auto-jump, toggle sprint/sneak and frame limiter match |
| Assets and resources | 1 | None: item models match over 196,290 stacks; the resource pack screen over 3,000 sessions |
| Network protocol and multiplayer | 1 | None: joins vanilla servers; the dedicated server matches vanilla's files, login, configuration, play and command tree; LAN discovery over 462 cases (offline mode only) |
| Chunk storage and palettes | 1 | None: client and saves share paletted containers, identical to vanilla over 266 scenarios |
| Section meshing | 1 | None: 112 scenes and over a million quads identical to vanilla |
| Rendering | 1 | None: entity models, block entity renderers (407 cases) and animations (296), HUD overlays (2,200 states), the recipe book (600), player list, subtitles and world border match |
| Lighting | 1 | None: saved and relit light matches vanilla, slabs and stairs included |
| World generation | 1 | None: 2,352 whole chunks over 225 structure sites, in all three dimensions, identical to vanilla |
| Region files and saves | 1 | None: saves round-trip with vanilla; worlds from 1.8 to 26.2 upgrade as vanilla's DataFixers do |
| Entities and physics | 1 | None: attributes, spawn rules, natural spawning (15,500 cases), raids and patrols (1,900), `finalizeSpawn` (24,000) and mob AI (92 scenarios) match |
| Interaction and inventory | 1 | None: menu clicks over 8,000 sequences; recipe book placement (6,000), buttons (1,200), saved book (1,500), loom and crafter screens (1,600) |
| Menus, HUD and chat | 1 | None: every command (758 over 39,685 inputs), scoreboards (81,000 operations), the player list (872,000 draw calls), chat and its suggestions (3,000 sessions) match |
| Audio | 1 | None: the sound engine, sound events, music, situational music, ambience and subtitles match vanilla |
| Block behaviours and ticking | 1 | None: block updates, random ticks, redstone, dispensers (6,000 cases), crafters (4,000), container spills (4,000), copper golems and the sculk family (435 scenarios) match |

### Known issues

Skins served over https show the default skin unless already cached; pack icons show the unknown-pack texture in game; item tooltips lack attribute lines.

## Licence

[MIT](LICENSE), for the project's original work; dependencies keep their own licences ([notices](THIRD_PARTY_NOTICES.md)). Minecraft is a trademark of Mojang Synergies AB; its code and assets belong to Mojang and Microsoft under the [Minecraft EULA](https://www.minecraft.net/en-us/eula).

## Author

Made by [Ludovic Marie](https://lu4ovic.com) ([@ludovic111](https://github.com/ludovic111)).

</details>

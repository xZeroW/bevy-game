# Copilot / AI Agent Instructions for bevy-game

Purpose: Give an AI coding agent the minimal, actionable context to be productive in this Bevy-based Rust game.

- Big picture: This is an ECS Bevy app. `src/main.rs` builds a `bevy::App`, initializes `GameState`, and loads `GamePlugin` which composes module-level plugins (resources, player, camera, enemies, collisions, etc.). Look at `src/game/game.rs` for the plugin composition.

- Key files & responsibilities:
  - `src/main.rs` — application entry and plugin registration.
  - `src/game/game.rs` — the central `GamePlugin` that wires plugins together.
  - `src/game/resources.rs` — loads the global sprite atlas and provides runtime resources like `GlobalTextureAtlas` and `CursorPosition`.
  - `src/game/config.rs` — single source of truth for sprite/tiles, camera, and gameplay constants (tile sizes, sprite sheet layout, bullet/enemy constants).
  - `src/game/player/*` — player entity, controls, weapon systems and events (see `player.rs`, `component.rs`, `controls.rs`, `weapon.rs`).
  - `assets/` — images, maps and tilesets. `SPRITE_SHEET_PATH` in `config.rs` points to `assets/tilesets/assets.png`.

- Startup / lifecycle patterns to follow:
  - Use module `Plugin`s per feature (example: `PlayerPlugin`, `CameraPlugin`, `ResourcesPlugin`). Add new features as plugins and register them in `GamePlugin`.
  - Asset loading happens in a `Startup` system (`load_assets` in `resources.rs`) which sets `NextState<GameState>::set(GameState::InGame)`. Systems that require loaded assets must schedule `setup` after the asset loader, e.g. `.after(crate::game::resources::load_assets)`.
  - System stages used: `Startup` (setup + load), `Update` (per-frame logic / input), and `FixedUpdate` (physics/position sync). Follow these conventions when placing new systems.
  - Always run `cargo build` after modifying files to ensure no compilation errors.

- Data flow & integration patterns:
  - Shared, long-lived data is exposed via `#[derive(Resource)]` (e.g., `GlobalTextureAtlas`, `CursorPosition`). Use `Res` / `ResMut` to access these.
  - One-off or decoupled notifications use `Event` types (example: `PlayerDamagedEvent` in `player.rs`).
  - Entities represent in-game objects; components are small marker/data structs (see `src/game/common/components/*`). Systems query components and resources.
  - Sprite atlas usage: `Sprite::from_atlas_image(handle.image.clone(), TextureAtlas { layout: handle.layout.clone(), index: N })`. The atlas layout is built in `resources.rs` using `TextureAtlasLayout::from_grid` with sizes from `config.rs`.

- Conventions and gotchas:
  - Global configuration constants live in `src/game/config.rs`. Prefer adding tunable values there rather than scattering magic numbers.
  - Ordering matters for asset-dependent setup: ensure `setup` systems depend on `resources::load_assets` (see `PlayerPlugin::build`).
  - Use `FixedUpdate` for deterministic position/physics syncing (`sync_position_transform` in `player::controls`).
  - Camera/world conversions: `resources::update_cursor_position` prefers `camera.viewport_to_world` then falls back to orthographic math.

- Developer workflows (discovered / reproducible):
  - Build & run (debug): `cargo run` from the repo root.
  - Run with logs/backtrace for debugging: `RUST_LOG=info RUST_BACKTRACE=1 cargo run` (set `debug` or `trace` as needed).
  - Fast iteration: run in debug for quicker compile/test loops; use `cargo run --release` for performance checks.

- When modifying systems or adding features:
  - Add a new `Plugin` under `src/game/<feature>` and register it in `src/game/game.rs`.
  - If the feature needs textures, use `GlobalTextureAtlas` and index into the atlas rather than loading new independent images unless justified.
  - Respect system stages and ordering: prefer `Startup` for spawns, `Update` for input, `FixedUpdate` for physics.

- Example concrete patterns to copy:
  - Spawn a sprite from atlas:
    `commands.spawn(Sprite::from_atlas_image(handle.image.clone(), TextureAtlas { layout: handle.layout.clone(), index: 2 }))`
  - Delay state transition until assets loaded: `next_state.set(GameState::InGame)` inside the loader.

- Current bevy version: 0.18 (check `Cargo.toml` for exact version and dependencies).

If anything here is unclear or you'd like more detail (examples of adding an enemy plugin, asset pipeline, or debugging tips), tell me which area to expand. I'll update this file accordingly.

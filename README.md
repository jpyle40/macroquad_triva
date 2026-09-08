# Quest Lab

A colorful, space-themed trivia adventure for curious kids around age 9. Written entirely in Rust with Macroquad; all illustrations and animations are drawn in code, so no asset downloads or accounts are needed.

## Play

```sh
cargo run
```

Choose **Pixel Planet**, **Tech Station**, or **Science Galaxy**. Each mission randomly selects eight of that world's sixteen questions and shuffles the answers. Read the explanation after each answer, then continue exploring.

- Click answers, or press **1–4**.
- Press **Enter** to continue after answering.
- Press **Esc** during a mission to open the leave-mission prompt.
- Open **My badges** to see the eight awards and their requirements.
- Resize the window freely; the interface scales while keeping its proportions.

Correct answers earn **25 XP**, other attempts earn **5 XP**, and finishing a mission earns **50 XP**. Every **200 XP** adds a level. There is no timer, penalty, purchase, or online chat. Topic badges count correct answers across missions, including repeat questions; streaks are within a single mission.

## Build requirements

Install a Rust toolchain and a platform C linker. Linux needs a graphical desktop and the native libraries used by Macroquad (X11, OpenGL, and their development packages). On Fedora, a typical development setup is:

```sh
sudo dnf install gcc libX11-devel libXi-devel mesa-libGL-devel
```

This workstation's host currently lacks `cc`; its existing `devbox` Toolbox provides a build environment:

```sh
toolbox run --container devbox cargo run
```

Run that command from this project directory. Other machines with the prerequisites can simply use `cargo run`. The first build may need internet access to download crates. Gameplay works offline.

## Saved progress

XP, topic totals, completed missions, and badges are saved locally after answers and mission completion. An unfinished mission itself is not resumed.

The save file is `quest-lab/progress.txt` below the first available directory from:

1. `QUEST_LAB_SAVE_DIR` (optional override)
2. `LOCALAPPDATA`
3. `XDG_DATA_HOME`
4. `$HOME/.local/share`
5. The current directory

To start a fresh profile, close the game and move that file somewhere safe. A message at the bottom of the window reports save errors.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

`src/questions.rs` contains the 48 questions and explanations. The first answer in each source entry is correct; answer positions are shuffled during play. `src/main.rs` contains the interface, progression, persistence, and tests.

Built with [Macroquad](https://docs.rs/macroquad/0.4.16/macroquad/).

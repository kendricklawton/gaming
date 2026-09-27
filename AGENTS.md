# Gaming — Agent Guidelines

**Stack:** Rust + Bevy engine (v0.18.1) 2.5D Isometric Game. 

## 🚧 Status
- Currently in early prototyping phase.
- Prototyping an isometric orthographic perspective with gamepad and keyboard support.

## 🗺️ Where Things Live
| Path | What |
|---|---|
| `src/` | Primary game logic, ECS systems, and components. |
| `assets/` | (If added) Sprites, models, audio, and font files. |
| `Cargo.toml` | Rust dependencies. |

## 🛠️ Verification & Commands
- **Run by Default After Every Change (Static Checks):** 
  - Rust: `cargo check`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`
- **Run ONLY When Explicitly Asked:**
  - `cargo run` or `cargo build` (unless specifically asked to test visually/functionally).
  - If unprompted to test visually, state that visual/gameplay tuning is untested.

## 🏗️ Architecture & Code Rules
- **Rust & Bevy ECS:** The entire game logic operates via Bevy's Entity Component System architecture.
  - Utilize `App::add_systems` and `Commands` to build game functionality.
  - Rely on Bevy's built-in components (`Transform`, `Mesh3d`, `Camera3d`, `Gamepad`, etc.).
- **Agent Hygiene:**
  - Never modify `AGENTS.md` unless the user asks outright; propose changes as a diff.
  - No obvious comments (document *why*, never *what*). No AI signatures in code or commits.
  - **No Scripted Edits:** Never use Python, shell, Node, or regex scripts to bulk-edit any tracked file: code, comments, docs (Markdown/MDX), SQL, or config (JSON, YAML, TOML). Edit each file deliberately. The only exceptions are the repo's own tools: formatters (`cargo fmt`). Place one-off analysis scripts strictly in untracked `scratch/`.

## 📝 Git Rules (Strict)
- **Commit Only on Command:** Never run `git add` or `git commit` unless explicitly instructed.
- **Format:** Conventional commits (`feat:`, `fix:`, `refactor:`, `chore:`). **Hard limit of 1–5 lines** (1 subject line + optional blank line + max 3 body lines explaining *why*). No file lists, test outputs, or task summaries.
- **No Branches or Pushes:** Never create branches (`checkout -b`, `switch -c`, `branch`, or worktrees)—commit directly to the current branch. Never run `git push`.

## ✅ Definition of Done
Before reporting a change as finished:
- [ ] Static checks pass (`cargo check`).
- [ ] Code is formatted strictly with `cargo fmt`.
- [ ] No `git add` or `git commit` unless explicitly requested.
- [ ] No unapproved dependency changes or external asset additions without asking first.

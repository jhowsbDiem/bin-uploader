# Contributing to bin-uploader

This guide covers how a change moves through the repository: the workflow,
and the commit and comment conventions. The project overview is in the
[README](README.md).

---

## ⚙️ Setting Up

* **Git:** to clone the repository and follow the workflow below.
* **esptool:** to flash the board.
* **Arduino IDE 2.x or arduino-cli with the ESP32 core:** to produce the builds the tool reads.

---

## 🔀 Workflow

1. Every change starts from an issue.
2. A larger feature gets its own branch from `main`: `feature/<name>`.
3. Each of its issues gets a branch from the feature branch:
   `feature/<N>-<subject>` (or `fix/`, `docs/`).
4. A change is committed only after it is verified.
5. The issue branch joins the feature branch by fast-forward; the feature
   branch joins `main` through a pull request.

```
main ─────────────────────────────●──── (pull request)
  └─ feature/<name> ──●──●──●────┘
         └─ feature/<N>-<subject>
```

"Verified" means the automated tests pass for build detection and file
collection, and the board boots the new firmware on the bench for flashing.

---

## 📝 Commit Convention

Conventional Commits, in English:

```
<type>: <imperative description, no final period>

Closes #<issue>
```

* **Types:** `feat`, `fix`, `refactor`, `docs`, `perf`, `test`, `style`, `chore`, with no scope in parentheses.
* **Body:** only when the reason is not obvious from the issue.

Example:

```
feat: detect the latest build of a sketch

Closes #3
```

---

## 💬 Comment Convention

* A comment explains **why**, never what — the code already says what.
* No reference to issues, tasks or conversations: that belongs in the commit.
* No commented-out code: history lives in git.

---

## 📚 Where Each Document Lives

* **[README.md](README.md):** overview and project status.
* **CONTRIBUTING.md:** this file.
* **[LICENSE](LICENSE):** the MIT license.
* **[AGENTS.md](AGENTS.md):** project overview and conventions for AI coding agents.

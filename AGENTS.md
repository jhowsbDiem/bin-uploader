# AGENTS.md

## Project Overview

bin-uploader is a terminal tool, for Windows and Linux, that finds the latest
ESP32 build of an Arduino sketch (Arduino IDE 2.x or arduino-cli), collects
its binaries and flashes them with esptool. It is in the design phase: no
tool code exists yet.

## Conventions

- Commits, code comments and docs: English.
- Commits: Conventional Commits, `<type>: <imperative description>` plus
  `Closes #<issue>`. Types: `feat`, `fix`, `refactor`, `docs`, `perf`,
  `test`, `style`, `chore`, with no scope.
- Branches: `feature/<name>` from `main`, and one branch per issue from it;
  the feature branch joins `main` through a pull request.
- Comments explain why, never what. No commented-out code.
- Line endings: LF everywhere, CRLF for `.bat`, `.cmd` and `.ps1`
  (enforced by `.gitattributes`).

## Documentation Map

- `README.md`: overview and project status.
- `CONTRIBUTING.md`: workflow, commit and comment conventions in detail.
- `LICENSE`: MIT.

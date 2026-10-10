# time-tracker (tmtr)

Minimal time tracker for the terminal: log activities with tags,
see daily and weekly stats, and run a TUI stopwatch

# tmtr

[![Crates.io](https://img.shields.io/crates/v/tmtr.svg)](https://crates.io/crates/tmtr)
[![Downloads](https://img.shields.io/crates/d/tmtr.svg)](https://crates.io/crates/tmtr)
[![License](https://img.shields.io/crates/l/tmtr.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange?logo=rust)](https://www.rust-lang.org)
[![CI](https://github.com/lixtrw/time-tracker/actions/workflows/ci.yml/badge.svg)](https://github.com/lixtrw/time-tracker/actions)

> **Status: alpha.** Commands and the data format may change.

## Features

- Add activities with tags and a duration
- Daily and weekly statistics
- Total time by tag
- Full-screen stopwatch with pause (TUI, built with ratatui)
- Data stored in a JSON file

## Installation

```sh
cargo install tmtr
```

Or from source:

```sh
git clone https://github.com/lixtrw/time-tracker
cd time-tracker
cargo install --path .
```

Rust 1.88+ (edition 2024)

## Usage

### Add an activity

```sh
tmtr add rust programming -t 1h30m   # today

tmtr add reading -t 45m -d y         # -d y - yesterday -d t - today 
                                        # -d 12-tt tt - this mouth this year
                                        # -d 12-01-t 12.01.this year
                                        
tmtr add math -t 2h -d 5-10-2026     # specific date
```

### Stopwatch

```sh
tmtr stop-watch rust programming
```

| Key     | Action             |
|---------|--------------------|
| `Space` | pause / resume     |
| `q`     | stop and save      |

### Statistics

```sh
tmtr stat                  # today
tmtr stat day y            # yesterday
tmtr stat week current     # current calendar week
tmtr stat week rolling     # last 7 days
tmtr stat tag rust         # total time for a tag
```

Example output:

```text
Week[Current]: 2026-10-05 - 2026-10-10
-------------------------------------------------------
Fri 09.10     0h 15m  # reading
Sat 10.10     4h  0m  # programming, rust
-------------------------------------------------------
Total:  4h 15m
```

### Remove an activity

```sh
tmtr remove 0              # by index from `stat`, today
tmtr remove 1 -d y         # yesterday
```

## Formats

**Duration:** `1h15m` with no space

**Date:** `t` / `today`, `y` / `yesterday`, `15-tt` (day 15 of this
month), `5-10-2026`, `5-10-t` (day-month-year)

## Data location

| OS      | Path                                                  |
|---------|-------------------------------------------------------|
| Linux   | `~/.local/share/time_tracker/data.json`                |
| macOS   | `~/Library/Application Support/time_tracker/data.json` |
| Windows | `%LOCALAPPDATA%\time_tracker\data.json`                |

## Roadmap

- [x] MVP prototype
- [x] Saving data
- [x] Stat
- [x] TUI stopwatch
- [ ] Make tui stopwatch more reliable
- [ ] Add tests
- [ ] `stat period`
- [ ] Edit existing activities
- [ ] PomodoroTimer
- [ ] FlowTimer
- [ ] Maybe Tui

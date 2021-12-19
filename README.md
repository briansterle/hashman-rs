![example workflow](https://github.com/briansterle/hashman-rs/actions/workflows/rust.yml/badge.svg)

# Hashman

## What is Hashman

Hashman is manager for NiceHash on Windows. It introduces automation on top of NiceHash that reacts to other GPU
activity on the target rig, for example gaming. Hashman detects when machine resources are idle and automatically starts
mining. If other apps contend with the GPU, Hashman will back off mining until resources are once again idle.

## How to run

#### build for release

```shell
cargo build --release
```

this outputs an exe at `target/release/hashman.exe`. Add it to your PATH for easy access.

#### run the exe:

```shell
hashman-rs.exe {num_loops} {interval_seconds}
```

### usage notes:

* num_loops:          n times the program will wake from sleep and check GPU contention
    * default 1
* interval_seconds:   loop every n seconds
    * default 1

## Configuration

Create your hashpath configuration file at `~/.hashman/hashpath.toml`

This is auto-generated and populated by default when you first run the app, if it does not exist.

template:

```toml
gaming_path = C:\Games\steamapps\common,D:\Games\origin
mining_path = NiceHashMiner.exe,app_nhm.exe
miner_exe = ~\AppData\Local\Programs\NiceHash Miner\NiceHashMiner.exe
```

## Testing

run tests

```shell
cargo test
```

run bench

```shell
cargo bench
```

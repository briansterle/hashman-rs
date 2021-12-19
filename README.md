![example workflow](https://github.com/briansterle/hashman-rs/actions/workflows/rust.yml/badge.svg)

# Hashman

## What is Hashman

Hashman is manager for NiceHash on Windows. It introduces automation on top of NiceHash that reacts to other GPU
activity on the target rig, for example gaming. Hashman detects when machine resources are idle and automatically starts
mining. If other apps contend with the GPU, Hashman will back off mining until resources are once again idle.

## How to run

#### build for release

clone repo. from root run: 

```shell
cargo build --release
```

this outputs an exe at `target/release/hashman.exe`. Add it to your PATH for easy access.

#### run the exe:

```shell
hashman-rs.exe {num_loops} {interval_seconds}
```

### args

* num_loops:          n times the program will wake from sleep and check GPU contention
  * default 21,000,000
* interval_seconds:   loop every n seconds
  * default 21

## Configuration

Create your hashpath configuration file at `~/.hashman/hashpath.toml`

If it does not exist, this file is auto-generated and populated by default when you first run the app.

#### Template

```toml
gaming_path = "C:\\Games\\steamapps\\common,D:\\Games\\origin"
mining_path = "NiceHashMiner.exe,app_nhm.exe"
miner_exe = "~\\AppData\\Local\\Programs\\NiceHash Miner\\NiceHashMiner.exe"
```
#### Schema
* miner_exe: String
   * the fully qualified path to your mining exe. This app is designed for a NiceHashMiner.exe but it should be compatible with other miners. 
* gaming_path: String
   * a comma-separated list of directories and .exe's of games. Directories will be recursively searched for any .exe's within. This does not have actually be a game executable, just any exe that needs exclusive access to the GPU. Gaming is the most common use case so this is the term used throughout hashman.
* mining_path: String
   * a comma-seprated list of executables that are considered "miner" proceseses

## Testing

run tests

```shell
cargo test
```

run bench

```shell
cargo bench
```

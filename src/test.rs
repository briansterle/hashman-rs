#[cfg(test)]
mod tests {
    use crate::{Config, config, GPU, main, Rig, WindowsGPU};
    use crate::mining::Mining;

    #[test]
    fn parses_config() {
        let config: Config = config::json();
        assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    }

    #[test]
    fn mining_kills_all() {
        Mining::kill_all();
    }

    #[test]
    fn mining_gets_state() {
        let conf: Config = config::json();
        let wgpu: WindowsGPU = GPU::new(conf.py_gputil, conf.py_exec);
        let state = Rig::get(&wgpu);
    }

    // #[test]
    // fn app_works() {
    //     main();
    // }
}

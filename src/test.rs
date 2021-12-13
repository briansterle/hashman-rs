#[cfg(test)]
mod tests {
    use crate::{Config, config, GPU, Rig, WindowsGPU};
    use crate::mining::Mining;

    #[test]
    fn parses_config() {
        let config: Config = config::json();
        assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    }

    // #[test]
    // fn mining_kills_all() {
    //     Mining::kill_all();
    // }


    #[test]
    fn gets_tasks() {
        let tasks = Rig::tasks();
        assert!(!tasks.is_empty())
    }
    
    #[test]
    fn rig_gets_state() {
        let conf: Config = config::json();
        let wgpu: WindowsGPU = GPU::new(conf.py_gputil, conf.py_exec);
        let _state = Rig::get_state(&wgpu);
    }

    // #[test]
    // fn app_works() {
    //     main();
    // }
}

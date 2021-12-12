#[cfg(test)]
mod tests {
    use crate::{Config, config, main};
    use crate::mining::Mining;

    #[test]
    fn parse_config() {
        let config: Config = config::json();
        assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    }

    #[test]
    fn stop_mining() {
        Mining::kill_all();
    }

    #[test]
    fn app_works() {
        main();
    }
}

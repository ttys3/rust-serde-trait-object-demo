use crate::config::Config;

mod config;

fn main() {
    let cfg_default = Config::default();
    println!("default:\n{}", cfg_default.to_string().unwrap());

    let mut cfg = Config::from_str().unwrap();
    println!("cfg_str:\n{}", cfg.to_string().unwrap());

    println!("cfg.provider: {}", cfg.provider());

    println!("current provider token: {:?}", cfg.current_provider_config().unwrap().token());

    let pcfg = cfg.current_provider_config().unwrap();
    pcfg.to_owned().set_token("new_token".to_string());

    cfg.provider_config.set(&cfg.provider(), pcfg.to_owned());
    println!("cfg_str updated:\n{}", cfg.to_string().unwrap());
    println!("current provider token updated: {:?}", cfg.current_provider_config().unwrap().token());
}

mod config_trait_object;
mod config_enum;
mod config_struct_boxed_trait;
mod config_struct;

fn main() {
    // config_trait_object impl
    println!("config_trait_object impl --------------------------------------------------");
    let cfg_default = config_trait_object::Config::default();
    println!("default:\n{}", cfg_default.to_string().unwrap());

    let mut cfg = config_trait_object::Config::from_str().unwrap();
    println!("cfg_str:\n{}", cfg.to_string().unwrap());

    println!("cfg.provider: {}", cfg.provider());

    println!("current provider token: {:?}", cfg.current_provider_config().unwrap().token());

    let pcfg = cfg.current_provider_config().unwrap();

    // update not work because to_owned object free after used
    let mut pcfg = pcfg.to_owned();
    pcfg.set_token("new_token".to_string());

    cfg.provider_config.set(&cfg.provider(), pcfg);
    println!("cfg_str updated:\n{}", cfg.to_string().unwrap());
    println!("current provider token updated: {:?}", cfg.current_provider_config().unwrap().token());

    // enum impl
    println!("enum impl --------------------------------------------------");

    let cfg_default = config_enum::Config::default();
    println!("default:\n{}", cfg_default.to_string().unwrap());

    let mut cfg = config_enum::Config::from_str().unwrap();
    println!("cfg_str:\n{}", cfg.to_string().unwrap());

    println!("cfg.provider: {}", cfg.provider());

    println!("current provider config: {:?}", cfg.current_provider_config().unwrap());

    // config_struct_boxed_trait impl
    println!("config_struct_boxed_trait impl --------------------------------------------------");

    let cfg_default = config_struct_boxed_trait::Config::default();
    println!("default:\n{}", cfg_default.to_string().unwrap());

    let mut cfg = config_struct_boxed_trait::Config::from_str().unwrap();
    println!("cfg_str:\n{}", cfg.to_string().unwrap());

    println!("cfg.provider: {}", cfg.provider());

    println!("current provider config: {:?}", cfg.current_provider_config().unwrap().as_any().downcast_ref::<config_struct_boxed_trait::OpenaiMobile>().unwrap());

    println!("config_struct impl --------------------------------------------------");
    let cfg_default = config_struct::Config::default();
    println!("default:\n{}", cfg_default.to_string().unwrap());

    let mut cfg = config_struct::Config::from_str().unwrap();
    println!("cfg_str:\n{}", cfg.to_string().unwrap());

    println!("cfg.provider: {}", cfg.provider());

    println!("current provider token: {:?}", cfg.current_provider_config().unwrap().token);

    let mut pcfg = cfg.current_provider_config().unwrap();
    pcfg.token = Some("new_token".to_string());

    cfg.provider_config.insert(cfg.provider(), pcfg);
    println!("cfg_str updated:\n{}", cfg.to_string().unwrap());
    println!("current provider token updated: {:?}", cfg.current_provider_config().unwrap().token);
}

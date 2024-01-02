use std::env;
use common::prelude::{EnvironmentType, MessageClientConfig, ServiceConfig, ServiceID};
use config_manager::ConfigManager;
use db_specs::prelude::get_local_db_config;

#[test]
fn test_get_svc_id() {
    env::set_var("ENV", "Local");

    let svc_id = ServiceID::Default;
    let config = ConfigManager::new(svc_id);
    assert_eq!(svc_id, config.get_svc_id());
}

#[test]
fn test_get_env_type() {
    env::set_var("ENV", "Local");

    let svc_id = ServiceID::Default;
    let config = ConfigManager::new(svc_id);
    assert_eq!(EnvironmentType::Local, config.get_env_type());
}

#[test]
fn test_get_db_config() {
    env::set_var("ENV", "Local");

    let svc_id = ServiceID::Default;
    let config = ConfigManager::new(svc_id);
    let db_config = get_local_db_config();

    assert_eq!(&db_config, config.get_db_config());
}

#[test]
fn test_get_svc_config() {
    env::set_var("ENV", "Local");

    let svc_id = ServiceID::Default;
    let config = ConfigManager::new(svc_id);

    let svc_config = ServiceConfig::default();
    assert_eq!(&svc_config, config.get_svc_config());
}

#[test]
fn test_get_message_client_config() {
    env::set_var("ENV", "Local");

    let svc_id = ServiceID::default();
    let config = ConfigManager::new(svc_id);
    let expected = MessageClientConfig::from_svc_id(svc_id);
    assert_eq!(expected, config.get_message_client_config());
}

#[test]
fn test_get_svc_metric_config() {
    env::set_var("ENV", "Local");

    let svc_config = ServiceConfig::default();
    let config = ConfigManager::new(ServiceID::default());
    let expected = svc_config.metrics().to_owned();
    assert_eq!(expected, config.get_svc_metric_config());
}



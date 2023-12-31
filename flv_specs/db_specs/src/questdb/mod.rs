use common::prelude::DBConfig;

const PORT: u16 = 9009;

pub fn get_local_db_config() -> DBConfig {
    DBConfig::new(PORT, "0.0.0.0".into())
}

pub fn get_cluster_db_config() -> DBConfig {
    DBConfig::new(PORT, "questdb.default.svc.cluster.local".into())
}

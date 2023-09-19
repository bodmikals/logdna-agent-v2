macro_rules! define_env_var {
    ($name:ident) => {
        pub const $name: &str = concat!("MZ_", stringify!($name));
    };
}

macro_rules! define_env_vars {
    ($($name:ident),+ $(,)? ) => {
        $(
            define_env_var!($name);
        )*
        pub const ENV_VARS_LIST: &'static [&'static str] = &[$($name),*];
    };
}

// env vars names prefixed with "MZ_"
define_env_vars!(
    INGESTION_KEY,
    CONFIG_FILE,
    LOG_DIRS,
    TAGS,
    HOST,
    ENDPOINT,
    USE_SSL,
    USE_COMPRESSION,
    GZIP_LEVEL,
    EXCLUSION_RULES,
    EXCLUSION_REGEX_RULES,
    INCLUSION_RULES,
    INCLUSION_REGEX_RULES,
    K8S_METADATA_LINE_INCLUSION,
    K8S_METADATA_LINE_EXCLUSION,
    HOSTNAME,
    IP,
    MAC,
    SYSTEMD_JOURNAL_TAILER,
    JOURNALD_PATHS,
    LOOKBACK,
    DB_PATH,
    METRICS_PORT,
    USE_K8S_LOG_ENRICHMENT,
    LOG_K8S_EVENTS,
    LOG_METRIC_SERVER_STATS,
    K8S_STARTUP_LEASE,
    LINE_EXCLUSION_REGEX,
    LINE_INCLUSION_REGEX,
    REDACT_REGEX,
    INGEST_TIMEOUT,
    INGEST_BUFFER_SIZE,
    RETRY_DIR,
    RETRY_DISK_LIMIT,
    INTERNAL_FS_DELAY,
    CLEAR_CACHE_INTERVAL,
    METADATA_RETRY_DELAY,
    META_APP,
    META_HOST,
    META_ENV,
    META_FILE,
    META_K8S_FILE,
    META_JSON,
    META_ANNOTATIONS,
    META_LABELS,
    NO_CAP,
    MOCK_NO_PODS
);

// unused or deprecated
pub const INGESTION_KEY_ALTERNATE: &str = "LOGDNA_AGENT_KEY";
pub const CONFIG_FILE_DEPRECATED: &str = "DEFAULT_CONF_FILE";
pub const HOST_DEPRECATED: &str = "LDLOGHOST";
pub const IBM_HOST_DEPRECATED: &str = "LOGDNA_LOGHOST";
pub const ENDPOINT_DEPRECATED: &str = "LDLOGPATH";
pub const USE_SSL_DEPRECATED: &str = "LDLOGSSL";
pub const USE_COMPRESSION_DEPRECATED: &str = "COMPRESS";
pub const GZIP_LEVEL_DEPRECATED: &str = "GZIP_COMPRESS_LEVEL";
pub const LOG_DIRS_DEPRECATED: &str = "LOG_DIRS";
pub const EXCLUSION_RULES_DEPRECATED: &str = "LOGDNA_EXCLUDE";
pub const EXCLUSION_REGEX_RULES_DEPRECATED: &str = "LOGDNA_EXCLUDE_REGEX";
pub const INCLUSION_RULES_DEPRECATED: &str = "LOGDNA_INCLUDE";
pub const INCLUSION_REGEX_RULES_DEPRECATED: &str = "LOGDNA_INCLUDE_REGEX";

pub(crate) const DEPRECATED_ENV_VARS_LIST: &[&str] = &[
    INGESTION_KEY_ALTERNATE,
    CONFIG_FILE_DEPRECATED,
    HOST_DEPRECATED,
    IBM_HOST_DEPRECATED,
    ENDPOINT_DEPRECATED,
    USE_SSL_DEPRECATED,
    GZIP_LEVEL_DEPRECATED,
    LOG_DIRS_DEPRECATED,
    EXCLUSION_RULES_DEPRECATED,
    EXCLUSION_REGEX_RULES_DEPRECATED,
    INCLUSION_RULES_DEPRECATED,
    INCLUSION_REGEX_RULES_DEPRECATED,
];

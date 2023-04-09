CREATE TABLE IF NOT EXISTS stats
(
    id                uuid                   NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_identifier character varying(255) NOT NULL,
    total_memory      bigint                 NOT NULL,
    used_memory       bigint                 NOT NULL,
    total_swap        bigint                 NOT NULL,
    used_swap         bigint                 NOT NULL,
    uptime            bigint                 NOT NULL,
    timestamp         character varying(255) NOT NULL
);

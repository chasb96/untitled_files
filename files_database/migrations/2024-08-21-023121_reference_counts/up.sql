CREATE TABLE reference_counts (
    file_id VARCHAR(16) NOT NULL PRIMARY KEY,
    count SMALLINT NOT NULL,
    expiry BIGINT NOT NULL
);
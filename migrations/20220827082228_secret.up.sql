-- Add up migration script here
CREATE TABLE `secret` (
    `id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'pk_id',
    `name` VARCHAR(255) NOT NULL COMMENT 'secret name',
    `user_id` BIGINT(20) UNSIGNED NOT NULL COMMENT 'user_id',
    `access_key` CHAR(24) NOT NULL COMMENT 'user AccessKey',
    `secret_access_key` CHAR(64) NOT NULL COMMENT 'user SecretAccessKey',
    `expire` BIGINT(20) NOT NULL DEFAULT '0' COMMENT 'expiration time',
    `deleted` BIGINT(20) UNSIGNED NOT NULL DEFAULT '0' COMMENT 'soft delete flag',
    `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT 'create time',
    `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT 'update time',
    `deleted_at` DATETIME(3) NULL DEFAULT NULL COMMENT 'delete time',
    PRIMARY KEY (`id`),
    INDEX `idx_user_id` (`user_id`) USING BTREE,
    UNIQUE `idx_access_key_deleted` (`access_key`, `deleted`) USING BTREE,
    INDEX `idx_deleted` (`deleted`) USING BTREE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_bin COMMENT = 'secret info';
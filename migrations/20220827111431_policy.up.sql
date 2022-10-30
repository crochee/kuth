-- Add up migration script here
CREATE TABLE `policy` (
    `id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `scope` VARCHAR(255) NOT NULL,
    `kind` VARCHAR(255) NOT NULL COMMENT 'kind enum system custom',
    `desc` VARCHAR(255) NOT NULL COMMENT 'policy description',
    `content` VARCHAR(255) NOT NULL COMMENT 'policy content',
    `deleted` BIGINT(20) UNSIGNED NOT NULL DEFAULT '0' COMMENT 'soft delete flag',
    `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT 'create time',
    `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT 'update time',
    `deleted_at` DATETIME(3) NULL DEFAULT NULL COMMENT 'delete time',
    PRIMARY KEY (`id`),
    CONSTRAINT `content` CHECK (json_valid(`content`)),
    INDEX `idx_deleted` (`deleted`) USING BTREE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_bin COMMENT = 'policy info';
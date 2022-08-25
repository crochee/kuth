-- Add up migration script here
CREATE TABLE `policy` (
    `id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'role_id',
    `desc` VARCHAR(255) NOT NULL COMMENT 'role description',
    `version` VARCHAR(255) NOT NULL COMMENT 'version',
    `policy_type` TINYINT(3) UNSIGNED NOT NULL DEFAULT '1' COMMENT '1.custom 2.system',
    `subjects` TEXT NOT NULL DEFAULT '[]' COMMENT 'subject array',
    `effect` VARCHAR(6) NOT NULL DEFAULT 'Deny' COMMENT 'Allow,Deny',
    `action` TEXT NOT NULL DEFAULT '[]' COMMENT 'example,iam:projects:get',
    `resources` TEXT NOT NULL DEFAULT '[]' COMMENT 'resource array',
    `collections` TEXT NOT NULL DEFAULT '[]' COMMENT 'collection array',
    `deleted` BIGINT(20) UNSIGNED NOT NULL DEFAULT '0' COMMENT 'soft delete flag',
    `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT 'create time',
    `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT 'update time',
    `deleted_at` DATETIME(3) NULL DEFAULT NULL COMMENT 'delete time',
    PRIMARY KEY (`id`),
    CONSTRAINT `subjects` CHECK (json_valid(`subjects`)),
    CONSTRAINT `action` CHECK (json_valid(`action`)),
    CONSTRAINT `resources` CHECK (json_valid(`resources`)),
    CONSTRAINT `collections` CHECK (json_valid(`collections`)),
    INDEX `idx_deleted` (`deleted`) USING BTREE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_bin COMMENT = 'policy info';
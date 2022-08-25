-- Add up migration script here
CREATE TABLE `bind` (
    `id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'bind_id',
    `group_id` BIGINT(20) UNSIGNED NOT NULL COMMENT 'group_id',
    `bind_type` TINYINT(3) UNSIGNED NOT NULL DEFAULT '1' COMMENT '1.user 2.policy',
    `object_id` BIGINT(20) UNSIGNED NOT NULL COMMENT 'user id or policy id',
    `deleted` BIGINT(20) UNSIGNED NOT NULL DEFAULT '0' COMMENT 'soft delete flag',
    `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT 'create time',
    `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT 'update time',
    `deleted_at` DATETIME(3) NULL DEFAULT NULL COMMENT 'delete time',
    PRIMARY KEY (`id`),
    UNIQUE `idx_group_id_bind_type_object_id_deleted` (`group_id`, `bind_type`, `object_id`, `deleted`) USING BTREE,
    INDEX `idx_deleted` (`deleted`) USING BTREE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_bin COMMENT = 'bind info';
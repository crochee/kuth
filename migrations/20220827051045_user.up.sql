-- Add up migration script here
CREATE TABLE `user` (
    `id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'user id',
    `account_id` BIGINT(20) UNSIGNED NOT NULL COMMENT 'account id',
    `admin` TINYINT(3) UNSIGNED NOT NULL DEFAULT '1' COMMENT '1.user not admin 2.user is admin',
    `name` VARCHAR(255) NOT NULL COMMENT 'user name',
    `desc` VARCHAR(255) NOT NULL COMMENT 'user description,admin,develop',
    `email` VARCHAR(255) NULL DEFAULT NULL COMMENT 'user email',
    `check` TINYINT(3) UNSIGNED NOT NULL DEFAULT '1' COMMENT '1.email not checked 2.checked email',
    `sex` VARCHAR(6) NULL DEFAULT NULL COMMENT 'user sex',
    `image` VARCHAR(255) NULL DEFAULT NULL COMMENT 'user image',
    `password` TEXT(457) NOT NULL COMMENT 'user password',
    `deleted` BIGINT(20) UNSIGNED NOT NULL DEFAULT '0' COMMENT 'soft delete flag',
    `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT 'create time',
    `updated_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3) COMMENT 'update time',
    `deleted_at` DATETIME(3) NULL DEFAULT NULL COMMENT 'delete time',
    PRIMARY KEY (`id`),
    INDEX `idx_account_id_admin_deleted` (`account_id`, `admin`, `deleted`) USING BTREE,
    INDEX `idx_deleted` (`deleted`) USING BTREE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_bin COMMENT = 'user info';
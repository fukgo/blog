create database blog;
create database auth;
use auth;
CREATE table auth_user(
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE ,
    password VARCHAR(20) UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
use blog;
SET GLOBAL event_scheduler = ON;


CREATE EVENT create_new_blog_table
ON SCHEDULE EVERY 1 MONTH
STARTS '2024-11-01 00:00:00'  -- 设置事件开始的时间为下个月的第一天
DO
BEGIN
    SET @current_month = DATE_FORMAT(NOW(), '%Y_%m');
    SET @table_name = CONCAT('blogs_table_', @current_month);

    SET @sql = CONCAT('CREATE TABLE IF NOT EXISTS ', @table_name, ' (
        id INT AUTO_INCREMENT PRIMARY KEY,
        title VARCHAR(50) UNIQUE NOT NULL,
        content TEXT NOT NULL,
        user_id INT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES user_table(id)
    )');

    PREPARE stmt FROM @sql;
    EXECUTE stmt;
    DEALLOCATE PREPARE stmt;
END;

CREATE table user_table(
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE ,
    email VARCHAR(250) NOT NULL UNIQUE,
    nickname VARCHAR(20) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE tags_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    tag VARCHAR(10) UNIQUE NOT NULL
);

CREATE TABLE blogs_table_2024_10 (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(20) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    digest VARCHAR(30) NOT NULL ,
    user_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP

);

CREATE TABLE blog_tags_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    blog_id INT,
    tag_id INT,
    UNIQUE (blog_id, tag_id)

);


create table resume(
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT,
    content text not null
);
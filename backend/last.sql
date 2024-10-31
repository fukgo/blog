CREATE table user_table(
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    INDEX (username)
);

create table user_detail_table(
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT not null ,
    nickname VARCHAR(20) UNIQUE,
    avatar varchar(255),
    skills varchar(255), -- python,c,c++,golang
    bio text,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX (nickname)
);

CREATE TABLE article_catalogues_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    catalogue_id INT not null ,
    article_id INT not null,
    sort_order INT NOT NULL DEFAULT 0  -- 添加顺序字段
);


create table catalogues_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_detail_id INT,
    catalogue VARCHAR(50) UNIQUE NOT NULL,
    info VARCHAR(255),
    index (user_detail_id)
);


CREATE TABLE tags_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    tag VARCHAR(10) UNIQUE NOT NULL
);

CREATE TABLE articles_table_2024_10 (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(50) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    digest VARCHAR(100) NOT NULL ,
    user_detail_id INT,
    feature bool default false, -- 精品文章
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE article_tags_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    article_id INT not null ,
    tag_id INT not null
);


create table resume_table(
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_detail_id INT,
    content text not null
);

CREATE TABLE comments_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    guest varchar(20) not null, -- 游客名称
    article_id INT NOT NULL,
    parent_id INT NULL,  -- 父评论 ID，如果是顶级评论则为 NULL
    comment VARCHAR(255) NOT NULL,  -- 评论内容
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 评论时间
    depth INT  -- 评论层级
);
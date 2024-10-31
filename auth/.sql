CREATE table auth_user(
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE ,
    password VARCHAR(100) UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

create table site_list(
id INT AUTO_INCREMENT PRIMARY KEY,
    site_title varchar(20) not null ,
    catalogue_id INT not null ,
    site_info varchar(255),
    site_url varchar(255)

);

create table site_catalogues(
    id INT AUTO_INCREMENT PRIMARY KEY,
    category_name varchar(20) not null
    );

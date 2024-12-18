CREATE DATABASE `actix-web`;
USE `actix-web`;
CREATE SCHEMA `actix-web`
CREATE TABLE todos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    description VARCHAR(512),
    status ENUM('new', 'completed') DEFAULT 'new',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

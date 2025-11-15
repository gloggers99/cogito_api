drop table if exists users;

create table users
(
    user_id         serial primary key not null,
    user_name       text not null,
    user_pass       text not null,
    user_last_login timestamptz not null default current_timestamp,
    login_id        uuid default null unique,
    admin           boolean not null default false
);

alter table users
    owner to postgres;

insert into users (user_name, user_pass) values ('mike', 'sherm')

drop table if exists users;

create table users
(
    user_id         serial primary key not null,
    user_email      text unique not null,
    user_phone      text unique not null,
    user_name       text not null,
    user_pass       text not null,
    user_last_login timestamptz not null default current_timestamp,
    login_id        uuid default null unique,
    verified        boolean not null default false,
    admin           boolean not null default false
);

alter table users
    owner to postgres;

--insert into users (user_email, user_phone, user_name, user_pass)
--values ('sherminator@gmail.com', '9162761362', 'mike', 'sherm');

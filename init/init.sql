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

create table conversations (
    conversation_id    serial primary key not null,
    user_id            integer references users(user_id) on delete cascade,

    conversation       json not null,
    conversation_title text not null default 'New Conversation',

    created_at         timestamptz not null default current_timestamp

);

-- Allow indexing by user_id for fetching all user convos.
create index idx_conversations_user_id on conversations(user_id);

alter table users
    owner to postgres;

alter table conversations
    owner to postgres;

--insert into users (user_email, user_phone, user_name, user_pass)
--values ('sherminator@gmail.com', '9162761362', 'mike', 'sherm');
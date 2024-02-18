-- Your SQL goes here
create table IF NOT EXISTS file_results
(
    id text not null
        constraint file_results_pk
            primary key,
    result   text not null
);

create unique index IF NOT EXISTS file_results_id_uindex
    on file_results (id);

#[allow(unused_macros)]
macro_rules! setup_pg_test_pool {
    ($pool:ident) => {
        use testcontainers::Docker;
        use testcontainers::images::postgres::Postgres;
        use testcontainers::clients::Cli;
        use reciclopedia::db;

        let cli = Cli::default();
        let container = cli.run(Postgres::default());

        let db_url = format!(
            "postgres://postgres:postgres@0.0.0.0:{}",
            container.get_host_port(5432).unwrap()
        );

        let $pool = db::get_conn_pool(db_url);
    }
}

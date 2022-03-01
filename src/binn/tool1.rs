use std::env;

fn main(){
    connect();
}

fn connect() -> redis::Connection {
    //format - host:port
    let redis_host_name =
    "matchredis.redis.cache.windows.net:6380";
    
    let redis_password = "3mMecS96MV1vta1OtZzPvdNrgACwGA125AzCaCWkEME=";
    //if Redis server needs secure connection
    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}


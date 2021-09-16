use etcd_client::{Client, Error};
pub mod setting;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = connect().await?;
    let mut etcd = EtcdClient {
        client
    };

    let greeting = etcd.get_value("greeting").await?;
    let nope = etcd.get_value("nope").await?;
    println!("{}", greeting);
    println!("{}", nope);

    Ok(())
}

async fn connect() -> Result<Client, Error> {
    let client = Client::connect(setting::ETCD_ENDPOINTS, None).await?;
    Ok(client)
}

struct EtcdClient {
    client: Client
}

impl EtcdClient {
    async fn get_value(&mut self, key: &str) -> Result<String, Error> {
        let resp = self.client.get(key, None).await?;
        let list = resp.kvs();
        let first = list.first();
        let val = match first {
            Some(v) => v.value_str(),
            None => Ok("Found nothing!"),
        };
        val.map(String::from)
    }   
}

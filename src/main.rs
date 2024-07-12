use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut etcd = etcd_client::Client::connect(["localhost:2379"], None).await?;

    let _ = etcd
    .put(
        "/playground",
        "",
        None,
    )
    .await?;

    let _ = etcd
    .put(
        "/playground/loona",
        "favourite",
        None
    )
    .await?;

    let _ = etcd
    .put(
        "/playground/gyubin",
        "reallylikeyou",
        None,
    )
    .await?;

    let resulting_values = etcd.get("/playground", Some(etcd_client::GetOptions::new().with_prefix())).await.unwrap().take_kvs();

    println!("{:#?}", resulting_values);

    for a in resulting_values.iter() {
        println!("{}, {}", a.key_str().unwrap(), a.value_str().unwrap());
    }

    Ok(())
}


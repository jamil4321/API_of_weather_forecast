pub mod api{
    pub async fn callapi(url:&str) -> Result<std::collections::HashMap<String,String>,Box<dyn std::error::Error>>{
    let get = reqwest::get(url)
    .await?
    .json::<std::collections::HashMap<String,String>>()
    .await?;

       Ok(get)
    }
}
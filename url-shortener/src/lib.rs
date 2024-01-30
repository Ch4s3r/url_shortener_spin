use serde_json::json;
use spin_sdk::{
    http::{IntoResponse, Method, Request, Response},
    http_component,
    key_value::Store,
};

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    let store = Store::open_default()?;
    return match req.method() {
        Method::Post => {
            store.set(req.path(), req.body())?;
            // println!(
            //     "Storing value in the KV store with {:?} as the key",
            //     req.path()
            // );
            let payload = json!({"path": req.uri().to_string()});
            println!("Returning payload {:?}", payload);
            Ok(Response::builder()
                .status(201)
                .body(payload.to_string())
                .build())
        }
        Method::Get => match store.get(req.path())? {
            Some(value) => {
                println!("Found value for the key {:?}", req.path());
                let redirect_url = String::from_utf8_lossy(&value).to_string();
                Ok(Response::builder()
                    .status(301)
                    .header("Location", redirect_url)
                    .build())
            }
            None => {
                println!("No value found for the key {:?}", req.path());
                Ok(Response::builder().status(404).build())
            }
        },
        _ => Ok(Response::builder().status(405).build()),
    };
}

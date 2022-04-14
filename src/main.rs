use lazy_static::lazy_static;
use openpgp::{
    armor::{Kind, Reader, ReaderMode},
    Message,
};
use sequoia_openpgp as openpgp;

use hashbrown::HashMap;
use sha1::{Digest, Sha1};
use std::{fs, io::Read, str::FromStr};
use zbase32::encode_full_bytes;

pub const BASE_PATH: &str = "/public_pgp_keys";

use hyper::{
    header::{HeaderName, HeaderValue},
    service::{make_service_fn, service_fn},
    HeaderMap,
};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

lazy_static! {
    pub static ref HEADER_MAP: HeaderMap = create_header_map();
    pub static ref PGP_KEYS: HashMap<String, Vec<u8>> = read_pgp_keys();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // reference variables declared with lazy_static because they are initialized on first access

    let _ = &HEADER_MAP.len();
    let _ = &PGP_KEYS.len();

    let addr = ([0, 0, 0, 0], 80).into();
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let pgp_keys = &PGP_KEYS;
    let uri = req.uri();
    let path = uri.path();

    let req_local_part = uri.query().unwrap_or("");
    let req_local_part = req_local_part.replace("l=", "");
    let req_local_part_encoded = path.rsplit('/').next().unwrap_or("");

    let path_replaced = path.replace("/.well-known/openpgpkey/", "");

    let req_domain = if path_replaced.starts_with("hu") {
        // simple method
        if req.uri().host().is_some() {
            req.uri().host().unwrap()
        } else {
            req.headers()
                .get(HeaderName::from_static("X-Forwarded-Host"))
                .unwrap()
                .to_str()
                .unwrap()
        }
    } else {
        // advanced method
        path_replaced.split("/hu").next().unwrap_or("")
    };

    let pgp_key_name = format!(
        "{} {} {}",
        req_local_part, req_local_part_encoded, req_domain
    );
    println!("{}", pgp_key_name);

    if pgp_keys.contains_key(&pgp_key_name) {
        let key = pgp_keys.get(&pgp_key_name).unwrap();
        let mut res = Response::new(Body::from(key.clone()));
        res.headers_mut().extend(HEADER_MAP.clone());
        Ok(res)
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("Not found"))
            .unwrap())
    }
}

fn read_pgp_keys() -> HashMap<String, Vec<u8>> {
    let mut pgp_keys = HashMap::new();
    let dir = fs::read_dir(BASE_PATH).expect("Could not read directory");
    for entry in dir {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        let (local_part, domain) = file_name.split_at(file_name.find('@').unwrap());
        let domain = domain.replace('@', "");
        let local_part_encoded = local_part_to_encoded_string(local_part);

        let key_str = fs::read_to_string(&path).expect("Could not read file");
        let key = armored_string_to_binary(&key_str);

        pgp_keys.insert(
            format!("{} {} {}", local_part, local_part_encoded, domain),
            key,
        );
    }
    println!("{:?}", pgp_keys);
    pgp_keys
}

pub fn create_header_map() -> HeaderMap<HeaderValue> {
    let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();
    headers.insert(
        HeaderName::from_str("content-type").unwrap(),
        HeaderValue::from_str("application/octet-stream").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("Access-Control-Allow-Origin").unwrap(),
        HeaderValue::from_str("*").unwrap(),
    );

    headers
}

fn local_part_to_encoded_string(local_part: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(local_part.as_bytes());
    let hashed = hasher.finalize();
    encode_full_bytes(&hashed)
}

fn armored_string_to_binary(armored_string: &str) -> Vec<u8> {
    let mut reader = Reader::new(armored_string.as_bytes(), ReaderMode::VeryTolerant);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("Could not read key");
    buf
}

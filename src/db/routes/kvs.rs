use crate::db::server as db;
use crate::db::utils::request as req;
use crate::db::utils::response as res;
use std::collections::HashMap;

pub fn handler(server: &mut db::Server, request: &req::Request) -> res::Response<String> {
    match request.method.as_str() {
        "get" => get_key(server, request.route.clone()),
        "post" => post(server, request.body.clone()),
        "delete" => delete(server, request.route.clone()),
        _ => res::get_not_allowed_response(),
    }
}

#[allow(clippy::single_char_pattern)]
fn get_key(server: &db::Server, route: String) -> res::Response<String> {
    // Get the key from the route.
    let route_parts: Vec<&str> = route.split("/").collect();
    let key = route_parts.last().unwrap().to_string();

    // If key is empty, return 400 with error message.
    if key.is_empty() || route_parts.len() < 3 {
        let mut map = HashMap::new();
        map.insert("error", "The key is required.");
        return res::create_response(400, Some(map));
    }

    // Get the value from the key-value store.
    let value = server.get(key.clone());

    // If value is None, return 404 with error message.
    if value.is_none() {
        let mut map = HashMap::new();
        let msg = "The value is not found.";
        map.insert("error", msg);
        return res::create_response(404, Some(map));
    }

    // Serialize value as string for the response.
    let body = {
        let _val: db::Value = value.unwrap();
        serde_json::to_string(&_val).unwrap()
    };

    res::Response::builder().status(200).body(body).unwrap()
}

fn post(server: &mut db::Server, request_body: req::RequestBody) -> res::Response<String> {
    // If request body is missing key or value.
    if request_body.get("key").is_none() || request_body.get("value").is_none() {
        let mut map = HashMap::new();
        map.insert("error", "Both key and value are required.");
        return res::create_response(400, Some(map));
    }

    // Get the key from request body.
    // Validate that key is string.
    let key: String = match request_body["key"].as_str() {
        Some(key) => key.to_string(),
        None => {
            let mut map = HashMap::new();
            map.insert("error", "The key must be a string.");
            return res::create_response(400, Some(map));
        }
    };

    // Get the value from request body.
    // Validate that value is a Value struct.
    let value: db::Value = match serde_json::from_value(request_body["value"].clone()) {
        Ok(value) => value,
        Err(_) => {
            let mut map = HashMap::new();
            let msg = "The value provided is invalid.";
            map.insert("error", msg);
            return res::create_response(400, Some(map));
        }
    };

    // Insert the key-value pair into the key-value store.
    let result = server.set(key, value);

    if result.is_err() {
        let message = result.err().unwrap().to_owned();
        let mut _map = HashMap::new();
        _map.insert("error", message.as_str());
        let _body = serde_json::to_string(&_map).unwrap();
        return res::Response::builder().status(400).body(_body).unwrap();
    }

    // Serialize value as string for the response.
    let body = {
        let _val: db::Value = serde_json::from_value(request_body["value"].clone()).unwrap();
        serde_json::to_string(&_val).unwrap()
    };

    res::Response::builder().status(201).body(body).unwrap()
}

fn delete(server: &db::Server, route: String) -> res::Response<String> {
    let route_parts: Vec<&str> = route.split('/').collect();
    let key = route_parts.last().unwrap().to_string();

    if key.is_empty() || route_parts.len() < 3 {
        let mut _map = HashMap::new();
        _map.insert("error", "The key is missing.");
        return res::create_response(400, Some(_map));
    }

    server.delete(key.clone());

    res::create_response(204, None)
}

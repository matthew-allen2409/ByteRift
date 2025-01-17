use std::collections::HashMap;
use crate::router::path_node::{ PathNode, Handler };
use crate::HttpMethod;
use crate::request::Request;
use crate::response::{ Response, StatusLine };

fn handler<String>(
    _path: Vec<String>,
    _request: &Request,
    _state: &String,
) -> Response {
    let status_line = StatusLine::new(200, "OK".to_string());
    Response {
        status_line,
        headers: Vec::new(),
        body: None,
    }
}

fn handler_two<String>(
    _path: Vec<String>,
    _request: &Request,
    _state: &String,
) -> Response {
    let status_line = StatusLine::new(200, "OK".to_string());
    Response {
        status_line,
        headers: Vec::new(),
        body: None,
    }
}

#[test]
fn path_node_new_expect() {
    let expected: PathNode<String> = PathNode {
        children: HashMap::new(),
        handlers: HashMap::new(),
    };

    let result: PathNode<String> = PathNode::new();

    assert_eq!(expected, result);
}

#[test]
fn path_node_add_route() {
    let mut handlers: HashMap<HttpMethod, Handler<String>> = HashMap::new();
    handlers.insert(HttpMethod::GET, handler);

    let expected = PathNode {
        children: HashMap::new(),
        handlers,
    };

    let mut result = PathNode::new();

    result.add_route(
        HttpMethod::GET,
        vec![].into(),
        handler,
    );

    assert_eq!(expected, result);
}

#[test]
fn path_node_add_route_leaf() {
    let mut handlers: HashMap<HttpMethod, Handler<String>> = HashMap::new();
    handlers.insert(HttpMethod::GET, handler);

    let bar = PathNode {
        children: HashMap::new(),
        handlers,
    };

    let foo = PathNode {
        children: HashMap::from([("bar".into(), bar)]),
        handlers: HashMap::new(),
    };

    let expected = PathNode {
        children: HashMap::from([("foo".into(), foo)]),
        handlers: HashMap::new(),
    };


    let mut result = PathNode::new();

    result.add_route(
        HttpMethod::GET,
        vec!["foo".into(), "bar".into()].into(),
        handler,
    );

    assert_eq!(expected, result);
}

#[test]
fn path_node_add_route_mid() {
    let mut handlers: HashMap<HttpMethod, Handler<String>> = HashMap::new();
    handlers.insert(HttpMethod::GET, handler);

    let mut handlers_two: HashMap<HttpMethod, Handler<String>> = HashMap::new();
    handlers_two.insert(HttpMethod::POST, handler_two);

    let bar = PathNode {
        children: HashMap::new(),
        handlers: handlers_two,
    };

    let foo = PathNode {
        children: HashMap::from([("bar".into(), bar)]),
        handlers,
    };

    let expected = PathNode {
        children: HashMap::from([("foo".into(), foo)]),
        handlers: HashMap::new(),
    };


    let mut result = PathNode::new();

    result.add_route(
        HttpMethod::GET,
        vec!["foo".into()].into(),
        handler,
    );

    result.add_route(
        HttpMethod::POST,
        vec!["foo".into(), "bar".into()].into(),
        handler_two,
    );

    assert_eq!(expected, result);
}

#[test]
fn path_node_handle_expect() {
    let mut tree: PathNode<String> = PathNode::new();
    tree.add_route(HttpMethod::GET, vec!["foo".into(), "bar".into()].into(), handler);

    let expected: (Option<Handler<String>>, Vec<String>) = (Some(handler), vec![]);

    let result = tree.handle(&HttpMethod::GET, vec!["foo".into(), "bar".into()].into());

    assert!(result.0.is_some());
    assert_eq!(expected.1, result.1);
}

#[test]
fn path_node_handle_wildcard_expect() {
    let mut tree: PathNode<String> = PathNode::new();
    tree.add_route(HttpMethod::GET, vec!["*".into(), "bar".into()].into(), handler);

    let expected: (Option<Handler<String>>, Vec<String>) = (Some(handler), vec!["foo".into()]);

    let result = tree.handle(&HttpMethod::GET, vec!["foo".into(), "bar".into()].into());

    assert!(result.0.is_some());
    assert_eq!(expected.1, result.1);
}

#[test]
fn path_node_handle_expect_none() {
    let mut tree: PathNode<String> = PathNode::new();
    tree.add_route(HttpMethod::GET, vec!["foo".into(), "bar".into()].into(), handler);

    let expected: (Option<Handler<String>>, Vec<String>) = (None, vec![]);

    let result = tree.handle(&HttpMethod::GET, vec!["foo".into(), "bars".into()].into());

    assert!(result.0.is_none());
    assert_eq!(expected.1, result.1);
}


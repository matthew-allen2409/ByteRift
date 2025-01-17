use crate::HttpMethod;
use crate::request::Request;
use crate::response::{ Response, StatusLine };
use path_node::PathNode;
use std::collections::VecDeque;

pub(crate) type Handler<T> = fn(Vec<String>, &Request, &T) -> Response;

pub(crate) mod path_node;

pub struct Router<T> {
    route_tree: PathNode<T>,
    state: T,
}

impl<T> Router<T> {
    pub fn new(state: T) -> Self {
        Self {
            route_tree: PathNode::new(),
            state,
        }
    }

    pub fn add_route(&mut self, method: HttpMethod, path: &str, handler: Handler<T>) {
        let path: VecDeque<String> = parse_path(path);

        self.route_tree.add_route(method, path, handler);
    }

    pub fn handle_route(&self, request: Request) -> Response {
        let path = parse_path(&request.request_line.target);
        let (handler, args) = self.route_tree.handle(&request.request_line.method, path);
        match handler {
            Some(handler) => handler(args, &request, &self.state),
            None => Response {
                status_line: StatusLine::new(404, "Not Found".to_string()),
                headers: Vec::new(),
                body: None,
            }
        }
    }
}

fn parse_path(path: &str) -> VecDeque<String> {
    path
        .trim()
        .split('/')
        .filter(|str| str.ne(&""))
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests;

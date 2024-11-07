use crate::request::Request;
use crate::response::Response;
use crate::HttpMethod;
use std::collections::{HashMap, VecDeque};

pub(crate) type Handler<T> = fn(Vec<String>, &Request, &T) -> Response;

#[derive(Debug, PartialEq)]
pub(super) struct PathNode<T> {
    children: HashMap<String, Self>,
    handlers: HashMap<HttpMethod, Handler<T>>,
}

impl<T> PathNode<T> {
    pub(crate) fn new() -> Self {
        Self {
            children: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    pub(super) fn add_route(
        &mut self,
        method: HttpMethod,
        mut path: VecDeque<String>,
        handler: Handler<T>,
    ) {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                self.handlers.insert(method, handler);
                return;
            }
        };

        let child: &mut PathNode<T> = match self.children.get_mut(&path_element) {
            Some(child) => child,
            None => self.children.entry(path_element).or_insert(PathNode::new()),
        };

        child.add_route(method, path, handler);
    }

    pub(super) fn find(
        &self,
        method: HttpMethod,
        mut path: VecDeque<String>,
    ) -> Option<&Handler<T>> {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                return self.handlers.get(&method);
            }
        };

        match self.children.get(&path_element) {
            Some(child) => return child.find(method, path),
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests;

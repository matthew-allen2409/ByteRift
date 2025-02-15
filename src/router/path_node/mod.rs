use super::Handler;
use crate::HttpMethod;
use std::collections::{HashMap, VecDeque};

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

    pub(super) fn handle(
        &self,
        method: &HttpMethod,
        path: VecDeque<String>,
    ) -> (Option<&Handler<T>>, Vec<String>) {
        let args: Vec<String> = vec![];
        self.find(method, path, args)
    }

    fn find(&self, method: &HttpMethod, mut path: VecDeque<String>, mut args: Vec<String>) -> (Option<&Handler<T>>, Vec<String>) {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => return (self.handlers.get(&method), args),
        };

        match self.children.get(&path_element) {
            Some(child) => return child.find(method, path, args),
            None => {
                if self.children.keys().find(|key| (*key).eq("*")).is_some() {
                    args.push(path_element);
                    return self.children.get("*").unwrap().find(method, path, args);
                }
                return (None, args)
            },
        }
    }
}

#[cfg(test)]
mod tests;

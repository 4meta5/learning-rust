/// Container Types Tutorial
/// [](https://abronan.com/rust-trait-objects-box-and-rc/)
///
/// [container cheatsheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p)
/// [The Periodic Table of Rust Types](http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/)
///
/// `Box<T>` copies values, `Rc<T>` clones references and keeps track of references in use.
/// (code from the first link above):
extern crate shoplift;

use shoplift::Docker as DockerClient;
use shoplift::ContainerOptions;

use std::rc::Rc; // for use farther below

struct Docker {
    client: DockerClient,
}

impl Docker {
    pub fn new() -> Docker {
        Docker { client: DockerClient::new() }
    }
}

impl Engine for Docker {
    fn run(&self, image: &str) -> Result<(), &str> {
        let containers = self.client.containers();
        let opts = &ContainerOptions::builder(image).build();
        if containers.create(opts).is_err() {
            return Err("Cannot create container");
        }
        Ok(())
    }
}

/// `Rc` doesn't copy the whole context and data when calling clone
/// it only copies and hands-off a reference to the object on the heap, the "fat pointer" with the virtual table pointing to the right Trait implementation
pub struct Server {
    engine: Rc<Engine>,
}

impl Default for Server {
    fn default() -> Server {
        // Instantiate a default Engine Client
        let client = Rc::new(Docker::new());

        Server {
            engine: client,
        }
    }
}

impl Server {
    pub fn new() -> Server {
        Default::default()
    }

    // Overrides the default engine
    pub fn init_engine(&mut self, engine: Rc<Engine>) -> &mut Server {
        self.engine = engine;
        self
    }

    // other init functions here

    pub fn build(&self) -> Server {
        Server {
            engine: self.engine.clone(),
        }
    }
}

fn main() {
    // examples of how to instantiate `Rkt` and `Docker`
    let rkt = Rc::new(Rkt::new());
    let docker = Rc::new(Docker::new());

    let server1 = Server::new()
                    .init_engine(rkt)
                    .build();
    let server2 = Server::new()
                    .init_engine(rkt)
                    .build();
}
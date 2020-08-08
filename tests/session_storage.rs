use gluesql::SessionStorage;
use gluesql_core::tests::*;
use gluesql_core::{execute, generate_tests, Payload, Query, Result};

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

struct SessionTester {
    storage: Option<SessionStorage>,
}

impl Tester for SessionTester {
    fn new(namespace: &str) -> Self {
        let storage = SessionStorage::new(namespace.to_string()).unwrap_or_else(|_| {
            panic!("SessionStorage::new {}", namespace);
        });
        let storage = Some(storage);

        Self { storage }
    }

    fn execute(&mut self, query: &Query) -> Result<Payload> {
        let storage = self.storage.take().unwrap();

        match execute(storage, query) {
            Ok((storage, payload)) => {
                self.storage = Some(storage);

                Ok(payload)
            }
            Err((storage, error)) => {
                self.storage = Some(storage);

                Err(error)
            }
        }
    }
}

generate_tests!(wasm_bindgen_test, SessionTester);

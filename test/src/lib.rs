#![allow(deprecated)]

use fankor::prelude::*;

mod accounts;
mod arguments;
mod errors;
mod instruction;
mod program;

#[cfg(all(test, feature = "test"))]
mod tests;

setup!("7JKciYMdWKBo1yPhjVe5eDDjoxYfB8YhkAL7DRpJj3xE");

#[cfg(not(feature = "library"))]
security_txt! {
    // Required fields
    name: "Example",
    project_url: "http://example.com",
    contacts: "email:example@example.com,link:https://example.com/security,discord:example#1234",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md"
}

pub mod embed;
pub mod memory_store;
pub mod document;
pub mod embedding;

fn hello_name(name: String) -> String {
    format!("Hello {}", name)
}

uniffi::include_scaffolding!("inference");

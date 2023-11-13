pub mod document;
pub mod embedding;
pub mod similarity;
pub mod store;

fn hello_name(name: String) -> String {
    format!("Hello {}", name)
}

uniffi::include_scaffolding!("inference");

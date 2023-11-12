pub mod embed;

fn hello_name(name: String) -> String {
    format!("Hello {}", name)
}

uniffi::include_scaffolding!("inference");

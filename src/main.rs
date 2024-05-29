use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

pub struct Cognitive {
    engine: Ollama,
    model: String,
}

impl Cognitive {
    pub fn new(model: String) -> Self {
        Self {
            engine: Ollama::default(),
            model,
        }
    }

    pub async fn answer(&self, prompt: String) -> String {
        let options = GenerationOptions::default()
            .temperature(0.2)
            .repeat_penalty(1.5)
            .top_k(25)
            .top_p(0.25);

        let res = self
            .engine
            .generate(GenerationRequest::new(self.model.clone(), prompt).options(options))
            .await;

        match res {
            Ok(res) => res.response,
            Err(_) => "Unknown".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Version 2.0\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        return Ok(());
    }
    let path = Path::new(&args[1]);

    if !path.is_dir() {
        eprintln!("Error: The path specified is not a directory.");
        return Ok(());
    }

    let cognitive = Cognitive::new("llama3:70b".to_string());
    let mut names_used = HashSet::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("pdf") {
            let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();
            print!("We are working following file {} \n", file_name);
            let bytes = std::fs::read(path.clone()).unwrap();
            let pdf_text = pdf_extract::extract_text_from_mem(&bytes).unwrap();
            let mut unique_name;
            loop {
                let prompt = format!("Following is the content of a pdf file: {}. Give me a meaningful name based on its content but limit to 10 characters. Avoid using these names: {:?}.   you just return file name without any additional character or explanation. you response should not be more than 10 character.", pdf_text, names_used);
                unique_name = cognitive.answer(prompt).await.to_string();
                if !names_used.contains(&unique_name) {
                    names_used.insert(unique_name.clone());
                    break;
                }
            }

            let new_path = path.with_file_name(format!("{}.pdf", unique_name));
            print!("New file name is: {}", new_path.display());
            fs::rename(&path, new_path)?;
        }
    }

    Ok(())
}

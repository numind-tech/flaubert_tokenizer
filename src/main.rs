use tokenizers::models::bpe::BPE;
use tokenizers::Tokenizer;

fn main() {
    match BPE::from_file(
        "tokenizer/vocab.json",
        "tokenizer/merges.txt"
    ).build() {
        Ok(model) => {
            let tokenizer = Tokenizer::new(model);
            match tokenizer.encode("Bonjour, le monde", false) {
                Ok(_) => println!("Tokenization successful"),
                Err(err) => println!("Tokenization failed: {}", err),
            }
        }
        Err(err) => println!("Creating tokenizer failed: {}", err)
    }
    println!("Done");
}

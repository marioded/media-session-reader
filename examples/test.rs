use media_session_reader;

fn main() {
    match media_session_reader::current_track() {
        Some(track) => match serde_json::to_string_pretty(&track) {
            Ok(json_string) => println!("{}", json_string),
            Err(e) => eprintln!("Errore nella serializzazione JSON: {}", e),
        },

        None => {
            println!("No player trovato");
        }
    }
}

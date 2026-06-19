use media_session_reader;

fn main() {
    match media_session_reader::current_track() {
        Some(track) => {
            println!("{:#?}", track);
        }

        None => {
            println!("No player found");
        }
    }
}
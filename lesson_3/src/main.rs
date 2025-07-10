struct Note<'a> {
    title: &'a str,
    content: &'a str,
    other_notes: Vec<&'a str>,
}

fn main() {
    let title1 = String::from("Note 1");
    let content1 = String::from("This is the content of note 1.");

    let title2 = String::from("Note 2");
    let content2 = String::from("This is the content of note 2.");

    let note1 = Note {
        title: &title1,
        content: &content1,
        other_notes: vec![&content2],
    };
    let note2 = Note {
        title: &title2,
        content: &content2,
        other_notes: vec![&content1],
    };

    println!("Note 1: {} - {}", note1.title, note1.content);
    println!("Other notes in Note 1: {:?}", note1.other_notes);
    println!("Note 2: {} - {}", note2.title, note2.content);
    println!("Other notes in Note 2: {:?}", note2.other_notes);
}

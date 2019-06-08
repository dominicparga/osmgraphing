//------------------------------------------------------------------------------------------------//
// xml reader stuff

use quick_xml;
use quick_xml::Reader;
use quick_xml::events::Event;

pub fn stuff(xml: &str) {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                println!("Event::Start(ref e) {:?}", String::from_utf8_lossy(e.name()));
                match e.name() {
                    b"tag1" => println!(
                        "attributes values: {:?}",
                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                    ),
                    b"tag2" => count += 1,
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                println!("Event::Text(e)");
                txt.push(e.unescape_and_decode(&reader).unwrap())
            },
            Ok(Event::Eof) => {
                println!("Event::Eof");
                break
            }, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => {
                println!("nix");
            }, // There are several other `Event`s we do not consider here
        }

        println!("{:?}", txt);

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        // buf.clear(); // probably wasted since "clear() has no effect on allocated capacity"
    }
}

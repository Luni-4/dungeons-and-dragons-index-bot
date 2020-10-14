extern crate phf_codegen;
extern crate unidecode;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use serde::Deserialize;
use unidecode::unidecode;

const FILE_PATH: &str = "data";

#[derive(Deserialize, Debug)]
struct Data {
    items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct Item {
    name: String,
    books: Vec<Book>,
}

#[derive(Deserialize, Debug)]
struct Book {
    name: String,
    pages: Vec<String>,
}

fn read_json_file(lang_filename: &str) -> Result<Data, std::io::Error> {
    let file = File::open(Path::new(FILE_PATH).join(lang_filename))?;
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

fn write_sets<W: Write>(
    file: &mut BufWriter<W>,
    json_data: &Data,
) -> Result<(), std::io::Error> {
    for item in &json_data.items {
        let mut builder = phf_codegen::Set::new();
        let mut set_strings: Vec<String> = Vec::new();

        for book in &item.books {
            let pages_str = book
                .pages
                .iter()
                .map(|s| &**s)
                .collect::<Vec<&str>>()
                .join(", ");
            set_strings.push(
                book.name.clone() + " ---> pages " + pages_str.as_str() + "\n",
            );
        }

        for set_str in &set_strings {
            builder.entry(set_str.as_str());
        }

        writeln!(
            file,
            "const {}: phf::Set<&'static str> =\n{};\n",
            unidecode(&item.name.to_uppercase()),
            builder.build()
        )
        .unwrap()
    }
    Ok(())
}

fn write_map<W: Write>(
    file: &mut BufWriter<W>,
    json_data: &Data,
    map_lang: &str,
) -> Result<(), std::io::Error> {
    let mut builder = phf_codegen::Map::new();

    for item in &json_data.items {
        builder
            .entry(item.name.as_str(), &unidecode(&item.name.to_uppercase()));
    }

    writeln!(
        file,
        "pub static {}: phf::Map<&'static str, phf::Set<&'static str>> =\n{};\n",
        map_lang,
        builder.build()
    )
}

fn write_lang(
    map_lang: &str,
    lang_filename: &str,
    output_filename: &str,
) -> Result<(), std::io::Error> {
    let json_data = read_json_file(lang_filename)?;

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(output_filename);
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
        "// This file is auto-generated. DO NOT EDIT!!!\n"
    )?;

    write_sets(&mut file, &json_data)?;

    write_map(&mut file, &json_data, map_lang)
}

fn main() -> Result<(), std::io::Error> {
    write_lang("ENG_MAP", "eng.json", "eng_items.rs")?;
    write_lang("ITA_MAP", "ita.json", "ita_items.rs")
}

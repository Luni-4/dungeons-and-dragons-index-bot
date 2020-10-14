use phf::phf_map;

pub static SUPPORTED_LANGS: [&str; 1] = ["ita"];

pub static ENG_STRS: phf::Map<&'static str, &'static str> = phf_map! {
    "heading" => "is not a valid item or not contained in our dictionary.",
    "results" => "Results for",
    "list" => "Supported items starting with letter",
    "empty" => "No items found.",
    "help" => "This bot retrieves the pages of some chosen items from the Dungeons and Dragons version 5.0 books.\n\n\
               It implements the following commands:\n\n\
               /eng `ITEM` --> Retrieves the pages of an `ITEM` using the English books.\n\n\
               /ita `ITEM` --> Retrieves the pages of an `ITEM` using the Italian books.\n\n\
               /list `LETTER` `[LANG]` --> Prints the list of supported items starting with `LETTER`.\n\
               This command can also print the supported items taken from the manuals written in your `LANG`.\n\
               If `LANG` is not inserted in, the default language is English.\n\n\
               /help `[LANG]` --> Prints this helper message in your `LANG`.\n\
               Possible `LANG` values: *ita*\n\
               If `LANG` is not inserted in, the helper message will be printed in English."
};

pub static ITA_STRS: phf::Map<&'static str, &'static str> = phf_map! {
    "heading" => "non è un elemento valido o non è contenuto nel nostro dizionario.",
    "results" => "Risultati per",
    "list" => "Elementi supportati con la lettera",
    "empty" => "Nessun elemento.",
    "help" => "Questo bot ritrova le pagine di alcuni elementi del gioco dai manuali della versione 5.0 di Dungeons and Dragons.\n\n\
               Implementa i seguenti comandi:\n\n\
               /eng `ELEMENTO` --> Ritrova le pagine di `ELEMENTO` nei manuali inglesi.\n\n\
               /ita `ELEMENTO` --> Ritrova le pagine di `ELEMENTO` nei manuali italiani.\n\n\
               /list `LETTERA` `[LINGUA]` --> Stampa la lista degli elementi del gioco supportati che iniziano per `LETTERA`.\n\
               Questo comando può inoltre stampare alcuni degli elementi del gioco presi da manuali scritti nella tua `LINGUA`.\n\
               Se `LINGUA` non è inserito, il linguaggio di default è l'inglese.\n\n\
               /help `[LINGUA]` --> Stampa questo messaggio di aiuto nella tua `LINGUA`.\n\
               Possibili valori per `LINGUA`: *ita*.\n\
               Se `LINGUA` non è inserito, di default viene usato l'inglese."
};

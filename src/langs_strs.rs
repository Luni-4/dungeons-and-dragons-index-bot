use phf::phf_map;

pub const SUPPORTED_LANGS: [&str; 1] = ["italiano"];

pub static ENG_STRS: phf::Map<&'static str, &'static str> = phf_map! {
    "error" => "needs an `ITEM` argument.\n\n\
                Run the `/help` command for major details.",
    "heading" => "is not a valid item or not contained in our dictionary.",
    "results" => "Results for",
    "help" => "This bot retrieves the pages of some chosen items from the Dungeons and Dragons version 5.0 books.\n\n\
               It implements the following commands:\n\n\
               /eng `ITEM` --> Retrieves the pages of an `ITEM` using the English books.\n\
               /ita `ITEM` --> Retrieves the pages of an `ITEM` using the Italian books.\n\
               /help `[LANG]` --> Prints this helper message in your `LANG`.\n\
               Ensure your `LANG` to be supported.\n\
               If `LANG` is not inserted, the message will be printed in English."
};

pub static ITA_STRS: phf::Map<&'static str, &'static str> = phf_map! {
    "error" => "richiede un `ELEMENTO` del gioco come input.\n\n\
                Lancia il comando `/help italiano` per avere maggiori informazioni.",
    "heading" => "non è un elemento valido o non è contenuto nel nostro dizionario.",
    "results" => "Risultati per",
    "help" => "Questo bot ritrova le pagine di alcuni elementi del gioco dai manuali della versione 5.0 di Dungeons and Dragons.\n\n\
               Implementa i seguenti comandi:\n\n\
               /eng `ELEMENTO` --> Ritrova le pagine di `ELEMENTO` nei manuali inglesi.\n\
               /ita `ELEMENTO` --> Ritrova le pagine di `ELEMENTO` nei manuali italiani.\n\
               /help `[LINGUA]` --> Stampa questo messaggio di aiuto nella tua `LINGUA`.\n\
               Assicurati che la tua `LINGUA` sia supportata.\n\
               Se `LINGUA` non è inserito, di default viene usato l'inglese."
};

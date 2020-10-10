// This file is auto-generated. DO NOT EDIT!!!

const HONOUR: phf::Set<&'static str> = ::phf::Set {
    map: ::phf::Map {
        key: 3213172566270843353,
        disps: ::phf::Slice::Static(&[(1, 0)]),
        entries: ::phf::Slice::Static(&[
            ("Dungeon\'s Guide ---> pages 260, 261\n", ()),
            ("Player\'s Handbook ---> pages 270, 271\n", ()),
        ]),
    },
};

const SANITY: phf::Set<&'static str> = ::phf::Set {
    map: ::phf::Map {
        key: 3213172566270843353,
        disps: ::phf::Slice::Static(&[(0, 0)]),
        entries: ::phf::Slice::Static(&[
            ("Dungeon\'s Guide ---> pages 220, 250\n", ()),
            ("Player\'s Handbook ---> pages 230, 251\n", ()),
        ]),
    },
};

pub static ENG_MAP: phf::Map<&'static str, phf::Set<&'static str>> = ::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[(1, 0)]),
    entries: ::phf::Slice::Static(&[("honour", HONOUR), ("sanity", SANITY)]),
};

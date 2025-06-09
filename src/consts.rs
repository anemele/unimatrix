use std::collections::HashMap;

use crossterm::style::Color;

const CHAR_SET: [(char, &str); 15] = [
    ('a', "qwertyuiopasdfghjklzxcvbnm"),
    ('A', "QWERTYUIOPASDFGHJKLZXCVBNM"),
    ('c', "абвгдежзиклмнопрстуфхцчшщъыьэюя"),
    ('C', "АБВГДЕЖЗИКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"),
    ('e', "☺☻✌♡♥❤⚘❀❃❁✼☀✌♫♪☃❄❅❆☕☂★"),
    ('g', "αβγδεζηθικλμνξοπρστυφχψως"),
    ('G', "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"),
    (
        'k',
        "ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ",
    ),
    (
        'm',
        r#"ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ1234567891234567890-=*_+|:<>"-=*_+|:<>"-=*_+|:<>"-=*_+|:<>""#,
    ),
    ('n', "1234567890"),
    (
        'o',
        r#"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM123456789`-=~!@#$%^&*()_+[]{}|\\;\':",./<>?""#,
    ),
    ('r', "mcclllxxxxvvvvviiiiii"),
    ('R', "MCCLLLXXXXVVVVVIIIIII"),
    ('s', r#"-=*_+|:<>""#),
    ('S', r#"`-=~!@#$%^&*()_+[]{}|\\;\':",./<>?""#),
];

pub fn get_char_map() -> HashMap<char, &'static str> {
    HashMap::from(CHAR_SET)
}

const COLOR_SET: [(&str, Color); 9] = [
    ("green", Color::Green),
    ("red", Color::Red),
    ("blue", Color::Blue),
    ("white", Color::White),
    ("yellow", Color::Yellow),
    ("cyan", Color::Cyan),
    ("magenta", Color::Magenta),
    ("black", Color::Black),
    ("default", Color::Reset),
];

pub fn get_color_map() -> HashMap<&'static str, Color> {
    HashMap::from(COLOR_SET)
}

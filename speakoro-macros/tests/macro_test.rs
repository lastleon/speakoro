use speakoro_macros;

enum Bla {
    C,
    D
}

#[test]
fn general_test() {
    speakoro_macros::associate_static_data!(
        type Enum = Bla;
        type Data = &'static str;

        Bla::C => "hello",
        Bla::D => "yo",
    );
}

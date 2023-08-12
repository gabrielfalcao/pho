use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
/// Renders a string to phonetic alphabet. Keeps unknown characters unchanged.
pub fn render(data: String, sep: String) -> String {
    let mut r = Vec::<String>::new();
    for o in data.bytes() {
        let x = match o {
            b'a' => "alpha",
            b'b' => "bravo",
            b'c' => "charlie",
            b'd' => "delta",
            b'e' => "echo",
            b'f' => "foxtrot",
            b'g' => "golf",
            b'h' => "hotel",
            b'i' => "india",
            b'j' => "juliet",
            b'k' => "kilo",
            b'l' => "lima",
            b'm' => "mike",
            b'n' => "november",
            b'o' => "oscar",
            b'p' => "papa",
            b'q' => "quebec",
            b'r' => "romeo",
            b's' => "sierra",
            b't' => "tango",
            b'u' => "uniform",
            b'v' => "victor",
            b'w' => "whiskey",
            b'x' => "xray",
            b'y' => "yankee",
            b'z' => "zulu",
            b'A' => "ALPHA",
            b'B' => "BRAVO",
            b'C' => "CHARLIE",
            b'D' => "DELTA",
            b'E' => "ECHO",
            b'F' => "FOXTROT",
            b'G' => "GOLF",
            b'H' => "HOTEL",
            b'I' => "INDIA",
            b'J' => "JULIET",
            b'K' => "KILO",
            b'L' => "LIMA",
            b'M' => "MIKE",
            b'N' => "NOVEMBER",
            b'O' => "OSCAR",
            b'P' => "PAPA",
            b'Q' => "QUEBEC",
            b'R' => "ROMEO",
            b'S' => "SIERRA",
            b'T' => "TANGO",
            b'U' => "UNIFORM",
            b'V' => "VICTOR",
            b'W' => "WHISKEY",
            b'X' => "XRAY",
            b'Y' => "YANKEE",
            b'Z' => "ZULU",
            _ => "",
        }.to_string();

        if &x == "" {
            if String::from_utf8_lossy(&[o]) != sep {
                r.push(format!("{}", o as char));
            }
        } else {
            r.push(x);
        };
    }
    let m = r.iter().filter(|c| c.len() > 0).map(|v| v.clone()).collect::<Vec<_>>().join(&sep);
    String::from(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(render(String::from("a"), " ".to_string()), "alpha");
        assert_eq!(render(String::from("A"), " ".to_string()), "ALPHA");
    }
    #[test]
    fn b() {
        assert_eq!(render(String::from("b"), " ".to_string()), "bravo");
        assert_eq!(render(String::from("B"), " ".to_string()), "BRAVO");
    }
    #[test]
    fn c() {
        assert_eq!(render(String::from("c"), " ".to_string()), "charlie");
        assert_eq!(render(String::from("C"), " ".to_string()), "CHARLIE");
    }
    #[test]
    fn d() {
        assert_eq!(render(String::from("d"), " ".to_string()), "delta");
        assert_eq!(render(String::from("D"), " ".to_string()), "DELTA");
    }
    #[test]
    fn e() {
        assert_eq!(render(String::from("e"), " ".to_string()), "echo");
        assert_eq!(render(String::from("E"), " ".to_string()), "ECHO");
    }
    #[test]
    fn f() {
        assert_eq!(render(String::from("f"), " ".to_string()), "foxtrot");
        assert_eq!(render(String::from("F"), " ".to_string()), "FOXTROT");
    }
    #[test]
    fn g() {
        assert_eq!(render(String::from("g"), " ".to_string()), "golf");
        assert_eq!(render(String::from("G"), " ".to_string()), "GOLF");
    }
    #[test]
    fn h() {
        assert_eq!(render(String::from("h"), " ".to_string()), "hotel");
        assert_eq!(render(String::from("H"), " ".to_string()), "HOTEL");
    }
    #[test]
    fn i() {
        assert_eq!(render(String::from("i"), " ".to_string()), "india");
        assert_eq!(render(String::from("I"), " ".to_string()), "INDIA");
    }
    #[test]
    fn j() {
        assert_eq!(render(String::from("j"), " ".to_string()), "juliet");
        assert_eq!(render(String::from("J"), " ".to_string()), "JULIET");
    }
    #[test]
    fn k() {
        assert_eq!(render(String::from("k"), " ".to_string()), "kilo");
        assert_eq!(render(String::from("K"), " ".to_string()), "KILO");
    }
    #[test]
    fn l() {
        assert_eq!(render(String::from("l"), " ".to_string()), "lima");
        assert_eq!(render(String::from("L"), " ".to_string()), "LIMA");
    }
    #[test]
    fn m() {
        assert_eq!(render(String::from("m"), " ".to_string()), "mike");
        assert_eq!(render(String::from("M"), " ".to_string()), "MIKE");
    }
    #[test]
    fn n() {
        assert_eq!(render(String::from("n"), " ".to_string()), "november");
        assert_eq!(render(String::from("N"), " ".to_string()), "NOVEMBER");
    }
    #[test]
    fn o() {
        assert_eq!(render(String::from("o"), " ".to_string()), "oscar");
        assert_eq!(render(String::from("O"), " ".to_string()), "OSCAR");
    }
    #[test]
    fn p() {
        assert_eq!(render(String::from("p"), " ".to_string()), "papa");
        assert_eq!(render(String::from("P"), " ".to_string()), "PAPA");
    }
    #[test]
    fn q() {
        assert_eq!(render(String::from("q"), " ".to_string()), "quebec");
        assert_eq!(render(String::from("Q"), " ".to_string()), "QUEBEC");
    }
    #[test]
    fn r() {
        assert_eq!(render(String::from("r"), " ".to_string()), "romeo");
        assert_eq!(render(String::from("R"), " ".to_string()), "ROMEO");
    }
    #[test]
    fn s() {
        assert_eq!(render(String::from("s"), " ".to_string()), "sierra");
        assert_eq!(render(String::from("S"), " ".to_string()), "SIERRA");
    }
    #[test]
    fn t() {
        assert_eq!(render(String::from("t"), " ".to_string()), "tango");
        assert_eq!(render(String::from("T"), " ".to_string()), "TANGO");
    }
    #[test]
    fn u() {
        assert_eq!(render(String::from("u"), " ".to_string()), "uniform");
        assert_eq!(render(String::from("U"), " ".to_string()), "UNIFORM");
    }
    #[test]
    fn v() {
        assert_eq!(render(String::from("v"), " ".to_string()), "victor");
        assert_eq!(render(String::from("V"), " ".to_string()), "VICTOR");
    }
    #[test]
    fn w() {
        assert_eq!(render(String::from("w"), " ".to_string()), "whiskey");
        assert_eq!(render(String::from("W"), " ".to_string()), "WHISKEY");
    }
    #[test]
    fn x() {
        assert_eq!(render(String::from("x"), " ".to_string()), "xray");
        assert_eq!(render(String::from("X"), " ".to_string()), "XRAY");
    }
    #[test]
    fn y() {
        assert_eq!(render(String::from("y"), " ".to_string()), "yankee");
        assert_eq!(render(String::from("Y"), " ".to_string()), "YANKEE");
    }
    #[test]
    fn z() {
        assert_eq!(render(String::from("z"), " ".to_string()), "zulu");
        assert_eq!(render(String::from("Z"), " ".to_string()), "ZULU");
    }

    #[test]
    fn operation_1() {
        assert_eq!(render(String::from("operation 1"), " ".to_string()), "oscar papa echo romeo alpha tango india oscar november 1");
    }

}

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
        assert_eq!(render(String::from("a"), " "), "alpha");
        assert_eq!(render(String::from("A"), " "), "ALPHA");
    }
    #[test]
    fn b() {
        assert_eq!(render(String::from("b"), " "), "bravo");
        assert_eq!(render(String::from("B"), " "), "BRAVO");
    }
    #[test]
    fn c() {
        assert_eq!(render(String::from("c"), " "), "charlie");
        assert_eq!(render(String::from("C"), " "), "CHARLIE");
    }
    #[test]
    fn d() {
        assert_eq!(render(String::from("d"), " "), "delta");
        assert_eq!(render(String::from("D"), " "), "DELTA");
    }
    #[test]
    fn e() {
        assert_eq!(render(String::from("e"), " "), "echo");
        assert_eq!(render(String::from("E"), " "), "ECHO");
    }
    #[test]
    fn f() {
        assert_eq!(render(String::from("f"), " "), "foxtrot");
        assert_eq!(render(String::from("F"), " "), "FOXTROT");
    }
    #[test]
    fn g() {
        assert_eq!(render(String::from("g"), " "), "golf");
        assert_eq!(render(String::from("G"), " "), "GOLF");
    }
    #[test]
    fn h() {
        assert_eq!(render(String::from("h"), " "), "hotel");
        assert_eq!(render(String::from("H"), " "), "HOTEL");
    }
    #[test]
    fn i() {
        assert_eq!(render(String::from("i"), " "), "india");
        assert_eq!(render(String::from("I"), " "), "INDIA");
    }
    #[test]
    fn j() {
        assert_eq!(render(String::from("j"), " "), "juliet");
        assert_eq!(render(String::from("J"), " "), "JULIET");
    }
    #[test]
    fn k() {
        assert_eq!(render(String::from("k"), " "), "kilo");
        assert_eq!(render(String::from("K"), " "), "KILO");
    }
    #[test]
    fn l() {
        assert_eq!(render(String::from("l"), " "), "lima");
        assert_eq!(render(String::from("L"), " "), "LIMA");
    }
    #[test]
    fn m() {
        assert_eq!(render(String::from("m"), " "), "mike");
        assert_eq!(render(String::from("M"), " "), "MIKE");
    }
    #[test]
    fn n() {
        assert_eq!(render(String::from("n"), " "), "november");
        assert_eq!(render(String::from("N"), " "), "NOVEMBER");
    }
    #[test]
    fn o() {
        assert_eq!(render(String::from("o"), " "), "oscar");
        assert_eq!(render(String::from("O"), " "), "OSCAR");
    }
    #[test]
    fn p() {
        assert_eq!(render(String::from("p"), " "), "papa");
        assert_eq!(render(String::from("P"), " "), "PAPA");
    }
    #[test]
    fn q() {
        assert_eq!(render(String::from("q"), " "), "quebec");
        assert_eq!(render(String::from("Q"), " "), "QUEBEC");
    }
    #[test]
    fn r() {
        assert_eq!(render(String::from("r"), " "), "romeo");
        assert_eq!(render(String::from("R"), " "), "ROMEO");
    }
    #[test]
    fn s() {
        assert_eq!(render(String::from("s"), " "), "sierra");
        assert_eq!(render(String::from("S"), " "), "SIERRA");
    }
    #[test]
    fn t() {
        assert_eq!(render(String::from("t"), " "), "tango");
        assert_eq!(render(String::from("T"), " "), "TANGO");
    }
    #[test]
    fn u() {
        assert_eq!(render(String::from("u"), " "), "uniform");
        assert_eq!(render(String::from("U"), " "), "UNIFORM");
    }
    #[test]
    fn v() {
        assert_eq!(render(String::from("v"), " "), "victor");
        assert_eq!(render(String::from("V"), " "), "VICTOR");
    }
    #[test]
    fn w() {
        assert_eq!(render(String::from("w"), " "), "whiskey");
        assert_eq!(render(String::from("W"), " "), "WHISKEY");
    }
    #[test]
    fn x() {
        assert_eq!(render(String::from("x"), " "), "xray");
        assert_eq!(render(String::from("X"), " "), "XRAY");
    }
    #[test]
    fn y() {
        assert_eq!(render(String::from("y"), " "), "yankee");
        assert_eq!(render(String::from("Y"), " "), "YANKEE");
    }
    #[test]
    fn z() {
        assert_eq!(render(String::from("z"), " "), "zulu");
        assert_eq!(render(String::from("Z"), " "), "ZULU");
    }

    #[test]
    fn operation_1() {
        assert_eq!(render(String::from("operation 1"), " "), "oscar papa echo romeo alpha tango india oscar november 1");
    }

}

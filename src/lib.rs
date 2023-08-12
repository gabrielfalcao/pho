use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Renders a string to phonetic alphabet. Keeps unknown characters unchanged.
#[wasm_bindgen]
pub fn render(data: String, sep: &str) -> String {
    let mut r = Vec::<String>::new();
    for o in data.bytes() {
        let x = match o {
            b'a' | b'A' => "alpha",
            b'b' | b'B' => "bravo",
            b'c' | b'C' => "charlie",
            b'd' | b'D' => "delta",
            b'e' | b'E' => "echo",
            b'f' | b'F' => "foxtrot",
            b'g' | b'G' => "golf",
            b'h' | b'H' => "hotel",
            b'i' | b'I' => "india",
            b'j' | b'J' => "juliet",
            b'k' | b'K' => "kilo",
            b'l' | b'L' => "lima",
            b'm' | b'M' => "mike",
            b'n' | b'N' => "november",
            b'o' | b'O' => "oscar",
            b'p' | b'P' => "papa",
            b'q' | b'Q' => "quebec",
            b'r' | b'R' => "romeo",
            b's' | b'S' => "sierra",
            b't' | b'T' => "tango",
            b'u' | b'U' => "uniform",
            b'v' | b'V' => "victor",
            b'w' | b'W' => "whiskey",
            b'x' | b'X' => "xray",
            b'y' | b'Y' => "yankee",
            b'z' | b'Z' => "zulu",
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
    let m = r.iter().filter(|c| c.len() > 0).map(|v| v.clone()).collect::<Vec<_>>().join(sep);
    String::from(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(render(String::from("a"), " "), "alpha");
    }
    #[test]
    fn b() {
        assert_eq!(render(String::from("b"), " "), "bravo");
    }
    #[test]
    fn c() {
        assert_eq!(render(String::from("c"), " "), "charlie");
    }
    #[test]
    fn d() {
        assert_eq!(render(String::from("d"), " "), "delta");
    }
    #[test]
    fn e() {
        assert_eq!(render(String::from("e"), " "), "echo");
    }
    #[test]
    fn f() {
        assert_eq!(render(String::from("f"), " "), "foxtrot");
    }
    #[test]
    fn g() {
        assert_eq!(render(String::from("g"), " "), "golf");
    }
    #[test]
    fn h() {
        assert_eq!(render(String::from("h"), " "), "hotel");
    }
    #[test]
    fn i() {
        assert_eq!(render(String::from("i"), " "), "india");
    }
    #[test]
    fn j() {
        assert_eq!(render(String::from("j"), " "), "juliet");
    }
    #[test]
    fn k() {
        assert_eq!(render(String::from("k"), " "), "kilo");
    }
    #[test]
    fn l() {
        assert_eq!(render(String::from("l"), " "), "lima");
    }
    #[test]
    fn m() {
        assert_eq!(render(String::from("m"), " "), "mike");
    }
    #[test]
    fn n() {
        assert_eq!(render(String::from("n"), " "), "november");
    }
    #[test]
    fn o() {
        assert_eq!(render(String::from("o"), " "), "oscar");
    }
    #[test]
    fn p() {
        assert_eq!(render(String::from("p"), " "), "papa");
    }
    #[test]
    fn q() {
        assert_eq!(render(String::from("q"), " "), "quebec");
    }
    #[test]
    fn r() {
        assert_eq!(render(String::from("r"), " "), "romeo");
    }
    #[test]
    fn s() {
        assert_eq!(render(String::from("s"), " "), "sierra");
    }
    #[test]
    fn t() {
        assert_eq!(render(String::from("t"), " "), "tango");
    }
    #[test]
    fn u() {
        assert_eq!(render(String::from("u"), " "), "uniform");
    }
    #[test]
    fn v() {
        assert_eq!(render(String::from("v"), " "), "victor");
    }
    #[test]
    fn w() {
        assert_eq!(render(String::from("w"), " "), "whiskey");
    }
    #[test]
    fn x() {
        assert_eq!(render(String::from("x"), " "), "xray");
    }
    #[test]
    fn y() {
        assert_eq!(render(String::from("y"), " "), "yankee");
    }
    #[test]
    fn z() {
        assert_eq!(render(String::from("z"), " "), "zulu");
    }

    #[test]
    fn operation_1() {
        assert_eq!(render(String::from("operation 1"), " "), "oscar papa echo romeo alpha tango india oscar november 1");
    }

}

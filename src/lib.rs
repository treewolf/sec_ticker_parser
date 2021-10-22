//! This retrieve and creates mapping between stock ticker and their current CIK.
//! These are defined per sec.gov website

use std::collections::HashMap;

/// Gets ticker and cik mappings
///
/// Returns a hashmap where key=value maps to ticker=cik
///
/// # Examples
/// ```
/// let map = sec_ticker_parser::list().unwrap();
/// assert_ne!(map.len(), 0)
/// ```
pub fn list() -> Result<HashMap<String, u64>, reqwest::Error> {
    let url = format!("https://www.sec.gov/include/ticker.txt");
    let body = reqwest::blocking::get(url.as_str())?.text();
    match body {
        Ok(r) => {
            let companies = r.split("\n").collect::<Vec<&str>>();
            let mut map = HashMap::new();
            for c in companies {
                let ticker_cik = c.split("\t").collect::<Vec<&str>>();
                let cik = match ticker_cik[1].parse::<u64>() {
                    Ok(r1) => r1,
                    Err(e1) => {
                        println!("Could not parse CIK: {:?}. {}", &c, &e1);
                        0
                    }
                };
                map.insert(ticker_cik[0].to_string(), cik);
            }
            Ok(map)
        }
        Err(e) => panic!("Error with query. URL: {}. {}", &url, &e),
    }
}

/// Given a valid map and ticker, return the cik.
/// If no valid cik matches, then will return 0.
///
/// # Example
/// ```
/// let map = sec_ticker_parser::list().unwrap();
/// let my_cik = sec_ticker_parser::cik(&map, "vz");
/// assert_eq!(my_cik, 732712);
/// ```
pub fn cik(map: &HashMap<String, u64>, ticker: &str) -> u64 {
    match map.get(&ticker.to_lowercase()) {
        Some(r) => *r,
        None => 0,
    }
}

/// Give a valid map and cik, return the ticket symbol.
/// This will iterate over the hashmap until value is found matching.
/// If no ticker matches, will return the string "none".
///
/// # Example
/// ```
/// let map = sec_ticker_parser::list().unwrap();
/// let my_ticker = sec_ticker_parser::ticker(&map, &732712);
/// assert_eq!(my_ticker, "vz");
/// ```
pub fn ticker(map: &HashMap<String, u64>, cik: &u64) -> String {
    for (ticker, value) in map {
        if value == cik {
            return ticker.to_string();
        }
    }
    "none".to_string()
}

/// Unit test  for each function
#[cfg(test)]
mod tests {
    #[test]
    fn test_list() {
        let o = super::list().unwrap();
        assert!(!o.is_empty());
        assert_eq!(o.get("vz"), Some(&732712))
    }

    #[test]
    fn test_cik() {
        let map = super::list().unwrap();
        let my_cik = super::cik(&map, "vz");
        assert_eq!(my_cik, 732712);
        let invalid_cik = super::cik(&map, "invalid");
        assert_eq!(invalid_cik, 0);
    }

    #[test]
    fn test_ticker() {
        let map = super::list().unwrap();
        let my_ticker = super::ticker(&map, &732712);
        assert_eq!(my_ticker, "vz");
        let invalid_ticker = super::ticker(&map, &0);
        assert_eq!(invalid_ticker, "none")
    }
}

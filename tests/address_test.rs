extern crate rosc;

#[cfg(feature = "std")]
use rosc::address::Matcher;

#[cfg(feature = "std")]
#[test]
fn test_matcher() {
    let matcher = Matcher::new("/oscillator/[0-9]/*/pre[!1234?*]post/{frequency,phase}/x?")
        .expect("Matcher::new");
    assert_eq!(
        matcher
            .match_address("/oscillator/1/something/preXpost/phase/xy")
            .expect("should match"),
        true
    );
    assert_eq!(
        matcher
            .match_address("/oscillator/1/something/pre1post/phase/xy")
            .expect("should not match"),
        false
    );
}

#[cfg(feature = "std")]
#[test]
fn test_bad_address_pattern() {
    let expected_err = "bad OSC address pattern: bad address pattern";
    assert_eq!(Matcher::new("").unwrap_err().to_string(), expected_err);
    assert_eq!(Matcher::new("/").unwrap_err().to_string(), expected_err);
    assert_eq!(Matcher::new("//empty/parts/").unwrap_err().to_string(), expected_err);
    assert_eq!(Matcher::new("////").unwrap_err().to_string(), expected_err);
    assert_eq!(Matcher::new("/{unclosed,alternative").unwrap_err().to_string(), expected_err);
    assert_eq!(Matcher::new("/unclosed/[range-").unwrap_err().to_string(), expected_err);
}

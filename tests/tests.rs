use raiku::{self, haiku, syllables::Word};

// To run tests with coverage do
// To test with coverage run `test_coverage.sh` with either 
//      - report (for procentage)
//      - or show (for details)
// Note: the `objects` (as defined in the script) correspond to the src .rs files, and must be
// manually updated if new files are added to the project

#[test]
fn haiku_test() {
    let actual = haiku("this is a test of the way haiku is made here that is kind of cool").unwrap();

    assert_eq!(actual.0, "this is a test of");
    assert_eq!(actual.1, "the way haiku is made here");
    assert_eq!(actual.2, "that is kind of cool")
}

#[test]
fn made_is_1_syllable_test() {
    let actual = Word::from("made").syllables.len();

    assert_eq!(actual, 1)
}

#[test]
fn haide_is_2_syllables_test() {
    let actual = Word::from("haide").syllables.len();
    assert_eq!(actual, 2)
}

#[test]
fn hide_is_1_syllable_test() {
    let actual = Word::from("hide").syllables.len();
    assert_eq!(actual, 1)
}

#[test]
fn jamie_is_2_syllables_test() {
    let actual = Word::from("jamie").syllables.len();
    assert_eq!(actual, 2)
}

#[test]
fn fly_is_1_syllable_test() {
    let actual = Word::from("fly").syllables.len();
    assert_eq!(actual, 1)
}

#[test]
fn flywheel_is_2_syllables_test() {
    let actual = Word::from("flywheel").syllables.len();
    assert_eq!(actual, 2)
}

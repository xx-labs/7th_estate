use seventh_estate::ballots::*;
use hex;
use std::path::Path;
use std::fs;
use mime_guess;
use mime::APPLICATION_PDF;

#[macro_use] extern crate slice_as_array;

fn slice_as_hash(xs: &[u8]) -> &[u8; 20] {
    slice_as_array!(xs, [u8; 20]).expect("bad hash length")
}

#[test]
fn test_pdf() {

    let vote1 = hex::decode("b28de6131ecdd6075b1473ca6525c0bf990fde7f").unwrap();
    let vote1 = *slice_as_hash(&vote1);
    
    let choice1: BallotChoice = BallotChoice {
        serial: 123456,
        votecode: vote1,
        choice: ChoiceValue::For
    };

    let choice2: BallotChoice = BallotChoice {
        serial: 123456,
        votecode: vote1,
        choice: ChoiceValue::Against
    };

    let ballot: Ballot = Ballot {
        serial: 123456,
        choice1: choice1,
        choice2: choice2
    };
    
    let filename = BALLOTS_PATH.to_string() + &ballot.serial.to_string()  + ".pdf";

    // Test if file was created
    assert_eq!((), print_ballot(&ballot, "Test Question", "Yes", "No"));
    assert_eq!(true, Path::new(&(filename)).exists());


    // Test if created file is pdf
    let filetype = mime_guess::from_path(filename.to_owned());
    assert_eq!(Some(APPLICATION_PDF), filetype.first());

    // Delete test file
    fs::remove_file(filename).unwrap();
}
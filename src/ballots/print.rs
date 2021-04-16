//! # Print ballot to PDF
//!
//! //! CreateBallot (ballot)
//!     Create new pdf
//!     Write title ("YES/NO Ballot")
//!     Write Instructions("vote online by..")
//!     Write Ballot serial ("ballot serial: {ballot.serial}")
//!     Write choices
//!         Write choice1 ("{ballot.choice1.votecode} {ballot.choice1.choice")
//!         Write choice2 ("{ballot.choice2.votecode} {ballot.choice2.choice")
//!     Leavy empty space for Decoy text ("This ballot is a decoy!...")

use super::*;
use std::result::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use super::untagged::{Ballot, BallotChoice, ChoiceValue, string_from_votecode, string_from_choicevalue};

struct Text<'a> {
    pub text: String,
    pub size: i64,
    pub startx: Mm,
    pub starty: Mm,
    pub font: &'a IndirectFontRef,
}

struct FileSize {
    pub height: Mm,
    pub width: Mm,
}

const _A5: FileSize = FileSize {
    height: Mm(210.0), 
    width: Mm(148.0)
};

const _A6: FileSize = FileSize {
    height: Mm(148.0), 
    width: Mm(105.0)
};

// Text to be printed to PDF file
const BALLOT_SIZE: FileSize = _A5;
const BALLOTS_PATH: &str = "ballots/";
const TITLE_TEXT: &str = "YES/NO Ballot";
const INST_TITLE: &str = "Instructions";
const INST_TEXT: &str = 
"vote online by\n
entering ballot serial\n
number and the vote\n
code printed under\n
the scratch-off next\n
to your choice:";
const BALLOT_SERIAL_TEXT: &str = "ballot serial: ";
const VOTE_CODE_TEXT: &str = "vote code:           choice:";
const _DECOY_TEXT: &str = 
"This ballot is a decoy!\n
Remove this sticker\n
and sell this vote!";


fn add_text(layer: &PdfLayerReference, text: &Text){
    layer.use_text(text.text.to_string(), text.size as f64, text.startx, text.starty, &text.font);
}

fn make_circle(radius: Pt, startx: Pt, starty: Pt) -> Line{

    // Make circle
    let circle = Line {
        points: utils::calculate_points_for_rect(radius, Pt(20.0), startx, starty),
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };
    
    circle
}

fn make_dir() -> Result<(), std::io::Error>{
    match DirBuilder::new().create(Path::new(BALLOTS_PATH)) {
        Ok(_) => Ok(()),
        Err(err) => {
            match err.kind() {
                // Do nothing if directory already exists
                ErrorKind::AlreadyExists => Ok(()),
                // Else panic
                _ => Err(err)
            }
        }
    }
}

pub fn print_ballot(ballot: &Ballot){

    // Create ballots dir
    make_dir().unwrap();

    // Create new document
    let file = BALLOTS_PATH.to_string() + &ballot.serial.to_string()  + ".pdf";
    let mut file_writer = BufWriter::new(File::create(file).unwrap());
    
    // Start new PDF
    let (doc, page1, layer1) = PdfDocument::new(ballot.serial.to_string(), BALLOT_SIZE.width, BALLOT_SIZE.height, "layer1".to_string());
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Add fonts for title and text
    let font_title = doc.add_builtin_font(BuiltinFont::CourierBold).unwrap();
    let font_text = doc.add_builtin_font(BuiltinFont::Courier).unwrap();

    let title: Text = Text {
        text: TITLE_TEXT.to_string(), 
        size: 30, 
        startx: Mm(30.0), 
        starty: BALLOT_SIZE.height - Mm(20.0),
        font: &font_title,
    };

    let instructions_title: Text = Text {
        text: INST_TITLE.to_string(),
        size: 15,
        startx: Mm(10.0),
        starty: BALLOT_SIZE.height - Mm(35.0),
        ..title
    };
    let instructions_text: Text = Text {
        text: INST_TEXT.to_string(),
        size: 10,
        starty: BALLOT_SIZE.height - Mm(40.0),
        font: &font_text,
        ..instructions_title
    };

    let ballot_serial: Text = Text {
        text: BALLOT_SERIAL_TEXT.to_string() + &ballot.serial.to_string(),
        size: 12,
        starty: BALLOT_SIZE.height - Mm(70.0),
        ..instructions_text
    };

    let vote_code: Text = Text {
        text: VOTE_CODE_TEXT.to_string(),
        ..ballot_serial
    };

    // Add ballot/instructions title
    add_text(&current_layer, &title);
    add_text(&current_layer, &instructions_title);

    // Start instructions text section
    current_layer.begin_text_section();
        current_layer.set_font(&font_text, instructions_text.size as f64);
        current_layer.set_text_cursor(instructions_text.startx, instructions_text.starty);
        current_layer.set_line_height(6.0);
        
        // Write lines of instructions
        instructions_text.text.lines()
            .for_each(|line|{
                current_layer.write_text(line, &font_text);
                current_layer.add_line_break();
            });
    current_layer.end_text_section();
    
    // Start ballot serial/vote code section
    current_layer.begin_text_section();
        current_layer.set_font(&font_title, ballot_serial.size as f64);
        current_layer.set_text_cursor(ballot_serial.startx, ballot_serial.starty);
        current_layer.set_line_height(10.0);
        
        // Write Ballot Serial
        current_layer.write_text(ballot_serial.text, &font_title);
        current_layer.add_line_break();

        // Write Ballot vote code text
        current_layer.write_text(vote_code.text, &font_title);
    current_layer.end_text_section();
    // End text section

    // Add choices
    make_choice(ballot.choice1, &current_layer, &font_text);
    make_choice(ballot.choice2, &current_layer, &font_text);

    // Save document
    doc.save(&mut file_writer).unwrap();
}

fn make_choice(choice: BallotChoice, layer: &PdfLayerReference, font: &IndirectFontRef){
    let votecode: String = string_from_votecode(&choice.votecode);
    let width = BALLOT_SIZE.width/2.0 - Mm(20.0);
    let height: Mm = match choice.choice {
        ChoiceValue::For => BALLOT_SIZE.height/2.0 + Mm(20.0),
        ChoiceValue::Against => BALLOT_SIZE.height/2.0 - Mm(20.0),
    };

    // Make dash
    let mut dash_pattern = LineDashPattern::default();
    dash_pattern.dash_1 = Some(3);

    // Make choice circle
    layer.set_line_dash_pattern(dash_pattern);
    layer.set_fill_color(Color::Greyscale(Greyscale::new(0.8, None)));
    let circle = make_circle(Mm((votecode.len() * 2) as f64).into(), width.into(), height.into());
    layer.add_shape(circle);

    // Make choice value
    dash_pattern.dash_1 = None;
    layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None)));
    let choice: Text = Text {
        text: string_from_choicevalue(&choice.choice), 
        size: 15, 
        startx: width + Mm(40.0), 
        starty: height,
        font: font,
    };
    add_text(layer, &choice);

    // Make vote code
    let votetext: Text = Text {
        text: votecode, 
        startx: width - Mm(22.0),
        size: 9, 
        ..choice
    };
    add_text(layer, &votetext);
}
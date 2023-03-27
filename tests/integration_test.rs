use ev3dlib::gcode::parse_gcode_file;

#[test]
fn test_parse_gcode_file() {
    let filename = "/Users/reeperto/dev/git/ev3dprinter/example.gcode".to_string();
    let instrs = parse_gcode_file(filename).unwrap();
    println!("{:#?}", instrs);
}

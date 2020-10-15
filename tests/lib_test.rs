use pe_lib::run;
use insta;
use itertools::Itertools;

#[test]
fn test_from_snapshots() {
    insta::glob!("csv_inputs/*.csv", |path| {
        let mut reader = csv::Reader::from_path(path).unwrap();
        let mut writer = csv::Writer::from_writer(vec![]);
        
        run(&mut reader, &mut writer).unwrap();

        let written = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        let mut lines = written.lines();
        let csv = lines.next().unwrap();

        insta::assert_snapshot!(String::from(csv) + "\n" + &lines.sorted().join("\n"));
    });
}
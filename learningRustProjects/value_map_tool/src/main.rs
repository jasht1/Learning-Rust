use csv::ReaderBuilder;
use csv::WriterBuilder;

fn main() {
    let input_list = "TestList.csv";
    let output_valueMap = "TestValueMap.csv";

    struct DayData {
        date: String,
        values: [f64; 48]
    }

    struct Data {
        DayData * 365
    }

    let mut r = ReaderBuilder::new()
        .has_headers(true)  // Assuming the input file has headers
        .from_path(input_file)?;

    let mut w = WriterBuilder::new().from_path(output_file)?;

    for row in r.records() {
        let data_point = row?;
        if date in data {
            data[date].
        }
        let date = &row[0];
        let value = &row[-1];
    }

}

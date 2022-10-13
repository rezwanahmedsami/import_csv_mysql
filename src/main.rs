use std::error::Error;
use std::process;
use mysql::*;
use mysql::prelude::*;

// fn read_csv_and_import(path: &str) -> Result<(), Box<dyn Error>> {
//     // Build the CSV reader and iterate over each record.
//     let mut rdr = csv::Reader::from_path(path)?;
//     let mut total = 0;
//     for result in rdr.records() {
//         // The iterator yields Result<StringRecord, Error>, so we check the
//         // error here.
//         let record = result?;
//         let company_id = record.as_slice();
//         println!("{}", company_id);
//         total += 1;
//         // break;
//     }
//     println!("Total data: {}", total);
//     Ok(())
// }

#[derive(Debug, PartialEq, Eq)]
struct Ext_company{
    ext_company_ids: String
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:@127.0.0.1:3306/test";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    println!("Inserting all data");
    let mut ext_company = vec![];

    let mut rdr = csv::Reader::from_path("./NM_companies.csv")?;
    let mut total = 0;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let company_id = record.as_slice();
        ext_company.push(Ext_company {ext_company_ids: company_id.to_string()});
        total += 1;
        // break;
    }
    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT IGNORE INTO ext_company (ext_company_ids)
          VALUES (:ext_company_ids)",
          ext_company.iter().map(|e| params! {
            "ext_company_ids" => e.ext_company_ids.to_string(),
        })
    )?;

    // Let's select payments from database. Type inference should do the trick here.
    println!("Yay! total data: {} inserted success", total);

    Ok(())
}
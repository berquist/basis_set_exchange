use pyo3::prelude::*;
use serde::Deserialize;
use std::collections::HashMap as Map;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn transform_basis_name(name: &str) -> String {
    name.to_lowercase()
        .replace("/", "_sl_")
        .replace("*", "_st_")
}

#[derive(Debug, Deserialize)]
struct SingleBasisVersion {
    file_relpath: String,
    revdesc: String,
    revdate: String,
    elements: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SingleBasisMetadata {
    display_name: String,
    other_names: Vec<String>,
    description: String,
    // TODO u32
    latest_version: String,
    tags: Vec<String>,
    basename: String,
    relpath: String,
    family: String,
    role: String,
    function_types: Vec<String>,
    // TODO value is Union<String, Vec<String>>...
    auxiliaries: Map<String, String>,
    versions: Map<String, SingleBasisVersion>,
}

#[derive(Debug, Deserialize)]
struct AllMetadata {
    basis_sets: Map<String, SingleBasisMetadata>,
}

fn get_all_basis_metadata(data_dir: &Path) -> Map<String, SingleBasisMetadata> {
    let metadata_file_loc = data_dir.join("METADATA.json");
    let file = File::open(&metadata_file_loc).expect("problem reading METADATA.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("problem deserializing JSON from reader")
}

// fn get_basis_metadata(name: &str, data_dir: &Path) {
//     let tr_name = transform_basis_name(&name);
//     let metadata = get_metadata(&data_dir);
// }

// fn get_basis(name: &str) {
//     let data_dir = Path::new("data");
//     let bs_data = get_basis_metadata(&name, &data_dir);
//     println!("{}", bs_data);
// }

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn bse(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_basis_name() {
        assert_eq!(transform_basis_name("sto-3g"), String::from("sto-3g"));
        assert_eq!(
            transform_basis_name("def2-sv(p)/jk"),
            String::from("def2-sv(p)_sl_jk")
        );
        assert_eq!(transform_basis_name("3-21g*"), String::from("3-21g_st_"));
    }

    #[test]
    fn test_deserialize_single_basis_metadata() -> serde_json::Result<()> {
        let data = r#"{
    "display_name": "STO-3G",
    "other_names": [],
    "description": "STO-3G Minimal Basis (3 functions/AO)",
    "latest_version": "1",
    "tags": [],
    "basename": "STO-3G",
    "relpath": "",
    "family": "sto",
    "role": "orbital",
    "function_types": [
      "gto",
      "gto_spherical"
    ],
    "auxiliaries": {},
    "versions": {
      "0": {
        "file_relpath": "STO-3G.0.table.json",
        "revdesc": "Data from the Original Basis Set Exchange",
        "revdate": "2007-01-15",
        "elements": [
          "1",
          "2",
          "3",
          "4",
          "5",
          "6",
          "7",
          "8",
          "9",
          "10",
          "11",
          "12",
          "13",
          "14",
          "15",
          "16",
          "17",
          "18",
          "19",
          "20",
          "21",
          "22",
          "23",
          "24",
          "25",
          "26",
          "27",
          "28",
          "29",
          "30",
          "31",
          "32",
          "33",
          "34",
          "35",
          "36",
          "37",
          "38",
          "39",
          "40",
          "41",
          "42",
          "43",
          "44",
          "45",
          "46",
          "47",
          "48",
          "49",
          "50",
          "51",
          "52",
          "53"
        ]
      },
      "1": {
        "file_relpath": "STO-3G.1.table.json",
        "revdesc": "Data from Gaussian09",
        "revdate": "2018-06-19",
        "elements": [
          "1",
          "2",
          "3",
          "4",
          "5",
          "6",
          "7",
          "8",
          "9",
          "10",
          "11",
          "12",
          "13",
          "14",
          "15",
          "16",
          "17",
          "18",
          "19",
          "20",
          "21",
          "22",
          "23",
          "24",
          "25",
          "26",
          "27",
          "28",
          "29",
          "30",
          "31",
          "32",
          "33",
          "34",
          "35",
          "36",
          "37",
          "38",
          "39",
          "40",
          "41",
          "42",
          "43",
          "44",
          "45",
          "46",
          "47",
          "48",
          "49",
          "50",
          "51",
          "52",
          "53",
          "54"
        ]
      }
    }
  }"#;
        let single_metadata: SingleBasisMetadata = serde_json::from_str(&data)?;
        println!("{:#?}", single_metadata);

        Ok(())
    }

    // #[test]
    // fn test_get_all_basis_metadata() {
    //     let data_dir = Path::new("src").join("data");
    //     let all_metadata = get_all_basis_metadata(&data_dir);
    //     println!("{:#?}", all_metadata);
    // }
}

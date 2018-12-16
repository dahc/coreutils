extern crate chrono;

use common::util::*;
use std::fs::metadata;
use test_pr::chrono::DateTime;
use test_pr::chrono::offset::Local;

fn file_last_modified_time(ucmd: &UCommand, path: &str) -> String {
    let tmp_dir_path = ucmd.get_full_fixture_path(path);
    let file_metadata = metadata(tmp_dir_path);
    return file_metadata.map(|i| {
        return i.modified().map(|x| {
            let datetime: DateTime<Local> = x.into();
            datetime.format("%b %d %H:%M %Y").to_string()
        }).unwrap_or(String::new());
    }).unwrap_or(String::new());
}


#[test]
fn test_without_any_options() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&[test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![(&"{last_modified_time}".to_string(), &value)]);
}

#[test]
fn test_with_numbering_option() {
    let test_file_path = "test_num_page.log";
    let expected_test_file_path = "test_num_page.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["-n", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![(&"{last_modified_time}".to_string(), &value)]);
}

#[test]
fn test_with_numbering_option_when_content_is_less_than_page() {
    let test_file_path = "test_num_page_less_content.log";
    let expected_test_file_path = "test_num_page_less_content.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["-n", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![(&"{last_modified_time}".to_string(), &value)]);
}

#[test]
fn test_with_numbering_option_with_number_width() {
    let test_file_path = "test_num_page.log";
    let expected_test_file_path = "test_num_page_2.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["-n", "2", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![(&"{last_modified_time}".to_string(), &value)]);
}

#[test]
fn test_with_header_option() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page_header.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    let header = "new file";
    scenario
        .args(&["-h", header, test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
            (&"{header}".to_string(), &header.to_string())
        ]);
}

#[test]
fn test_with_long_header_option() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page_header.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    let header = "new file";
    scenario
        .args(&["--header=new file", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
            (&"{header}".to_string(), &header.to_string())
        ]);
}

#[test]
fn test_with_double_space_option() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page_double_line.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["-d", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
}

#[test]
fn test_with_long_double_space_option() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page_double_line.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["--double-space", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
}

#[test]
fn test_with_first_line_number_option() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page_first_line.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["-N", "5", "-n", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
}

#[test]
fn test_with_first_line_number_long_option() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page_first_line.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["--first-line-number=5", "-n", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
}

#[test]
fn test_with_number_option_with_custom_separator_char() {
    let test_file_path = "test_num_page.log";
    let expected_test_file_path = "test_num_page_char.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["-nc", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
}

#[test]
fn test_with_number_option_with_custom_separator_char_and_width() {
    let test_file_path = "test_num_page.log";
    let expected_test_file_path = "test_num_page_char_one.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["-nc1", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
}

#[test]
fn test_with_valid_page_ranges() {
    let test_file_path = "test_num_page.log";
    let mut scenario = new_ucmd!();
    scenario
        .args(&["--pages=20:5", test_file_path])
        .fails()
        .stderr_is("pr: invalid --pages argument '20:5'")
        .stdout_is("");
    new_ucmd!()
        .args(&["--pages=1:5", test_file_path])
        .succeeds();
    new_ucmd!()
        .args(&["--pages=1", test_file_path])
        .succeeds();
    new_ucmd!()
        .args(&["--pages=-1:5", test_file_path])
        .fails()
        .stderr_is("pr: invalid --pages argument '-1:5'")
        .stdout_is("");
    new_ucmd!()
        .args(&["--pages=1:-5", test_file_path])
        .fails()
        .stderr_is("pr: invalid --pages argument '1:-5'")
        .stdout_is("");
    new_ucmd!()
        .args(&["--pages=5:1", test_file_path])
        .fails()
        .stderr_is("pr: invalid --pages argument '5:1'")
        .stdout_is("");
}

#[test]
fn test_with_page_range() {
    let test_file_path = "test.log";
    let expected_test_file_path = "test_page_range_1.log.expected";
    let expected_test_file_path1 = "test_page_range_2.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["--pages=15", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
    new_ucmd!()
        .args(&["--pages=15:17", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path1, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);
}

#[test]
fn test_with_no_header_trailer_option() {
    let test_file_path = "test_one_page.log";
    let expected_test_file_path = "test_one_page_no_ht.log.expected";
    let mut scenario = new_ucmd!();
    scenario
        .args(&["-t", test_file_path])
        .succeeds()
        .stdout_is_fixture(expected_test_file_path);
}

#[test]
fn test_with_page_length_option() {
    let test_file_path = "test.log";
    let expected_test_file_path = "test_page_length.log.expected";
    let expected_test_file_path1 = "test_page_length1.log.expected";
    let mut scenario = new_ucmd!();
    let value = file_last_modified_time(&scenario, test_file_path);
    scenario
        .args(&["--pages=2:3", "-l", "100", "-n", test_file_path])
        .succeeds()
        .stdout_is_templated_fixture(expected_test_file_path, vec![
            (&"{last_modified_time}".to_string(), &value),
        ]);

    new_ucmd!()
        .args(&["--pages=2:3", "-l", "5", "-n", test_file_path])
        .succeeds()
        .stdout_is_fixture(expected_test_file_path1);
}

#[test]
fn test_with_suppress_error_option() {
    let test_file_path = "test_num_page.log";
    let mut scenario = new_ucmd!();
    scenario
        .args(&["--pages=20:5", "-r", test_file_path])
        .fails()
        .stderr_is("")
        .stdout_is("");
}

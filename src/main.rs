use regex::Regex;
use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

/// メールのテキストから送信日を抜き出し、ファイル名の先頭に付加します。
fn main() {
    const TARGET_DIR: &str = r"D:\temp\data";
    let target_files = get_target_files(TARGET_DIR);

    for file_name in target_files {
        let mut file = File::open(&file_name).unwrap();
        let mut txt = String::new();
        file.read_to_string(&mut txt).unwrap();

        let date = get_date(&txt);

        // ファイル名
        let to_file_name = format!(
            "{}\\{}_{}",
            TARGET_DIR,
            date,
            file_name
                .replace(&format!("{}\\", TARGET_DIR), "")
                .replace("Fwd ", "")
                .replace("FW ", "")
        );
        fs::rename(file_name, to_file_name).unwrap();
    }
}

/// 変換対象のファイルリストを返します。
fn get_target_files(target_dir: &str) -> Vec<String> {
    let target_dir = WalkDir::new(target_dir);
    let mut target_files: Vec<String> = Vec::new();
    for file in target_dir {
        let file = file.unwrap();
        if file.path().is_dir() {
            continue;
        }

        target_files.push(file.path().display().to_string());
    }
    target_files
}

/// 日付を返します。複数取得できた場合は、最も過去の日付を返します。
fn get_date(txt: &str) -> String {
    let reg = Regex::new(r"(Sent: |日付: )(?P<date>.*?)<br").unwrap();

    let mut rtn_date = "99999999".to_string();

    for cap in reg.captures_iter(txt) {
        let date = &cap["date"];
        if !date.contains("年") {
            let date_en = change_date_en(date);
            rtn_date = match rtn_date.cmp(&date_en) {
                Ordering::Less => rtn_date,
                Ordering::Greater => date_en,
                Ordering::Equal => rtn_date,
            }
        } else {
            let date_jp = change_date_jp(date);
            rtn_date = match rtn_date.cmp(&date_jp) {
                Ordering::Less => rtn_date,
                Ordering::Greater => date_jp,
                Ordering::Equal => rtn_date,
            }
        }
    }

    rtn_date
}

/// 英語の送信日から日付を返します。
fn change_date_en(date: &str) -> String {
    let sp: Vec<&str> = date.split_whitespace().collect();
    let year = sp[3];
    let month = sp[1];
    let day = sp[2].replace(",", "");

    // 月を数字に変換
    let month = match month {
        "January" => "01",
        "February" => "02",
        "March" => "03",
        "April" => "04",
        "May" => "05",
        "June" => "06",
        "July" => "07",
        "August" => "08",
        "September" => "09",
        "October" => "10",
        "November" => "11",
        "December" => "12",
        _ => panic!("月の変換に失敗しました。"),
    };

    let mut rtn_date = year.to_string();
    rtn_date.push_str(month);
    rtn_date.push_str(&day);
    rtn_date
}

/// 日本語の送信日から日付を返します。
fn change_date_jp(date: &str) -> String {
    let reg = Regex::new(r"(?P<year>\d+)年(?P<month>\d+)月(?P<day>\d+)日").unwrap();
    let cap = reg.captures(date).unwrap();
    let year = &cap["year"];
    let month = &cap["month"];
    let day = &cap["day"];

    let mut rtn_date = year.to_string();
    rtn_date.push_str(&format!("{:0>2}", month));
    rtn_date.push_str(&format!("{:0>2}", day));
    rtn_date
}

#[test]
/// 英語の送信日
fn get_date_test1() {
    let actual = get_date("Sent: Saturday, November 22, 2008 5:02 PM<br>");
    let expected = "20081122".to_string();
    assert_eq!(expected, actual);
}

#[test]
/// 日本語の送信日
fn get_date_test2() {
    let actual = get_date("日付: 2012年9月5日 21:14<br>");
    let expected = "20120905".to_string();
    assert_eq!(expected, actual);
}

#[test]
/// 複数の送信日
fn get_date_test3() {
    let actual =
        get_date("Sent: Saturday, November 22, 2008 5:02 PM<br>日付: 2012年9月18日 21:14<br>");
    let expected = "20081122".to_string();
    assert_eq!(expected, actual);
}

#[test]
/// 複数の送信日
fn get_date_test4() {
    let actual =
        get_date("Sent: Saturday, November 22, 2008 5:02 PM<br>日付: 2007年9月18日 21:14<br>");
    let expected = "20070918".to_string();
    assert_eq!(expected, actual);
}

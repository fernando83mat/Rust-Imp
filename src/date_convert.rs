use chrono::NaiveDate;

#[inline]
pub fn convert_month(month_str: &str) -> Option<u32> {
    match month_str {
        "Jan." => Some(1),
        "Feb." => Some(2),
        "Mar." => Some(3),  
        "Apr." => Some(4),
        "May" => Some(5),
        "Jun." => Some(6),
        "Jul." => Some(7),
        "Aug." => Some(8),
        "Sept." => Some(9),
        "Oct." => Some(10),
        "Nov." => Some(11),
        "Dec." => Some(12),
        _ => None,
    }
}
pub fn convert_date(date_str: &str) -> Option<NaiveDate> {
    let date_str = date_str.trim_start_matches("Reviewed ");
   

#[cfg(test)]
    use super::*;

    #[test]
    fn test_date_convert() {
        const  DATE_STR: &str = "Reviewed Sept. 13, 2023";
        let convert_date = convert_date(DATE_STR).unwrap();
        assert_eq!(convert_date, NaiveDate::from_ymd_opt(2023, 9, 13).expect("Invalid Date"));
    }   
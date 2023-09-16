use std::num::ParseIntError;
fn number_sum(data:&str) -> Result<u32, ParseIntError>{ 
    let mut ot = Vec::new();
    for line in data.split("\n"){
        ot.push(line.parse::<u32>()?);
    }
    Ok(ot.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_list_sum_ok(){
        assert_eq!(number_sum("323\n392").unwrap(), 323+392);
    }
    #[test]
    fn test_number_list_sum_error(){
        assert!(number_sum("hi\n392").is_err());
    }
}

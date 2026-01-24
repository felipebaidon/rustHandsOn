use std::io::BufRead;
use std:: fs:: File;
use std:: io:: BufReader;

pub fn create_file(filename: String) -> u8
{
    let result = File::create(filename);
    println!("{:?}", result);
    return 0

}

// pub fn overwrite_file() -> u8
// {
//     let result = File::create(filename);
//     result

// }

pub fn open_file(filename: String) -> u8
{
    let file = File::open(filename);
    let result = match file {
        Ok(file) => 0,
        Err(error) => 1,
    };

    result
}

pub fn read_file(filename: String) -> u8
{
    let file = File::open(filename);
    let mut result = 1;
    match file {
        Ok(file) => 
        {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
            result = 0;
        }
        Err(error) => match error.kind()
        {
            std::io:: ErrorKind:: NotFound => {
                println!("File not found! {}", error);
            }
            _=> {
                println!("Error opening file: {}", error);
                }

        },

    };
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_nonexistentfile()
    {
        let result = open_file("non_existent_fle.txt".to_string());
        assert_eq!(result,1);
    }

    #[test]
    fn test_open_existentfile()
    {
        let result = open_file("src/test.txt".to_string());
        assert_eq!(result,0);
    }

    #[test]
    fn test_existent_file_read()
    {
        let result = read_file("src/test.txt".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_nonexistent_file_read()
    {
        let result = read_file("non-existent-file.txt".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_file_creation()
    {
        let result = create_file("newfile.txt".to_string());
        assert_eq!(result, 0);
    }

}

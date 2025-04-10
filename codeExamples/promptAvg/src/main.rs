use std::io;

fn avg(numbers: &[i32]) -> i32 {
    let mut result = 0;
    let elements = numbers.len() as i32;
    for number in numbers {
        result += number;
    }

    result/elements
}

fn main() {

    println!("How many elements do you want to avg?");
    let mut s_Element = String::new();
    io::stdin().read_line(&mut s_Element).unwrap();

    let elements = s_Element.trim().parse().unwrap();
    let mut numbers = vec![0;elements];

    for i in 0..elements
    {
        println!("Enter an element");
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();

        numbers[i] = value.trim().parse().unwrap();
    }

    let result = avg(&numbers);
    println!("The avg is {}", result);

}

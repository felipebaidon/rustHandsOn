
#[derive(Debug)]

enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn Get_largestShape( shapesVec : &Vec<Shape> ) -> &Shape
{
    let areaVec = get_Area(shapesVec);
    let largestArea = areaVec[0];
    
    for area in areaVec.iter()
    {
        if area > largestArea
        {
            largestArea = area;
        }

    }

    largestArea

}

fn get_Area(shapesVec: &Vec<Shape>) -> Vec<Shape>
{
    let areaVec : Vec<i32> = Vec::new();

    shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => areaVec.push(std::f64::consts::PI * radius * radius),
            Shape::Square(length) => areaVec.push(length * length),
            Shape::Triangle(base, height) => areaVec.push((base * height)/2.0),
        })

        areaVec
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(3.0,2.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => (base * height)/2.0,
        })
        .sum();

    println!("Total area: {} sq. units", total_area);

    let largestShape = Get_largestShape(&shapes);

    println!("The largest shape is {:?}", largestShape);
}

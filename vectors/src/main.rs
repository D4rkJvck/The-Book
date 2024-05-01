fn main() {
    let v1 = vec![1, 2, 3, 4, 5];

    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    let third: &i32 = &v1[2];
    println!("The third element in {:?} is {:?}", v1, third);

    let second: Option<&i32> = v2.get(1);
    match second {
        Some(second) => println!("The second element in {:?} is {:?}", v2, second),
        None => println!("There is no second element in {:?}", v2),
    }

    for i in &v1 {
        println!("{i}");
    }

    for i in &mut v2 {
        *i += 50 - *i
    }

    println!("{:?}", v2);

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for elem in &row {
        match elem {
            SpreadSheetCell::Int(n) => println!("{}", n),
            SpreadSheetCell::Float(f) => println!("{}", f),
            SpreadSheetCell::Text(t) => println!("{}", t),
        };
    }

    {
        let v3 = vec![1, 2, 3 ,4];
        println!("Inner Scope: {:?}", v3)
    }
    // println!("Outer Scope: {:?}", v3) //UGLY: Error...
}

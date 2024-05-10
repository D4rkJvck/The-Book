fn main() {
    {
        let v = vec![1, 2, 3];
        let v_iter = v.iter();

        for val in v_iter {
            println!("Got {val}");
        }

        let v1: Vec<_> = v.iter().map(|x| x + 1).collect();
        println!("After Mapping: {:?}", v1);
    }
}

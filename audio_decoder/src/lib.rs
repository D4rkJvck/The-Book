pub fn decode(buffer: &mut [i32], coefficient: [i64; 12], qlq_shift: i16) {
    for i in 12..buffer.len() {
        let prediction = coefficient
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlq_shift;

        buffer[i] = prediction as i32 + buffer[i]
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn break_down() {
        let mut buffer = [0; 20];
        let coefficient = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let qlq_shift = 5;

        decode(&mut buffer, coefficient, qlq_shift);

        println!("{:?}", buffer);

        assert_eq!(buffer, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}

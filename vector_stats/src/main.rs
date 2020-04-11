use std::collections::HashMap;

fn main() {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut integer_vector: Vec<usize> = vec![
        4308, 6068, 4847, 3167, 8846, 9185, 9615, 4383, 8312, 2275, 6240, 3054, 179, 8878, 3317,
        9753, 3695, 7367, 1740, 833, 943, 5329, 5054, 3419, 2100, 5457, 2493, 1086, 3131, 5527,
        7606, 1137, 8174, 4078, 2860, 2890, 8192, 8070, 180, 3753, 7861, 3421, 3260, 4072, 9799,
        9390, 8106, 3627, 4045, 3230, 8121, 685, 949, 4795, 6055, 6759, 4754, 2309, 3355, 1732,
        614, 656, 4413, 6285, 864, 540, 3335, 649, 4128, 7170, 8104, 7854, 1052, 7864, 1984, 8643,
        8980, 5497, 7558, 3909, 3794, 9943, 295, 8384, 6475, 6694, 5636, 2526, 9427, 9095, 144,
        2520, 1242, 3933, 2625, 4106, 6466, 8846, 8451, 912,
    ];

    let mut vector_sum: usize = 0;
    let vector_median: usize;
    integer_vector.sort();

    // vector sum and count
    for element in &integer_vector {
        vector_sum += element;
        let element_counter = map.entry(*element).or_insert(0);
        *element_counter += 1;
    }

    // median
    if integer_vector.len() % 2 == 0 {
        vector_median = (integer_vector[(integer_vector.len() - 1) / 2]
            + integer_vector[(integer_vector.len()) / 2])
            / 2;
    } else {
        vector_median = integer_vector[(integer_vector.len() - 1) / 2];
    }
    let mode = map
        .iter()
        .max_by(|(_xk, xv), (_yk, yv)| xv.cmp(yv))
        .unwrap();

    println!("Input i32 vector: {:?}", integer_vector);
    println!(
        "Vector sum total is: {} in {} elements",
        vector_sum,
        integer_vector.len()
    );
    println!("Vector mean/average: {}", vector_sum / integer_vector.len());
    println!("Vector median: {}", vector_median);
    println!("Vector mode is {} with {} occurences", mode.0, mode.1);
}

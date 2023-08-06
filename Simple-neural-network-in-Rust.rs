// setting main parameters
// 3 neurons, 3 layers

fn main() {
    let input = vec![1.0, 2.0, 1.0];
    let (weight1, weight2, weight3) = (vec![0.1, 1.0, 0.7], vec![-5.0, 2.5, 0.5], vec![-1.0, 2.0, -1.5]);
    let (bias1, bias2, bias3) = (2.5, 3.0, 1.0);

    let output = [
(input[0] * weight1[0] + input[1] * weight1[1] + input[2] * weight1[2] + bias1), 
(input[0] * weight2[0] + input[1] * weight2[1] + input[2] * weight2[2] + bias2), 
(input[0] * weight3[0] + input[1] * weight3[1] + input[2] * weight3[2] + bias3)
];

    println!("{}", output);
}

... 

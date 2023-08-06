// setting main parameters
// 3 neurons, 3 layers

fn main() {
    let (input1, input2, input3) = (vec![1.0, 2.0, 1.0], vec![5.0, 2.5, 3.0], vec![1.0, 6.0, 1.5]);
    let (weight1, weight2, weight3) = (vec![0.1, 1.0, 0.7], vec![-5.0, 2.5, 0.5], vec![-1.0, 2.0, -1.5]);
    let (bias1, bias2, bias3) = (2.5, 3.0, 1.0);

    let output = [
(input1[0] * weight1[0] + input1[1] * weight1[1] + input1[2] * weight1[2] + bias1), 
(input2[0] * weight2[0] + input2[1] * weight2[1] + input2[2] * weight2[2] + bias2), 
(input3[0] * weight3[0] + input3[1] * weight3[1] + input3[2] * weight3[2] + bias3)
];

    println!("{}", output);
}

... 

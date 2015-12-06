use std::env;

fn main() {

    // Get arguments
    let mut arg_expression : Vec<_> = env::args().collect();

    // Remove the original name of the program
    arg_expression.remove(0);

    // Initialize the stack
    let mut stack_int : Vec<i32> = Vec::new();

    // For each token...
    for token in arg_expression.iter() {
        // Get the reference
        match token.as_ref() {
            // If this reference is an operator...
            o @ "+" | o @ "-" | o @ "x" | o @ "/" => {
                // Get the right expression and the left expression
                let right_exp = stack_int.pop().unwrap();
                let left_exp = stack_int.pop().unwrap();
                // Compute the score!
                match o {
                    "+" => stack_int.push(plus(left_exp, right_exp)),
                    "-" => stack_int.push(minus(left_exp, right_exp)),
                    "x" => stack_int.push(mul(left_exp, right_exp)),
                    "/" => stack_int.push(div(left_exp, right_exp)),
                    _ => (),
                }
            },
            // Else, we consider it's a 32 bits integer!
            _ => stack_int.push(token.parse::<i32>().unwrap()),
        }
    }

    // Get the evaluation
    println!("Eval : {}", eval(&mut stack_int));

}

// Function to evaluate the expression
// Pop the last value of the stack
fn eval(stack_int : &mut Vec<i32>) -> i32 {
    return stack_int.pop().unwrap();
}

// Plus function
fn plus(left_exp : i32, right_exp : i32) -> i32 {
    return left_exp + right_exp;
}

// Minus function
fn minus(left_exp : i32, right_exp : i32) -> i32 {
    return left_exp - right_exp;
}

// Mul. function
fn mul(left_exp : i32, right_exp : i32) -> i32 {
    return left_exp * right_exp;
}

// Div. function
fn div(left_exp : i32, right_exp : i32) -> i32 {
    return left_exp / right_exp;
}

pub fn exercise03_05() {
    println!("\x1B[2J"); // clear the terminal
    println!();
    println!(
        "5) the sample mean can be derived as the solution to the following optimization problem:"
    );
    println!("    min_μ sum(||μ - x_i||^2))");
    println!("    5a) Is this problem convex? What class of optimization problem is it?");
    println!("    This optimization problem is indeed convex, because the objective function is the sum of squared
    Euclidean distances from the observations x_t to the optimization variable µ, so it is actually a
    convex quadratic problem. In fact, it is a least squares problem.");
    println!(
        "    5b) Derive the solution to this optimization problem by setting the gradient to zero."
    );
    println!(
        "    To derive the solution, we can compute the gradient of the objective function with respect to µ and set it to zero. The objective function can be expressed as:"
    );
    println!("    f(µ) = sum(||µ - x_i||^2)");
    println!(
        "    To compute the gradient, we can use the fact that the gradient of a function of the form ||µ - x_i||^2 is given by:"
    );
    println!("    ∇f(µ) = 2 * sum(µ - x_i)");
    println!("    Setting the gradient to zero gives us:");
    println!("    2 * sum(µ - x_i) = 0");
    println!("    This can be rearranged to:");
    println!("    sum(µ) = sum(x_i)");
    println!(
        "    Since there are n observations, we can express the sum of µ as n * µ, leading to:"
    );
    println!("    n * µ = sum(x_i)");
    println!("    Finally, we can solve for µ to find the sample mean:");
    println!("    µ = (1/n) * sum(x_i)");
}

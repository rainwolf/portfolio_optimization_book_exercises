pub fn exercise03_06() {
    println!("\x1B[2J"); // clear the terminal
    println!();
    println!(
        "6) the spatial median can be derived as the solution to the following optimization problem:"
    );
    println!("      min_x sum ||x - x_i||");
    println!("6a) Is this problem convex? What class of optimization problem is it?");
    println!(
        "    This optimization problem is convex because the objective function is the sum of Euclidean distances, which is a convex function. Therefore, it is a convex optimization problem."
    );
    println!("      Unlike the sample mean problem, which is a quadratic optimization problem, the spatial median
    involves minimizing a sum of L2-norms rather than squared L2-norms. This makes the problem a
    second-order cone problem (SOCP).");
    println!("6b) Can a closed-form solution be obtained as in the case of the sample mean?");
    println!(
        "    No, a closed-form solution cannot be obtained for the spatial median problem as it involves minimizing a sum of L2-norms, which does not have a closed-form solution. Instead, iterative algorithms such as Weiszfeld's algorithm are typically used to find the spatial median."
    );
    println!(
        "      (I imagine the polynomial to be solved is of degree larger than 4, so no closed-form solution exists.)"
    );
    println!(
        "    (The gradient is undefined at the points themselves, no analytical solution in general)"
    );
    println!(
        "6c) Develop an iterative algorithm to compute the spatial median by solving a sequence of
    weighted sample means. Hint: find a majorizer of the L2-norm in the form of a squared
    L2-norm and then employ the majorization-minimization framework."
    );
    println!(
        "    We can use the majorization-minimization framework to develop an iterative algorithm for computing the spatial median. The idea is to find a majorizer of the L2-norm in the form of a squared L2-norm, which allows us to solve a sequence of weighted sample mean problems."
    );
    println!(
        "    The majorizer can be defined as follows: for a given point x_k, we can define the majorizer M(x) as:"
    );
    println!("    M(x) = sum(||x - x_i||^2 / (||x_k - x_i|| + ε))");
    println!(
        "    where ε is a small positive constant to avoid division by zero. The majorizer M(x) is a convex function that upper bounds the original objective function at the point x_k."
    );
    println!("    The iterative algorithm can then be defined as follows:");
    println!("    1. Initialize x_0 (e.g., as the sample mean of the observations).");
    println!("    2. For each iteration k, compute the majorizer M(x) at the current point x_k.");
    println!(
        "    3. Update x_{{k+1}} by minimizing the majorizer M(x), which can be done by solving a weighted sample mean problem:"
    );
    println!("       x_{{k+1}} = argmin_x M(x)");
    println!(
        "    4. Repeat steps 2 and 3 until convergence (e.g., until the change in x is below a certain threshold)."
    );
    todo!("come back to this later maybe");
}

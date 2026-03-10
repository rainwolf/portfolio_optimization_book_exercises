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
    second-order cone problem (SOCP).")
    println!(
        "6b) Can a closed-form solution be obtained as in the case of the sample mean?"
    );
    println!(
        "    No, a closed-form solution cannot be obtained for the spatial median problem as it involves minimizing a sum of L2-norms, which does not have a closed-form solution. Instead, iterative algorithms such as Weiszfeld's algorithm are typically used to find the spatial median."
    );
    println!("      (I imagine the polynomial to be solved is of degree larger than 4, so no closed-form solution exists.)");
    println!()
}

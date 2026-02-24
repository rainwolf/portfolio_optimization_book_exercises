pub fn exercise03_02() {
    println!();
    println!(
        "2a) The expected value of the sum of T iid normal random variables with mean μ and covariance matrix Σ minus a vector of true means, multiplied by the same transposed."
    );
    println!("Answer: E [ (X_1 - μ)(X_1 - μ)^t + ... + (X_T - μ)(X_T - μ)^t ] = ");
    println!(
        "        E [ (X_1 - μ)(X_1 - μ)^t ] + ... + E [ (X_T - μ)(X_T - μ)^t ] =  // linearity of E"
    );
    println!("        T * Σ  // assume column vector and transpose is row vector");
    println!();
    println!("2b) Same as 2a but with the true mean vector replaced by the sample mean vector.");
    println!("Answer: E [ (X_1 - X̄)(X_1 - X̄)^t + ... + (X_T - X̄)(X_T - X̄)^t ] = ");
    println!(
        "        E [ (X_1 - X̄)(X_1 - X̄)^t ] + ... + E [ (X_T - X̄)(X_T - X̄)^t ]  // linearity of E"
    );
    println!("focus on the first term: E [ (X_1 - X̄)(X_1 - X̄)^t ] = ");
    println!("        E [ (X_1 - μ + μ - X̄)(X_1 - μ + μ - X̄)^t ] = ");
    println!("        E [ (X_1 - μ)(X_1 - μ)^t + (μ - X̄)(μ - X̄)^t ");
    println!(
        "        + (X_1 - μ)(μ - X̄)^t + (μ - X̄)(X_1 - μ)^t ] =  // flip the sign of these two terms"
    );
    println!("        E [ (X_1 - μ)(X_1 - μ)^t + (μ - X̄)(μ - X̄)^t ");
    println!("        - (μ - X_1)(μ - X̄)^t - (μ - X̄)(μ - X_1)^t ]  // sum all and you get");
    println!(
        "        E [ (X_1 - μ)(X_1 - μ)^t + ... + (X_T - μ)(X_T - μ)^t - T * (μ - X̄)(μ - X̄)^t ] = "
    );
    println!(
        "        E [ (X_1 - μ)(X_1 - μ)^t + ... + (X_T - μ)(X_T - μ)^t ] - T * E [ (μ - X̄)(μ - X̄)^t ]   // replace X̄ with its definition and use the fact that E [ X_i - μ ] = 0 or that the X_i's are independent and identically distributed"
    );
    println!("        T * Σ - T * Σ / T = (T - 1) * Σ  // the sample covariance matrix is biased");
}

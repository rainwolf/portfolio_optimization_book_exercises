pub fn exercise03_02() {
    println!();
    println!(
        "The expected value of the sum of T iid normal random variables with mean μ and covariance matrix Σ minus a vector of true means, multiplied by the same transposed."
    );
    println!("Answer: E [ (X_1 - μ)(X_1 - μ)^t + ... + (X_T - μ)(X_T - μ)^t ] = ");
    println!(
        "        E [ (X_1 - μ)(X_1 - μ)^t ] + ... + E [ (X_T - μ)(X_T - μ)^t ] =  // linearity of E"
    );
    println!("        T * Σ  // assume column vector and transpose is row vector");
    println!();
}

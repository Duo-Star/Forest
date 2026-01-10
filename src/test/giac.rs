
use giacrs::{
    context::{release_globals, Context},
    gen::Gen,
    GiacError,
};

fn main() -> Result<(), GiacError> {
    let ctx = Context::new();

    // Create a polynomial and factorize it
    let e = Gen::from_str("x^4", &ctx)?;
    let f = ctx.eval("x^5")?;
    let mut g = e.clone() * &f;
    g /= &e;
    println!("{}", g.factor(&ctx)?);

    // Compute the determinant of a matrix
    let mat = Gen::from_str("[[1,2],[3,4]]", &ctx)?;
    println!("{}", mat.det(&ctx)?.to_int()?);

    // Release giac globals variables to avoid memory leaks
    release_globals();
    Ok(())
}
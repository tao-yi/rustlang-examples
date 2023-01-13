// Make this macro available elsewhere
#[macro_export]
// Establish the name of the macro
macro_rules! vec {
    // `$x:expr`: Match any Rust expression
    // `,*`: allow one or more of them
    ( $( $x:expr ),*) => {
      {
        let mut temp_vec = Vec::new();
        // Generate this command for all expression `($x)`
        $(
          temp_vec.push(($x));
        )*
        temp_vec
      }
    };
}

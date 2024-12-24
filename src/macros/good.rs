/// Tries to get the inner value of the "good" enum variant, or else propagate
///
/// You can propagate the whole enum, an expression, or apply a closure to the whole enum.
///
/// # Examples
///
/// ### Propagate the whole enum
/// Tries to get the inner value or return the whole enum.
///
/// You can propagate all enums that implement the [`Good`] trait. Not just `Result` or `Option`!
/// Deriving [`Propagate`] and have at least one `#[good]` attribute implements `Good` automatically.
///
/// [`Propagate`]: crate::Propagate
/// [`Good`]: crate::traits::Good
/// ```
/// use propagate::{good, Propagate};
/// #[derive(Propagate)]
/// enum Weather {
///     #[good]
///     Sunny(Vec<String>),
///     Cloudy,
///     Rainy,
/// }
///
/// fn list_things_to_do_when_sunny(weather: &Weather) -> &Weather {
///     let todo_list: &[String] = good!(weather);
///     println!("{todo_list:?}");
///     weather
/// }
/// ```
///
/// ### Propagate an expression
/// Tries to get the inner value or return a default value.
///
/// Use a semicolon after your enum, then your expression.
/// ```
/// # use propagate::good;
/// fn average_of_ints(ints: Option<&[i32]>) -> f64 {
///     let ints: &[i32] = good!(ints; 0.0);
///     good!(!ints.is_empty(); 0.0);
///     ints.iter().sum::<i32>() as f64 / ints.len() as f64
/// }
/// ```
/// If the function returns `()`, you can simply write `good!(your_enum;)`.
/// ```
/// # use propagate::good;
/// use std::fs::File;
/// fn open_a_file(path: &str) {
///     let greeting_file = good!(File::open("hello.txt"););
/// }
/// ```
///
/// ### Continue statements
/// Tries to get the inner value or continue a loop, you can also use labels here.
///
/// Also uses a semicolon, but followed by a continue expression.
/// ```
/// # use propagate::good;
/// fn sum_of_strings(strings: &[&str]) -> f64 {
///     let mut sum = 0.0;
///     for s in strings {
///         let num = good!(s.parse::<f64>(); continue);
///         sum += num;
///     }
///     sum
/// }
/// ```
///
/// ### Break statements
/// Tries to get the inner value or break a loop, you can break with value and/or
/// with labels here.
///
/// Also uses a semicolon, but followed by a break expression.
/// ```
/// # use propagate::good;
/// fn peek_success_slice<T, E>(results: &[Result<T, E>]) -> &[Result<T, E>] {
///     let mut index = 0;
///     for res in results {
///         good!(res.is_ok(); break);
///         index += 1;
///     }
///     &results[..index]
/// }
/// ```
///
/// ### Apply closures (two states)
/// Tries to get the good value, or apply closure for the bad value.
/// Requires the enum to implement [`TwoStates`], or exactly one good and one bad variant,
/// with no other variants.
///
/// [`TwoStates`]: crate::TwoStates
///
/// Use a fat arrow (`=>`) after your enum, followed by a closure.
/// You can prepend `else` or `break` before the closure, and will give you a value or break
/// the loop instead of returning.
///
/// ```
/// # use propagate::good;
/// fn get_or_double_it(num: Result<u32, u32>) -> u32 {
///     let got_num = good!(num => |num| num * 2);
///     println!("Got number: {got_num}");
///     0
/// }
/// ```
///
/// ```rust no_run
/// # use propagate::good;
/// use std::net::TcpListener;
/// use std::io::Error;
/// use std::io::ErrorKind;
/// fn main() {
///     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
///     let err: ErrorKind = loop {
///         let (stream, addr) = good!(listener.accept() => break |err: Error| err.kind());
///         // Do something with stream and address
///         todo!()
///     };
///     // Handle error kind
/// }
/// ```
///
/// ### Apply closures (not two states)
/// Tries to get the inner value, or apply the closure to **the whole enum**.
///
/// Use a fat arrow (`=>`) after your enum, then a `full` keyword, followed by a closure.
/// You can prepend `else` or `break` before the closure, and will give you a value or break
/// the loop instead of returning.
///
/// ```rust no_run
/// # use propagate::{good, Propagate};
/// // An enum that doesn't implement `TwoStates`
/// #[derive(Propagate)]
/// enum HttpResponse {
///     Continue,
///     #[good]
///     Ok(String),
///     NotFound,
///     ServerError,
/// }
///
/// impl HttpResponse {
///     fn status_code(&self) -> u32 {
///         match self {
///         HttpResponse::Continue => 100,
///         HttpResponse::Ok(_) => 200,
///         HttpResponse::NotFound => 404,
///         HttpResponse::ServerError => 500,
///         }
///     }
/// }
///
/// fn get_message() -> HttpResponse {
///     // Suppose it blocks the calling thread until it receives an HTTP response
///     todo!()
/// }
///
/// fn main() {
///     let err_code: u32 = loop {
///         let msg: String = good!(get_message() => full break |resp| resp.status_code());
///         // Do something with message
///     };
///     // Handle error code
/// }
/// ```
///
/// ### Run consuming closures
/// Tries to get the inner value, or run a consuming closure, after that you can return a value,
/// continue, break, or default to a value.
///
/// Prepend a `do` keyword before your closure, end with semicolon. Then add return expressions,
/// continue or break statements (optional).
///
/// ```
/// # use propagate::good;
/// fn sum_and_errors<'a>(results: &'a [Result<i32, &'a str>]) -> (i32, Vec<&'a str>) {
///     let mut errors: Vec<&str> = Vec::new();
///     let mut sum = 0;
///     for res in results {
///         let num = good!(res => do |msg| errors.push(msg); continue);
///         sum += num;
///     }
///     (sum, errors)
/// }
/// ```
#[macro_export]
macro_rules! good {
    ($enum_:expr) => {
        $crate::__take!(Ok, Err, $crate::Good::good($enum_))
    };
    ($enum_:expr; $($propagate:tt)*) => {
        $crate::__take!(Ok, Err, $crate::Good::good($enum_); $($propagate)*)
    };
    ($enum_:expr => full $($propagate_closure:tt)*) => {
        $crate::__take!(Ok, Err, $crate::Good::good($enum_) => $($propagate_closure)*)
    };
    ($enum_:expr => $($propagate_closure:tt)*) => {
        $crate::__take!(Ok, Err, $crate::TwoStates::two_states($enum_) => $($propagate_closure)*)
    };
}

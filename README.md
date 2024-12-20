# propagate

The `propagate` crate provides simple and flexible propagation of successful and error values using intuitive macros.

## propagate crate

The `propagate` crate simplifies the propagation of enum values into "good" or "bad" variants, offering a more flexible and concise alternative to traditional error handling methods. Manage how enums are propagated, whether by returning, continuing, breaking, or providing default values. Any enum that implements the `Good` or `Bad` trait can work with the `good!` or `bad!` macros. `Result`, `Option`, `ControlFlow`, `bool`, and primitive integers implement the `Good` and `Bad` trait. 

### Usage Examples

```rust
let mut a: Result<Vec<i32>, String> = Ok(vec![0, 1, 2]);
let failed: Vec<String> = Vec::new();

//! Different handling for `Err`
//! ============================

//! `?`-like
//! --------
let b: Vec<i32> = good!(a);   // returns `Result<Vec<i32>, String>`, similar to the `?` operator

//! Simple `return`/ `continue`/ `break`/ default value
//! ---------------------------------------------------
let b: Vec<i32> = good!(a; 42);         // return 42
let b: Vec<i32> = good!(a; return ());  // explicit return, up to personal preference
let b: Vec<i32> = good!(a;);            // shorthand for returning (), notice the semicolon
let b: &[i32] = good!(&a;);             // immutable reference
let b: &mut [i32] = good!(&mut a;);     // mutable reference
let b: Vec<i32> = good!(a; continue);          // continue
let b: Vec<i32> = good!(a; continue 'label);   // continue a label
let b: Vec<i32> = good!(a; break);             // break
let b: Vec<i32> = good!(a; break 0);           // break with value
let b: Vec<i32> = good!(a; break 'label 0);    // break a label with value
let b: Vec<i32> = good!(a; else vec![0]);      // set default value to [0], similar to `unwrap_or`, but lazily evaluated

//! Apply closure to `return`/ `continue`/ `break`/ default value/ consumer function (`do` statement)
//! -------------------------------------------------------------------------------------------------
use anyhow::bail;
let b: Vec<i32> = good!(a => |res| bail!("{:?}", res));  // return `anyhow::Result`
// let b: Vec<i32> = good!(a => continue |res| res);  // Won't compile: continue doesn't take any value, just use `; continue` instead
let b: Vec<i32> = good!(a => break |res| res);  // break with result 
let b: Vec<i32> = good!(a => else |res| vec![res.unwrap().parse()]);  // default value to closure(result)
let b: Vec<i32> = good!(a => do |res| failed.push(res.unwrap()););  // do something with the result, then return ()
let b: Vec<i32> = good!(a => do |res| eprintln!("{:?}, res"); continue);  // do something with the result, then continue

//! bad! macro (bad counterpart of `good!`)
//! ----------
let c: String = bad!(a);  // get the value of the bad variant, returns `Result<Vec<i32>, String>`
let c: String = bad!(a; else "No error".to_owned());  // set default value to String "No error"

//! take! macro (similar to `good!` but works on any variant)
//! -----------
// get a specific enum variant, then continue. Remember to pass the arguments in square brackets
let c: String = take!(a, Result::Err[v]; continue);  

 // get the value of the `Ok` variant, then return ()
let b: Vec<i32> = take!(a, Result::Ok[v];); 
```

### Why Not Just Use `?`, `unwrap_or_else`, `if let`, or `match`?

1. **Flexibility**:
    - The `?` operator only returns the whole enum, and cannot directly use control flow statements like bare `return`, `continue`, or `break`, and `unwrap` methods can't have control flow keywords. So, we will have to use `if let` or `match`, which can be more verbose. This crate provides macros simplify this process.

    ```rust
    // Task: get a vec or continue

    let a: Result<Vec<i32>, String> = Err("an error".to_owned());
    // Failed attempts to continue
    // let b: Vec<i32> = a?;  // We wanted to continue, not return
    // let b: Vec<i32> = a.unwrap_or_else(continue);     // Won't compile
    // let b: Vec<i32> = a.unwrap_or_else(|| continue);  // Won't compile
    // let b: Vec<i32> = a.unwrap_or(continue);  // This will compile, but will continue regardless of the variant

    // the usual way:
    let b: Vec<i32> = match a {
        Ok(b) => b,
        Err(_) => continue,
    };
    // verses with `good!` macro
    let b: Vec<i32> = good!(a; continue);
    ```

2. **References**:
    - The `?` operator does not work with references like `&Result` or `&Option`, and `unwrap` methods take ownership. To reference the inner value, developers often need to use `if let` or `match`. The `good!` macro streamlines this process.

    ```rust
    // Task: get the good value or return ()
    let mut a: Result<Vec<i32>, String> = Ok(vec![1, 2, 3]);
    
    // error: the trait `Try` is not implemented for `&Result<Vec<i32>, String>`
    // besides, this statement "impies" to return `&Result<Vec<i32>, String>`, not `()`
    // let b: &[i32] = (&a)?;

    // doing it the usual way...
    // taking an immutable reference
    let b: &[i32] = if let Ok(ref b) = a { b } else { return };
    // taking a mutable reference
    let b: &mut [i32] = if let Ok(ref mut b) = a { b } else { return };
    // taking ownership
    let b: Vec<i32> = if let Ok(b) = a { b } else { return };

    // verses with `good!` macro
    // taking an immutable reference
    let b: &[i32] = good!(&a;);
    // taking a mutable reference
    let b: &mut [i32] = good!(&mut a;);
    // taking ownership
    let b: Vec<i32> = good!(a;);
    ```

3. **Propagate Good/Specific Variants**:
    - The `?` operator only tries to get the **"good"** variant or propagate the whole enum, while `unwrap` methods can't even return. The `bad!` and `take!` macros provide a straightforward way to handle these scenarios. `bad!` tries to get the value of the **"bad"** variant, while `take!` tries to get the value of the **specific** variant.

    ```rust
    // Task: get the error value or return the whole thing
    let a: Result<Vec<i32>, String> = Ok("an error".to_owned());
    
    // error: expected `String`, found `Vec<i32>`
    // let b: String = a?;

    // doing it the usual way...
    let b: String = if let Err(b) = a { b } else { return a };

    // `bad!` comes in handy!
    let b: String = bad!(a);

    // You can also use `take!`
    let b: String = take!(a, Result::Err[v]);
    ```

4. **Custom Types**:
    - Any type that implements the `Good` or `Bad` trait can propagate using the `good!` or `bad!` macro, including `Result`, `Option`, and `ControlFlow`. You can derive `Good` or `Bad` and mark the variants with `#[good]` or `#[bad]`. To use `?` on a custom enum, the unstable trait `Try` has to be implemented, but `Try` is not generic, meaning it cannot be overloaded for the same enum. You can, however, overload `#[good]` and `#[bad]`.

    ```rust
    #[derive(Good, Bad)]
    enum MyMsg {
        #[good]
        SuccessMsg(String),
        #[good]
        InfoMsg(String),
        DebugMsg(String),
        #[bad]
        ErrorMsg(String),
        #[bad]
        ErrorCode(i32),
    }

    let my_msg = MyMsg::SuccessMsg("a success message".to_owned());
    
    // only `SuccessMsg` and `InfoMsg` will not propagate
    let good_msg: &str = good!(&my_msg;);
    
    // only `ErrorMsg` will not propagate, even `ErrorCode` will propagate, which is marked as #[bad]
    let bad_msg: &str = bad!(&my_msg;);
    
    // only `DebugMsg` will not propagate
    let debug_msg: &str = take!(&my_msg, MyMsg::DebugMsg[v];);
    ```

The `propagate` crate offers a powerful and flexible alternative to traditional error handling in Rust, making it easier to write clear and concise code while maintaining control over error propagation.

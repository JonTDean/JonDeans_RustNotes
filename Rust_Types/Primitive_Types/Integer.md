# Integer Types in Rust

## **What defines an Integer in Rust?**

In Rust, we have the ability to assign <i>**numerical values**</i> to a type of data known as an <a href="https://en.wikipedia.org/wiki/Integer_(computer_science)">Integer Type</a>. Integers require an **<i>amount</i>** (ex: 1, 2, 3, etc.); also it require us to specify the **<i>accuracy</i>** which is known as <a href="https://en.wikipedia.org/wiki/Precision_(computer_science)">precision</a>.

## **What Integer Types are available?**

In rust we have Integer specifications which are known as Signed and Unsigned variables. These allow us to dictate our **<i>Range</i>**

1. Signed Variables store positive and negative integers.

2. Unsigned variables store Positive integers with a slightly larger value comparative to their Signed counter parts.

### **Signed Integers**

| Type | Min Exponent: | Min Numerical:                                        | Max Exponent: | Max Numerical:                                      |
|------|---------------|-------------------------------------------------------|---------------|-----------------------------------------------------|
| i8   | -(2^7)        | -128                                                  | (2^7)-1       | 127                                                 |
| i16  | -(2^15)       | -32,768                                               | (2^15)-1      | 32,767                                              |
| i32  | -(2^31)       | -2,147,483,648                                        | (2^31)-1      | 2,147,483,647                                       |
| i64  | -(2^63)       | -9,223,372,036,854,775,808                            | (2^63)-1      | 9,223,372,036,854,775,807                           |
| i128 | -(2^127)      | -170,141,183,460,469,231,731,687,303,715,884,105,728  | (2^127)-1     | 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| f32  | -(3.4028235 × 10^21) | -3.402823500000000000000000000000000000000 | 3.4028235 × 10^21 | 3.402823500000000000000000000000000000000
| f64  | -( 1.7976931348623 × 10^28) | -1.797693134862315700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000| 1.7976931348623000 × 10^28 | 1.797693134862315700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000 |

### **Unsigned Integers**

| Type | Min Exponent:  | Min Numerical: | Max Exponent: | Max Exponent:                                       |
|------|----------------|----------------|---------------|-----------------------------------------------------|
| u8   | 0              | 0              | (2^8)-1       | 255                                                 |
| u16  | 0              | 0              | (2^16)-1      | 65,535                                              |
| u32  | 0              | 0              | (2^32)-1      | 4,294,967,295                                       |
| u64  | 0              | 0              | (2^64)-1      | 18,446,744,073,709,551,615                          |
| u128 | 0              | 0              | (2^128)-1     | 340,282,366,920,938,463,463,374,607,431,768,211,456 |

### **What if we cast a variable without calling it's type?**

```rust
    let x = 32 // The Immutable Variable of X has a value of 32.
```

<sub>If a variable has been called without a Type cast, in Rust, then our <a href="https://stackoverflow.com/questions/55903243/what-is-the-default-integer-type-in-rust"> integer will default to i32.</a></sub>

## **What is Precision and How does it affect my Integers?**

Precision affects how large of a Value your Variable can store, many people may never have to switch away from the generic **i32**.

When you're trying to quantify data that exists in exorbitant amounts such as 2<sup>127</sup>-1, **i32** will not be sufficient as it's maximum possible positive value is 2<sup>31</sup>-1. Because of this we have the option of using another integer type which is known as **i128** (or **u128**) which has the maximum upper limit of 2<sup>127</sup>-1, exactly what we need!

<sub>Deans Non Fun Facts: 2<sup>127</sup>-1 is also the largest prime number found by hand! In non-lemon language this equates to *170,141,183,460,469,231,731,687,303,715,884,105,727* !</sub>

However there is a tax to using these *Long Integers* <sub><sup>- which are i64, u64, i128, u128 -</sup></sub>, and that is in the form of increased memory usage when the variable is being stored.

----

## **Great Sources on Integers**:

1. [Oregonstate pdf on Precision](http://sites.science.oregonstate.edu/~landaur/INSTANCES/WebModules/1_ComputerPrecision/PrecisionFiles/Pdfs/PrecisionI_15Sept.pdf)

2. [Default of unspecified INTEGER type in Rust](https://github.com/rust-lang/rfcs/blob/master/text/0212-restore-int-fallback.md)

3. [Why does a byte only have 0 to 255?](https://stackoverflow.com/questions/4986486/why-does-a-byte-only-have-0-to-255)

4. [Broad look on Integer Types in Rust](https://medium.com/@marcinbaraniecki/on-integer-types-in-rust-b3dc1b0a23d3)

5. [Downside to using Long Integers?](https://stackoverflow.com/questions/26409117/why-use-integer-instead-of-long)

6. [Calculations done on Desmos](https://www.desmos.com/scientific)

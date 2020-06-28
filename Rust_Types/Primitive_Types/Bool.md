# Booleans in Rust

## Boolean States

In rust there are two states for boolean values.

```Rust
true  :: If cast into an integer is 1
false :: If cast into an integer is 0
```

## Boolean Operators and Logic Gates

![digital-logic-gates-truthtables](https://instrumentationtools.com/wp-content/uploads/2017/07/instrumentationtools.com_digital-logic-gates-truthtables.png)

There are also operators in rust that allow us to manipulate the Boolean logic via Binary and Unary Operators.

### **Unary:**

1. Logical NOT: Whatever this operator is next to is reversed.
    * Operator: ``` ! ``` :: ``` LOGICAL NOT ```
    * Example:

    ``` Rust
        fn main() {
            let y = true;

            if y == !true{
                println!("Y is equal to False");
            }else{
                println!("Y is equal to True");
            }
        }

        // Output: Y is equal to True

    ```

### **Binary:**

1. **Logical AND**: *IF* both values are true then the output will be true.

    * Operator: ``` && ``` :: ``` LOGICAL AND ```

    * Example:

    ``` Rust
        fn main() {
            let y = 2;
            let x = 3;

            if y == !false{
                println!("Y is equal to True");
            }else{
                println!("Y is equal to False");
            }
        }

        // Output: Y is equal to 2, and X is equal to 3.

    ```

2. **Logical OR**: *IF* either values are true *OR* both values are true then the output will be true.

    * Operator: ``` || ``` :: ``` LOGICAL OR ```

    * Example:

    ``` Rust
        fn main() {
            let y = 2;
            let x = 3;

            if y == !false{
                println!("Y is equal to True");
            }else{
                println!("Y is equal to False");
            }
        }

        // Output: Y is equal to 2, and X is equal to 3.

    ```

3. **Logical NAND**: *IF* **NEITHER** values are true *OR* a **SINGULAR** value is true then the output will be true.

    * Operator: ``` !& ``` :: ``` LOGICAL NOT + LOGICAL AND ```

    * Example:

    ``` Rust

    ```

4. **Logical NOR**: *IF* **NEITHER** value is true then the output will be true.

    * Operator: ``` !| ``` :: ``` LOGICAL NOT + LOGICAL OR ```

    * Example:

    ``` Rust

    ```

5. **Logical XNOR**: *IF* **NEITHER** value is true *OR* both values **ARE** true then the output will be true.

    * Operator: ``` !^ ``` :: ``` LOGICAL NOT + LOGICAL XOR ```

    * Example:

    ``` Rust

    ```

6. Is Equal To: The Operator on the left is compared to the Operator on the Right, if the values are equal to each other then the result is true.

    * Operator:  ``` == ``` :: ``` IS EQUAL TO ```

    * Example:

    ``` Rust
    fn main() {
        let y = true;

        if y == true{
            println!("Y is equal to True");
        }else{
            println!("Y is equal to False");
        }
    }
    ```

7. Is Not Equal To:

8. Greater Than

9. Less than

10. Greater Than or Equal To:

11. Less Than or Equal To:

----

## Great Sources on Booleans:

1. [Booleans in Rust](https://doc.rust-lang.org/std/primitive.bool.html)

2. [Operators in Rust](https://doc.rust-lang.org/reference/expressions/operator-expr.html)

3. [Great Video on Logic Gates](https://www.youtube.com/watch?v=gI-qXk7XojA) && [A Base look on Logic Gates](https://www.youtube.com/watch?v=RhS-AL2ZcyE)

4. []()

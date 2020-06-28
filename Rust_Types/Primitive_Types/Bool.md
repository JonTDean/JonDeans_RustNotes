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

    * Operator: ``` !Expression ``` :: ``` LOGICAL NOT ```

    * Example:

    ``` Rust
        fn main() {
            let y = true;

            if !y{
                println!("Y is equal to False");
            }else{
                println!("Y is equal to True");
            }
        }

        // Output: Y is equal to True.
    ```

### **Binary:**

1. Is Equal To: The Operator on the left is compared to the Operator on the Right, if the values are equal to each other then the result is true.

    * Operator:  ``` Expression == Expression ``` :: ``` IS EQUAL TO ```

    * Example:

    ``` Rust
        fn main() {
            let y = true;

            if y{
                println!("Y is equal to True");
            }else{
                println!("Y is equal to False");
            }
        }

        // Output: Y is equal to True
    ```

2. **Logical AND**: *IF* both values are true then the output will be true.

    * Operator: ``` Expression && Expression ``` :: ``` LOGICAL AND ```

    * Example:

    ``` Rust
        fn main() {
            let y = 0;
            let x = 1;

            if  y == 0 && x == 1{
                println!("Y and X evaluate to true");
            }else{
                println!("Either Y or X are false, or both are False");
            }
        }

        // Output: Y and X evaluate to true
    ```

3. **Logical OR**: *IF* either values are true *OR* both values are true then the output will be true.

    * Operator: ``` Expression || Expression ``` :: ``` LOGICAL OR ```

    * Example:

    ``` Rust
        fn main() {
            let y = 0;
            let x = 1;

            if  y == 0 || x == 1{
                println!("Y or X evaluate to true");
            }else{
                println!("Neither Y nor X are true, so both are False");
            }
        }

        // Output: Y and X evaluate to true
    ```

4. **Logical NAND**: *IF* **NEITHER** values are true *OR* a **SINGULAR** value is true then the output will be true.

    * Operator: ``` Expression && !Expression ``` :: ``` LOGICAL NOT + LOGICAL AND ```

    * Example:

    ``` Rust
        fn main() {
            let x = true;
            let y = false;

            if  x && !y{
                println!("Y and X evaluate to true");
            }else{
                println!("Either Y or X are false, so both are False");
            }
        }

        // Output: Y and X evaluate to true
    ```

5. **Logical NOR**: *IF* **NEITHER** value is true then the output will be true.

    * Operator: ``` Expression || !Expression ``` :: ``` LOGICAL NOT + LOGICAL OR ```

    * Example:

    ``` Rust
        fn main() {
            let x = 3;
            let y = 2;

            if y == 2 || x == 2{
                println!("Y or X evaluate to true");
            }else{
                println!("Either Y or X are false, or both are False");
            }
        }

        // Output: Y is equal to True
    ```

6. Is Not Equal To:

    * Operator:  ``` Expression != Expression ``` :: ``` IS NOT EQUAL TO ```

    * Example:

    ``` Rust
        fn main() {
            let y = true;

            if y != false{
                println!("Y is equal to True");
            }else{
                println!("Y is equal to False");
            }
        }

        // Output: Y is equal to True
    ```

7. Greater Than

    * Operator:  ``` Expression > Expression ``` :: ``` IS GREATER THAN ```

    * Example:

    ``` Rust
        fn main() {
            let y = 1;

            if y > 0{
                println!("Y evaluates to True");
            }else{
                println!("Y evaluates to False");
            }
        }

        // Output: Y evaluates to True
    ```

8. Less than

    * Operator:  ``` Expression < Expression ``` :: ``` IS LESS THAN ```

    * Example:

    ``` Rust
        fn main() {
            let y = 1;

            if y < 2{
                println!("Y evaluates to True");
            }else{
                println!("Y evaluates to False");
            }
        }

        // Output: Y evaluates to True
    ```

9. Greater Than or Equal To:

    * Operator:  ``` Expression >= Expression ``` :: ``` IS GREATER THAN OR EQUAL TO ```

    * Example:

    ``` Rust
        fn main() {
            let y = 20;

            if y >= 20{
                println!("Y evaluates to True");
            }else{
                println!("Y evaluates to False");
            }
        }

        // Output: Y evaluates to True
    ```

10. Less Than or Equal To:

    * Operator:  ``` Expression <= Expression ``` :: ``` IS LESS THAN OR EQUAL TO ```

    * Example:

    ``` Rust
        fn main() {
            let y = 1;

            if y <= 20{
                println!("Y evaluates to True");
            }else{
                println!("Y evaluates to False");
            }
        }

        // Output: Y evaluates to True
    ```

----

## Great Sources on Booleans:

1. [Booleans in Rust](https://doc.rust-lang.org/std/primitive.bool.html)

2. [Operators in Rust](https://doc.rust-lang.org/reference/expressions/operator-expr.html)

3. [Great Video on Logic Gates](https://www.youtube.com/watch?v=gI-qXk7XojA) && [A Base look on Logic Gates](https://www.youtube.com/watch?v=RhS-AL2ZcyE)

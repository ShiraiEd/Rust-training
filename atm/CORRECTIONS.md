# ATM Project - Corrections and Improvements

## 1. `login()` — `.all()` changed to `.any()`

**Problem:** `.all()` returns `true` only if **every** user in the Vec matches the name and password. With 2+ users, login always fails.

**Fix:** `.any()` returns `true` if **at least one** user matches.

```rust
// Before (broken)
.all(|x| x.name == name && x.password == password)

// After (correct)
.any(|x| x.name == name && x.password == password)
```

---

## 2. `pay_debts()` — E0716: temporary value dropped while borrowed

**Problem:** Chaining `.trim().to_lowercase().as_str()` in a single `let` creates a temporary `String` that gets dropped immediately, leaving a dangling `&str` reference.

**Fix:** Split into two steps — store the owned `String` first, then call `.as_str()` at the `match` site.

```rust
// Before (E0716 error)
let input = input.trim().to_lowercase().as_str();
match input { ... }

// After (correct)
let input = input.trim().to_lowercase();
match input.as_str() { ... }
```

---

## 3. Compound assignment operators (`-=`)

**Problem:** Verbose subtraction repeating the variable name.

**Fix:** Use `-=` compound assignment — shorter, idiomatic, less error-prone.

```rust
// Before
user.balance = user.balance - user.debt;
user.debt = user.debt - amount;

// After
user.balance -= user.debt;
user.debt -= amount;
```

---

## 4. `0f64` replaced with `0.0`

**Problem:** `0f64` is a valid float literal but unconventional.

**Fix:** `0.0` is the standard, readable way to write float literals in Rust.

```rust
// Before
user.debt = 0f64;

// After
user.debt = 0.0;
```

---

## 5. `read_input()` helper function — DRY principle

**Problem:** The same 4-line pattern was repeated ~10 times throughout the code:

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");
let input = input.trim().to_string();
```

**Fix:** Extracted into a single reusable function:

```rust
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
```

Now every input read is just `let name = read_input();`.

---

## 6. Early return guard clause in `pay_debts()`

**Problem:** If the user had no debt (`debt == 0.0`), the function silently did nothing — no feedback to the user.

**Fix:** Check for the "nothing to do" case first and return early. This also flattens the nesting.

```rust
// Before (silent, deeply nested)
if user.debt > 0.0 {
    // ... entire function body indented
}

// After (guard clause)
if user.debt <= 0.0 {
    println!("You have no debts!");
    return;
}
// ... rest of function at normal indentation
```

---

## 7. Consistent `{:.2}` float formatting

**Problem:** Some `println!` calls used `{}` for money values, which can display numbers like `99.99999999997`.

**Fix:** All money values now use `{:.2}` to always show exactly 2 decimal places.

```rust
// Before
println!("Balance: {}", user.balance);

// After
println!("Balance: {:.2}", user.balance);
```

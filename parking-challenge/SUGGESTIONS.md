# Suggestions for `parking-challenge`

## 1. Fix Rust Edition

Your `Cargo.toml` uses `edition = "2024"` which is not a stable Rust edition. Change it to `"2021"` or use `"2024"` only if you're on nightly with that feature enabled.

## 2. Return a value from `calculate_price` instead of printing

Right now `calculate_price` both calculates **and** prints. It's better to separate concerns — have it return the price and let the caller decide what to do with it.

```rust
fn calculate_price(car: &Car) -> f64 { ... }
```

## 3. Consistent language

You mix Portuguese (`horas`, `tipo`, `carro`, `Numero do carro`) and English (`plate`, `hours`, `Employee`). Pick one and stick with it for readability.

## 4. Fix error messages

All three `expect()` calls say `"Plate input error"` even for hours and type inputs. Give each a proper message.

## 5. Reduce duplicated pricing logic

The VIP and Customer branches are almost identical (only the rate differs). You could extract the surcharge logic:

```rust
fn price_for_rate(hours: u16, rate: u16) -> u16 {
    let base = hours * rate;
    if hours > 8 { base + 20 } else { base }
}
```

## 6. Handle invalid type more gracefully

Currently, an invalid type silently defaults to `Customer`. Consider looping until the user provides a valid type, or exiting with a clear error.

## 7. Use struct shorthand

Instead of:

```rust
Car { plate: plate, hours: horas, tipo: tipo }
```

You can use field init shorthand when the variable name matches:

```rust
Car { plate, hours: horas, tipo }
```

## 8. Add unit tests

A great Rust practice — add a `#[cfg(test)]` module to test your pricing logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn employee_is_free() { ... }

    #[test]
    fn vip_surcharge_over_8h() { ... }
}
```

## 9. Consider using `impl` on `Car` or `Tipo`

You could make `calculate_price` a method on `Car`:

```rust
impl Car {
    fn price(&self) -> u16 { ... }
}
```

## 10. Input validation for plate

There's no validation on the plate format. You could add a basic check (e.g., length, alphanumeric pattern) depending on your requirements.

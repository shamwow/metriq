# Metriq - Logging for Solana Smart Contracts

## What Is It

Provides functions to produce structured logs in solana smart contracts.

See benchmarks for speed comparisons.

## Usage

```rust
use metriq;
use metriq::FastDisplay;

fn example_function() {
    // Output: some log message^another message^
    metriq::log!("some log message", "another message");
    // Output: key1`value^key2`1^key3`b1^key4`b0^
    metriq::log!(
        metriq::kv("key1", "value"),
        metriq::kv("key2", 1),
        metriq::kv("key3", true),
        metriq::kv("key4", false),
    );
}
```

`metriq::kv` accepts a `&str` key and any value which implements the `metriq::FastDisplay` trait.

The delimiter between messages in a `log!` is set to `^` and the delimiter between key value pairs is set to `` ` ``.

`true` gets converted to `b1`. `false` gets converted to `b0`.

## Benchmarks

Benchmarks compare speed of formatting using `format!` and `metriq::fast_fmt!`.


| type                       | `format!` | `metriq::fast_fmt!` |
|----------------------------|-----------|---------------------|
| `Pubkey`                   | 2.8627 us | 2.7455 us           |
| `&str`                     | 248.97 ns | 225.68 ns           |
| `i32`                      | 105.85 ns | 75.998 ns           |
| `ui32`                     | 97.742 ns | 73.063 ns           |
| key pair (value == `&str`) | 254.15 ns | 207.01 ns           |

Ran on:
```
MacBook Pro (16-inch, 2019)
2.4 GHz 8-Core Intel Core i9
32 GB 2667 MHz DDR4
30% Battery
```

## Development

To run benchmarks: `cargo bench`

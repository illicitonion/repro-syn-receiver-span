# repro-syn-receiver-span

Bad spans:
```
$ cargo build -p example
error: Saw receiver with span #0 bytes(63..64)
 --> example/src/lib.rs:7:19
  |
7 |     fn by_mut_ref(&mut self) -> bool {
  |                   ^

error: Saw receiver with span #0 bytes(126..130)
  --> example/src/lib.rs:12:12
   |
12 |     fn own(self) -> bool {
   |            ^^^^

error: Saw receiver with span #0 bytes(188..191)
  --> example/src/lib.rs:17:16
   |
17 |     fn own_mut(mut self) -> bool {
   |                ^^^

error: Saw receiver with span #0 bytes(253..254)
  --> example/src/lib.rs:22:15
   |
22 |     fn by_ref(&self) -> bool {
   |               ^

error: aborting due to 4 previous errors

error: could not compile `example`.

To learn more, run the command again with --verbose.
```

Good spans:
```
$ cargo run -p parses-str 2>/dev/null
bytes(21..30)
bytes(86..90)

```

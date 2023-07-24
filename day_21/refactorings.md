# Refactorings

## Multiplication
- `a * b` works,  because `a` `*` => `/` `a`, which can be done on both sides of the equation, leaving `b` and `rhs/a`.
- `v * b` works, because `b` `*` => `/` `b`, which can be done on both sides of the equation, leaving `v` and `rhs/b`.

# Division
- `a / b` does not work, because `a` `/` => `*` `a` => `a^2/b` and `rhs * a`.
  
Instead we need:
- `b` `*` => `a` and `rhs * b`, then `rhs` `/` => `a/rhs` and `b`

-- we know that b may be v: 
- `a / v`, in this case we still need `v` `*` => `a` and `rhs * v`, then `rhs` `/` => `a/rhs` and `v`

it could be said, we should divide a by rhs (instead of `rhs OP token`)

- `v / b` works, because `b` `/` => `*` `b` => `v` and `rhs * b`

## Addition
- `a + b` works, because `a` `+` => `-` `a` => `b` and `rhs - a`
- `v + b` works, because `b` `+` => `-` `b` => `v` and `rhs - b`

## Subtraction
- `a - b` does not work, because `a` `-` => `+` `a` => `a+a - b` and `rhs + a` (instead of `-b` and `rhs - a`)

so we can subtract (not reverse op), then multiply lhs by `-1`.

- 3-2 = 1
- 2 = 1-3
- -2 = 1-3
- -2 = -2

- `v - b` works, because `b` `-` => `+ b` => `v` and `rhs + b`
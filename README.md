# Dice Roller

A simple dice roller made with rust.

# Usage

## Clone

```sh
git clone http://github.com/juanbelieni/dice-roller.git
```

## Build

```sh
cargo build --release
```


## Run
```
./target/release/dice-roller
```

After it, write your roll like this:
```md
d<SIDES> + ...
```

or 

```md
<NUMBER_OF_DICES>d<SIDES> + ...
```

### Example:
```
d6 + 2d10
```
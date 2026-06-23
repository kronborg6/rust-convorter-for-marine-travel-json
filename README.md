# convoter ax to bc json

this is a small convort made for marine-travel ax data to bc data for the customer table

this is not made for used outisde of marine-travel
this is a small project for a fast convorter
and for me to learn stuff

## how to run

best way is to build or run from soruce

the best way is to do
```console
cargo run . -- [args]

```
example

```console
cargo run . -- --print

```
to get both the result.json and print the output

### build

```console
cargo build --release
```
or

```console
cargo build --release --target [system]
```


## command flags


### all print flags

```console
./program -p --print Print

```

### all debug flgas

```console
./program -d --debug Debug

```

### all no result.json flags

```console
./program -no --no-result --no-json --no-output

```

### all togetter then you get both debug and normal json print and no result.json
```console
./program -no --no-result --no-json --no-output -d --debug Debug -p --print Print

```


## Example
### input.json
```json
[
  {
    "customerNo": "60677",
    "name": "King Kronborg",
    "fakturaCreditNotaTo": "king@castle.dk",
    "fakturaCreditNotaCc": "queen@castle.dk hoff@castle.dk",
    "statmentTo": "king@castle.dk",
    "statmentCc": "queen@castle.dk",
    "DCT_EMAILPAYMENT": ""
  },

  {
    "customerNo": "60674",
    "name": "Queen Kronborg",
    "fakturaCreditNotaTo": "queen@castle.dk",
    "fakturaCreditNotaCc": "",
    "statmentTo": "queen@castle.dk",
    "statmentCc": "King@castle.dk",
    "DCT_EMAILPAYMENT": ""
  }
]
```

### result.json
```json
[
  {
    "customerNo": "60677",
    "emailType": 1,
    "email": "king@castle.dk"
  },
  {
    "customerNo": "60677",
    "emailType": 2,
    "email": "queen@castle.dk hoff@castle.dk"
  },
  {
    "customerNo": "60677",
    "emailType": 3,
    "email": "king@castle.dk"
  },
  {
    "customerNo": "60677",
    "emailType": 4,
    "email": "queen@castle.dk"
  },
  {
    "customerNo": "60674",
    "emailType": 1,
    "email": "queen@castle.dk"
  },
  {
    "customerNo": "60674",
    "emailType": 3,
    "email": "queen@castle.dk"
  },
  {
    "customerNo": "60674",
    "emailType": 4,
    "email": "King@castle.dk"
  }
]
```

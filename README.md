# convoter ax to bc json

this is a small convort made for marine-travel ax data to bc data for the customer table

this is not made for used outisde of marine-travel
this is a small project for a fast convorter
and for me to learn stuff

## Example
### input.json
```json
[
  {
    "customerNo": "60677",
    "name": "King Kronborg",
    "fakturaCreditNotaTo": "king@castle.dk",
    "fakturaCreditNotaCc": "",
    "statmentTo": "king@castle.dk",
    "statmentTo": "queen@castle.dk",
    "DCT_EMAILPAYMENT": ""
  },

  {
    "customerNo": "60674",
    "name": "Queen Kronborg",
    "fakturaCreditNotaTo": "queen@castle.dk",
    "fakturaCreditNotaCc": "",
    "statmentTo": "queen@castle.dk",
    "statmentTo": "King@castle.dk",
    "DCT_EMAILPAYMENT": ""
  },
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
    "emailType": 3,
    "email": "king@castle.dk"
  },
  {
    "customerNo": "60677",
    "emailType": 3,
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
    "emailType": 3,
    "email": "King@castle.dk"
  }
]
```

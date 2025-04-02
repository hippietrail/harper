---
title: Local Statistics
---

Harper keeps track of certain aspects of your writing.
Things like:

- What words to misspell most often?
- How often do you accept Harper's suggestions?
- How much do you write?

Harper does this to help _you_ improve _your_ writing.
In the interest of maintaining our user's data sovereignty, Harper aims to do all this processing __on the device__.
None of this data is sent anywhere without your explicit permission.

This document seeks to detail how Harper's statistics logging works under the hood.

## The `stats.txt` File.

The `stats.txt` file (whose name is subject to change) is a log of actions taken by Harper or the user.
It records specific events, along with some contextual information (like which word was misspelled).

The `stats.txt` file is formatted into lines (so it is easy to open in append-mode), each containing a JSON object.

```
{"kind":{"Lint":{"kind":"Capitalization","context":[{"content":["i"],"kind":{"kind":"Word","value":{"noun":null,"pronoun":{"is_plural":false,"is_possessive":null,"person":null,"case":null},"verb":null,"adjective":null,"adverb":null,"conjunction":null,"swear":null,"dialect":null,"determiner":false,"preposition":false,"common":true,"derived_from":null}}}]}},"when":1743428384,"uuid":"b095c60b-6ae0-4c82-bcb8-c7de737d6509"}
{"kind":{"Lint":{"kind":"Spelling","context":[{"content":["m","i","s","p","e","l","l"],"kind":{"kind":"Word","value":null}}]}},"when":1743428388,"uuid":"70850697-a3f1-4eb6-94c8-be0be6d5b16b"}
{"kind":{"Lint":{"kind":"Spelling","context":[{"content":["t","h","i","g","s"],"kind":{"kind":"Word","value":null}}]}},"when":1743428392,"uuid":"c0ff601a-f285-4f6d-9c47-1536213bee3f"}
{"kind":{"Lint":{"kind":"Spelling","context":[{"content":["t","h","i","g","s"],"kind":{"kind":"Word","value":null}}]}},"when":1743428397,"uuid":"c49a67c2-4d03-460a-b450-1ae9b20f243c"}
{"kind":{"Lint":{"kind":"Spelling","context":[{"content":["m","i","s","p","e","l","l","e","d"],"kind":{"kind":"Word","value":null}}]}},"when":1743429129,"uuid":"6ee7634e-dc8d-489b-9e13-f3b7dcd50741"}
{"kind":{"Lint":{"kind":"Miscellaneous","context":[{"content":["a"],"kind":{"kind":"Word","value":{"noun":null,"pronoun":null,"verb":null,"adjective":null,"adverb":null,"conjunction":null,"swear":null,"dialect":null,"determiner":true,"preposition":true,"common":true,"derived_from":null}}}]}},"when":1743525210,"uuid":"cbb2ca64-2073-4714-a346-628bb31cfc65"}
```

In `harper-ls` it is written to the Harper `data` directory.
In `harper.js` it is available through methods of objects that implement the [`Linter`](/docs/harperjs/ref/harper.js.linter.html) interface.

A simple dashboard to view a summary of these statistics is available [on our website](/stats).

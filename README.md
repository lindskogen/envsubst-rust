# envsubst-rust

A simple port of `envsubst(1)` that also supports fallback expansions.

Supports the following syntax:

| __Expression__     | __Meaning__                                                     |
|--------------------|-----------------------------------------------------------------|
| `${var}` or `$var` | Value of var                                                    |
| `${var-$DEFAULT}`  | If var not set, evaluate expression as `$DEFAULT`               |
| `${var:-$DEFAULT}` | If var not set (or is empty), evaluate expression as `$DEFAULT` |

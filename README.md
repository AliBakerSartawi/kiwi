<!-- markdownlint-disable MD033 -->

# Kiwi 🥝

Kiwi is a type-safe key-value in-memory data structure store

## Commands

### Redis Commands

Note: Some commands might be moved from **Will Implement** to _Might Implement_, or vice versa.

| Command           | Group   | Status             | Redis Description                                                                       | Notes                                                           |
| ----------------- | ------- | ------------------ | --------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| `DEL`             | Generic | ✅ Implemented     | Deletes one key.                                                                        |                                                                 |
| `DEL`             | Generic | **Will Implement** | Deletes one or more keys.                                                               |                                                                 |
| `COPY`            | Generic | **Will Implement** | Copies the value of a key to a new key.                                                 |                                                                 |
| `EXISTS`          | Generic | **Will Implement** | Determines whether one or more keys exist.                                              |                                                                 |
| `EXPIRE`          | Generic | **Will Implement** | Sets the expiration time of a key in seconds.                                           |                                                                 |
| `EXPIREAT`        | Generic | **Will Implement** | Sets the expiration time of a key to a Unix timestamp.                                  |                                                                 |
| `EXPIRETIME`      | Generic | **Will Implement** | Returns the expiration time of a key as a Unix timestamp.                               |                                                                 |
| `KEYS`            | Generic | **Will Implement** | Returns all key names that match a pattern.                                             |                                                                 |
| `MOVE`            | Generic | **Will Implement** | Moves a key to another database.                                                        |                                                                 |
| `PERSIST`         | Generic | **Will Implement** | Removes the expiration time of a key.                                                   |                                                                 |
| `PEXPIRE`         | Generic | **Will Implement** | Sets the expiration time of a key in milliseconds.                                      |                                                                 |
| `PEXPIREAT`       | Generic | **Will Implement** | Sets the expiration time of a key to a Unix milliseconds timestamp.                     |                                                                 |
| `PEXPIRETIME`     | Generic | **Will Implement** | Returns the expiration time of a key as a Unix milliseconds timestamp.                  |                                                                 |
| `PTTL`            | Generic | **Will Implement** | Returns the expiration time in milliseconds of a key.                                   |                                                                 |
| `RANDOMKEY`       | Generic | **Will Implement** | Returns a random key name from the database.                                            |                                                                 |
| `RENAME`          | Generic | **Will Implement** | Renames a key and overwrites the destination.                                           | The new name must have a matching [type prefix](#type-prefixes) |
| `RENAMENX`        | Generic | **Will Implement** | Renames a key only when the target key name doesn't exist.                              | The new name must have a matching [type prefix](#type-prefixes) |
| `SCAN`            | Generic | **Will Implement** | Iterates over the key names in the database.                                            |                                                                 |
| `SORT`            | Generic | **Will Implement** | Sorts the elements in a list, a set, or a sorted set, optionally storing the result.    |                                                                 |
| `SORT_RO`         | Generic | **Will Implement** | Returns the sorted elements of a list, a set, or a sorted set.                          |                                                                 |
| `TOUCH`           | Generic | **Will Implement** | Updates the time keys were last accessed, returns the number of keys touched.           |                                                                 |
| `TTL`             | Generic | **Will Implement** | Returns the expiration time in seconds of a key.                                        |                                                                 |
| `TYPE`            | Generic | **Will Implement** | Determines the type of value stored at a key.                                           |                                                                 |
| `UNLINK`          | Generic | **Will Implement** | Asynchronously deletes one or more keys.                                                |                                                                 |
| `DUMP`            | Generic | _Might Implement_  | Returns a serialized representation of the value stored at a key.                       |                                                                 |
| `MIGRATE`         | Generic | _Might Implement_  | Atomically transfers a key from one Redis instance to another.                          |                                                                 |
| `OBJECT ENCODING` | Generic | _Might Implement_  | Returns the internal encoding of a Redis object.                                        |                                                                 |
| `OBJECT FREQ`     | Generic | _Might Implement_  | Returns the logarithmic access frequency counter of a Redis object.                     |                                                                 |
| `OBJECT IDLETIME` | Generic | _Might Implement_  | Returns the time since the last access to a Redis object.                               |                                                                 |
| `OBJECT REFCOUNT` | Generic | _Might Implement_  | Returns the reference count of a value of a key.                                        |                                                                 |
| `RESTORE`         | Generic | _Might Implement_  | Creates a key from the serialized representation of a value.                            |                                                                 |
| `WAIT`            | Generic | _Might Implement_  | Blocks until the asynchronous replication of all preceding write commands is completed. |                                                                 |
| `WAITAOF`         | Generic | _Might Implement_  | Blocks until all preceding write commands are written to the append-only file.          |                                                                 |

## Type Prefixes

A type prefix is a suggested naming technique for keys to easily communicate the type of a key's value. If you have a key that's called `int-count`, you know you can safely do `INCR` on it, etc etc.

The syntax is `<type>-<key_name>`.

Let's discuss what the delimiter might be, `-` or `:`, or something else. Using `:` would mean that the store is namespaced by type.

Types are:

- str (String)
- int (Integer)
- float (Float)
- bool (Boolean)

Examples:

- ```shell
  SET int-count surprise # this won't work because `surprise` can't be parsed as an integer.
  ```

- ```shell
  SET str-xxx 10 # 10 is set here as a string, not a number.
  INCR str-xxx # This won't work, even if the string value can be parsed as an integer.
  ```

- ```shell
  SET str-a "lorem ipsum"
  RENAME str-a a # This works, because not specifying a type prefix defaults to string.
  RENAME a int-a # This won't work.

  SET int-x 10
  RENAME int-x x # If we decide to allow this, the value `10` must be stringified and treated as a string.
  ```

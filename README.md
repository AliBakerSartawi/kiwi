<!-- markdownlint-disable MD033 -->

# Kiwi 🥝

Kiwi is a type-safe key-value in-memory data structure store

## Commands

### Redis Commands

Note: Some commands might be moved from <span style="color:peachpuff">Will Implement</span> to <span style="color:crimson">Might Implement</span>, or vice versa.

| Command           | Group   | Status                                               | Redis Description                                                                       | Notes                                                           |
| ----------------- | ------- | ---------------------------------------------------- | --------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| `DEL`             | Generic | <span style="color:springgreen">✓ Implemented</span> | Deletes one key.                                                                        |                                                                 |
| `DEL`             | Generic | <span style="color:peachpuff">Will Implement</span>  | Deletes one or more keys.                                                               |                                                                 |
| `COPY`            | Generic | <span style="color:peachpuff">Will Implement</span>  | Copies the value of a key to a new key.                                                 |                                                                 |
| `EXISTS`          | Generic | <span style="color:peachpuff">Will Implement</span>  | Determines whether one or more keys exist.                                              |                                                                 |
| `EXPIRE`          | Generic | <span style="color:peachpuff">Will Implement</span>  | Sets the expiration time of a key in seconds.                                           |                                                                 |
| `EXPIREAT`        | Generic | <span style="color:peachpuff">Will Implement</span>  | Sets the expiration time of a key to a Unix timestamp.                                  |                                                                 |
| `EXPIRETIME`      | Generic | <span style="color:peachpuff">Will Implement</span>  | Returns the expiration time of a key as a Unix timestamp.                               |                                                                 |
| `KEYS`            | Generic | <span style="color:peachpuff">Will Implement</span>  | Returns all key names that match a pattern.                                             |                                                                 |
| `MOVE`            | Generic | <span style="color:peachpuff">Will Implement</span>  | Moves a key to another database.                                                        |                                                                 |
| `PERSIST`         | Generic | <span style="color:peachpuff">Will Implement</span>  | Removes the expiration time of a key.                                                   |                                                                 |
| `PEXPIRE`         | Generic | <span style="color:peachpuff">Will Implement</span>  | Sets the expiration time of a key in milliseconds.                                      |                                                                 |
| `PEXPIREAT`       | Generic | <span style="color:peachpuff">Will Implement</span>  | Sets the expiration time of a key to a Unix milliseconds timestamp.                     |                                                                 |
| `PEXPIRETIME`     | Generic | <span style="color:peachpuff">Will Implement</span>  | Returns the expiration time of a key as a Unix milliseconds timestamp.                  |                                                                 |
| `PTTL`            | Generic | <span style="color:peachpuff">Will Implement</span>  | Returns the expiration time in milliseconds of a key.                                   |                                                                 |
| `RANDOMKEY`       | Generic | <span style="color:peachpuff">Will Implement</span>  | Returns a random key name from the database.                                            |                                                                 |
| `RENAME`          | Generic | <span style="color:peachpuff">Will Implement</span>  | Renames a key and overwrites the destination.                                           | The new name must have a matching [type prefix](#type-prefixes) |
| `RENAMENX`        | Generic | <span style="color:peachpuff">Will Implement</span>  | Renames a key only when the target key name doesn't exist.                              | The new name must have a matching [type prefix](#type-prefixes) |
| `SCAN`            | Generic | <span style="color:peachpuff">Will Implement</span>  | Iterates over the key names in the database.                                            |                                                                 |
| `SORT`            | Generic | <span style="color:peachpuff">Will Implement</span>  | Sorts the elements in a list, a set, or a sorted set, optionally storing the result.    |                                                                 |
| `SORT_RO`         | Generic | <span style="color:peachpuff">Will Implement</span>  | Returns the sorted elements of a list, a set, or a sorted set.                          |                                                                 |
| `TOUCH`           | Generic | <span style="color:peachpuff">Will Implement</span>  | Updates the time keys were last accessed, returns the number of keys touched.           |                                                                 |
| `TTL`             | Generic | <span style="color:peachpuff">Will Implement</span>  | Returns the expiration time in seconds of a key.                                        |                                                                 |
| `TYPE`            | Generic | <span style="color:peachpuff">Will Implement</span>  | Determines the type of value stored at a key.                                           |                                                                 |
| `UNLINK`          | Generic | <span style="color:peachpuff">Will Implement</span>  | Asynchronously deletes one or more keys.                                                |                                                                 |
| `DUMP`            | Generic | <span style="color:crimson">Might Implement</span>   | Returns a serialized representation of the value stored at a key.                       |                                                                 |
| `MIGRATE`         | Generic | <span style="color:crimson">Might Implement</span>   | Atomically transfers a key from one Redis instance to another.                          |                                                                 |
| `OBJECT ENCODING` | Generic | <span style="color:crimson">Might Implement</span>   | Returns the internal encoding of a Redis object.                                        |                                                                 |
| `OBJECT FREQ`     | Generic | <span style="color:crimson">Might Implement</span>   | Returns the logarithmic access frequency counter of a Redis object.                     |                                                                 |
| `OBJECT IDLETIME` | Generic | <span style="color:crimson">Might Implement</span>   | Returns the time since the last access to a Redis object.                               |                                                                 |
| `OBJECT REFCOUNT` | Generic | <span style="color:crimson">Might Implement</span>   | Returns the reference count of a value of a key.                                        |                                                                 |
| `RESTORE`         | Generic | <span style="color:crimson">Might Implement</span>   | Creates a key from the serialized representation of a value.                            |                                                                 |
| `WAIT`            | Generic | <span style="color:crimson">Might Implement</span>   | Blocks until the asynchronous replication of all preceding write commands is completed. |                                                                 |
| `WAITAOF`         | Generic | <span style="color:crimson">Might Implement</span>   | Blocks until all preceding write commands are written to the append-only file.          |                                                                 |

## Type Prefixes

A type prefix is a suggested naming technique for keys to easily communicate the type of a key's value. If you have a key that's called `int-count`, you know you can safely do `INCR` on it, etc etc.

The syntax is `<type>-<key_name>`

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

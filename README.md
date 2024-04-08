# Kiwi 🥝 - A Type-Safe Key-Value In-Memory Data Store

Kiwi is designed with safety and performance in mind, leveraging the power of Rust 🦀. Below is a breakdown of the commands Kiwi will implement, providing a wide range of functionalities for managing key-value pairs efficiently.

## Commands

### Redis Commands

| Command           | Group   | Status          | Description                                                                             |
| ----------------- | ------- | --------------- | --------------------------------------------------------------------------------------- |
| `DEL`             | Generic | Will Implement  | Deletes one or more keys.                                                               |
| `COPY`            | Generic | Will Implement  | Copies the value of a key to a new key.                                                 |
| `EXISTS`          | Generic | Will Implement  | Determines whether one or more keys exist.                                              |
| `EXPIRE`          | Generic | Will Implement  | Sets the expiration time of a key in seconds.                                           |
| `EXPIREAT`        | Generic | Will Implement  | Sets the expiration time of a key to a Unix timestamp.                                  |
| `EXPIRETIME`      | Generic | Will Implement  | Returns the expiration time of a key as a Unix timestamp.                               |
| `KEYS`            | Generic | Will Implement  | Returns all key names that match a pattern.                                             |
| `MOVE`            | Generic | Will Implement  | Moves a key to another database.                                                        |
| `PERSIST`         | Generic | Will Implement  | Removes the expiration time of a key.                                                   |
| `PEXPIRE`         | Generic | Will Implement  | Sets the expiration time of a key in milliseconds.                                      |
| `PEXPIREAT`       | Generic | Will Implement  | Sets the expiration time of a key to a Unix milliseconds timestamp.                     |
| `PEXPIRETIME`     | Generic | Will Implement  | Returns the expiration time of a key as a Unix milliseconds timestamp.                  |
| `PTTL`            | Generic | Will Implement  | Returns the expiration time in milliseconds of a key.                                   |
| `RANDOMKEY`       | Generic | Will Implement  | Returns a random key name from the database.                                            |
| `RENAME`          | Generic | Will Implement  | Renames a key and overwrites the destination.                                           |
| `RENAMENX`        | Generic | Will Implement  | Renames a key only when the target key name doesn't exist.                              |
| `SCAN`            | Generic | Will Implement  | Iterates over the key names in the database.                                            |
| `SORT`            | Generic | Will Implement  | Sorts the elements in a list, a set, or a sorted set, optionally storing the result.    |
| `SORT_RO`         | Generic | Will Implement  | Returns the sorted elements of a list, a set, or a sorted set.                          |
| `TOUCH`           | Generic | Will Implement  | Updates the time keys were last accessed, returns the number of keys touched.           |
| `TTL`             | Generic | Will Implement  | Returns the expiration time in seconds of a key.                                        |
| `TYPE`            | Generic | Will Implement  | Determines the type of value stored at a key.                                           |
| `UNLINK`          | Generic | Will Implement  | Asynchronously deletes one or more keys.                                                |
| `DUMP`            | Generic | Might Implement | Returns a serialized representation of the value stored at a key.                       |
| `MIGRATE`         | Generic | Might Implement | Atomically transfers a key from one Redis instance to another.                          |
| `OBJECT ENCODING` | Generic | Might Implement | Returns the internal encoding of a Redis object.                                        |
| `OBJECT FREQ`     | Generic | Might Implement | Returns the logarithmic access frequency counter of a Redis object.                     |
| `OBJECT IDLETIME` | Generic | Might Implement | Returns the time since the last access to a Redis object.                               |
| `OBJECT REFCOUNT` | Generic | Might Implement | Returns the reference count of a value of a key.                                        |
| `RESTORE`         | Generic | Might Implement | Creates a key from the serialized representation of a value.                            |
| `WAIT`            | Generic | Might Implement | Blocks until the asynchronous replication of all preceding write commands is completed. |
| `WAITAOF`         | Generic | Might Implement | Blocks until all preceding write commands are written to the append-only file.          |

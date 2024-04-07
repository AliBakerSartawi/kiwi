# Kiwi 🥝 - A Type-Safe Key-Value In-Memory Data Store

Kiwi is designed with safety and performance in mind, leveraging the power of Rust 🦀. Below is a breakdown of the commands Kiwi will implement, providing a wide range of functionalities for managing key-value pairs efficiently.

## Commands

### Implemented Redis Commands

| Command | Group   | Description               |
| ------- | ------- | ------------------------- |
| `DEL`   | Generic | Deletes one or more keys. |

### Will-Implement Redis Commands

| Command       | Group   | Description                                                                          |
| ------------- | ------- | ------------------------------------------------------------------------------------ |
| `COPY`        | Generic | Copies the value of a key to a new key.                                              |
| `EXISTS`      | Generic | Determines whether one or more keys exist.                                           |
| `EXPIRE`      | Generic | Sets the expiration time of a key in seconds.                                        |
| `EXPIREAT`    | Generic | Sets the expiration time of a key to a Unix timestamp.                               |
| `EXPIRETIME`  | Generic | Returns the expiration time of a key as a Unix timestamp.                            |
| `KEYS`        | Generic | Returns all key names that match a pattern.                                          |
| `MOVE`        | Generic | Moves a key to another database.                                                     |
| `PERSIST`     | Generic | Removes the expiration time of a key.                                                |
| `PEXPIRE`     | Generic | Sets the expiration time of a key in milliseconds.                                   |
| `PEXPIREAT`   | Generic | Sets the expiration time of a key to a Unix milliseconds timestamp.                  |
| `PEXPIRETIME` | Generic | Returns the expiration time of a key as a Unix milliseconds timestamp.               |
| `PTTL`        | Generic | Returns the expiration time in milliseconds of a key.                                |
| `RANDOMKEY`   | Generic | Returns a random key name from the database.                                         |
| `RENAME`      | Generic | Renames a key and overwrites the destination.                                        |
| `RENAMENX`    | Generic | Renames a key only when the target key name doesn't exist.                           |
| `SCAN`        | Generic | Iterates over the key names in the database.                                         |
| `SORT`        | Generic | Sorts the elements in a list, a set, or a sorted set, optionally storing the result. |
| `SORT_RO`     | Generic | Returns the sorted elements of a list, a set, or a sorted set.                       |
| `TOUCH`       | Generic | Updates the time keys were last accessed, returns the number of keys touched.        |
| `TTL`         | Generic | Returns the expiration time in seconds of a key.                                     |
| `TYPE`        | Generic | Determines the type of value stored at a key.                                        |
| `UNLINK`      | Generic | Asynchronously deletes one or more keys.                                             |

### Might-Implement Redis Commands

| Command           | Group   | Description                                                                             |
| ----------------- | ------- | --------------------------------------------------------------------------------------- |
| `DUMP`            | Generic | Returns a serialized representation of the value stored at a key.                       |
| `MIGRATE`         | Generic | Atomically transfers a key from one Redis instance to another.                          |
| `OBJECT ENCODING` | Generic | Returns the internal encoding of a Redis object.                                        |
| `OBJECT FREQ`     | Generic | Returns the logarithmic access frequency counter of a Redis object.                     |
| `OBJECT IDLETIME` | Generic | Returns the time since the last access to a Redis object.                               |
| `OBJECT REFCOUNT` | Generic | Returns the reference count of a value of a key.                                        |
| `RESTORE`         | Generic | Creates a key from the serialized representation of a value.                            |
| `WAIT`            | Generic | Blocks until the asynchronous replication of all preceding write commands is completed. |
| `WAITAOF`         | Generic | Blocks until all preceding write commands are written to the append-only file.          |

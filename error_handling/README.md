## Error Handling
There are two kinds of errors which is **recoverable** and **unrecoverable** errors.
<p>
Recoverable errors are those where the error that occured is not serious enough to stop the problem can recover to continue.
Unrecoverable errors are those which stops the program from proceeding further.

Rust handles recoverable and unrecoverable errors through Result and Panic.

`enum Result<T,E> {
   OK(T),
   Err(E)
}`

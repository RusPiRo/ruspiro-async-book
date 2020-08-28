## Future

A `Future` represents the processing whose result may be available at a *future* point in time. The actual procesing result need to be actively requested. This is called *polling* and is implemented as the `poll` function on the `Future` trait. The result of the `poll` function represents the the state of the `Future`. It could either be `Ready` yielding the actual result of the processing, or `Pending` indicating that the actual result is still not available.

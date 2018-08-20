# Reduce Boilerplate

We need to make the process of adding new structs to the persistent interface
easier. Options:

1) Make the alteration methods functions instead
2) Have the alteration methods be default implementations

# Reduce Type Confusion

1) Somehow make users of the library able to just deal with one type and not
a different type based on which operation was just used.

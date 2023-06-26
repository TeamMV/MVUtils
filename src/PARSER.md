## 1. Argument Types
The parser should support the following argument types:
* Required: For example, `program filename`
* Optional: For example, `program -v` or `program --version`
* Positional: For example, `program filename`
* Named: For example, `program --input=filename`
* Arguments with default values: For example, `program --timeout=60` where 60 is the default
* Boolean Flags: For example, `program --verbose`
* Multi-value: For example, `program --input=file1 --input=file2`
* Sub-commands: For example, `program convert --input=file --output=otherfile`
* Variadic: For example, `program file1 file2 file3` where file1, file2, and file3 are multiple values for the same argument
* Composite: For example, `program -abc` is equivalent to `program -a -b -c`
* Enumerated: For example, `program --color=red` where the possible choices are 'red', 'blue', 'green'
* Different Value Types: For example, `--compile fast`, `--compile=fast`, `-c fast`, `-cfast` and `-c=fast` are all the same

## 2. Argument Validation
The parser should validate:
* Types
* Ranges (for numeric arguments)
* Choices (for enumerated arguments)
* Presence (for required arguments)
* Combinations (for mutually exclusive arguments)

## 3. Error Reporting
The parser should report:
* Missing required arguments
* Unknown arguments
* Invalid argument types
* Invalid argument ranges
* Invalid argument combinations
* Display a help message with argument descriptions and usage examples when there are errors or when explicitly requested

## 4. Argument Help
The parser should:
* Support a `--help` flag that prints a help message
* Support a `--version` flag that prints the program version
* Generate a usage message based on the argument configuration
* Support descriptions for arguments and sub-commands

## 6. Efficiency
* Work in linear time complexity (O(n))
* Use constant space complexity (O(1))

## 7. Flexibility
The parser should allow for customisation of:
* The help message
* The error message
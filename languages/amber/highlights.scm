(comment) @comment

[
    "and"
    "as"
    "break"
    "const"
    "continue"
    "echo"
    "else"
    "exit"
    "exited"
    "fail"
    "failed"
    "for"
    "from"
    "fun"
    "if"
    "import"
    "in"
    "is"
    "len"
    "let"
    "lines"
    "loop"
    "main"
    "mv"
    "nameof"
    "not"
    "or"
    "pub"
    "return"
    "silent"
    "succeeded"
    "sudo"
    "then"
    "trust"
    "unsafe"
    "while"
] @keyword

; Literals
(boolean) @constant.builtin.boolean
(number) @constant.numeric
(null) @constant.numeric
(string) @string
(status) @keyword
(command) @string
(block) @delimiter
(parentheses) @delimiter
(while_loop) @keyword
(variable_init) @keyword
(variable_assignment) @delimiter
(builtin_stmt) @keyword
(builtin_expr) @keyword
(variable) @variable
(escape_sequence) @constant.character.escape
(type_name_symbol) @type
(interpolation) @delimiter
(reference) @keyword
(preprocessor_directive) @comment
(shebang) @comment
(parameter_list) @delimiter
(handler_propagation) @delimiter
(function_definition
    name: (variable) @function.method)
(function_call
    name: (variable) @function.method)
(import_statement
    "pub" @keyword
    "import" @keyword
    "from" @keyword)

initSidebarItems({"macro":[["help!",""],["panictry!",""],["register_diagnostic!",""],["register_diagnostics!",""],["register_long_diagnostics!",""],["span_err!",""],["span_err_or_warn!",""],["span_fatal!",""],["span_help!",""],["span_note!",""],["span_warn!",""],["struct_span_err!",""],["struct_span_err_or_warn!",""],["struct_span_fatal!",""],["struct_span_warn!",""],["walk_list!",""]],"mod":[["abi",""],["ast",""],["attr",""],["codemap","The CodeMap tracks all the source code used within a single crate, mapping from integer byte positions to the original source code location. Each bit of source parsed during crate parsing (typically files, in-memory strings, or various bits of macro expansion) cover a continuous range of bytes in the CodeMap and are represented by FileMaps. Byte positions are stored in `spans` and used pervasively in the compiler. They are absolute positions within the CodeMap, which upon request can be converted to line and column information, source code snippets, etc."],["config",""],["diagnostics",""],["entry",""],["ext",""],["feature_gate","Feature gating"],["fold","A Folder represents an AST->AST fold; it accepts an AST piece, and returns a piece of the same type. So, for instance, macro expansion is a Folder that walks over an AST and produces another AST."],["json","A JSON emitter for errors."],["parse","The main parser interface"],["print",""],["ptr","The AST pointer"],["show_span","Span debugger"],["std_inject",""],["str",""],["syntax",""],["test",""],["tokenstream","Token Trees TokenTrees are syntactic forms for dealing with tokens. The description below is more complete; in short a TokenTree is a single token, a delimited sequence of token trees, or a sequence with repetition for list splicing as part of macro expansion."],["util",""],["visit","AST walker. Each overridden visit method has full control over what happens with its node, it can do its own traversal of the node's children, call `visit::walk_*` to apply the default traversal algorithm, or prevent deeper traversal by doing nothing."]]});
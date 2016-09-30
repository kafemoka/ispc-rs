initSidebarItems({"fn":[["compile_library","Compile the list of ISPC files into a static library and generate bindings using bindgen. The library name should not contain a lib prefix or a lib extension like '.a' or '.lib', the appropriate prefix and suffix will be added based on the compilation target."],["print_instrumenting_summary","Print out a summary of performace data gathered from instrumenting ISPC. Must enable instrumenting to have this record and print data, see `Config::instrument`."],["set_instrument","If you have implemented your own instrument for logging ISPC performance data you can use this function to provide it for use instead of the default one. This function **must** be called before calling into ISPC code, otherwise the instrumenter will already be set to the default."],["set_task_system","If you have implemented your own task system you can provide it for use instead of the default threaded one. This must be done prior to calling ISPC code which spawns tasks otherwise the task system will have already been initialized to `Parallel`, which you can also see as an example for implementing a task system."]],"macro":[["ispc_module!","Convenience macro for generating the module to hold the raw/unsafe ISPC bindings."]],"mod":[["exec","Defines the trait that must be implemented by ISPC task execution systems and provides a default threaded one for use."],["instrument","Defines the trait that must be implemented by ISPC instrumentation callbacks structs and provides a default one."],["opt","This module has various option flags and configs we can pass to ISPC, located here for convience and clutter reduction."],["task","Defines structs for operating on ISPC task groups and getting chunks of a task to be scheduled on to threads"]],"struct":[["Config","Extra configuration to be passed to ISPC"]]});
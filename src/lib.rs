use zed_extension_api as zed;

struct TodoListExtension {
    // ... state
}

impl zed::Extension for TodoListExtension {
    // ...
}

zed::register_extension!(TodoListExtension);

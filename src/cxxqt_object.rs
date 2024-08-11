#[cxx_qt::bridge]
mod qobject {
    unsafe extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name MyObject
        // based on the Rust struct HelloRust
        #[qobject]
        #[qml_element]
        type Hello = super::HelloRust;
    }

    unsafe extern "RustQt" {
        // Declare the invokable methods we want to expose on the QObject
        #[qinvokable]
        fn say_hello(self: &Hello);
    }
}

// The Rust struct for the QObject
#[derive(Default)]
pub struct HelloRust {}

impl qobject::Hello {
    pub fn say_hello(&self) {
        println!("Hello world!")
    }
}

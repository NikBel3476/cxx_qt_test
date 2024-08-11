use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "com.nb.cxx_qt.test",
            rust_files: &["src/cxxqt_object.rs"],
            qml_files: &["../qml/main.qml"],
            ..Default::default()
        })
        .build();
}
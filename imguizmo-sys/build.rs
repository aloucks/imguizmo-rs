use std::env;
use std::path::Path;

fn main() {
    let cimgui_include = env::var("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY");
    let imgui_include = Path::new(&cimgui_include)
        .join("imgui")
        .display()
        .to_string();
    let includes = [
        "src",
        "third_party/ImGuizmo",
        imgui_include.as_str(),
        cimgui_include.as_str(),
    ];

    let mut build = cc::Build::new();
    for include in includes.iter() {
        build.include(include);
    }
    build
        .file("third_party/ImGuizmo/ImGuizmo.cpp")
        .file("src/cimguizmo.cpp")
        //.file("third_party/ImGuizmo/ImCurveEdit.cpp")
        //.file("third_party/ImGuizmo/ImGradient.cpp")
        //.file("third_party/ImGuizmo/ImSequencer.cpp")
        .compile("imguizmo");
}

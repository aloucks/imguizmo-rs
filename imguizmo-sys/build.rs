fn main() {
    cc::Build::new()
        .include("src")
        .include("third_party/imgui")
        .include("third_party/ImGuizmo")
        //.file("third_party/ImGuizmo/ImCurveEdit.cpp")
        //.file("third_party/ImGuizmo/ImGradient.cpp")
        //.file("third_party/ImGuizmo/ImSequencer.cpp")
        .file("third_party/ImGuizmo/ImGuizmo.cpp")
        .file("src/cimguizmo.cpp")
        .compile("imguizmo");
}

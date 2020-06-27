#include "imgui.h"
#include "cimguizmo.h"
#include <type_traits>

static_assert(std::is_pod<CImVec2>(), "CImVec2 must be pod");

IMGUIZMO_C_API void ImGuizmo_SetDrawlist() {
    ImGuizmo::SetDrawlist();
}

IMGUIZMO_C_API void ImGuizmo_BeginFrame() {
    ImGuizmo::BeginFrame();
}

IMGUIZMO_C_API bool ImGuizmo_IsOver() {
    return ImGuizmo::IsOver();
}

IMGUIZMO_C_API bool ImGuizmo_IsUsing() {
    return ImGuizmo::IsUsing();
}

IMGUIZMO_C_API void ImGuizmo_Enable(bool enable) {
    ImGuizmo::Enable(enable);
}

IMGUIZMO_C_API void ImGuizmo_DecomposeMatrixToComponents(const float *matrix, float *translation, float *rotation, float *scale) {
    ImGuizmo::DecomposeMatrixToComponents(matrix, translation, rotation, scale);
}

IMGUIZMO_C_API void ImGuizmo_RecomposeMatrixFromComponents(const float *translation, const float *rotation, const float *scale, float *matrix) {
    ImGuizmo::RecomposeMatrixFromComponents(translation, rotation, scale, matrix);
}

IMGUIZMO_C_API void ImGuizmo_SetRect(float x, float y, float width, float height) {
    ImGuizmo::SetRect(x, y, width, height);
}

IMGUIZMO_C_API void ImGuizmo_SetOrthographic(bool isOrthographic) {
    ImGuizmo::SetOrthographic(isOrthographic);
}

IMGUIZMO_C_API void ImGuizmo_DrawCubes(const float *view, const float *projection, const float *matrix, int matrixCount) {
    ImGuizmo::DrawCubes(view, projection, matrix, matrixCount);
}

IMGUIZMO_C_API void ImGuizmo_DrawGrid(const float *view, const float *projection, const float *matrix, const float gridSize) {
    ImGuizmo::DrawGrid(view, projection, matrix, gridSize);
}

IMGUIZMO_C_API void ImGuizmo_Manipulate(const float *view, const float *projection, ImGuizmo_Operation operation, ImGuizmo_Mode mode, float *matrix, float *deltaMatrix, float *snap, float *localBounds, float *boundsSnap) {
    auto o = static_cast<ImGuizmo::OPERATION>(operation);
    auto m = static_cast<ImGuizmo::MODE>(mode);
    ImGuizmo::Manipulate(view, projection, o, m, matrix, deltaMatrix, snap, localBounds, boundsSnap);
}

IMGUIZMO_C_API void ImGuizmo_ViewManipulate(float* view, float length, const CImVec2* position, const CImVec2* size, ImU32 backgroundColor) {
    auto p = ImVec2(position->x, position->y);
    auto s = ImVec2(size->x, size->y);
    ImGuizmo::ViewManipulate(view, length, p, s, backgroundColor);
}
#include "cimguizmo.h"

CIMGUI_API void ImGuizmo_SetDrawlist() {
    ImGuizmo::SetDrawlist();
}

CIMGUI_API void ImGuizmo_BeginFrame() {
    ImGuizmo::BeginFrame();
}

CIMGUI_API bool ImGuizmo_IsOver() {
    return ImGuizmo::IsOver();
}

CIMGUI_API bool ImGuizmo_IsUsing() {
    return ImGuizmo::IsUsing();
}

CIMGUI_API void ImGuizmo_Enable(bool enable) {
    ImGuizmo::Enable(enable);
}

CIMGUI_API void ImGuizmo_DecomposeMatrixToComponents(const float *matrix, float *translation, float *rotation, float *scale) {
    ImGuizmo::DecomposeMatrixToComponents(matrix, translation, rotation, scale);
}

CIMGUI_API void ImGuizmo_RecomposeMatrixFromComponents(const float *translation, const float *rotation, const float *scale, float *matrix) {
    ImGuizmo::RecomposeMatrixFromComponents(translation, rotation, scale, matrix);
}

CIMGUI_API void ImGuizmo_SetRect(float x, float y, float width, float height) {
    ImGuizmo::SetRect(x, y, width, height);
}

CIMGUI_API void ImGuizmo_SetOrthographic(bool isOrthographic) {
    ImGuizmo::SetOrthographic(isOrthographic);
}

CIMGUI_API void ImGuizmo_DrawCube(const float *view, const float *projection, const float *matrix) {
    ImGuizmo::DrawCube(view, projection, matrix);
}

CIMGUI_API void ImGuizmo_DrawGrid(const float *view, const float *projection, const float *matrix, const float gridSize) {
    ImGuizmo::DrawGrid(view, projection, matrix, gridSize);
}

CIMGUI_API void ImGuizmo_Manipulate(const float *view, const float *projection, ImGuizmo_OPERATION operation, ImGuizmo_MODE mode, float *matrix, float *deltaMatrix, float *snap, float *localBounds, float *boundsSnap) {
    auto o = static_cast<ImGuizmo::OPERATION>(operation);
    auto m = static_cast<ImGuizmo::MODE>(mode);
    ImGuizmo::Manipulate(view, projection, o, m, matrix, deltaMatrix, snap, localBounds, boundsSnap);
}
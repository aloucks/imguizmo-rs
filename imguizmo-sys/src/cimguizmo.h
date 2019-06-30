#ifndef CIMGUIZMO_INCLUDED
#define CIMGUIZMO_INCLUDED

#include "cimgui.h"
#include "ImGuizmo.h"

enum ImGuizmo_OPERATION
{
	TRANSLATE = ImGuizmo::OPERATION::TRANSLATE,
	ROTATE    = ImGuizmo::OPERATION::ROTATE,
	SCALE     = ImGuizmo::OPERATION::SCALE,
	BOUNDS    = ImGuizmo::OPERATION::BOUNDS
};

enum ImGuizmo_MODE
{
    LOCAL = ImGuizmo::MODE::LOCAL,
    WORLD = ImGuizmo::MODE::WORLD
};

CIMGUI_API void ImGuizmo_SetDrawlist();
CIMGUI_API void ImGuizmo_BeginFrame();
CIMGUI_API bool ImGuizmo_IsOver();
CIMGUI_API bool ImGuizmo_IsUsing();
CIMGUI_API void ImGuizmo_Enable(bool enable);
CIMGUI_API void ImGuizmo_DecomposeMatrixToComponents(const float *matrix, float *translation, float *rotation, float *scale);
CIMGUI_API void ImGuizmo_RecomposeMatrixFromComponents(const float *translation, const float *rotation, const float *scale, float *matrix);
CIMGUI_API void ImGuizmo_SetRect(float x, float y, float width, float height);
CIMGUI_API void ImGuizmo_SetOrthographic(bool isOrthographic);
CIMGUI_API void ImGuizmo_DrawCube(const float *view, const float *projection, const float *matrix);
CIMGUI_API void ImGuizmo_DrawGrid(const float *view, const float *projection, const float *matrix, const float gridSize);
CIMGUI_API void ImGuizmo_Manipulate(const float *view, const float *projection, ImGuizmo_OPERATION operation, ImGuizmo_MODE mode, float *matrix, float *deltaMatrix = 0, float *snap = 0, float *localBounds = NULL, float *boundsSnap = NULL);

#endif // CIMGUIZMO_INCLUDED
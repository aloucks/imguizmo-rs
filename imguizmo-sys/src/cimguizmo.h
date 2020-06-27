#ifndef CIMGUIZMO_H
#define CIMGUIZMO_H

#include <stdio.h>
#include <stdint.h>
#if defined _WIN32 || defined __CYGWIN__
    #ifdef IMGUIZMO_C_NO_EXPORT
        #define IMGUIZMO_C_EXPORT
    #else
        #define IMGUIZMO_C_EXPORT __declspec(dllexport)
    #endif
    #ifndef __GNUC__
    #define snprintf sprintf_s
    #endif
#else
    #define IMGUIZMO_C_EXPORT
#endif

#if defined __cplusplus
    #define IMGUIZMO_C_EXTERN extern "C"
#else
    #include <stdarg.h>
    #include <stdbool.h>
    #define IMGUIZMO_C_EXTERN extern
#endif

#define IMGUIZMO_C_API IMGUIZMO_C_EXTERN IMGUIZMO_C_EXPORT

#include "imgui.h"
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

IMGUIZMO_C_API void ImGuizmo_SetDrawlist();
IMGUIZMO_C_API void ImGuizmo_BeginFrame();
IMGUIZMO_C_API bool ImGuizmo_IsOver();
IMGUIZMO_C_API bool ImGuizmo_IsUsing();
IMGUIZMO_C_API void ImGuizmo_Enable(bool enable);
IMGUIZMO_C_API void ImGuizmo_DecomposeMatrixToComponents(const float *matrix, float *translation, float *rotation, float *scale);
IMGUIZMO_C_API void ImGuizmo_RecomposeMatrixFromComponents(const float *translation, const float *rotation, const float *scale, float *matrix);
IMGUIZMO_C_API void ImGuizmo_SetRect(float x, float y, float width, float height);
IMGUIZMO_C_API void ImGuizmo_SetOrthographic(bool isOrthographic);
IMGUIZMO_C_API void ImGuizmo_DrawCubes(const float *view, const float *projection, const float *matrix, int matrixCount);
IMGUIZMO_C_API void ImGuizmo_DrawGrid(const float *view, const float *projection, const float *matrix, const float gridSize);
IMGUIZMO_C_API void ImGuizmo_Manipulate(const float *view, const float *projection, ImGuizmo_OPERATION operation, ImGuizmo_MODE mode, float *matrix, float *deltaMatrix = 0, float *snap = 0, float *localBounds = NULL, float *boundsSnap = NULL);

#endif // CIMGUIZMO_H
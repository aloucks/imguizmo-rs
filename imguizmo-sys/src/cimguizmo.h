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

#include "ImGuizmo.h"

enum ImGuizmo_Operation
{
	Translate = ImGuizmo::OPERATION::TRANSLATE,
	Rotate    = ImGuizmo::OPERATION::ROTATE,
	Scale     = ImGuizmo::OPERATION::SCALE,
	Bounds    = ImGuizmo::OPERATION::BOUNDS
};

enum ImGuizmo_Mode
{
    Local = ImGuizmo::MODE::LOCAL,
    World = ImGuizmo::MODE::WORLD
};

typedef struct CImVec2 {
    float x;
    float y;
} CImVec2;

#ifndef ImU32
#define ImU32 unsigned int
#endif

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
IMGUIZMO_C_API void ImGuizmo_Manipulate(const float *view, const float *projection, ImGuizmo_Operation operation, ImGuizmo_Mode mode, float *matrix, float *deltaMatrix = 0, float *snap = 0, float *localBounds = NULL, float *boundsSnap = NULL);
IMGUIZMO_C_API void ImGuizmo_ViewManipulate(float* view, float length, const CImVec2* position, const CImVec2* size, ImU32 backgroundColor);

#endif // CIMGUIZMO_H
use std::os::raw::{c_float, c_int};

#[link(name = "imguizmo")]
extern "C" {}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    Translate = 0,
    Rotate = 1,
    Scale = 2,
    Bounds = 3,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Local = 0,
    World = 1,
}

extern "C" {
    pub fn ImGuizmo_SetDrawlist();
    pub fn ImGuizmo_BeginFrame();
    pub fn ImGuizmo_IsOver() -> bool;
    pub fn ImGuizmo_IsUsing() -> bool;
    pub fn ImGuizmo_Enable(enable: bool);
    pub fn ImGuizmo_DecomposeMatrixToComponents(
        matrix: *const c_float,
        translation: *mut c_float,
        rotation: *mut c_float,
        scale: *mut c_float,
    );
    pub fn ImGuizmo_RecomposeMatrixFromComponents(
        translation: *const c_float,
        rotation: *const c_float,
        scale: *const c_float,
        matrix: *mut c_float,
    );
    pub fn ImGuizmo_SetRect(x: c_float, y: c_float, width: c_float, height: c_float);
    pub fn ImGuizmo_SetOrthographic(isOrthographic: bool);
    pub fn ImGuizmo_DrawCubes(
        view: *const c_float,
        projection: *const c_float,
        matrix: *const c_float,
        matrixCount: c_int,
    );
    pub fn ImGuizmo_DrawGrid(
        view: *const c_float,
        projection: *const c_float,
        matrix: *const c_float,
        gridSize: c_float,
    );
    pub fn ImGuizmo_Manipulate(
        view: *const c_float,
        projection: *const c_float,
        operation: Operation,
        mode: Mode,
        matrix: *mut c_float,
        deltaMatrix: *mut c_float,
        snap: *mut c_float,
        localBounds: *mut c_float,
        boundsSnap: *mut c_float,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imguizmo_links() {
        let null = std::ptr::null();
        let null_mut = std::ptr::null_mut();
        if false {
            unsafe {
                ImGuizmo_SetDrawlist();
                ImGuizmo_BeginFrame();
                ImGuizmo_IsOver();
                ImGuizmo_IsUsing();
                ImGuizmo_Enable(true);
                ImGuizmo_DecomposeMatrixToComponents(null, null_mut, null_mut, null_mut);
                ImGuizmo_RecomposeMatrixFromComponents(null, null, null, null_mut);
                ImGuizmo_SetRect(0.0, 0.0, 1.0, 1.0);
                ImGuizmo_SetOrthographic(false);
                ImGuizmo_DrawCubes(null, null, null, 0);
                ImGuizmo_DrawGrid(null, null, null, 10.0);
                ImGuizmo_Manipulate(
                    null,
                    null,
                    Operation::Rotate,
                    Mode::Local,
                    null_mut,
                    null_mut,
                    null_mut,
                    null_mut,
                    null_mut,
                );
            }
        }
    }
}

use std::os::raw::{c_float, c_int};

#[link(name = "imguizmo")]
extern "C" {}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    Translate = 0,
    Rotate = 1,
    Scale = 2,
    Bounds = 3,
}

#[repr(i32)]
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
        matrix: *const [[c_float; 4]; 4],
        translation: *mut [c_float; 3],
        rotation: *mut [c_float; 3],
        scale: *mut [c_float; 3],
    );
    pub fn ImGuizmo_RecomposeMatrixFromComponents(
        translation: *const [c_float; 3],
        rotation: *const [c_float; 3],
        scale: *const [c_float; 3],
        matrix: *mut [[c_float; 4]; 4],
    );
    pub fn ImGuizmo_SetRect(x: c_float, y: c_float, width: c_float, height: c_float);
    pub fn ImGuizmo_SetOrthographic(isOrthographic: bool);
    pub fn ImGuizmo_DrawCubes(
        view: *const [[c_float; 4]; 4],
        projection: *const [[c_float; 4]; 4],
        matrix: *const [[c_float; 4]; 4],
        matrixCount: c_int,
    );
    pub fn ImGuizmo_DrawGrid(
        view: *const [[c_float; 4]; 4],
        projection: *const [[c_float; 4]; 4],
        matrix: *const [[c_float; 4]; 4],
        gridSize: c_float,
    );
    pub fn ImGuizmo_Manipulate(
        view: *const [[c_float; 4]; 4],
        projection: *const [[c_float; 4]; 4],
        operation: Operation,
        mode: Mode,
        matrix: *mut [[c_float; 4]; 4],
        deltaMatrix: *mut [[c_float; 4]; 4],
        snap: *mut c_float,
        localBounds: *mut [[c_float; 3]; 2],
        boundsSnap: *mut [c_float; 3],
    );
    pub fn ImGuizmo_ViewManipulate(
        view: *mut [[c_float; 4]; 4],
        length: f32,
        position: *const [c_float; 2],
        size: *const [c_float; 2],
        backgroundColor: u32,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imguizmo_links() {
        use std::ptr;
        if false {
            unsafe {
                ImGuizmo_SetDrawlist();
                ImGuizmo_BeginFrame();
                ImGuizmo_IsOver();
                ImGuizmo_IsUsing();
                ImGuizmo_Enable(true);
                ImGuizmo_DecomposeMatrixToComponents(
                    ptr::null(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                ImGuizmo_RecomposeMatrixFromComponents(
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                    ptr::null_mut(),
                );
                ImGuizmo_SetRect(0.0, 0.0, 1.0, 1.0);
                ImGuizmo_SetOrthographic(false);
                ImGuizmo_DrawCubes(ptr::null(), ptr::null(), ptr::null(), 0);
                ImGuizmo_DrawGrid(ptr::null(), ptr::null(), ptr::null(), 1.0);
                ImGuizmo_Manipulate(
                    ptr::null(),
                    ptr::null(),
                    Operation::Rotate,
                    Mode::Local,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                ImGuizmo_ViewManipulate(ptr::null_mut(), 0.0, ptr::null(), ptr::null(), 0);
            }
        }
    }
}

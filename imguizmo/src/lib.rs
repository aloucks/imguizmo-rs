//! # ImGuizmo
//!
//! ```rust,no_run
//! # use imguizmo::{Gizmo, Matrix4};
//! # let view_matrix = Matrix4::default();
//! # let mut model_matrix = Matrix4::default();
//! # let ui = unsafe { std::mem::uninitialized() };
//! let gizmo = Gizmo::begin_frame(ui);
//!
//! gizmo
//!     .builder(&view_matrix, &mut model_matrix)
//!     .manipulate();
//! ```
//!
use imguizmo_sys as ffi;

pub use imguizmo_sys::{Mode, Operation};

use imgui::Ui;

use std::ptr;

pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];
pub type Matrix4 = [Vector4; 4];

#[derive(Copy, Clone, Debug)]
pub enum Projection {
    /// Perspective projection with the given `fovy` in degrees.
    Perspective {
        fovy: f32,
    },
    Orthographic {
        view_width: f32,
    },
}

impl Projection {
    pub fn is_orthographic(self) -> bool {
        match self {
            Projection::Perspective { .. } => false,
            Projection::Orthographic { .. } => true,
        }
    }
}

/// The gizmo context for a single frame.
pub struct Gizmo<'a, 'ui> {
    ui: &'a Ui<'ui>,
}

impl<'a, 'ui> Gizmo<'a, 'ui> {
    pub fn begin_frame(ui: &'a Ui<'ui>) -> Gizmo<'a, 'ui> {
        begin_frame(ui)
    }

    pub fn builder(&'a self, view: &'a Matrix4, model: &'a mut Matrix4) -> Builder<'a, 'ui> {
        Builder::new(self, view, model)
    }

    pub fn ui(&self) -> &'a Ui<'ui> {
        self.ui
    }

    /// Call inside of a window, before `manipulate` in order to draw a gizmo in that window.
    pub fn set_draw_list(&self) {
        set_draw_list(self);
    }

    /// Returns true if the mouse cursor is over any gizmo control (e.g. axis, plan, or screen component).
    pub fn is_over(&self) -> bool {
        is_over(self)
    }

    /// Returns true is the mouse is over a gizmo control and the gizmo is in a moving state.
    pub fn is_using(&self) -> bool {
        is_using(self)
    }

    /// Enable or disable the gizmo. This state is sticky until the the next call to `enable`.
    /// Gizmos are rendered with grey half transparent color when disabled.
    pub fn enable(&self, enabled: bool) {
        enable(self, enabled)
    }

    /// Render as orthorgraphic. The default is false.
    pub fn set_orthographic(&self, is_orthographic: bool) {
        set_orthographic(self, is_orthographic)
    }

    /// Set the viewport for rendering. Set to the display size or combine with
    /// `set_draw_list` to render inside of a window.
    pub fn set_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        set_rect(self, x, y, width, height)
    }

    /// Render a gizmo for manipulating a transformation. See [`Gizmo::manipulate`](struct.Gizmo.html#method.manipulate).
    #[allow(clippy::too_many_arguments)]
    pub fn manipulate(
        &self,
        view: &Matrix4,
        projection: &Matrix4,
        operation: Operation,
        mode: Mode,
        model: &mut Matrix4,
        delta_matrix: Option<&mut Matrix4>,
        snap: Option<&mut Vector3>,
        local_bounds: Option<&mut [Vector3; 2]>,
        bounds_snap: Option<&mut Vector3>,
    ) {
        manipulate(
            self,
            view,
            projection,
            operation,
            mode,
            model,
            delta_matrix,
            snap,
            local_bounds,
            bounds_snap,
        )
    }

    pub fn view_manipulate(
        &self,
        view: &mut Matrix4,
        camera_distance: f32,
        position: Vector2,
        size: Vector2,
        background_color: u32,
    ) {
        unsafe {
            ffi::ImGuizmo_ViewManipulate(view, camera_distance, &position, &size, background_color);
        }
    }

    /// Draw a cube for debugging with `manipulate`.
    pub fn draw_cube(&self, view: &Matrix4, projection: &Matrix4, model: &Matrix4) {
        draw_cube(self, view, projection, model)
    }

    /// Draw a grid for debugging.
    pub fn draw_grid(&self, view: &Matrix4, projection: &Matrix4, model: &Matrix4, grid_size: f32) {
        draw_grid(self, view, projection, model, grid_size)
    }

    fn prepare_projection(&self, windowed: bool, proj: Projection) -> Matrix4 {
        let rect = if windowed {
            self.set_draw_list();
            Rect::from_window(self.ui())
        } else {
            Rect::from_display(self.ui())
        };
        let projection = match proj {
            Projection::Perspective { fovy } => {
                let aspect_ratio = rect.width / rect.height;
                perspective(fovy, aspect_ratio, 0.1, 100.0)
            }
            Projection::Orthographic { view_width } => {
                let width = rect.width;
                let height = rect.height;
                let view_height = view_width * height / width;
                orthographic(
                    -view_width,
                    view_width,
                    -view_height,
                    view_height,
                    -view_width,
                    view_width,
                )
            }
        };

        self.set_orthographic(proj.is_orthographic());
        self.set_rect(rect.x, rect.y, rect.width, rect.height);
        projection
    }
}

/// Call at the start of a new ImGui frame.
fn begin_frame<'a, 'ui>(ui: &'a Ui<'ui>) -> Gizmo<'a, 'ui> {
    unsafe {
        ffi::ImGuizmo_BeginFrame();
    }
    Gizmo { ui }
}

/// Call inside of a window, before `manipulate` in order to draw a gizmo in that window.
fn set_draw_list<'a, 'ui>(_frame: &Gizmo<'a, 'ui>) {
    unsafe {
        ffi::ImGuizmo_SetDrawlist();
    }
}

/// Returns true if the mouse cursor is over any gizmo control (e.g. axis, plan, or screen component).
fn is_over<'a, 'ui>(_frame: &Gizmo<'a, 'ui>) -> bool {
    unsafe { ffi::ImGuizmo_IsOver() }
}

/// Returns true is the mouse is over a gizmo control and the gizmo is in a moving state.
fn is_using<'a, 'ui>(_frame: &Gizmo<'a, 'ui>) -> bool {
    unsafe { ffi::ImGuizmo_IsUsing() }
}

/// Enable or disable the gizmo. This state is sticky until the the next call to `enable`.
/// Gizmos are rendered with grey half transparent color when disabled.
fn enable<'a, 'ui>(_frame: &Gizmo<'a, 'ui>, enable: bool) {
    unsafe {
        ffi::ImGuizmo_Enable(enable);
    }
}

/// Helper function to decompose a matrix into its components.
pub fn decompose_matrix_to_components(
    matrix: &Matrix4,
    translation: &mut Vector3,
    rotation: &mut Vector3,
    scale: &mut Vector3,
) {
    unsafe {
        ffi::ImGuizmo_DecomposeMatrixToComponents(matrix, translation, rotation, scale);
    }
}

/// Helper function to compose a matrix from it's components.
pub fn recompose_matrix_from_components(
    translation: &Vector3,
    rotation: &Vector3,
    scale: &Vector3,
    matrix: &mut Matrix4,
) {
    unsafe {
        ffi::ImGuizmo_RecomposeMatrixFromComponents(translation, rotation, scale, matrix);
    }
}

/// Set the viewport for rendering. Set to the display size or combine with
/// `set_draw_list` to render inside of a window.
fn set_rect<'a, 'ui>(_frame: &Gizmo<'a, 'ui>, x: f32, y: f32, width: f32, height: f32) {
    unsafe {
        ffi::ImGuizmo_SetRect(x, y, width, height);
    }
}

/// Render as orthorgraphic. The default is false.
fn set_orthographic<'a, 'ui>(_frame: &Gizmo<'a, 'ui>, is_orthographic: bool) {
    unsafe {
        ffi::ImGuizmo_SetOrthographic(is_orthographic);
    }
}

pub fn set_ortho(is_ortho: bool) {
    unsafe {
        ffi::ImGuizmo_SetOrthographic(is_ortho);
    }
}

/// Draw a cube for debugging with `manipulate`.
fn draw_cube<'a, 'ui>(
    _frame: &Gizmo<'a, 'ui>,
    view: &Matrix4,
    projection: &Matrix4,
    model: &Matrix4,
) {
    unsafe {
        ffi::ImGuizmo_DrawCubes(view, projection, model, 1);
    }
}

/// Draw a grid for debugging.
fn draw_grid<'a, 'ui>(
    _frame: &Gizmo<'a, 'ui>,
    view: &Matrix4,
    projection: &Matrix4,
    model: &Matrix4,
    grid_size: f32,
) {
    unsafe {
        ffi::ImGuizmo_DrawGrid(view, projection, model, grid_size);
    }
}

/// Render a gizmo for manipulating a transformation. See [`Gizmo::manipulate`](struct.Gizmo.html#method.manipulate).
#[allow(clippy::too_many_arguments)]
fn manipulate<'a, 'ui>(
    _frame: &Gizmo<'a, 'ui>,
    view: &Matrix4,
    projection: &Matrix4,
    operation: Operation,
    mode: Mode,
    model: &mut Matrix4,
    delta_matrix: Option<&mut Matrix4>,
    snap: Option<&mut Vector3>,
    local_bounds: Option<&mut [Vector3; 2]>,
    bounds_snap: Option<&mut Vector3>,
) {
    unsafe {
        let delta_matrix = delta_matrix.map_or_else(ptr::null_mut, |v| v.as_mut_ptr() as _);
        let snap = snap.map_or_else(ptr::null_mut, |v| v.as_mut_ptr());
        let local_bounds = local_bounds.map_or_else(ptr::null_mut, |v| v.as_mut_ptr() as _);
        let bounds_snap = bounds_snap.map_or_else(ptr::null_mut, |v| v.as_mut_ptr() as _);
        ffi::ImGuizmo_Manipulate(
            view,
            projection,
            operation,
            mode,
            model,
            delta_matrix,
            snap,
            local_bounds,
            bounds_snap,
        );
    }
}

/// Build a viewport from a window or the whole display.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Creates a viewport `Rect` from the current window position and size.
    pub fn from_window<'ui>(ui: &Ui<'ui>) -> Rect {
        let [x, y] = ui.window_pos();
        let [width, height] = ui.window_size();
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a viewport `Rect` from the display size.
    pub fn from_display<'ui>(ui: &Ui<'ui>) -> Rect {
        let [width, height] = ui.io().display_size;
        Rect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }
}

/// Configure a gizmo for transformation manipulation.
pub struct Builder<'a, 'ui> {
    pub gizmo: &'a Gizmo<'a, 'ui>,
    pub view: &'a Matrix4,
    pub model: &'a mut Matrix4,
    pub projection: Projection,
    pub operation: Operation,
    pub windowed: bool,
    pub mode: Mode,
    pub delta_matrix: Option<&'a mut Matrix4>,
    pub snap: Option<&'a mut Vector3>,
    pub local_bounds: Option<&'a mut [Vector3; 2]>,
    pub bounds_snap: Option<&'a mut Vector3>,
}

impl<'a, 'ui> Builder<'a, 'ui> {
    pub fn new(
        gizmo: &'a Gizmo<'a, 'ui>,
        view: &'a Matrix4,
        model: &'a mut Matrix4,
    ) -> Builder<'a, 'ui> {
        Builder {
            gizmo,
            view,
            model,
            projection: Projection::Perspective { fovy: 45.0 },
            operation: Operation::Rotate,
            windowed: false,
            mode: Mode::Local,
            delta_matrix: None,
            snap: None,
            local_bounds: None,
            bounds_snap: None,
        }
    }

    /// Set the view projection. Defaults to `Perspective` with a `fovy` of `45` degrees.
    pub fn with_projection(mut self, projection: Projection) -> Self {
        self.projection = projection;
        self
    }

    /// Set the transformation editing attribute. Defaults to `Rotate`.
    pub fn with_operation(mut self, operation: Operation) -> Self {
        self.operation = operation;
        self
    }

    /// Set to `true` to draw the Gizmo inside of the current window. Defaults to `false`.
    pub fn with_windowed(mut self, windowed: bool) -> Self {
        self.windowed = windowed;
        self
    }

    /// Set the transformation matrix mode. Defaults to `Local`.
    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_delta_matrix<T: Into<Option<&'a mut Matrix4>>>(mut self, delta_matrix: T) -> Self {
        self.delta_matrix = delta_matrix.into();
        self
    }

    pub fn with_snap<T: Into<Option<&'a mut Vector3>>>(mut self, snap: T) -> Self {
        self.snap = snap.into();
        self
    }

    pub fn with_local_bounds<T: Into<Option<&'a mut [Vector3; 2]>>>(
        mut self,
        local_bounds: T,
    ) -> Self {
        self.local_bounds = local_bounds.into();
        self
    }

    pub fn with_bounds_snap<T: Into<Option<&'a mut Vector3>>>(mut self, bounds_snap: T) -> Self {
        self.bounds_snap = bounds_snap.into();
        self
    }

    /// Draw the transformation manipulation gizmo. Automates the usage of [set_draw_list](fn.set_draw_list.html),
    /// [set_rect](fn.set_rect.html), [set_orthographic](fn.set_orthographic.html), and [manipulate](fn.manipulate.html)
    /// based on this `Gizmo`s attributes.
    pub fn manipulate(self) {
        let projection = self
            .gizmo
            .prepare_projection(self.windowed, self.projection);
        manipulate(
            self.gizmo,
            self.view,
            &projection,
            self.operation,
            self.mode,
            self.model,
            self.delta_matrix,
            self.snap,
            self.local_bounds,
            self.bounds_snap,
        );
    }

    /// Draw a grid for debugging.
    pub fn draw_grid(self, grid_size: f32) -> Self {
        let projection = self
            .gizmo
            .prepare_projection(self.windowed, self.projection);
        self.gizmo
            .draw_grid(self.view, &projection, &self.model, grid_size);
        self
    }

    /// Draw a cube for debugging with `manipulate`.
    pub fn draw_cube(self) -> Self {
        let projection = self
            .gizmo
            .prepare_projection(self.windowed, self.projection);
        self.gizmo.draw_cube(self.view, &projection, &self.model);
        self
    }
}

/// Helper for creating a frustum.
pub fn frustum(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    znear: f32,
    zfar: f32,
) -> [[f32; 4]; 4] {
    let mut m: [[f32; 4]; 4] = Default::default();

    let t1 = 2.0 * znear;
    let t2 = right - left;
    let t3 = top - bottom;
    let t4 = zfar - znear;

    m[0][0] = t1 / t2;
    m[0][1] = 0.0;
    m[0][2] = 0.0;
    m[0][3] = 0.0;

    m[1][0] = 0.0;
    m[1][1] = t1 / t3;
    m[1][2] = 0.0;
    m[1][3] = 0.0;

    m[2][0] = (right + left) / t2;
    m[2][1] = (top + bottom) / t3;
    m[2][2] = (-zfar - znear) / t4;
    m[2][3] = -1.0;

    m[3][0] = 0.0;
    m[3][1] = 0.0;
    m[3][2] = (-t1 * zfar) / t4;
    m[3][3] = 0.0;

    m
}

/// Helper for creating a perspective projection matrix.
pub fn perspective(fovy_degrees: f32, aspect_ratio: f32, znear: f32, zfar: f32) -> [[f32; 4]; 4] {
    let ymax = znear * (fovy_degrees * std::f32::consts::PI / 180.0).tan();
    let xmax = ymax * aspect_ratio;
    frustum(-xmax, xmax, -ymax, ymax, znear, zfar)
}

/// Helper for creating an orthographic projection matrix.
pub fn orthographic(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    znear: f32,
    zfar: f32,
) -> [[f32; 4]; 4] {
    let mut m: [[f32; 4]; 4] = Default::default();

    m[0][0] = 2.0 / (right - left);
    m[0][1] = 0.0;
    m[0][2] = 0.0;
    m[0][3] = 0.0;

    m[1][0] = 0.0;
    m[1][1] = 2.0 / (top - bottom);
    m[1][2] = 0.0;
    m[1][3] = 0.0;

    m[2][0] = 0.0;
    m[2][1] = 0.0;
    m[2][2] = 1.0 / (zfar - znear);
    m[2][3] = 0.0;

    m[3][0] = (left + right) / (left - right);
    m[3][1] = (top + bottom) / (bottom - top);
    m[3][2] = znear / (znear - zfar);
    m[3][3] = 1.0;

    m
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn foo() {
//        let view = Default::default();
//        let mut model = Default::default();
//
//        let mut local_bounds = [Vector3::default(); 2];
//
//        GizmoBuilder::new(&view, &mut model)
//            .with_local_bounds(&mut local_bounds)
//            .with_operation(Operation::Translate);
//    }
//}

/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct Struct_D3DCOLORVALUE {
    pub r: FLOAT,
    pub g: FLOAT,
    pub b: FLOAT,
    pub a: FLOAT,
}
impl ::std::default::Default for Struct_D3DCOLORVALUE {
    fn default() -> Struct_D3DCOLORVALUE { unsafe { ::std::mem::zeroed() } }
}
pub type D3DCOLORVALUE = Struct_D3DCOLORVALUE;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_D2D_POINT_2U {
    pub x: UINT32,
    pub y: UINT32,
}
impl ::std::default::Default for Struct_D2D_POINT_2U {
    fn default() -> Struct_D2D_POINT_2U { unsafe { ::std::mem::zeroed() } }
}
pub type D2D_POINT_2U = Struct_D2D_POINT_2U;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_D2D_POINT_2F {
    pub x: FLOAT,
    pub y: FLOAT,
}
impl ::std::default::Default for Struct_D2D_POINT_2F {
    fn default() -> Struct_D2D_POINT_2F { unsafe { ::std::mem::zeroed() } }
}
pub type D2D_POINT_2F = Struct_D2D_POINT_2F;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_D2D_RECT_F {
    pub left: FLOAT,
    pub top: FLOAT,
    pub right: FLOAT,
    pub bottom: FLOAT,
}
impl ::std::default::Default for Struct_D2D_RECT_F {
    fn default() -> Struct_D2D_RECT_F { unsafe { ::std::mem::zeroed() } }
}
pub type D2D_RECT_F = Struct_D2D_RECT_F;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_D2D_RECT_U {
    pub left: UINT32,
    pub top: UINT32,
    pub right: UINT32,
    pub bottom: UINT32,
}
impl ::std::default::Default for Struct_D2D_RECT_U {
    fn default() -> Struct_D2D_RECT_U { unsafe { ::std::mem::zeroed() } }
}
pub type D2D_RECT_U = Struct_D2D_RECT_U;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_D2D_SIZE_F {
    pub width: FLOAT,
    pub height: FLOAT,
}
impl ::std::default::Default for Struct_D2D_SIZE_F {
    fn default() -> Struct_D2D_SIZE_F { unsafe { ::std::mem::zeroed() } }
}
pub type D2D_SIZE_F = Struct_D2D_SIZE_F;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_D2D_SIZE_U {
    pub width: UINT32,
    pub height: UINT32,
}
impl ::std::default::Default for Struct_D2D_SIZE_U {
    fn default() -> Struct_D2D_SIZE_U { unsafe { ::std::mem::zeroed() } }
}
pub type D2D_SIZE_U = Struct_D2D_SIZE_U;
pub type D2D_COLOR_F = D3DCOLORVALUE;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_D2D_MATRIX_3X2_F {
    pub _11: FLOAT,
    pub _12: FLOAT,
    pub _21: FLOAT,
    pub _22: FLOAT,
    pub _31: FLOAT,
    pub _32: FLOAT,
}
impl ::std::default::Default for Struct_D2D_MATRIX_3X2_F {
    fn default() -> Struct_D2D_MATRIX_3X2_F {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type D2D_MATRIX_3X2_F = Struct_D2D_MATRIX_3X2_F;

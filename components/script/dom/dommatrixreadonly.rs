/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;
use std::ptr::NonNull;
use std::{f64, ptr};

use cssparser::{Parser, ParserInput};
use dom_struct::dom_struct;
use euclid::default::Transform3D;
use euclid::Angle;
use js::jsapi::JSObject;
use js::rust::{CustomAutoRooterGuard, HandleObject};
use js::typedarray::{CreateWith, Float32Array, Float64Array};
use style::parser::ParserContext;

use crate::dom::bindings::cell::{DomRefCell, Ref};
use crate::dom::bindings::codegen::Bindings::DOMMatrixBinding::{DOMMatrixInit, DOMMatrixMethods};
use crate::dom::bindings::codegen::Bindings::DOMMatrixReadOnlyBinding::DOMMatrixReadOnlyMethods;
use crate::dom::bindings::codegen::Bindings::DOMPointBinding::DOMPointInit;
use crate::dom::bindings::codegen::UnionTypes::StringOrUnrestrictedDoubleSequence;
use crate::dom::bindings::error;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::typedarrays::create_float32_array;
use crate::dom::dommatrix::DOMMatrix;
use crate::dom::dompoint::DOMPoint;
use crate::dom::globalscope::GlobalScope;
use crate::dom::window::Window;
use crate::script_runtime::JSContext;

#[dom_struct]
#[allow(non_snake_case)]
pub struct DOMMatrixReadOnly {
    reflector_: Reflector,
    #[no_trace]
    matrix: DomRefCell<Transform3D<f64>>,
    is2D: Cell<bool>,
}

#[allow(non_snake_case)]
impl DOMMatrixReadOnly {
    pub fn new(global: &GlobalScope, is2D: bool, matrix: Transform3D<f64>) -> DomRoot<Self> {
        Self::new_with_proto(global, None, is2D, matrix)
    }

    #[allow(crown::unrooted_must_root)]
    fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        is2D: bool,
        matrix: Transform3D<f64>,
    ) -> DomRoot<Self> {
        let dommatrix = Self::new_inherited(is2D, matrix);
        reflect_dom_object_with_proto(Box::new(dommatrix), global, proto)
    }

    pub fn new_inherited(is2D: bool, matrix: Transform3D<f64>) -> Self {
        DOMMatrixReadOnly {
            reflector_: Reflector::new(),
            matrix: DomRefCell::new(matrix),
            is2D: Cell::new(is2D),
        }
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-dommatrixreadonly
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        init: Option<StringOrUnrestrictedDoubleSequence>,
    ) -> Fallible<DomRoot<Self>> {
        if init.is_none() {
            return Ok(Self::new_with_proto(
                global,
                proto,
                true,
                Transform3D::identity(),
            ));
        }
        match init.unwrap() {
            StringOrUnrestrictedDoubleSequence::String(ref s) => {
                if global.downcast::<Window>().is_none() {
                    return Err(error::Error::Type(
                        "String constructor is only supported in the main thread.".to_owned(),
                    ));
                }
                if s.is_empty() {
                    return Ok(Self::new(global, true, Transform3D::identity()));
                }
                transform_to_matrix(s.to_string())
                    .map(|(is2D, matrix)| Self::new_with_proto(global, proto, is2D, matrix))
            },
            StringOrUnrestrictedDoubleSequence::UnrestrictedDoubleSequence(ref entries) => {
                entries_to_matrix(&entries[..])
                    .map(|(is2D, matrix)| Self::new_with_proto(global, proto, is2D, matrix))
            },
        }
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-frommatrix
    pub fn FromMatrix(global: &GlobalScope, other: &DOMMatrixInit) -> Fallible<DomRoot<Self>> {
        dommatrixinit_to_matrix(&other).map(|(is2D, matrix)| Self::new(global, is2D, matrix))
    }

    pub fn matrix(&self) -> Ref<Transform3D<f64>> {
        self.matrix.borrow()
    }

    pub fn is2D(&self) -> bool {
        self.is2D.get()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m11
    pub fn set_m11(&self, value: f64) {
        self.matrix.borrow_mut().m11 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m12
    pub fn set_m12(&self, value: f64) {
        self.matrix.borrow_mut().m12 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m13
    pub fn set_m13(&self, value: f64) {
        self.matrix.borrow_mut().m13 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m14
    pub fn set_m14(&self, value: f64) {
        self.matrix.borrow_mut().m14 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m21
    pub fn set_m21(&self, value: f64) {
        self.matrix.borrow_mut().m21 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m22
    pub fn set_m22(&self, value: f64) {
        self.matrix.borrow_mut().m22 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m23
    pub fn set_m23(&self, value: f64) {
        self.matrix.borrow_mut().m23 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m24
    pub fn set_m24(&self, value: f64) {
        self.matrix.borrow_mut().m24 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m31
    pub fn set_m31(&self, value: f64) {
        self.matrix.borrow_mut().m31 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m32
    pub fn set_m32(&self, value: f64) {
        self.matrix.borrow_mut().m32 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m33
    pub fn set_m33(&self, value: f64) {
        self.matrix.borrow_mut().m33 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m34
    pub fn set_m34(&self, value: f64) {
        self.matrix.borrow_mut().m34 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m41
    pub fn set_m41(&self, value: f64) {
        self.matrix.borrow_mut().m41 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m42
    pub fn set_m42(&self, value: f64) {
        self.matrix.borrow_mut().m42 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m43
    pub fn set_m43(&self, value: f64) {
        self.matrix.borrow_mut().m43 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m44
    pub fn set_m44(&self, value: f64) {
        self.matrix.borrow_mut().m44 = value;
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-multiplyself
    pub fn multiply_self(&self, other: &DOMMatrixInit) -> Fallible<()> {
        // Step 1.
        dommatrixinit_to_matrix(&other).map(|(is2D, other_matrix)| {
            // Step 2.
            let mut matrix = self.matrix.borrow_mut();
            *matrix = other_matrix.then(&matrix);
            // Step 3.
            if !is2D {
                self.is2D.set(false);
            }
            // Step 4 in DOMMatrix.MultiplySelf
        })
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-premultiplyself
    pub fn pre_multiply_self(&self, other: &DOMMatrixInit) -> Fallible<()> {
        // Step 1.
        dommatrixinit_to_matrix(&other).map(|(is2D, other_matrix)| {
            // Step 2.
            let mut matrix = self.matrix.borrow_mut();
            *matrix = matrix.then(&other_matrix);
            // Step 3.
            if !is2D {
                self.is2D.set(false);
            }
            // Step 4 in DOMMatrix.PreMultiplySelf
        })
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-translateself
    pub fn translate_self(&self, tx: f64, ty: f64, tz: f64) {
        // Step 1.
        let translation = Transform3D::translation(tx, ty, tz);
        let mut matrix = self.matrix.borrow_mut();
        *matrix = translation.then(&matrix);
        // Step 2.
        if tz != 0.0 {
            self.is2D.set(false);
        }
        // Step 3 in DOMMatrix.TranslateSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-scaleself
    pub fn scale_self(
        &self,
        scaleX: f64,
        scaleY: Option<f64>,
        scaleZ: f64,
        mut originX: f64,
        mut originY: f64,
        mut originZ: f64,
    ) {
        // Step 1.
        self.translate_self(originX, originY, originZ);
        // Step 2.
        let scaleY = scaleY.unwrap_or(scaleX);
        // Step 3.
        {
            let scale3D = Transform3D::scale(scaleX, scaleY, scaleZ);
            let mut matrix = self.matrix.borrow_mut();
            *matrix = scale3D.then(&matrix);
        }
        // Step 4.
        originX = -originX;
        originY = -originY;
        originZ = -originZ;
        // Step 5.
        self.translate_self(originX, originY, originZ);
        // Step 6.
        if scaleZ != 1.0 || originZ != 0.0 {
            self.is2D.set(false);
        }
        // Step 7 in DOMMatrix.ScaleSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-scale3dself
    pub fn scale_3d_self(&self, scale: f64, originX: f64, originY: f64, originZ: f64) {
        // Step 1.
        self.translate_self(originX, originY, originZ);
        // Step 2.
        {
            let scale3D = Transform3D::scale(scale, scale, scale);
            let mut matrix = self.matrix.borrow_mut();
            *matrix = scale3D.then(&matrix);
        }
        // Step 3.
        self.translate_self(-originX, -originY, -originZ);
        // Step 4.
        if scale != 1.0 {
            self.is2D.set(false);
        }
        // Step 5 in DOMMatrix.Scale3dSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-rotateself
    pub fn rotate_self(&self, mut rotX: f64, mut rotY: Option<f64>, mut rotZ: Option<f64>) {
        // Step 1.
        if rotY.is_none() && rotZ.is_none() {
            rotZ = Some(rotX);
            rotX = 0.0;
            rotY = Some(0.0);
        }
        // Step 2.
        let rotY = rotY.unwrap_or(0.0);
        // Step 3.
        let rotZ = rotZ.unwrap_or(0.0);
        // Step 4.
        if rotX != 0.0 || rotY != 0.0 {
            self.is2D.set(false);
        }
        if rotZ != 0.0 {
            // Step 5.
            let rotation = Transform3D::rotation(0.0, 0.0, 1.0, Angle::radians(rotZ.to_radians()));
            let mut matrix = self.matrix.borrow_mut();
            *matrix = rotation.then(&matrix);
        }
        if rotY != 0.0 {
            // Step 6.
            let rotation = Transform3D::rotation(0.0, 1.0, 0.0, Angle::radians(rotY.to_radians()));
            let mut matrix = self.matrix.borrow_mut();
            *matrix = rotation.then(&matrix);
        }
        if rotX != 0.0 {
            // Step 7.
            let rotation = Transform3D::rotation(1.0, 0.0, 0.0, Angle::radians(rotX.to_radians()));
            let mut matrix = self.matrix.borrow_mut();
            *matrix = rotation.then(&matrix);
        }
        // Step 8 in DOMMatrix.RotateSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-rotatefromvectorself
    pub fn rotate_from_vector_self(&self, x: f64, y: f64) {
        // don't do anything when the rotation angle is zero or undefined
        if y != 0.0 || x < 0.0 {
            // Step 1.
            let rotZ = Angle::radians(f64::atan2(y, x));
            let rotation = Transform3D::rotation(0.0, 0.0, 1.0, rotZ);
            let mut matrix = self.matrix.borrow_mut();
            *matrix = rotation.then(&matrix);
        }
        // Step 2 in DOMMatrix.RotateFromVectorSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-rotateaxisangleself
    pub fn rotate_axis_angle_self(&self, x: f64, y: f64, z: f64, angle: f64) {
        // Step 1.
        let (norm_x, norm_y, norm_z) = normalize_point(x, y, z);
        // Beware: pass negated value until https://github.com/servo/euclid/issues/354
        let rotation =
            Transform3D::rotation(norm_x, norm_y, norm_z, Angle::radians(angle.to_radians()));
        let mut matrix = self.matrix.borrow_mut();
        *matrix = rotation.then(&matrix);
        // Step 2.
        if x != 0.0 || y != 0.0 {
            self.is2D.set(false);
        }
        // Step 3 in DOMMatrix.RotateAxisAngleSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-skewxself
    pub fn skew_x_self(&self, sx: f64) {
        // Step 1.
        let skew = Transform3D::skew(Angle::radians(sx.to_radians()), Angle::radians(0.0));
        let mut matrix = self.matrix.borrow_mut();
        *matrix = skew.then(&matrix);
        // Step 2 in DOMMatrix.SkewXSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-skewyself
    pub fn skew_y_self(&self, sy: f64) {
        // Step 1.
        let skew = Transform3D::skew(Angle::radians(0.0), Angle::radians(sy.to_radians()));
        let mut matrix = self.matrix.borrow_mut();
        *matrix = skew.then(&matrix);
        // Step 2 in DOMMatrix.SkewYSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrix-invertself
    pub fn invert_self(&self) {
        let mut matrix = self.matrix.borrow_mut();
        // Step 1.
        *matrix = matrix.inverse().unwrap_or_else(|| {
            // Step 2.
            self.is2D.set(false);
            Transform3D::new(
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
            )
        })
        // Step 3 in DOMMatrix.InvertSelf
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-fromfloat32array
    #[allow(unsafe_code)]
    pub fn FromFloat32Array(
        global: &GlobalScope,
        array: CustomAutoRooterGuard<Float32Array>,
    ) -> Fallible<DomRoot<DOMMatrixReadOnly>> {
        let vec: Vec<f64> = array.to_vec().iter().map(|&x| x as f64).collect();
        DOMMatrixReadOnly::Constructor(
            global,
            None,
            Some(StringOrUnrestrictedDoubleSequence::UnrestrictedDoubleSequence(vec)),
        )
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-fromfloat64array
    #[allow(unsafe_code)]
    pub fn FromFloat64Array(
        global: &GlobalScope,
        array: CustomAutoRooterGuard<Float64Array>,
    ) -> Fallible<DomRoot<DOMMatrixReadOnly>> {
        let vec: Vec<f64> = array.to_vec();
        DOMMatrixReadOnly::Constructor(
            global,
            None,
            Some(StringOrUnrestrictedDoubleSequence::UnrestrictedDoubleSequence(vec)),
        )
    }
}

#[allow(non_snake_case)]
impl DOMMatrixReadOnlyMethods for DOMMatrixReadOnly {
    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m11
    fn M11(&self) -> f64 {
        self.matrix.borrow().m11
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m12
    fn M12(&self) -> f64 {
        self.matrix.borrow().m12
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m13
    fn M13(&self) -> f64 {
        self.matrix.borrow().m13
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m14
    fn M14(&self) -> f64 {
        self.matrix.borrow().m14
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m21
    fn M21(&self) -> f64 {
        self.matrix.borrow().m21
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m22
    fn M22(&self) -> f64 {
        self.matrix.borrow().m22
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m23
    fn M23(&self) -> f64 {
        self.matrix.borrow().m23
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m24
    fn M24(&self) -> f64 {
        self.matrix.borrow().m24
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m31
    fn M31(&self) -> f64 {
        self.matrix.borrow().m31
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m32
    fn M32(&self) -> f64 {
        self.matrix.borrow().m32
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m33
    fn M33(&self) -> f64 {
        self.matrix.borrow().m33
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m34
    fn M34(&self) -> f64 {
        self.matrix.borrow().m34
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m41
    fn M41(&self) -> f64 {
        self.matrix.borrow().m41
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m42
    fn M42(&self) -> f64 {
        self.matrix.borrow().m42
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m43
    fn M43(&self) -> f64 {
        self.matrix.borrow().m43
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-m44
    fn M44(&self) -> f64 {
        self.matrix.borrow().m44
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-a
    fn A(&self) -> f64 {
        self.M11()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-b
    fn B(&self) -> f64 {
        self.M12()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-c
    fn C(&self) -> f64 {
        self.M21()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-d
    fn D(&self) -> f64 {
        self.M22()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-e
    fn E(&self) -> f64 {
        self.M41()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-f
    fn F(&self) -> f64 {
        self.M42()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-is2d
    fn Is2D(&self) -> bool {
        self.is2D.get()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-isidentity
    fn IsIdentity(&self) -> bool {
        let matrix = self.matrix.borrow();
        matrix.m12 == 0.0 &&
            matrix.m13 == 0.0 &&
            matrix.m14 == 0.0 &&
            matrix.m21 == 0.0 &&
            matrix.m23 == 0.0 &&
            matrix.m24 == 0.0 &&
            matrix.m31 == 0.0 &&
            matrix.m32 == 0.0 &&
            matrix.m34 == 0.0 &&
            matrix.m41 == 0.0 &&
            matrix.m42 == 0.0 &&
            matrix.m43 == 0.0 &&
            matrix.m11 == 1.0 &&
            matrix.m22 == 1.0 &&
            matrix.m33 == 1.0 &&
            matrix.m44 == 1.0
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-translate
    fn Translate(&self, tx: f64, ty: f64, tz: f64) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).TranslateSelf(tx, ty, tz)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-scale
    fn Scale(
        &self,
        scaleX: f64,
        scaleY: Option<f64>,
        scaleZ: f64,
        originX: f64,
        originY: f64,
        originZ: f64,
    ) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self)
            .ScaleSelf(scaleX, scaleY, scaleZ, originX, originY, originZ)
    }

    // https://drafts.fxtf.org/geometry/#dom-dommatrixreadonly-scalenonuniform
    fn ScaleNonUniform(&self, scaleX: f64, scaleY: f64) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).ScaleSelf(
            scaleX,
            Some(scaleY),
            1.0,
            0.0,
            0.0,
            0.0,
        )
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-scale3d
    fn Scale3d(&self, scale: f64, originX: f64, originY: f64, originZ: f64) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).Scale3dSelf(scale, originX, originY, originZ)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-rotate
    fn Rotate(&self, rotX: f64, rotY: Option<f64>, rotZ: Option<f64>) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).RotateSelf(rotX, rotY, rotZ)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-rotatefromvector
    fn RotateFromVector(&self, x: f64, y: f64) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).RotateFromVectorSelf(x, y)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-rotateaxisangle
    fn RotateAxisAngle(&self, x: f64, y: f64, z: f64, angle: f64) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).RotateAxisAngleSelf(x, y, z, angle)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-skewx
    fn SkewX(&self, sx: f64) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).SkewXSelf(sx)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-skewy
    fn SkewY(&self, sy: f64) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).SkewYSelf(sy)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-multiply
    fn Multiply(&self, other: &DOMMatrixInit) -> Fallible<DomRoot<DOMMatrix>> {
        DOMMatrix::from_readonly(&self.global(), self).MultiplySelf(&other)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-flipx
    fn FlipX(&self) -> DomRoot<DOMMatrix> {
        let is2D = self.is2D.get();
        let flip = Transform3D::new(
            -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let matrix = flip.then(&self.matrix.borrow());
        DOMMatrix::new(&self.global(), is2D, matrix)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-flipy
    fn FlipY(&self) -> DomRoot<DOMMatrix> {
        let is2D = self.is2D.get();
        let flip = Transform3D::new(
            1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let matrix = flip.then(&self.matrix.borrow());
        DOMMatrix::new(&self.global(), is2D, matrix)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-inverse
    fn Inverse(&self) -> DomRoot<DOMMatrix> {
        DOMMatrix::from_readonly(&self.global(), self).InvertSelf()
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-transformpoint
    fn TransformPoint(&self, point: &DOMPointInit) -> DomRoot<DOMPoint> {
        // Euclid always normalizes the homogeneous coordinate which is usually the right
        // thing but may (?) not be compliant with the CSS matrix spec (or at least is
        // probably not the behavior web authors will expect even if it is mathematically
        // correct in the context of geometry computations).
        // Since this is the only place where this is needed, better implement it here
        // than in euclid (which does not have a notion of 4d points).
        let mat = self.matrix.borrow();
        let x = point.x * mat.m11 + point.y * mat.m21 + point.z * mat.m31 + point.w * mat.m41;
        let y = point.x * mat.m12 + point.y * mat.m22 + point.z * mat.m32 + point.w * mat.m42;
        let z = point.x * mat.m13 + point.y * mat.m23 + point.z * mat.m33 + point.w * mat.m43;
        let w = point.x * mat.m14 + point.y * mat.m24 + point.z * mat.m34 + point.w * mat.m44;

        DOMPoint::new(&self.global(), x, y, z, w)
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-tofloat32array
    fn ToFloat32Array(&self, cx: JSContext) -> Float32Array {
        let vec: Vec<f32> = self
            .matrix
            .borrow()
            .to_array()
            .iter()
            .map(|&x| x as f32)
            .collect();
        rooted!(in (*cx) let mut array = ptr::null_mut::<JSObject>());
        create_float32_array(cx, &vec, array.handle_mut())
            .expect("Converting matrix to float32 array should never fail")
    }

    // https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-tofloat64array
    #[allow(unsafe_code)]
    fn ToFloat64Array(&self, cx: JSContext) -> NonNull<JSObject> {
        let arr = self.matrix.borrow().to_array();
        unsafe {
            rooted!(in (*cx) let mut array = ptr::null_mut::<JSObject>());
            let _ = Float64Array::create(*cx, CreateWith::Slice(&arr), array.handle_mut()).unwrap();
            NonNull::new_unchecked(array.get())
        }
    }
}

// https://drafts.fxtf.org/geometry-1/#create-a-2d-matrix
fn create_2d_matrix(entries: &[f64]) -> Transform3D<f64> {
    Transform3D::new(
        entries[0], entries[1], 0.0, 0.0, entries[2], entries[3], 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        entries[4], entries[5], 0.0, 1.0,
    )
}

// https://drafts.fxtf.org/geometry-1/#create-a-3d-matrix
fn create_3d_matrix(entries: &[f64]) -> Transform3D<f64> {
    Transform3D::new(
        entries[0],
        entries[1],
        entries[2],
        entries[3],
        entries[4],
        entries[5],
        entries[6],
        entries[7],
        entries[8],
        entries[9],
        entries[10],
        entries[11],
        entries[12],
        entries[13],
        entries[14],
        entries[15],
    )
}

// https://drafts.fxtf.org/geometry-1/#dom-dommatrixreadonly-dommatrixreadonly-numbersequence
pub fn entries_to_matrix(entries: &[f64]) -> Fallible<(bool, Transform3D<f64>)> {
    if entries.len() == 6 {
        Ok((true, create_2d_matrix(&entries)))
    } else if entries.len() == 16 {
        Ok((false, create_3d_matrix(&entries)))
    } else {
        let err_msg = format!("Expected 6 or 16 entries, but found {}.", entries.len());
        Err(error::Error::Type(err_msg.to_owned()))
    }
}

// https://drafts.fxtf.org/geometry-1/#validate-and-fixup
pub fn dommatrixinit_to_matrix(dict: &DOMMatrixInit) -> Fallible<(bool, Transform3D<f64>)> {
    // Step 1.
    if dict.parent.a.is_some() &&
        dict.parent.m11.is_some() &&
        dict.parent.a.unwrap() != dict.parent.m11.unwrap() ||
        dict.parent.b.is_some() &&
            dict.parent.m12.is_some() &&
            dict.parent.b.unwrap() != dict.parent.m12.unwrap() ||
        dict.parent.c.is_some() &&
            dict.parent.m21.is_some() &&
            dict.parent.c.unwrap() != dict.parent.m21.unwrap() ||
        dict.parent.d.is_some() &&
            dict.parent.m22.is_some() &&
            dict.parent.d.unwrap() != dict.parent.m22.unwrap() ||
        dict.parent.e.is_some() &&
            dict.parent.m41.is_some() &&
            dict.parent.e.unwrap() != dict.parent.m41.unwrap() ||
        dict.parent.f.is_some() &&
            dict.parent.m42.is_some() &&
            dict.parent.f.unwrap() != dict.parent.m42.unwrap() ||
        dict.is2D.is_some() &&
            dict.is2D.unwrap() &&
            (dict.m31 != 0.0 ||
                dict.m32 != 0.0 ||
                dict.m13 != 0.0 ||
                dict.m23 != 0.0 ||
                dict.m43 != 0.0 ||
                dict.m14 != 0.0 ||
                dict.m24 != 0.0 ||
                dict.m34 != 0.0 ||
                dict.m33 != 1.0 ||
                dict.m44 != 1.0)
    {
        Err(error::Error::Type("Invalid matrix initializer.".to_owned()))
    } else {
        let mut is_2d = dict.is2D;
        // Step 2.
        let m11 = dict.parent.m11.unwrap_or(dict.parent.a.unwrap_or(1.0));
        // Step 3.
        let m12 = dict.parent.m12.unwrap_or(dict.parent.b.unwrap_or(0.0));
        // Step 4.
        let m21 = dict.parent.m21.unwrap_or(dict.parent.c.unwrap_or(0.0));
        // Step 5.
        let m22 = dict.parent.m22.unwrap_or(dict.parent.d.unwrap_or(1.0));
        // Step 6.
        let m41 = dict.parent.m41.unwrap_or(dict.parent.e.unwrap_or(0.0));
        // Step 7.
        let m42 = dict.parent.m42.unwrap_or(dict.parent.f.unwrap_or(0.0));
        // Step 8.
        if is_2d.is_none() &&
            (dict.m31 != 0.0 ||
                dict.m32 != 0.0 ||
                dict.m13 != 0.0 ||
                dict.m23 != 0.0 ||
                dict.m43 != 0.0 ||
                dict.m14 != 0.0 ||
                dict.m24 != 0.0 ||
                dict.m34 != 0.0 ||
                dict.m33 != 1.0 ||
                dict.m44 != 1.0)
        {
            is_2d = Some(false);
        }
        // Step 9.
        if is_2d.is_none() {
            is_2d = Some(true);
        }
        let matrix = Transform3D::new(
            m11, m12, dict.m13, dict.m14, m21, m22, dict.m23, dict.m24, dict.m31, dict.m32,
            dict.m33, dict.m34, m41, m42, dict.m43, dict.m44,
        );
        Ok((is_2d.unwrap(), matrix))
    }
}

#[inline]
fn normalize_point(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let len = (x * x + y * y + z * z).sqrt();
    if len == 0.0 {
        (0.0, 0.0, 0.0)
    } else {
        (x / len, y / len, z / len)
    }
}

pub fn transform_to_matrix(value: String) -> Fallible<(bool, Transform3D<f64>)> {
    use style::properties::longhands::transform;

    let mut input = ParserInput::new(&value);
    let mut parser = Parser::new(&mut input);
    let url = ::servo_url::ServoUrl::parse("about:blank").unwrap();
    let context = ParserContext::new(
        ::style::stylesheets::Origin::Author,
        &url,
        Some(::style::stylesheets::CssRuleType::Style),
        ::style_traits::ParsingMode::DEFAULT,
        ::style::context::QuirksMode::NoQuirks,
        /* namespaces = */ Default::default(),
        None,
        None,
    );

    let transform = match parser.parse_entirely(|t| transform::parse(&context, t)) {
        Ok(result) => result,
        Err(..) => return Err(error::Error::Syntax),
    };

    let (m, is_3d) = match transform.to_transform_3d_matrix_f64(None) {
        Ok(result) => result,
        Err(..) => return Err(error::Error::Syntax),
    };

    Ok((!is_3d, m))
}

use std::ops::{Div, Sub, Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct IVec2(pub i32, pub i32);

impl From<Vec2> for IVec2 {
    fn from(v: Vec2) -> Self {
        IVec2(v.0 as i32, v.1 as i32)
    }
}

impl Div<i32> for IVec2 {
    type Output = Self;
    fn div(self, a: i32) -> Self::Output {
        IVec2(self.0 / a, self.1 / a)
    }
}

impl Sub for IVec2 {
    type Output = Self;
    fn sub(self, v: Self) -> Self::Output {
        IVec2(self.0 - v.0, self.1 - v.1)
    }
}

impl Add for IVec2 {
    type Output = Self;
    fn add(self, v: Self) -> Self::Output {
        IVec2(self.0 + v.0, self.1 + v.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2(pub f32, pub f32);

impl From<IVec2> for Vec2 {
    fn from(v: IVec2) -> Self {
        Vec2(v.0 as f32, v.1 as f32)
    }
}

impl Div for Vec2 {
    type Output = Self;
    fn div(self, v: Self) -> Self::Output {
        Vec2(self.0 / v.0, self.1 / v.1)
    }
}

impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, v: Self) -> Self::Output {
        Vec2(self.0 * v.0, self.1 * v.1)
    }
}

/// Bounding box with the min & max pixel definining the
/// top left and bottom right coordinate respectively.
pub struct BoundingBox {
    pub min: IVec2,
    pub max: IVec2,
}

impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.min.0, self.min.1, self.max.0, self.max.1)
    }
}

impl From<CenterBoundingBox> for BoundingBox {
    fn from(cbbox: CenterBoundingBox) -> Self {
        let min = cbbox.center - (cbbox.size/2);
        let max = cbbox.size + min;
        BoundingBox {
            min, max
        }
    }
}

impl From<(NormalizedCenterBoundingBox, Vec2)> for BoundingBox {
    fn from((bbox, s): (NormalizedCenterBoundingBox, Vec2)) -> Self {
        let cbbox = CenterBoundingBox::from((bbox, s));
        BoundingBox::from(cbbox)
    }
}

impl From<TopLeftBoundingBox> for BoundingBox {
    fn from(tlbbox: TopLeftBoundingBox) -> Self {
        BoundingBox {
            min: tlbbox.top_left,
            max: tlbbox.top_left + tlbbox.size,
        }

    }
}

/// Bounding box defined by it's top left coordinate, and
/// the size of the bounding box (width, height).
/// (Assumes that origin (0,0) of the image is the top left of the image)
///
/// The bounding box in the COCO annotations json format is a topleft bounding box.
pub struct TopLeftBoundingBox {
    pub top_left: IVec2,
    pub size: IVec2,
}

impl std::fmt::Display for TopLeftBoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.top_left.0, self.top_left.1, self.size.0, self.size.1)
    }
}


impl From<BoundingBox> for TopLeftBoundingBox {
    fn from(bbox: BoundingBox) -> Self {
        TopLeftBoundingBox {
            top_left: bbox.min,
            size: bbox.max - bbox.min,
        }
    }
}

/// Bounding box defined by the center of the bounding box, and
/// the size (width, height).
pub struct CenterBoundingBox {
    pub center: IVec2,
    pub size: IVec2,
}

impl std::fmt::Display for CenterBoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.center.0, self.center.1, self.size.0, self.size.1)
    }
}


impl From<BoundingBox> for CenterBoundingBox {
    fn from(bbox: BoundingBox) -> Self {
        let size = bbox.max - bbox.min;
        let center = (size/2) + bbox.min;
        CenterBoundingBox {
            center,
            size,
        }
    }
}

impl From<(NormalizedCenterBoundingBox, Vec2)> for CenterBoundingBox {
    fn from((bbox, full_size): (NormalizedCenterBoundingBox, Vec2)) -> Self {
        let center: IVec2 = (bbox.center * full_size).into();
        let size: IVec2 = (bbox.size * full_size).into();

        CenterBoundingBox {
            center,
            size,
        }
    }
}

/// Center bounding box relative to image size. Values are
/// between [0,1] normalized to the width & height of the image.
///
/// The bounding box for Yolov5 labels is a normalized center bbox with
/// origin at the top left.
#[derive(Debug)]
pub struct NormalizedCenterBoundingBox {
    pub center: Vec2,
    pub size: Vec2,
}

impl std::fmt::Display for NormalizedCenterBoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.center.0, self.center.1, self.size.0, self.size.1)
    }
}

impl From<(CenterBoundingBox, Vec2)> for NormalizedCenterBoundingBox {
    fn from((cbbox, full_size): (CenterBoundingBox, Vec2)) -> NormalizedCenterBoundingBox {
        NormalizedCenterBoundingBox {
            center: Vec2::from(cbbox.center) / full_size.clone(),
            size: Vec2::from(cbbox.size) / full_size
        }
    }
}

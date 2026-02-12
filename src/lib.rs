//! This library includes icons from Google's [material design icons
//! repository](https://github.com/google/material-design-icons).

use kurbo::{PathEl, Point, Rect, Shape, Size};

/// Material icons as kurbo paths
#[derive(Debug, Copy, Clone)]
pub struct IconPaths {
    pub paths: &'static [IconPath],
    pub size: Size,
}

#[derive(Debug, Copy, Clone)]
pub struct IconPath {
    pub els: &'static [PathEl],
    pub opacity: f64,
}

impl Shape for IconPath {
    type PathElementsIter<'a> = std::iter::Copied<std::slice::Iter<'static, PathEl>>;
    fn path_elements(&self, _tolerance: f64) -> Self::PathElementsIter<'_> {
        self.els.iter().copied()
    }

    fn area(&self) -> f64 {
        self.els.area()
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        self.els.perimeter(accuracy)
    }

    fn winding(&self, pt: Point) -> i32 {
        self.els.winding(pt)
    }
    fn bounding_box(&self) -> Rect {
        self.els.bounding_box()
    }

    fn as_path_slice(&self) -> Option<&[PathEl]> {
        Some(self.els)
    }
}

include!("./icons.rs.in");

#[macro_use]
extern crate assert_float_eq;

pub mod cie;
pub mod hyperbolic;

use cie::CIELAB;
pub use cie::SRGB;

use hyperbolic::HPoint;
use num_complex::Complex;



#[derive(Clone, Copy)]
pub struct H99{
    pub luma : f64,
    pub chroma : hyperbolic::HPoint
}


impl H99{
    pub fn new(luma : f64, chroma_poincare : Complex<f64>)->Self{
        H99{luma,chroma:HPoint(chroma_poincare)}
    }
}



const HYPER_R : f64 = 28.6;

//45°
const THERMAL_ANGLE : f64 = 0.78539816339;






impl From<CIELAB> for H99{
    fn from(lab:CIELAB) -> Self{
        let (l_star, a_star, b_star) = (lab.l_star, lab.a_star, lab.b_star);
        let l99c = 317.65 * (1.+0.0037*l_star).ln();
        let e99c = a_star;
        let f99c = 0.94*b_star;

        let g = (e99c*e99c + f99c*f99c).sqrt();

        let h99c = f99c.atan2(e99c) - THERMAL_ANGLE;

        let chroma99c = 23.0 * (1.+0.066*g).ln();

        let geodesic_radius = chroma99c / HYPER_R;

        let r = (geodesic_radius/2.0).tanh();


        let x = r*h99c.cos();
        let y = r*h99c.sin();

        H99{luma:l99c,chroma : HPoint(Complex{re:x,im:y})}
    }
}

impl From<H99> for CIELAB {
    fn from(h99: H99) -> Self {
        let l99c = h99.luma;
        
        let geodesic_radius = h99.chroma.distance(&HPoint::ORIGIN);
        let (x,y) = (h99.chroma.0.re,h99.chroma.0.im);
        
        let h99c = y.atan2(x) + THERMAL_ANGLE;

        let chroma99c = geodesic_radius * HYPER_R;

        let g = ((chroma99c/23.0).exp() - 1.0)/0.066;
        let e = g*h99c.cos();
        let f = g*h99c.sin();

        let l_star = ((l99c/317.65).exp() - 1.0)/0.0037;

        let a_star = e;
        let b_star = f/0.94;

        CIELAB{
            l_star,
            a_star,
            b_star
        }

    }
}

impl From<SRGB> for H99 {
    fn from(srgb: SRGB) -> Self {
        H99::from(CIELAB::from(srgb))
    }
}

impl From<H99> for SRGB{
    fn from(value: H99) -> Self {
        SRGB::from(CIELAB::from(value))
    }
}




#[cfg(test)]
mod tests {
    
    use assert_float_eq::assert_f64_near;

    use crate::cie::{CIELAB, SRGB};
    use super::H99;

    #[test]
    fn hyper_roundtrips() {
        let trgb = SRGB{r:0.3,g:0.01,b:0.8};
        let tlab : CIELAB = trgb.into();

        let th99 : H99 = tlab.into();
        // assert_f64_near!(th99.chroma.norm2(),1.0,64);

        let back : CIELAB = th99.into();

        

        assert_f64_near!(tlab.a_star,back.a_star);
        assert_f64_near!(tlab.l_star,back.l_star,64);

    }
}

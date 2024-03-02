#[macro_use]
extern crate assert_float_eq;

use nalgebra::Normed;

pub mod cie;
pub mod hyperbolic;

use num_complex::Complex;
use cie::{SRGB,CIELAB,CIEXYZ};
use hyperbolic::HPoint;



#[derive(Clone, Copy)]
pub struct H99{
    pub luma : f64,
    pub chroma : hyperbolic::HPoint
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

        let t = geodesic_radius.cosh();
        let r = geodesic_radius.sinh();

        let x = r*h99c.cos();
        let y = r*h99c.sin();

        H99{luma:l99c,chroma:HPoint::new(t,x,y)}
    }
}

impl From<H99> for CIELAB {
    fn from(h99: H99) -> Self {
        let l99c = h99.luma;
        let (t,x,y) = h99.chroma.txy();
        let geodesic_radius = t.acosh();
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


impl H99 {
    fn to_srgb_in_gamut(&self) -> SRGB{
        const RANGE : f64 = 0.65;
        const DELTA : f64 = 2.0;
        let poinchroma = self.chroma.poinc();
        if poinchroma.norm_squared() > RANGE*RANGE {return SRGB::BLACK;}

        let mut luma = self.luma;

        for _ in 0..30{
            if !(0.0..100.0).contains(&luma){break;}

            let rgb = SRGB::from(H99{luma,chroma:self.chroma});
            if rgb.in_gamut(){return rgb}

            if rgb.sub_gamut(){
                luma += DELTA;
                continue;
            }

            if rgb.super_gamut(){
                luma -= DELTA;
            }


        }

        SRGB::BLACK
    }

    fn to_srgb_in_gamut_polar(&self,pow:f64) -> SRGB{
        let poles_h99 : Vec<H99> = SRGB::GAMUT_POLES.iter().map(|&p|p.into()).collect();

        let inv_distances : Vec<f64> = poles_h99.iter().map(|&p|
            (p.chroma.distance(&self.chroma)).powf(-pow)
        ).collect();

        let tot_inv_dist : f64 = inv_distances.iter().sum();

        let luma : f64 = inv_distances.iter().zip(poles_h99).map(|(d,p)|
            (p.luma)*d/tot_inv_dist 
        ).sum();

        SRGB::from(H99{luma,chroma:self.chroma})
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
        assert_f64_near!(th99.chroma.norm2(),1.0,64);

        let back : CIELAB = th99.into();

        

        assert_f64_near!(tlab.a_star,back.a_star);
        assert_f64_near!(tlab.l_star,back.l_star,256);

    }
}

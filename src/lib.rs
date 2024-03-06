#[macro_use]
extern crate assert_float_eq;

pub mod cie;
pub mod hyperbolic;

use std::f64::consts::PI;

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

    pub fn hlerp2(c1:H99,c2:H99,t:f64) -> H99{
        assert!((0.0..=1.0).contains(&t));
        let luma = (1.0-t)*c1.luma + t*c2.luma;
        let chroma = HPoint::hlerp2(c1.chroma,c2.chroma,t);
        H99{luma,chroma}
    }

    pub fn hlerp3(c1:H99,c2:H99,c3:H99,l1:f64,l2:f64)->H99{
        const TOL : f64 = 1e-9;
        let range = -TOL..=1.0+TOL;

        if !range.contains(&l1){
            panic!("Weight {l1} is out of range.");
        };
        if !range.contains(&l2){
            panic!("Weight {l2} is out of range.");
        };
        
        let l3 = 1.0-l1-l2;
        let luma = l1*c1.luma + l2*c2.luma + l3*c3.luma;
        let chroma = HPoint::hlerp3(c1.chroma, c2.chroma, c3.chroma, l1, l2);
        H99{luma,chroma}

    }

    pub fn distance(&self, other : &H99)->f64{
        let lumadist2 = (self.luma-other.luma).powi(2);
        let chromadist2 = (HYPER_R*self.chroma.distance(&other.chroma)).powi(2);

        (lumadist2+chromadist2).sqrt()
    }


    const MIN_NEUTRAL_TEMPERATURE : f64 = -1.076;
    const MAX_NEUTRAL_TEMPERATURE : f64 = 1.624;
    const COOLEST_NEUTRAL_LUMA : f64 = 85.938;
    const WARMEST_NEUTRAL_LUMA : f64 = 65.3611;
    pub fn neutral(temperature : f64) -> H99{
        let lambda = (temperature - Self::MIN_NEUTRAL_TEMPERATURE) / (Self::MAX_NEUTRAL_TEMPERATURE - Self::MIN_NEUTRAL_TEMPERATURE);
        let chroma = HPoint(Complex{re: (temperature*0.5).tanh(),im:0.0});
        let luma = (1.0-lambda)*Self::COOLEST_NEUTRAL_LUMA + lambda*Self::WARMEST_NEUTRAL_LUMA;
        H99{luma,chroma}
    }
}



pub const HYPER_R : f64 = 28.6;

//40°
pub const THERMAL_ANGLE : f64 = 0.6981317007977318;



fn hk_f1(hue:f64)->f64{
    let sine = ((hue - PI/2.)/2.).sin().abs();
    0.116 * sine + 0.085
}


impl From<CIELAB> for H99{
    fn from(lab:CIELAB) -> Self{
        let (l_star, a_star, b_star) = (lab.l_star, lab.a_star, lab.b_star);
        
        // chroma
        let e99c = a_star;
        let f99c = 0.94*b_star;

        let g = (e99c*e99c + f99c*f99c).sqrt();

        let h99c = f99c.atan2(e99c);
        let hue_thermal = h99c - THERMAL_ANGLE;

        let chroma99c = 23.0 * (1.+0.066*g).ln();

        let geodesic_radius = chroma99c / HYPER_R;

        let r = (geodesic_radius/2.0).tanh();


        let x = r*hue_thermal.cos();
        let y = r*hue_thermal.sin();
        let chroma = HPoint(Complex{re:x,im:y});

        // HK adjustment
        let f1 = hk_f1(h99c);
        let f1c = f1*chroma99c;
        let mu = 2.5 * f1c;
        let nu = 0.025 * f1c;

        let l_starstar = l_star + mu - nu*l_star;


        let l99c = 317.65 * (1.+0.0037*l_starstar).ln();





        H99{luma:l99c,chroma}
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

        let l_starstar = ((l99c/317.65).exp() - 1.0)/0.0037;

        // HK adjustment
        let f1 = hk_f1(h99c);
        let f1c = f1*chroma99c;
        let mu = 2.5 * f1c;
        let nu = 0.025 * f1c;

        let l_star = (l_starstar - mu)/(1.-nu);

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


pub fn meshed_triangle(
    v1 : H99, v2 : H99, v3 : H99, n : usize
)-> Vec<((f64,f64), H99 )>{
    let mut weights = Vec::with_capacity((n*n)/2);

    for i in 0..n{
        for j in 0..(n.checked_sub(i).unwrap()){
            let l1 = (i as f64)/(n as f64 - 1.0);
            let l2 = (j as f64)/(n as f64 - 1.0);
            weights.push((l1,l2));
        }
    }

    let center = H99::hlerp3(v1, v2, v3, 1./3., 1./3.).chroma;
    

    weights.iter().map(|(l1,l2)| {
        let c = H99::hlerp3(v1, v2, v3, *l1, *l2);
        (c.chroma.equidistant_azimuthal(&center), c)
    }
    ).collect()

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



        let red = SRGB{r:1.,g:0.,b:0.};

        // let labred = CIELAB::from(red);
        // let redbacklab = SRGB::from(labred);

        // assert_f64_near!(red.r,redbacklab.r);
        // assert_f64_near!(red.g,redbacklab.g);
        // assert_f64_near!(red.b,redbacklab.b);


        let hred = H99::from(red);
        let redback = SRGB::from(hred);

        println!("redback {redback:?}");

        assert!(redback.in_gamut());

        // assert_f64_near!(redback.r,red.r,10000);
        // assert_f64_near!(redback.g,red.g);
        // assert_f64_near!(redback.b,red.b);
    }

    #[test]
    fn color_distance(){
        let c = H99::new(0.5,0.0.into());
        let cup = H99::new(0.5+0.1,0.0.into());
        let cdown = H99::new(0.5-0.1,0.0.into());

        assert_f64_near!(c.distance(&c),0.);
        assert_f64_near!(c.distance(&cup),c.distance(&cdown));

        let hred = H99::from(SRGB::RED);

        assert_f64_near!(hred.distance(&hred),0.0);

    }


    #[test]
    fn lerptest(){
        let red = SRGB{r:1.0,g:0.0,b:0.0};
        let blue = SRGB{r:0.0,g:0.0,b:1.0};

        let hred = H99::from(red);
        let hblue = H99::from(blue);

        let vertred = H99::hlerp2(hred, hblue, 0.);
        assert_f64_near!(vertred.chroma.0.re,hred.chroma.0.re);

        let hcyan = H99::from(SRGB{r:0.0,g:1.0,b:1.0});

        let hl2 = H99::hlerp2(hred, hcyan, 0.5);
        let hl3 = H99::hlerp3(hblue, hred, hcyan, 0.0, 0.5);

        assert_f64_near!(hl2.distance(&hl3),0.0);

        let hl3_bis = H99::hlerp3(hred, hcyan, hblue, 0.5, 0.5);
        assert_f64_near!(hl2.distance(&hl3_bis),0.0);
    }

    #[test]
    fn more_lerptest(){
        let hred = H99::from(SRGB::RED);
        let hcyan = H99::from(SRGB::CYAN);
        let hyellow = H99::from(SRGB::YELLOW);

        let blend = H99::hlerp3(hyellow, hred, hcyan, 0.333, 0.333);

        assert_f64_near!(blend.distance(&blend),0.0);
    }
}

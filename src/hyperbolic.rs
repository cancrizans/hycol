use nalgebra::{Vector3,Matrix3,vector,matrix};
use std::ops::Mul;
use num_complex::Complex;

#[derive(Copy,Clone)]
pub struct HPoint(pub Complex<f64>);
// #[derive(Copy,Clone)]
// pub struct HTransform(Matrix3<f64>);




impl HPoint {
    // pub fn new(t:f64,x:f64,y:f64)->HPoint{
    //     let v = vector![t,x,y];
    //     HPoint(v)
    // }

    pub const ORIGIN : HPoint = HPoint(Complex{re:0.,im:0.});

    // fn dot(&self,other:&HPoint) -> f64{
    //     let v= self.0;
    //     let w = other.0;
    //     v[0]*w[0] - v[1]*w[1] - v[2]*w[2]
    // }

    // pub fn norm2(&self) -> f64{
    //     self.dot(self)
    // }

    // pub fn quick_normalize(self) -> HPoint{
    //     let xi = self.norm2() - 1.0;
    //     let factor = 1.0 - 0.5 * xi;
    //     HPoint(factor*self.0)
    // }

    pub fn distance(&self, other : &HPoint) -> f64{
        let (u,v) = (self.0,other.0);
        let delta : f64 = 2.0* (u-v).norm_sqr() / ( (1.-u.norm_sqr())*(1.-v.norm_sqr()));

        (1.0+delta).acosh()
    }


    // #[inline]
    // pub fn txy(&self) -> (f64,f64,f64){
    //     (self.0[0],self.0[1],self.0[2])
    // }

    #[inline]
    pub fn poinc(&self) -> Complex<f64>{
        self.0
    }

    // #[inline]
    // pub fn from_poinc(y:Complex<f64>)->Self{
    //     let y2 = y.norm_sqr();
    //     let fac = 1.0/(1.-y2);
    //     Self::new(fac*(1.+y2), fac*y.re, fac*y.im)
    // }


}

// impl Mul<HPoint> for HTransform {
//     type Output = HPoint;
//     fn mul(self, rhs: HPoint) -> Self::Output {
//         HPoint(self.0 * rhs.0)
//     }
// }

// impl Mul<HTransform> for HTransform{
//     type Output = HTransform;
//     fn mul(self, rhs: HTransform) -> Self::Output {
//         HTransform(self.0 * rhs.0)
//     }
// }

// impl HTransform{
//     pub fn inverse(self) -> HTransform{
//         HTransform(self.0.try_inverse().unwrap())
//     }

//     pub fn rotation(angle:f64)-> HTransform{
//         let (s,c) = angle.sin_cos();
//         HTransform(matrix![
//             1., 0., 0.;
//             0., c, -s;
//             0., s,  c
//         ])
//     }

//     pub fn translation_x(displacement:f64) -> HTransform{
//         let (s,c) = (displacement.sinh(), displacement.cosh());
//         HTransform(matrix![
//             c, s, 0.;
//             s, c, 0.;
//             0.,0.,1.
//         ])
//     }


// }
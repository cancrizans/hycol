use num_complex::Complex;

#[derive(Copy,Clone,Debug)]
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

    pub fn equidistant_azimuthal(self,center:&HPoint)->(f64,f64){
        let rel = HPoint(-center.0).translate(self);
        let logrel = rel.hlog();

        (logrel.re,logrel.im)
    }


    fn hlog(self) -> Complex<f64>{
        let norm = self.0.norm();
        if norm < 1e-10{
            2.*self.0
        }
        else{
            if norm >= 1.0{
                panic!("Cannot compute hlog of {:?}",self);
            }
            2.*(norm.atanh()) / norm * self.0
        }
    }

    
    fn hexp(l : Complex<f64>) -> Self{
        let norm = l.norm();
        if norm < 1e-10{
            HPoint(l/2.)
        }
        else{
            HPoint((norm/2.).tanh()/norm * l)
        }
    }

    fn translate(&self, other : HPoint) -> HPoint{
        let q = self.0;
        let p = other.0;
        HPoint((p + q)/(q.conj()*p + 1.))
    }

    pub fn hlerp2(
        v1:HPoint,v2:HPoint,t:f64
    )-> HPoint{
        assert!((0.0..=1.0).contains(&t));

        Self::hlerp(vec![(1.0-t,v1),(t,v2)])
    }

    pub fn hlerp3(
        v1:HPoint,v2:HPoint,v3:HPoint,
        l1:f64,l2:f64
    )->HPoint{
        
        // let range = 0.0..=1.0;
        // assert!(range.contains(&l1));
        // assert!(range.contains(&l2));

        Self::hlerp(vec![
            (l1,v1),(l2,v2),(1.0-l1-l2,v3)
        ])
    }

    #[inline]
    fn hlerp(vertices : Vec<(f64,HPoint)>) -> HPoint{

        if vertices.iter().any(|(l,_)|f64::is_nan(*l)) {
            panic!("NaN weights");
        }

        println!("Starting lerp.");
        println!("Vertices and weights {:?}",vertices);

        const TOL : f64 = 1e-13;

        let tot_weight : f64 = vertices.iter().map(|(l,_)|*l).sum();
        println!("Total weight {}",tot_weight);
        let arith_sum : Complex<f64> = vertices.iter().map(|(l,p)|*l*p.0).sum();
        let mut arith_mean : Complex<f64> = arith_sum / tot_weight;
        println!("Arithmetic mean {}",arith_mean);

        if arith_mean.norm_sqr() > 1.0-1e-8{
            arith_mean *= 0.9 / arith_mean.norm();
        }

        assert!(arith_mean.norm_sqr() < 1.0);

        let mut w : HPoint = HPoint(arith_mean);
        let mut itercount = 0;
        loop{
            
            let mw = HPoint(-w.0);

            let u : HPoint = HPoint::hexp(
                vertices.iter().map(|(l,p)| 
                *l * (mw.translate(*p)).hlog()
            ).sum()
            );

            w = w.translate(u);

            if u.0.norm_sqr() < TOL*TOL{
                break;
            }
            // println!("{}__{}",w.0,u.0);

            if itercount > 256{
                panic!("Maximum iterations reached");
            }
            itercount += 1;
        }
        w
    }



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

#[cfg(test)]
mod tests {
    
    use assert_float_eq::assert_f64_near;

    
    use super::*;

    #[test]
    fn hlerp_test(){
        let v1 = HPoint(0.0.into());
        let v2 = HPoint(0.5.into());
        let v3 = HPoint(Complex{re:0.0,im:0.5});

        let test1 = HPoint::hlerp3(v1, v2, v3, 0.5, 0.5);

        assert_f64_near!(HPoint::ORIGIN.distance(&test1), 0.5*v1.distance(&v2));

        let test2 = HPoint::hlerp3(v1, v2, v3, 0.5, 0.0);

        assert_f64_near!(HPoint::ORIGIN.distance(&test2), 0.5*v1.distance(&v3));


        let test3_a = HPoint::hlerp3(v1, v2, v3, 0.0, 0.3);
        let test3_b = HPoint::hlerp2(v2, v3, 0.7);

        assert_f64_near!(test3_a.0.re,test3_b.0.re,4096);

        assert_f64_near!(0.3*v2.distance(&test3_b), 0.7 * v3.distance(&test3_b),512);


        let test4 = HPoint::hlerp2(v2,v3,0.0);

        assert_f64_near!(test4.distance(&v2),0.0);
    }

}
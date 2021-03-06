mod math3d;
extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use rand::{thread_rng,Rng};
use std::time::{Duration, Instant};


pub fn CreateRandomScene(world:& mut math3d::World){
    let n=500;
    
}

fn main() {
    type v3f=math3d::Vec3D<f32>;
    type v3i=math3d::Vec3D<i32>;
    type Ray=math3d::Ray;
    type Sphere=math3d::Sphere;
    type World= math3d::World;
    type Camera=math3d::Camera;
    type CameraOld=math3d::CameraOld;
    type Lambertian=math3d::Lambertian;
    type Metal=math3d::Metal;
    type Dialectric=math3d::Dielectric;
    type HitRecord=math3d::HitRecord;

    println!("Start Ray Tracing !!!");
    
    

    let mut buffer=File::create("image.ppm").unwrap();

    let nx:i32=1200;
    let ny:i32=800;
    let ns:i32=10;

    writeln!(buffer,"P3");
    writeln!(buffer,"{} {}",nx,ny);
    writeln!(buffer,"255");
    
    let lower_lef_corner:v3f=v3f{x:-2.0,y:-1.0,z:-1.0};
    let horizontal:v3f=v3f{x:4.0,y:0.0,z:0.0};
    let vertical:v3f=v3f{x:0.0,y:2.0,z:0.0};
    let origin:v3f=v3f{x:0.0,y:0.0,z:0.0};



    let s1=Box::new(Sphere{
                    center:v3f{x:0.0,y:0.0,z:-1.0},
                    radius:0.5,
                    material:Box::new(Lambertian{
                        albedo:v3f{x:0.8,y:0.3,z:0.3}
                    })
            });

    let s2=Box::new(Sphere{
                    center:v3f{x:0.0,y:-100.5,z:-1.0},
                    radius:100.0,
                    material:Box::new(Lambertian{
                        albedo:v3f{x:0.8,y:0.8,z:0.0}
                    })
            });

    let s3=Box::new(Sphere{
                    center:v3f{x:1.0,y:0.0,z:-1.0},
                    radius:0.5,
                    material:Box::new(Metal{
                        albedo:v3f{x:0.8,y:0.6,z:0.2}
                        ,fuzz:0.3
                    })
            });


   let s4=Box::new(Sphere{
                    center:v3f{x:-1.0,y:0.0,z:-1.0},
                    radius:0.5,
                    material:Box::new(Dialectric{ref_idx:1.5})
            });

    let s5=Box::new(Sphere{
                    center:v3f{x:-2.0,y:0.0,z:-1.0},
                    radius:-0.45,
                    material:Box::new(Dialectric{ref_idx:1.5})
            });

    let mut world=World::new();
    world.objects.push(s1);
    world.objects.push(s2);
    world.objects.push(s3);
    world.objects.push(s4);
    world.objects.push(s5);

    let aspect:f32= (nx as f32)/(ny as f32);
    let look_from=v3f{x:13.0,y:2.0,z:3.0};
    let look_at=v3f{x:0.0,y:0.0,z:0.0};
    let vup=v3f{x:0.0,y:1.0,z:0.0};
    let vfov=20.0;

    let aperture=0.1;
    let v00=math3d::VecSub3D(&look_from,&look_at);
    let dist_to_focus=10.0;

    let camera=Camera::new(
        look_from,
        look_at,
        vup,
        vfov,aspect
        ,aperture
        ,dist_to_focus
    );
    
    //let camera=CameraOld::new();

    let now = Instant::now();

    let mut j:i32=ny-1;
    
        while j>=0 {

            for i in 0..nx {
                let mut r1:f32=0.0;
                let mut r2:f32=0.0;
                let mut clr=v3f{x:0.0,y:0.0,z:0.0};

                for s in 0..ns{
                    r1=thread_rng().gen_range(0.0,1.0);
                    r2=thread_rng().gen_range(0.0,1.0);

                    let u:f32=((i as f32)+r1)/(nx as f32);
                    let v:f32=((j as f32)+r2)/(ny as f32);
                
                    let r=camera.GetRay(u,v);
                    let tmp_clr=math3d::GetColor(r,&world,0);
                    clr=math3d::VecAdd3D(&clr,&tmp_clr);
                }

                clr=math3d::VecDiv3D(&clr,ns as f32);
                
                clr=v3f{
                    x:clr.x.sqrt(),
                    y:clr.y.sqrt(),                
                    z:clr.z.sqrt(),                
                };

                clr=math3d::VecMul3D(&clr,255.99);

                let iclr:math3d::Vec3D<i32>=clr.ToI32();
                writeln!(buffer,"{} {} {}",iclr.x,iclr.y,iclr.z);
            }
            j-=1;
        }


    println!("secs:{}", now.elapsed().as_secs());

}

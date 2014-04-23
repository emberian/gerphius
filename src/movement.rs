//inherited mutability: player is mutable if let is mutable

//todo: Test in ticks
//      Implement player acceleration based on ticks between inputs.
//      Potentially ticks holding a button and ticks not
//      Rotation accel/vel
use std;
use game;

/*fn main(){
    let mut p:Player = Player{accel:0., velocity:0., position:0., accel_mod:0};
    loop{
    let key = get_input();
    accel(key, &mut p);
    }
}

struct Player{

    accel:f32,
    velocity:f32,
    position:f32,
    accel_mod:int

}
*/

fn get_input() -> char{
    let mut input = std::io::stdin();
    let key = input.read_line().unwrap();
    return key.char_at(0);
}

pub fn accel(cond:bool, p:&mut game::Player){ //mut player:&mut player would allow to play w/ pointer
    if p.velocity >= -0.15 && p.velocity <= 0.15{
        p.velocity += p.accel;
    }
    if p.velocity < -0.05{
        p.velocity = -0.05;
    }
    if p.velocity >= 0.05{
        p.velocity = 0.05
    }
    p.position += p.velocity;
    let (acc, amod) = accel_compute(true, p.accel, p.accel_mod);
    p.accel = acc;
    p.accel_mod = amod;
    println!("accel: {} accel_mod: {}, velocity: {}, position: {} ", p.accel, p.accel_mod, p.velocity, p.position);
}


pub fn accel_compute (cond:bool, mut accel:f32, mut accel_mod:int) -> (f32, int) {//this will use accel/accel_mod to compute the rate of increase of acceleration.

    if cond == true{//player wishes to accelerate forward
        if accel_mod >=-85 && accel_mod < -75{
            accel_mod += 25;
        }
        else if accel_mod >=-75 && accel_mod < -60{
            accel_mod += 22;
        }
        else if accel_mod >=-60 && accel_mod < -41{
            accel_mod += 19;
        }
        else if accel_mod >=-40 && accel_mod < -15{
            accel_mod += 17;
        }
        else if accel_mod >=-15 && accel_mod < 0{
            accel_mod = 0;
        }
        else if accel_mod == 0{
            accel_mod = 15;
        }
        else if accel_mod >= 0 && accel_mod < 15{
            accel_mod += 12;
        }
        else if accel_mod >= 15 && accel_mod <= 40{
            accel_mod +=10;
        }
        else if accel_mod > 40 && accel_mod <= 60{
            accel_mod +=8;
        }
        else if accel_mod >60 && accel_mod <= 75{
            accel_mod += 5;
        }
        else if accel_mod >75 && accel_mod <=85{
            accel_mod += 2;
        }
    }
    else if cond == false{//player wishes to accelerate backward
        if accel_mod >=-85 && accel_mod > -75{
            accel_mod += -2;
        }
        else if accel_mod >=-75 && accel_mod > -60{
            accel_mod += -5;
        }
        else if accel_mod >=-60 && accel_mod > -41{
            accel_mod += -8;
        }
        else if accel_mod >=-40 && accel_mod > -15{
            accel_mod += -10;
        }
        else if accel_mod >=-15 && accel_mod < 0{
            accel_mod += -12;
        }
        else if accel_mod == 0{
            accel_mod = -15;
        }
        else if accel_mod > 0 && accel_mod <= 15{
            accel_mod = 0;
        }
        else if accel_mod >= 15 && accel_mod <= 40{
            accel_mod +=-17;
        }
        else if accel_mod > 40 && accel_mod <= 60{
            accel_mod +=-19;
        }
        else if accel_mod >60 && accel_mod <= 75{
            accel_mod += -22;
        }
        else if accel_mod >75 && accel_mod <=85{
            accel_mod += -25;
        }
    }
    if accel <= 0.05 && accel >= -0.05{
        accel = accel + (0.0000003 * (accel_mod as f32));
    }
    (accel, accel_mod) //returns accel and accel mod
}

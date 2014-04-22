//inherited mutability: player is mutable if let is mutable


//todo: Implement negative scaling on accel_compute
//      Implement velocity_compute based on the above

fn main(){
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

fn get_input() -> char{
    let mut input = std::io::stdin();
    let key = input.read_line().unwrap();
    return key.char_at(0);
}

fn accel(key:char, p:&mut Player){ //mut player:&mut player would allow to play w/ pointer
    std::io::println(key.to_str());
    if p.velocity >-0.05 && p.velocity <0.05{
        p.velocity += p.accel
    }
    if p.velocity < -0.05{
        p.velocity = -0.05;
    }
    if p.velocity >= 0.05{
        p.velocity = 0.05
    }
    p.position += p.velocity;
    if key == 'w'{
        let (acc, amod) = accel_compute(true, p.accel, p.accel_mod);
        *p = Player {accel:acc, accel_mod: amod, ..*p};//create new player, in place of old one. Replace accel_mod and accel, and then use all old values.
    }
    if key == 's'{
        let (acc, amod) = accel_compute(false, p.accel, p.accel_mod);
        *p = Player {accel:acc, accel_mod: amod, ..*p};
    }
    println!("accel: {} accel_mod: {}, velocity: {}, position: {} ", p.accel, p.accel_mod, p.velocity, p.position);
}


fn accel_compute (cond:bool, mut accel:f32, mut accel_mod:int) -> (f32, int) {

    if cond == true{//player wishes to accelerate forward
        if accel_mod <=-85 && accel_mod > -75{
            accel_mod += 25;
        }
        else if accel_mod <=-75 && accel_mod > -60{
            accel_mod += 22;
        }
        else if accel_mod <=-60 && accel_mod > -41{
            accel_mod += 19;
        }
        else if accel_mod <=-40 && accel_mod > -15{
            accel_mod += 17;
        }
        else if accel_mod <=-15 && accel_mod < 0{
            accel_mod = 0;
        }
        else if accel_mod == 0{
            accel_mod = 15;
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
        if accel_mod <=-85 && accel_mod > -75{
            accel_mod += 25;
        }
        else if accel_mod <=-75 && accel_mod > -60{
            accel_mod += 22;
        }
        else if accel_mod <=-60 && accel_mod > -41{
            accel_mod += 19;
        }
        else if accel_mod <=-40 && accel_mod > -15{
            accel_mod += 17;
        }
        else if accel_mod <=-15 && accel_mod < 0{
            accel_mod = 0;
        }
        else if accel_mod == 0{
            accel_mod = 15;
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
    if accel <= 0.05 && accel >= -0.05{
        accel = accel + (0.0000003 * (accel_mod as f32));
    }
    (accel, accel_mod) //returns accel and accel mod
}

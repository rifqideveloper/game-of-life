extern crate rand;
use rand::Rng;
fn main(){
    // alive = true
    // dead = false
    //5,11,17,23,29,35
    let mut tile = [false,false,false,false,false,false,
                    false,false,false,false,false,false,
                    false,false,false,false,false,false,
                    false,false,false,false,false,false,
                    false,false,false,false,false,false,
                    false,false,false,false,false,false];
    let mut genlist : Vec<usize> = Vec::with_capacity(5);
    let mut sample : Vec<usize> = Vec::with_capacity(10);
    let empty = " .";
    let cell = " o";
    let mut gen = 0;
    let mut random:usize = 0;
    loop {
        for _x in 0..2{
            nextgen( &mut tile , &mut gen ,&mut genlist ,&mut random,&mut sample);
            display( &tile, &empty , &cell , &mut gen);
        }
        //dead cell
        tile[genlist[0]] = false;
        genlist.remove(0);
    }
    

}

fn display( z:&[bool;36], x:&str , y:&str , gen:&mut i32)
{
    for i in 0..36
    {
        if i % 6 == 0 {println!()}
        if !z[i] {print!("{}",x)}
        else {print!("{}",y)} 
    }
    println!("\n Generation: {}",gen)
}

fn nextgen(z:&mut[bool;36],gen:&mut i32,list:&mut Vec<usize>
            ,random:&mut usize,sample:&mut Vec<usize>)
{
    match gen {
        0 =>{
            *random = rand::thread_rng().gen_range(0,36);
            z[*random] = true;
            list.push(*random);
        }
        _ => {
            rand_num_gen(random,list,sample,z);     
            z[*random] = true;
            list.push(*random);
        }
    }
    *gen += 1;
    
}
fn rand_num_gen(random:&mut usize,list:&mut Vec<usize>,sample:&mut Vec<usize>,z:&mut[bool;36])
{
    //genereted location
    rand_location(sample,list,z);
    //choose random location
    *random = sample[rand::thread_rng().gen_range(0,sample.len()-1)];
    //remove all element
    for _i in 0..sample.len()-1{
        sample.remove(0);
    }
}
fn rand_location(sample:&mut Vec<usize>,list:&mut Vec<usize>,z:&mut[bool;36])
{
    if list.len() != 1 {
        for x in 0..list.len()-1{
            rand_sample(list,sample,z,x as usize);
        }
    } else {
        rand_sample(list,sample,z,0 as usize);
    }
}
fn rand_sample(list:&mut Vec<usize>,sample:&mut Vec<usize>,z:&mut[bool;36],x:usize)
{
    if list[x] == 0{
        for i in vec![1,6,7]{
            if !z[list[x]+i] {sample.push(list[x]+i)}
        }
    }
    else if list[x] == 30 {
        if !z[list[x]+1] {sample.push(list[x]+1)}
        for i in vec![5,6]{
            if !z[list[x]-i] {sample.push(list[x]-i)}
        }
    }
    else if list[x] == 35 {
        for i in vec![1,5,6]{
            if !z[list[x]-i] {sample.push(list[x]-i)}
        }
    }
    else if list[x] == 5 {
        if !z[list[x]-1] {sample.push(list[x]-1)}
        for i in vec![5,6]{
            if !z[list[x]+i] {sample.push(list[x]+i)}
        }
    }
    else if list[x] % 6 == 0 {
        if !z[list[x]+1] {sample.push(list[x]+1)}
        if !z[list[x]+6] {sample.push(list[x]+6)}
        if !z[list[x]-6] {sample.push(list[x]-6)}
    }
    else if list[x] == 1||list[x] ==2||list[x] ==3||list[x] ==4 {
        if !z[list[x]-1] {sample.push(list[x]-1)}
        if !z[list[x]+1] {sample.push(list[x]+1)}
        for i in vec![5,6,7]{
            if !z[list[x]+i] {sample.push(list[x]+i)}
        }
    }
    else if list[x] == 31||list[x] ==32||list[x] ==33||list[x] ==34{
        if !z[list[x]-1] {sample.push(list[x]-1)}
        if !z[list[x]+1] {sample.push(list[x]+1)}
        for i in vec![5,6,7]{
            if !z[list[x]-i] {sample.push(list[x]-i)}
        }
    }
    else if list[x] == 11||list[x] ==17||list[x] ==23||list[x] ==29||list[x] ==35{
        if !z[list[x]-1] {sample.push(list[x]-1)}
        if !z[list[x]+6] {sample.push(list[x]+6)}
        if !z[list[x]-6] {sample.push(list[x]-6)}
    }
    else{
        for i in vec![1,5,6,7]{
            if !z[list[x]+i] {sample.push(list[x]+i)}
            if !z[list[x]-i] {sample.push(list[x]-i)}
        }
    }
}
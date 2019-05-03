// see chapter2 binding.rs

fn main(){
    let warning_energy = 5; // value 5 is bound to variable energy
    let _nowarning_energy = 6; // suppress that warning
    let _energy_with_type = 5u16;
    let copy_energy = warning_energy;
    println!("Your energy is {}", copy_energy);
}
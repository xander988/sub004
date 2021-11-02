pub trait traffic {
    fn retime(&self) -> u8;
}

enum TrafficLigght {
    Red,
    Green,
    Yellow,
}

impl  traffic for TrafficLigght {
    fn retime(&self) -> u8{
      match self{
          Self::Red => 20,
          Self::Green =>40,
          Self::Yellow =>60,
      }
    }
}

fn main() {
  
    let light = TrafficLigght::Red;
    println!("light is {}", light.retime())
}

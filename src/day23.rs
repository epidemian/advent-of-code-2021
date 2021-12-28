pub fn run() {
  // #############
  // #...........#
  // ###D#B#D#B###
  //   #C#A#A#C#
  //   #########
  let _start = State {
    hallway: [None; 11],
    rooms: [
      [Some(Amphipod::D), Some(Amphipod::C)],
      [Some(Amphipod::B), Some(Amphipod::A)],
      [Some(Amphipod::D), Some(Amphipod::A)],
      [Some(Amphipod::B), Some(Amphipod::C)],
    ],
  };

  let _goal = State {
    hallway: [None; 11],
    rooms: [
      [Some(Amphipod::A); 2],
      [Some(Amphipod::B); 2],
      [Some(Amphipod::C); 2],
      [Some(Amphipod::D); 2],
    ],
  };

  // TODO: find shortest path between start and goal using valid moves.
}

#[derive(Clone)]
struct State<const ROOM_SIZE: usize> {
  hallway: [Option<Amphipod>; 11],
  rooms: [[Option<Amphipod>; ROOM_SIZE]; 4],
}

#[derive(Clone, Copy)]
enum Amphipod {
  A,
  B,
  C,
  D,
}

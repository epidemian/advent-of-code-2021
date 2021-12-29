use crate::dijkstra;

pub fn run() {
  // #############
  // #...........#
  // ###D#B#D#B###
  //   #C#A#A#C#
  //   #########
  let part1_start = State {
    hallway: [None; 11],
    rooms: [
      [Some(D), Some(C)],
      [Some(B), Some(A)],
      [Some(D), Some(A)],
      [Some(B), Some(C)],
    ],
  };

  // #############
  // #...........#
  // ###D#B#D#B###
  //   #D#C#B#A#
  //   #D#B#A#C#
  //   #C#A#A#C#
  //   #########
  let part2_start = State {
    hallway: [None; 11],
    rooms: [
      [Some(D), Some(D), Some(D), Some(C)],
      [Some(B), Some(C), Some(B), Some(A)],
      [Some(D), Some(B), Some(A), Some(A)],
      [Some(B), Some(A), Some(C), Some(C)],
    ],
  };

  println!("{}", part1_start.min_organization_energy());
  println!("{}", part2_start.min_organization_energy());
}

type Amphipod = u8;
const A: Amphipod = 0;
const B: Amphipod = 1;
const C: Amphipod = 2;
const D: Amphipod = 3;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<const ROOM_SIZE: usize> {
  hallway: [Option<Amphipod>; 11],
  rooms: [[Option<Amphipod>; ROOM_SIZE]; 4],
}

impl<const S: usize> State<S> {
  fn min_organization_energy(&self) -> usize {
    let goal = State {
      hallway: [None; 11],
      rooms: [[Some(A); S], [Some(B); S], [Some(C); S], [Some(D); S]],
    };

    dijkstra::shortest_path(self, &goal, Self::next_moves).unwrap()
  }

  fn next_moves(&self) -> Vec<(State<S>, usize)> {
    [self.room_to_hallway_moves(), self.hallway_to_room_moves()].concat()
  }

  fn room_to_hallway_moves(&self) -> Vec<(State<S>, usize)> {
    let mut moves = vec![];
    for room_index in 0..self.rooms.len() {
      if let Some((room_slot, amphipod)) = self.get_amphipod_to_leave_room(room_index) {
        let hallway_from = room_index_to_hallway_index(room_index);
        for hallway_to in 0..11 {
          if self.can_move_to_hallway(hallway_from, hallway_to) {
            let mut next_state = self.clone();
            next_state.hallway[hallway_to] = Some(amphipod);
            next_state.rooms[room_index][room_slot] = None;

            let dx = usize_diff(hallway_from, hallway_to);
            let dy = room_slot + 1;
            let energy = (dx + dy) * amphipod_energy(amphipod);

            moves.push((next_state, energy))
          }
        }
      }
    }
    moves
  }

  fn hallway_to_room_moves(&self) -> Vec<(State<S>, usize)> {
    let mut moves = vec![];
    for (hallway_from, &a) in self.hallway.iter().enumerate() {
      if let Some(amphipod) = a {
        let room_index = amphipod as usize;
        if self.can_move_to_room(room_index, hallway_from) {
          let room_slot = get_empty_room_slot(&self.rooms[room_index]);
          let hallway_to = room_index_to_hallway_index(room_index);

          let mut next_state = self.clone();
          next_state.hallway[hallway_from] = None;
          next_state.rooms[room_index][room_slot] = Some(amphipod);

          let dx = usize_diff(hallway_from, hallway_to);
          let dy = room_slot + 1;
          let energy = (dx + dy) * amphipod_energy(amphipod);

          moves.push((next_state, energy))
        }
      }
    }
    moves
  }

  fn get_amphipod_to_leave_room(&self, room_index: usize) -> Option<(usize, Amphipod)> {
    let room = self.rooms[room_index];
    let all_amphipods_in_their_destination = room
      .iter()
      .filter_map(|a| *a)
      .all(|a| a as usize == room_index);
    if all_amphipods_in_their_destination {
      return None;
    }
    room
      .iter()
      .enumerate()
      .filter_map(|(i, a)| a.map(|a| (i, a)))
      .next()
  }

  fn can_move_to_hallway(&self, from: usize, to: usize) -> bool {
    let is_in_from_of_room = [2, 4, 6, 8].contains(&to);
    if is_in_from_of_room {
      return false;
    }
    self.amphipods_in_hallway(from, to) == 0
  }

  fn can_move_to_room(&self, room_index: usize, hallway_from: usize) -> bool {
    let room_has_amphipod_of_other_type = self.rooms[room_index]
      .iter()
      .filter_map(|a| *a)
      .any(|a| a as usize != room_index);
    if room_has_amphipod_of_other_type {
      return false;
    }
    let hallway_to = room_index_to_hallway_index(room_index);
    self.amphipods_in_hallway(hallway_from, hallway_to) == 1
  }

  fn amphipods_in_hallway(&self, from: usize, to: usize) -> usize {
    (from.min(to)..=from.max(to))
      .filter(|i| self.hallway[*i].is_some())
      .count()
  }
}

fn get_empty_room_slot(room: &[Option<Amphipod>]) -> usize {
  for (i, a) in room.iter().enumerate().rev() {
    if a.is_none() {
      return i;
    }
  }
  unreachable!();
}

fn room_index_to_hallway_index(room_index: usize) -> usize {
  2 + room_index * 2
}

fn amphipod_energy(a: Amphipod) -> usize {
  10_usize.pow(a as u32)
}

fn usize_diff(a: usize, b: usize) -> usize {
  a.max(b) - a.min(b)
}

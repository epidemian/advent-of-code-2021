pub fn run() {
  let input = include_str!("inputs/day16").trim();
  let packet = BitStream::from_hex(input).read_packet();
  println!("{}", packet.sum_versions());
  println!("{}", packet.value());
}

struct BitStream {
  data: Vec<u8>,
  index: usize,
}

impl BitStream {
  fn from_hex(hex_str: &str) -> BitStream {
    let half_bytes: Vec<_> = hex_str
      .chars()
      .map(|ch| ch.to_digit(16).unwrap() as u8)
      .collect();
    let data = (0..half_bytes.len())
      .step_by(2)
      .map(|i| half_bytes[i] << 4 | half_bytes[i + 1])
      .collect();
    BitStream { data, index: 0 }
  }

  fn read_bits(&mut self, n: usize) -> u32 {
    let mut res = 0;
    for _ in 0..n {
      res = res << 1 | self.read_bit();
    }
    res
  }

  fn read_bit(&mut self) -> u32 {
    let byte = self.data[self.index / 8];
    let bit = byte >> (7 - self.index % 8) & 1;
    self.index += 1;
    bit as u32
  }

  fn read_packet(&mut self) -> Packet {
    let version = self.read_bits(3);
    let type_id = self.read_bits(3);
    let content = match type_id {
      4 => self.read_number(),
      _ => self.read_operator(),
    };
    Packet {
      version,
      type_id,
      content,
    }
  }

  fn read_number(&mut self) -> PacketContent {
    let mut result = 0;
    loop {
      let group = self.read_bits(5) as u64;
      result = result << 4 | (group & 0b1111);
      if group >> 4 == 0 {
        break;
      }
    }
    PacketContent::Number(result)
  }

  fn read_operator(&mut self) -> PacketContent {
    let length_type_id = self.read_bit();
    let mut sub_packets = vec![];
    match length_type_id {
      0 => {
        let bit_length = self.read_bits(15) as usize;
        let initial_index = self.index;
        while self.index < initial_index + bit_length {
          sub_packets.push(self.read_packet());
        }
      }
      1 => {
        let packet_length = self.read_bits(11);
        for _ in 0..packet_length {
          sub_packets.push(self.read_packet());
        }
      }
      _ => unreachable!(),
    }
    PacketContent::Operator(sub_packets)
  }
}

struct Packet {
  version: u32,
  type_id: u32,
  content: PacketContent,
}

enum PacketContent {
  Number(u64),
  Operator(Vec<Packet>),
}

impl Packet {
  fn sum_versions(&self) -> u32 {
    match &self.content {
      PacketContent::Number(_) => self.version,
      PacketContent::Operator(sub_packets) => {
        self.version + sub_packets.iter().map(Packet::sum_versions).sum::<u32>()
      }
    }
  }

  fn value(&self) -> u64 {
    match &self.content {
      PacketContent::Number(n) => *n,
      PacketContent::Operator(sub_packets) => {
        let values: Vec<_> = sub_packets.iter().map(Packet::value).collect();
        match self.type_id {
          0 => values.iter().sum(),
          1 => values.iter().fold(1, |a, b| a * b),
          2 => *values.iter().min().unwrap(),
          3 => *values.iter().max().unwrap(),
          5 => (values[0] > values[1]) as u64,
          6 => (values[0] < values[1]) as u64,
          7 => (values[0] == values[1]) as u64,
          _ => unreachable!(),
        }
      }
    }
  }
}

use utils::*;

pub fn marble_game_winner(num_players: usize, last_marble: usize) -> PuzzleResult {
  let mut buf = CircularList::new();
  buf.insert(0);
  let mut current_marble = 1;
  let mut players = Vec::with_capacity(num_players);
  for _ in 0..num_players {
    players.push(0);
  }
  while current_marble <= last_marble {
    if current_marble % 23 == 0 {
      for _ in 0..7 {
        buf.move_prev();
      }
      let player_idx = current_marble % num_players;
      unsafe {
        let player = players.get_unchecked_mut(player_idx);
        *player += current_marble;
        *player += buf.remove_current().unwrap();
      }
    } else {
      buf.move_next();
      buf.insert(current_marble);
    }
    current_marble += 1;
  }

  Ok(PuzzleSolution::usize(players.into_iter().max().unwrap()))
}

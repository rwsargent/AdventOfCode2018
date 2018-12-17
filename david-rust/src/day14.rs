use utils::*;

pub fn score_string(input: usize) -> PuzzleResult {
  let recipes = get_recipes(input + 10);

  let mut s = String::new();
  for i in input..(input + 10) {
    s.push(('0' as u8 + recipes[i]) as char);
  }
  Ok(PuzzleSolution::String(s))
}

pub fn num_recipes_until(input: &str) -> PuzzleResult {
  let mut recipes = Vec::with_capacity(1000000);
  recipes.push(3);
  recipes.push(7);

  let mut elf1 = 0;
  let mut elf2 = 1;

  let zero = '0' as u8;

  let input: Vec<u8> = input.chars().map(|c| c as u8 - zero).collect();

  println!("input {:?}", input);

  while true {
    let elf1_recipe = recipes[elf1] as usize;
    let elf2_recipe = recipes[elf2] as usize;
    let sum = elf1_recipe + elf2_recipe;
    for c in sum.to_string().chars() {
      recipes.push(c as u8 - zero);
      if recipes.len() >= input.len() && recipes[recipes.len() - input.len()..] == input[..] {
        return Ok(PuzzleSolution::usize(recipes.len() - input.len()));
      }
    }
    elf1 = (elf1 + elf1_recipe + 1) % recipes.len();
    elf2 = (elf2 + elf2_recipe + 1) % recipes.len();
  }
  unreachable!()
}

fn get_recipes(num_recipes: usize) -> Vec<u8> {
  let mut recipes = Vec::with_capacity(num_recipes + 11);
  recipes.push(3);
  recipes.push(7);

  let mut elf1 = 0;
  let mut elf2 = 1;

  let zero = '0' as u8;

  while recipes.len() < num_recipes {
    let elf1_recipe = recipes[elf1] as usize;
    let elf2_recipe = recipes[elf2] as usize;
    let sum = elf1_recipe + elf2_recipe;
    for c in sum.to_string().chars() {
      recipes.push(c as u8 - zero);
    }
    elf1 = (elf1 + elf1_recipe + 1) % recipes.len();
    elf2 = (elf2 + elf2_recipe + 1) % recipes.len();
  }

  recipes
}

use utils::*;

pub fn score_string(input: usize) -> PuzzleResult {
  let recipes = get_recipes(input + 10);

  let mut s = String::new();
  for i in input..(input + 10) {
    s.push(('0' as u8 + *recipes.get(i).unwrap()) as char);
  }
  Ok(PuzzleSolution::String(s))
}

fn get_recipes(num_recipes: usize) -> Vec<u8> {
  let mut recipes = Vec::with_capacity(num_recipes + 11);
  recipes.push(3);
  recipes.push(7);

  let mut elf1 = 0;
  let mut elf2 = 1;

  let zero = '0' as u8;

  while recipes.len() < num_recipes {
    let (elf1_recipe, elf2_recipe) = unsafe {
      (*recipes.get_unchecked(elf1) as usize,
       *recipes.get_unchecked(elf2) as usize)
    };
    let sum = elf1_recipe + elf2_recipe;
    for c in sum.to_string().chars() {
      recipes.push(c as u8 - zero);
    }
    elf1 = (elf1 + elf1_recipe + 1) % recipes.len();
    elf2 = (elf2 + elf2_recipe + 1) % recipes.len();
  }

  recipes
}

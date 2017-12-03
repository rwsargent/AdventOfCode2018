package rwsargent.day01;

import rwsargent.AdventOfCodeDay;

public class Day01 extends AdventOfCodeDay {
	public void solveFirst() {
		String input = read("first.txt").get(0);
		char[] numbers = input.toCharArray();

		int sum = 0;
		for(int idx = 0; idx < input.length(); idx++) {
			char first = numbers[idx];
			char second = numbers[(idx + 1) % input.length()];
			
			if(first == second) {
				sum += (int)(first - '0');
			}
		}
		System.out.println(sum);
	}
	
	@Override
	public void solveSecond() {
		String input = read("first.txt").get(0);
		char[] numbers = input.toCharArray();

		int sum = 0;
		for(int idx = 0; idx < input.length(); idx++) {
			char first = numbers[idx];
			char second = numbers[(idx + (input.length() / 2)) % input.length()];
			
			if(first == second) {
				sum += (int)(first - '0');
			}
		}

		System.out.println(sum);
	}
}

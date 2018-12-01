package rwsargent;

import java.util.List;

public class Day02 extends AdventOfCodeDay{

	@Override
	public void solveFirst() {
		List<String> lines = read("day02_first.txt");
		
		int sum = 0;
		for(String line : lines) {
			String[] numbers = line.split("\t");
			int min = Integer.MAX_VALUE, max=0;
			for(String number : numbers) {
				int num = Integer.parseInt(number);
				//find min
				min = Math.min(min, num);
				// find max
				max = Math.max(max, num);
			}
			
			sum += (max - min);
		}
		System.out.println("First: " + sum);
	}
	
	@Override
	public void solveSecond() {
		List<String> lines = read("day02_first.txt");
		
		int sum = 0;
		for(String line : lines) {
			String[] numbers = line.split("\t");
			line:
			for(int numIdx = 0; numIdx < numbers.length -1; numIdx++) {
				int numerator = Integer.parseInt(numbers[numIdx]);
				for(int demIdx = numIdx + 1; demIdx< numbers.length; demIdx++) {
					int denomonator  = Integer.parseInt(numbers[demIdx]);
					
					int top = Math.max(numerator, denomonator);
					int bot  = Math.min(numerator, denomonator);
					
					if(top % bot == 0) {
						sum += top / bot;
						break line;
					}
				}
			}
		}
		System.out.println("Second: " + sum);
	}
}

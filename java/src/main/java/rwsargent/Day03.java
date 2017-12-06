package rwsargent;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;

public class Day03 extends AdventOfCodeDay{

	@Override
	public void solveFirst() {
		List<String> lines = read("day03.txt");
		int goal = Integer.parseInt(lines.get(0));
		int counter = 1;
		Coordinate cursor = new Coordinate(0,0);
		Bound boundary = new Bound();
		
		List<Spiraler> spiralers = createSpiralers();
		int spiralIdx = 0;
		
		do {
			Spiraler spiraler = spiralers.get(spiralIdx);
			spiraler.coordinateAdvancer.accept(cursor);
			if(outOfBounds(cursor, boundary)) {
				spiraler.boundAdvancer.accept(boundary);
				spiralIdx = (spiralIdx+1) % 4;
			}
			counter++;
//			System.out.println(String.format("%d is at (%d, %d)", counter, cursor.x, cursor.y));
		}while(goal != counter);
		
		System.out.println(Math.abs(cursor.x) + Math.abs(cursor.y));
	}
	
	@Override
	public void solveSecond() {
		List<String> lines = read("day03.txt");
		int goal = Integer.parseInt(lines.get(0));
		int storedValue = 1;
		Coordinate cursor = new Coordinate(0,0);
		Bound boundary = new Bound();
		Map<Coordinate, Integer> squares = new HashMap<>();
		
		List<Spiraler> spiralers = createSpiralers();
		int spiralIdx = 0;
		
		do {
			squares.put(new Coordinate(cursor), storedValue);
			
			Spiraler spiraler = spiralers.get(spiralIdx);
			spiraler.coordinateAdvancer.accept(cursor);
			if(outOfBounds(cursor, boundary)) {
				spiraler.boundAdvancer.accept(boundary);
				spiralIdx = (spiralIdx+1) % 4;
			}
			
			storedValue = calculateValueSquare(squares, cursor);
//			System.out.println(String.format("(%d, %d) stores %d", cursor.x, cursor.y, storedValue));
		}while(goal >= storedValue);
		
		System.out.println(storedValue);
	}

	private int calculateValueSquare(Map<Coordinate, Integer> squares, Coordinate cursor) {
		int sum = 0;
		//left
		 sum += squares.getOrDefault(new Coordinate(cursor.x-1, cursor.y), 0);
		 //right
		 sum += squares.getOrDefault(new Coordinate(cursor.x+1, cursor.y), 0);
		 //up
		 sum += squares.getOrDefault(new Coordinate(cursor.x, cursor.y+1), 0);
		 //down
		 sum += squares.getOrDefault(new Coordinate(cursor.x, cursor.y-1), 0);
		 
		 //upright
		 sum += squares.getOrDefault(new Coordinate(cursor.x+1, cursor.y+1), 0);
		 //upleft
		 sum += squares.getOrDefault(new Coordinate(cursor.x-1, cursor.y+1), 0);
		 //botleft
		 sum += squares.getOrDefault(new Coordinate(cursor.x-1, cursor.y-1), 0);
		 //botright
		 sum += squares.getOrDefault(new Coordinate(cursor.x+1, cursor.y-1), 0);
		return sum;
	}

	private List<Spiraler> createSpiralers() {
		Spiraler rightSpiraler = new Spiraler(co-> co.x++, bound -> bound.right++, "right");
		Spiraler topSpiraler = new Spiraler(co-> co.y++, bound -> bound.top++, "top");
		Spiraler leftSpiraler = new Spiraler(co-> co.x--, bound -> bound.left--, "left");
		Spiraler botSpiraler = new Spiraler(co-> co.y--, bound -> bound.bottom--, "bot");
		
		List<Spiraler> spiralers = Arrays.asList(rightSpiraler, topSpiraler, leftSpiraler, botSpiraler);
		return spiralers;
	}

	private boolean outOfBounds(Coordinate cursor, Bound boundary) {
		if(cursor.x > boundary.right) {
			return true;
		} else if (cursor.x < boundary.left) {
			return true;
		} else if (cursor.y > boundary.top) {
			return true;
		} else if (cursor.y < boundary.bottom) {
			return true;
		}
		return false;
	}


	private static class Spiraler {
		public Consumer<Coordinate> coordinateAdvancer;
		public Consumer<Bound> boundAdvancer;
		private String name;

		public Spiraler(Consumer<Coordinate> coordinateAdvaner, Consumer<Bound> boundAdvancer, String name) {
			this.coordinateAdvancer = coordinateAdvaner;
			this.boundAdvancer = boundAdvancer;
			this.name = name;
		}
		
		@Override
		public String toString() {
			return name;
		}
	}
	
	private static class Coordinate {
		public int x, y;
		
		public Coordinate(int x, int y) {
			this.x = x; this.y = y;
		}
		
		//copy constructor
		public Coordinate(Coordinate copy) {
			this.x = copy.x;
			this.y = copy.y;
		}

		@Override
		public boolean equals(Object obj) {
			if(obj instanceof Coordinate) {
				Coordinate other = ((Coordinate)obj);
				return other.x == this.x && other.y == this.y;
			}
			return false;
		}
		@Override
		public int hashCode() {
			return this.x ^ this.y;
		}
	}
	
	private static class Bound {
		public int top, right, bottom, left;
	}

}

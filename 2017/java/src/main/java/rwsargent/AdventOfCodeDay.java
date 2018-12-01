package rwsargent;

import java.io.IOException;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public abstract class AdventOfCodeDay {
	
	public abstract void solveFirst();
	public void solveSecond() {
		// no op for first pass
	}
	
	public List<String> read(String filename) {
		URL url = getClass().getResource(filename);
		try {
			return Files.readAllLines(Paths.get(url.toURI()));
		} catch (IOException | URISyntaxException e) {
			throw new RuntimeException(e);
		}
	}
}

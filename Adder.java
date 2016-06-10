import java.nio.file.Path;
import java.nio.file.Paths;

class Adder {

  static {
    Path p = Paths.get("target/debug/libgeekout.so");
    System.load(p.toAbsolutePath().toString());
  }

  public static native int add(int v1, int v2);

  public static void main(String... args) {
    System.out.println("2 + 3 = " + Adder.add(2, 3));
  }

}

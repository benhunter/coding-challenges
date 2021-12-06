public class Twofer {
    public String twofer(String name) {
        if (name == null) {
            name = "you";
        }
        return "One for " + name + ", one for me.";
    }

    public static void main(String[] args) {
        System.out.println();
    }
}

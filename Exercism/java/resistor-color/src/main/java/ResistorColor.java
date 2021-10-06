import java.util.HashMap;
import java.util.Map;

import static java.util.Map.entry;

class ResistorColor {

    String[] colorList = {"black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"};

    int colorCode(String color) {
        return java.util.Arrays.asList(colorList).indexOf(color);
    }

    String[] colors() {
        return colorList;
        // return this.colorMap.keySet().toArray(String[]::new);
    }

    public static void main(String[] args) {
        ResistorColor rc = new ResistorColor();
        System.out.println(rc);
//        System.out.println(rc.colorMap);
//        System.out.println(rc.colorMap.entrySet());
//        System.out.println(rc.colorMap.keySet());
//        System.out.println(rc.colorMap.values());
    }

    /*
    Map<String, Integer> colorMap = Map.ofEntries(
            entry("black", 0),
            entry("brown", 1),
            entry("red", 2),
            entry("orange", 3),
            entry("yellow", 4),
            entry("green", 5),
            entry("blue", 6),
            entry("violet", 7),
            entry("grey", 8),
            entry("white", 9)
    );

    import static java.util.Map.entry;
Map<String, String> test2 = Map.ofEntries(
    entry("a", "b"),
    entry("c", "d")
);

Black: 0
Brown: 1
Red: 2
Orange: 3
Yellow: 4
Green: 5
Blue: 6
Violet: 7
Grey: 8
White: 9
*/
}

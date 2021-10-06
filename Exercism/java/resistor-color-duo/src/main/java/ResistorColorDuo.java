class ResistorColorDuo {

    String[] colorList = {"black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"};

    int colorCode(String color) {
        return java.util.Arrays.asList(colorList).indexOf(color);
    }

    String[] colors() {
        return colorList;
    }

    int value(String[] colors) {
        return java.util.Arrays.asList(colorList).indexOf(colors[0]) * 10 +
                java.util.Arrays.asList(colorList).indexOf((colors[1]));
    }
}

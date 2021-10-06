public class Hamming {
    private  String leftStrand;
    private  String rightStrand;

    public Hamming(String leftStrand, String rightStrand) {
        if (leftStrand == null || rightStrand == null) {
            throw new IllegalArgumentException();
        }
        if (leftStrand.length() == rightStrand.length()) {
            this.leftStrand = leftStrand;
            this.rightStrand = rightStrand;
        } else if (leftStrand.length() == 0) {
            throw new IllegalArgumentException("left strand must not be empty.");
        } else if (rightStrand.length() == 0) {
            throw new IllegalArgumentException("right strand must not be empty.");
        } else {
            throw new IllegalArgumentException("leftStrand and rightStrand must be of equal length.");
        }

    }

    public int getHammingDistance() {
        int distanceCount = 0;
        for (int i = 0; i < leftStrand.length(); i++) {
            if (leftStrand.charAt(i) != rightStrand.charAt(i)) {
                distanceCount += 1;
            }
        }
        return distanceCount;
    }
}

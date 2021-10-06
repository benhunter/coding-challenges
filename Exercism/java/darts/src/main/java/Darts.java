class Darts {
    double x;
    double y;

    Darts(double x, double y) {
        this.x = x;
        this.y = y;
    }

    int score() {
        double distFromCenter = Math.sqrt(this.x * this.x + this.y * this.y);
        int score = 0;

        if (distFromCenter <= 1D) {
            score = 10;
        } else if (distFromCenter <= 5D) {
            score = 5;
        } else if (distFromCenter <= 10D) {
            score = 1;
        } else {
            score = 0;
        }
        return score;
    }

}
